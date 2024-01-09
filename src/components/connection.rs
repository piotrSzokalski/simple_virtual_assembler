use serde::{de::SeqAccess, de::Visitor, Deserialize, Deserializer, Serialize, Serializer};
use std::sync::{Arc, Mutex};
/// Shared data used to connect vms, analogs to a wire connecting them
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Connection {
    data: Arc<Mutex<i32>>,

    /// ids of ports connected
    ///
    /// optional, helper for ui app
    ports: Vec<String>,
    /// Optional id of port
    id: Option<usize>,
}

impl Connection {
    pub fn new() -> Connection {
        Connection {
            data: Arc::new(Mutex::new(0)),
            ports: Vec::new(),
            id: None,
        }
    }

    pub fn new_with_id(id: usize) -> Self {
        Connection {
            data: Arc::new(Mutex::new(0)),
            ports: Vec::new(),
            id: Some(id),
        }
    }

    pub fn get_id(&self) -> Option<usize> {
        self.id
    }

    pub fn get(&self) -> Arc<Mutex<i32>> {
        self.data.clone()
    }

    /// Ads port id to the list of connected ports
    ///
    /// Helper to manage connections
    ///
    /// Mainly to rebuild connections in process of deserialization
    ///
    /// # Arguments
    ///
    /// * id String - arbitrary name used to identify connected port
    pub fn add_port_id(&mut self, id: String) {
        self.ports.push(id);
    }

    /// Removes port id form the list of connected ports
    ///
    /// Helper to manage connections
    ///
    /// Mainly to rebuild connections in process of deserialization
    ///
    /// # Arguments
    ///
    /// * id String - arbitrary name used to identify connected port
    pub fn remove_port_id(&mut self, id: String) {
        self.ports.retain(|_id| _id != &id);
    }

    /// Gets ids to ports connected to connection
    ///
    /// Helper to manage connections
    ///
    /// Mainly to rebuild connections in process of deserialization
    ///
    /// # Arguments
    ///
    /// * id String - arbitrary name used to identify connected port
    pub fn get_connected_ports_ids(&self) -> &Vec<String> {
        &self.ports
    }

    /// Gets ids of vms and index of port connected to connection
    ///
    /// Helper to manage connections
    ///
    /// Mainly to rebuild connections in process of deserialization
    ///
    /// # Arguments
    ///
    /// * id String - arbitrary name used to identify connected port
    pub fn get_connected_vms_and_ports(&mut self, delimiter: char) -> Vec<(i32, usize)> {
        let x: Vec<(i32, usize)> = self
            .ports
            .iter()
            .filter(|id| id.starts_with("R"))
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
