use crate::ctx::{WasiSerialCtx, WasiSerialError};

wiggle::from_witx!({
    witx: ["$WASI_ROOT/phases/ephemeral/witx/wasi_ephemeral_serial.witx"],
    ctx: WasiSerialCtx,
    errors: { errno => WasiSerialError }
});

use types::Errno;

impl types::GuestErrorConversion for WasiSerialCtx {
    fn into_errno(&self, e: wiggle::GuestError) -> Errno {
        unimplemented!("Guest error: {:?}", e)
    }
}

impl wiggle::GuestErrorType for Errno {
    fn success() -> Self {
        Errno::Success
    }
}

impl<'a> types::UserErrorConversion for WasiSerialCtx {
    fn errno_from_wasi_serial_error(&self, e: WasiSerialError) -> Errno {
        unimplemented!("Host error: {:?}", e)
    }
}
