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
    Connected(Arc<Mutex<i32>>, Option<usize>),
    Disconnected(i32),
}

impl Port {
    pub fn new(value: i32) -> Port {
        Port::Disconnected(value)
    }

    pub fn get_id(&self) -> Option<usize> {
        match self {
            Port::Connected(_, id) => id.clone(),
            Port::Disconnected(_) => None,
        }
    }

    pub fn get(&mut self) -> i32 {
        match self {
            Port::Connected(value, _) => *value.lock().unwrap(),
            Port::Disconnected(value) => *value,
        }
    }

    pub fn get_ref_mut(&mut self) -> &mut Self {
        self
    }

    pub fn set(&mut self, new_value: i32) {
        match self {
            Port::Connected(value, _) => *value.lock().unwrap() = new_value,
            Port::Disconnected(value) => *value = new_value,
        }
    }

    pub fn get_conn_id(&mut self) -> Option<usize> {
        match self {
            Port::Connected(_, id) => *id,
            Port::Disconnected(_) => None,
        }
    }

    pub fn connect(&mut self, connection: &mut Connection) {
        *self = Port::Connected(connection.get(), connection.get_id());
    }

    pub fn disconnect_and_unlist(&mut self, connection: &mut Connection) {
        let value = self.get();
        *self = Port::Disconnected(value);

    }
    pub fn disconnect(&mut self, connection: &mut Connection) {
        let value = self.get();
        *self = Port::Disconnected(value);
    }

}

impl PartialEq for Port {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Connected(l, _), Self::Connected(r, _)) => {
                l.lock().unwrap().clone() == r.lock().unwrap().clone()
            }
            (Self::Disconnected(l), Self::Disconnected(r)) => *l == *r,
            (Self::Connected(l, _), Self::Disconnected(r)) => l.lock().unwrap().clone() == *r,
            (Self::Disconnected(l), Self::Connected(r, _)) => *l == r.lock().unwrap().clone(),
            _ => false,
        }
    }
}

impl fmt::Display for Port {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = match self {
            Port::Connected(lock, _) => *lock.lock().unwrap(),
            Port::Disconnected(value) => *value,
        };
        match  self {
            Port::Connected(_, _) => write!(f, "C:{}", data),
            Port::Disconnected(_) => write!(f, "D:{}", data),
        }
        
    }
}
