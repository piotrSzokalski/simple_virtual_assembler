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
        Connection {
            data: Arc::new(Mutex::new(0)),
            ports: Vec::new(),
        }
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
    pub fn add_port_id(&mut self, id: String) {
        self.ports.push(id);
    }

    pub fn remove_port_id(&mut self, id: String) {
        self.ports.retain(|_id| _id != &id);
    }

    pub fn get_connected_ports_ids(&self) -> &Vec<String> {
        &self.ports
    }

    pub fn get_connected_vms_and_ports(&mut self, delimiter: char) -> Vec<(i32, usize)> {
        let x: Vec<(i32, usize)> = self
            .ports
            .iter()
            .map(|id| {
                let split = id.split(delimiter).collect::<Vec<&str>>();
                let vm_id = split[0].parse::<i32>().unwrap();
                let port_index = split[1].parse::<usize>().unwrap();
                (vm_id, port_index)
            })
            .collect();
        x
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

    #[test]
    fn test_getting_connected_vms_and_ports_list() {
        let port_ids = Vec::from([
            "0P0", "0P1", "0P2", "0P3", "1P0", "1P1", "1P2", "1P3", "1P3", //
            "10P3", "9993P3",
        ]);
        let mut connection = Connection::new();
        for id in port_ids.iter() {
            connection.add_port_id(id.to_string());
        }
        let result = connection.get_connected_vms_and_ports('P');
        print!("{:?}", result);
    }
}
