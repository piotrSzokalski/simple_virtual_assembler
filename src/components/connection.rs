use serde::{de::SeqAccess, de::Visitor, Deserialize, Deserializer, Serialize, Serializer};
use std::sync::{Arc, Mutex};
/// Shared data used to connect vms, analogs to a wire connecting them
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Connection {
    data: Arc<Mutex<i32>>,
    /// optional, helper for ui app
    /// ids of ports connected
    ports: Vec<String>,
}

impl Connection {

    pub fn new() -> Connection {
        Connection { data: Arc::new(Mutex::new(0)), ports: Vec::new() }
    }

    pub fn get(&mut self) -> Arc<Mutex<i32>> {
        self.data.clone()
    }
    /// Helper to manage connections
    /// Mainly to rebuild connections in process of deserialization
    /// 
    /// # Arguments
    /// 
    /// * id String - arbitrary name used to identify connected port
    pub fn add_port_id(&mut self,id: String) {
        self.ports.push(id);
    }

    pub fn remove_port_id(&mut self, id: String) {
        self.ports.retain(|_id| _id != &id); 
    }
}

// impl Serialize for Connection {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         self.data.lock().unwrap().serialize(serializer)
//     }
// }

// impl<'de> Deserialize<'de> for Connection {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         struct ConnectionVisitor;

//         impl<'de> Visitor<'de> for ConnectionVisitor {
//             type Value = Connection;

//             fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
//                 formatter.write_str("an i32 inside an Arc<Mutex<>>")
//             }

//             fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
//             where
//                 A: SeqAccess<'de>,
//             {
//                 let inner_value = seq
//                     .next_element()?
//                     .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;

//                 Ok(Connection {
//                     data: Arc::new(Mutex::new(inner_value)),
//                 })
//             }
//         }

//         deserializer.deserialize_seq(ConnectionVisitor)
//     }
// }

mod test {
    use super::*;


}