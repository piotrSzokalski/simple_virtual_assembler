use serde::{
    de::{value, EnumAccess, SeqAccess, VariantAccess, Visitor},
    ser::SerializeTupleVariant,
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::{sync::{Arc, Mutex}, path::Display, fmt::{self, write}};

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

// impl Serialize for Port {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         match self {
//             Port::Connected(connected) => {
//                 let connected_value = connected.lock().unwrap(); // You may want to handle the lock result more gracefully
//                 serializer.serialize_newtype_variant("Port", 0, "Connected", &connected_value.clone())
//             }
//             Port::Disconnected(disconnected) => {
//                 serializer.serialize_newtype_variant("Port", 1, "Disconnected", disconnected)
//             }
//         }
//     }
// }
//TODO:
// impl<'de> Deserialize<'de> for Port {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         todo!()
//     }
// }

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

// impl fmt::Display for Port {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             Port::Connected(value) => write!(f, "{}", value.lock().unwrap()),
//             Port::Disconnected(value) => write!(f, "{}", value.lock().unwrap()),
//         }
        
//     }
// }

mod test {
    use super::*;

    #[test]
    fn test_serialization() {
        let mut port1 = Port::new(0);
        let mut port2 = Port::new(1);

        port1.connect_port(&mut port2);
        port2.set_value(12);

        let x = serde_json::to_string_pretty(&port1);

        print!("{:?}", x.unwrap());
    }


    #[derive(Serialize)]
    struct mu {
        x: Arc<i32>
    }
}
