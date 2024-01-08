use serde::{Serialize, Deserialize};

use super::port::Port;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Ram {
    id: Option<usize>,
    index: usize,
    data: Vec<i32>,
    index_port: Port,
    data_port: Port,
}

impl Ram {
    pub fn new() -> Self {
        Self {
            id: None,
            index: 0,
            data: vec![0; 32],
            index_port: Port::new(0),
            data_port: Port::new(0),
        }
    }
    pub fn new_with_id(id: usize) -> Self {
        Self {
            id: Some(id),
            index: 0,
            data: vec![0; 32],
            index_port: Port::new(0),
            data_port: Port::new(0),
        }
    }

    pub fn get_data_ref(&mut self) -> &Vec<i32> {
        &self.data
    }

    pub fn refresh(&mut self) {
        self.index = self.index_port.get().try_into().unwrap_or(0);
        self.data[self.index] = self.data_port.get();
    }
}

