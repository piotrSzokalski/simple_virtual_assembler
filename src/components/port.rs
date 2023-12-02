use serde::{
    de::{value, EnumAccess, SeqAccess, VariantAccess, Visitor},
    ser::SerializeTupleVariant,
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::sync::{Arc, Mutex};

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

    pub fn set(&mut self, new_value: i32) {
        match self {
            Port::Connected(value) => *value.lock().unwrap() = new_value,
            Port::Disconnected(value) => *value = new_value,
        }
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
