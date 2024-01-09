use std::usize;

use serde::{Deserialize, Serialize};

use super::{port::Port, connection::{self, Connection}};

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

    pub fn with_size(mut self, size: usize) -> Self {
        self.data = vec![0; size];
        self
    }

    pub fn with_id(mut self, id: usize) -> Self {
        self.id = Some(id);
        self
    }

    pub fn with_id_and_size(mut self, id: usize, size: usize) -> Self {
        self.id = Some(id);
        self.data = vec![0; size];
        self
    }

    pub fn new_with_size(size: usize) -> Self {
        Self {
            id: None,
            index: 0,
            data: vec![0; size],
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

    pub fn new_with_id_and_size(id: usize, size: usize) -> Self {
        Self {
            id: Some(id),
            index: 0,
            data: vec![0; size],
            index_port: Port::new(0),
            data_port: Port::new(0),
        }
    }

    pub fn connect_index_port(&mut self, connection: &mut Connection) {
        self.index_port.connect(connection);
    }

    pub fn connect_dart_port(&mut self, connection: &mut Connection) {
        self.data_port.connect(connection);
    }


    pub fn get_index_port_ref(&mut self) -> &mut Port {
        &mut self.index_port
    }

    pub fn get_data_port_ref(&mut self) -> &mut Port {
        &mut self.index_port
    }

    pub fn get_index_port(&self) -> Port {
        self.index_port.clone()
    }

    pub fn get_data_port(&self) -> Port {
        self.index_port.clone()
    }

    pub fn get_data_ref(&mut self) -> &Vec<i32> {
        &self.data
    }

    pub fn refresh(&mut self) {
        self.index = self.index_port.get().try_into().unwrap_or(0);
        self.data[self.index] = self.data_port.get();
    }
}
