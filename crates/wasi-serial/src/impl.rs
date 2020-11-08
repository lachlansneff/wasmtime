use crate::{
    ctx::{WasiSerialCtx, WasiSerialError, PortInfo, OpenedPort},
    witx::{
        wasi_ephemeral_serial::WasiEphemeralSerial,
        types::{SerialPort, UnopenedSerialPort, FilterOptions, OpenOptions, FlowControl, ParityType},
    },
};
use std::convert::TryInto;
use wiggle::GuestPtr;
use serialport::{self, SerialPortType, SerialPortSettings, DataBits, Parity, StopBits};

impl WasiEphemeralSerial for WasiSerialCtx {
    fn request_port<'a>(
        &self,
        filters: &GuestPtr<'a, [FilterOptions]>,
    ) -> Result<UnopenedSerialPort, WasiSerialError> {
        let filters = filters.as_slice()?;
        let available_ports = serialport::available_ports().unwrap(); // TODO: convert errors later.

        let found = available_ports.iter().find_map(|info| match info.port_type {
            SerialPortType::UsbPort(ref usb_info) => {
                filters.iter().find_map(|filter| {
                    let matches =
                        (filter.vendor_id == u32::MAX || filter.vendor_id == usb_info.vid as u32)
                        && (filter.product_id == u32::MAX || filter.product_id == usb_info.pid as u32);
                    if matches {
                        Some(PortInfo {
                            name: info.port_name.clone(),
                            vendor_id: usb_info.vid,
                            product_id: usb_info.pid,
                        })
                    } else {
                        None
                    }
                })
            },
            // we don't support anything else
            _ => None,
        });

        if let Some(port) = found {
            let mut ctx = self.ctx.borrow_mut();

            Ok(ctx.unopened_ports.insert(port))
        } else {
            Err(WasiSerialError::UnableToFindPort)
        }
    }

    fn open_port(
        &self,
        unopened_port: UnopenedSerialPort,
        open_options: &OpenOptions,
    ) -> Result<SerialPort, WasiSerialError> {
        let mut ctx = self.ctx.borrow_mut();

        let port = {
            let port_info = ctx.unopened_ports.get(unopened_port).unwrap();

            let settings = SerialPortSettings {
                baud_rate: open_options.baud_rate,
                data_bits: match open_options.data_bits {
                    5 => DataBits::Five,
                    6 => DataBits::Six,
                    7 => DataBits::Seven,
                    8 => DataBits::Eight,
                    _ => return Err(WasiSerialError::GuestError(wiggle::GuestError::InvalidEnumValue("data_bits"))),
                },
                flow_control: match open_options.flow_control {
                    FlowControl::None => serialport::FlowControl::None,
                    FlowControl::Hardware => serialport::FlowControl::Hardware,
                },
                parity: match open_options.parity {
                    ParityType::None => Parity::None,
                    ParityType::Even => Parity::Even,
                    ParityType::Odd => Parity::Odd,
                    _ => unimplemented!(),
                },
                stop_bits: match open_options.stop_bits {
                    1 => StopBits::One,
                    2 => StopBits::Two,
                    _ => panic!("invalid value"),
                },
                .. SerialPortSettings::default()
            };

            serialport::open_with_settings(&port_info.name, &settings)
                .map_err(|_| WasiSerialError::UnableToOpenPort)?
        };

        let info = ctx.unopened_ports.remove(unopened_port).unwrap();

        Ok(ctx.opened_ports.insert(OpenedPort {
            info,
            port,
        }))
    }

    fn write<'a>(
        &self,
        port: SerialPort,
        data: &GuestPtr<'a, u8>,
        size: u32,
    ) -> Result<u32, WasiSerialError> {
        let slice = data.as_array(size).as_slice()?;

        let mut ctx = self.ctx.borrow_mut();
        let opened_port = ctx.opened_ports.get_mut(port).unwrap();

        Ok(opened_port.port.write(&slice).unwrap().try_into().unwrap())
    }

    fn read<'a>(
        &self,
        port: SerialPort,
        buffer: &GuestPtr<'a, u8>,
        buffer_size: u32,
    ) -> Result<u32, WasiSerialError> {
        let mut slice = buffer.as_array(buffer_size).as_slice()?;

        let mut ctx = self.ctx.borrow_mut();
        let opened_port = ctx.opened_ports.get_mut(port).unwrap();

        Ok(opened_port.port.read(&mut slice).unwrap().try_into().unwrap())
    }
}
