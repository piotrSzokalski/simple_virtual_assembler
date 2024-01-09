use core::fmt;
use serde::{
    de::{value, EnumAccess, SeqAccess, VariantAccess, Visitor},
    ser::SerializeTupleVariant,
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::sync::{Arc, Mutex};

use super::connection::{self, Connection};

/// Port used for communication between vm and other components
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Port {
    Connected(Arc<Mutex<i32>>),
    Disconnected(i32),
}

impl Port {
    pub fn new(value: i32) -> Port {
        Port::Disconnected(value)
    }

    pub fn get(&mut self) -> i32 {
        match self {
            Port::Connected(value) => *value.lock().unwrap(),
            Port::Disconnected(value) => *value,
        }
    }

    pub fn get_ref_mut(&mut self) -> &mut Self {
        self
    }

    pub fn set(&mut self, new_value: i32) {
        match self {
            Port::Connected(value) => *value.lock().unwrap() = new_value,
            Port::Disconnected(value) => *value = new_value,
        }
    }

    pub fn connect(&mut self, connection: &mut Connection) {
        *self = Port::Connected(connection.get());
    }
}

impl PartialEq for Port {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Connected(l), Self::Connected(r)) => {
                l.lock().unwrap().clone() == r.lock().unwrap().clone()
            }
            (Self::Disconnected(l), Self::Disconnected(r)) => *l == *r,
            (Self::Connected(l), Self::Disconnected(r)) => l.lock().unwrap().clone() == *r,
            (Self::Disconnected(l), Self::Connected(r)) => *l == r.lock().unwrap().clone(),
            _ => false,
        }
    }
}

impl fmt::Display for Port {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = match self {
            Port::Connected(lock) => *lock.lock().unwrap(),
            Port::Disconnected(value) => *value,
        };
        match  self {
            Port::Connected(_) => write!(f, "C:{}", data),
            Port::Disconnected(_) => write!(f, "D:{}", data),
        }
        
    }
}
