use crate::witx::types::{SerialPort, UnopenedSerialPort};
use std::{
    collections::HashMap,
    hash::Hash,
    cell::RefCell,
};
use wiggle::GuestError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum WasiSerialError {
    #[error("guest error")]
    GuestError(#[from] GuestError),
    #[error("could not find filtered port")]
    UnableToFindPort,
    #[error("unable to open port")]
    UnableToOpenPort,
}

/// From the wasi-nn crate.
pub struct Table<K, V> {
    entries: HashMap<K, V>,
    next_key: u32,
}

impl<K, V> Default for Table<K, V> {
    fn default() -> Self {
        Self {
            entries: HashMap::new(),
            next_key: 0,
        }
    }
}

impl<K, V> Table<K, V>
where
    K: Eq + Hash + From<u32> + Copy,
{
    pub fn insert(&mut self, value: V) -> K {
        let key = self.use_next_key();
        self.entries.insert(key, value);
        key
    }

    pub fn remove(&mut self, key: K) -> Option<V> {
        self.entries.remove(&key)
    }

    pub fn get(&self, key: K) -> Option<&V> {
        self.entries.get(&key)
    }

    pub fn get_mut(&mut self, key: K) -> Option<&mut V> {
        self.entries.get_mut(&key)
    }

    fn use_next_key(&mut self) -> K {
        let current = self.next_key;
        self.next_key += 1;
        K::from(current)
    }
}

pub(crate) struct OpenedPort {
    pub info: PortInfo,
    pub port: Box<dyn serialport::SerialPort>,
}

pub(crate) struct PortInfo {
    pub name: String,
    pub vendor_id: u16,
    pub product_id: u16,
}

pub(crate) struct Ctx {
    pub opened_ports: Table<SerialPort, OpenedPort>,
    pub unopened_ports: Table<UnopenedSerialPort, PortInfo>,
}

pub struct WasiSerialCtx {
    pub(crate) ctx: RefCell<Ctx>
}

impl WasiSerialCtx {
    pub fn new() -> Self {
        Self {
            ctx: RefCell::new(Ctx {
                opened_ports: Table::default(),
                unopened_ports: Table::default(),
            }),
        }
    }
}
