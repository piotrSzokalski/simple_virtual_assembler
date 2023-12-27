use serde::{
    de::{value, EnumAccess, SeqAccess, VariantAccess, Visitor},
    ser::SerializeTupleVariant,
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::sync::{Arc, Mutex};

use super::connection::{self, Connection};

/// Port used for communication between vm and other components
#[derive(Clone, Debug)]
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

    pub fn set_value(&mut self, new_value: i32) {
        match self {
            Port::Connected(value) => *value.lock().unwrap() = new_value,
            Port::Disconnected(value) => *value = new_value,
        }
    }

    pub fn connect(&mut self, connection: &mut Connection) {
        *self = Port::Connected(connection.get());
    }

    pub fn connect_port(&mut self, port: &mut Port) {
        match (self.clone(), port.clone()) {
            (Port::Connected(v1), Port::Connected(v2)) => *self = Port::Connected(v2.clone()),
            (Port::Connected(v1), Port::Disconnected(v2)) => port.set_to_shared(v1.clone()),
            (Port::Disconnected(v1), Port::Connected(v2)) => *self = Port::Connected(v2.clone()),
            (Port::Disconnected(v1), Port::Disconnected(v2)) => {
                let shared = Arc::new(Mutex::new(v1));
                *self = Port::Connected(shared.clone());
                port.set_to_shared(shared.clone());
            }
        }
    }



    pub fn set_to_shared(&mut self, shared_data: Arc<Mutex<i32>>) {
        *self = Port::Connected(shared_data)
    }
}

//TODO:
impl Serialize for Port {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        todo!()
    }
}
//TODO:
impl<'de> Deserialize<'de> for Port {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        todo!()
    }
}

//FIXME:
impl PartialEq for Port {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Connected(l0), Self::Connected(r0)) => false,
            (Self::Disconnected(l0), Self::Disconnected(r0)) => false,
            _ => false,
        }
    }
}
