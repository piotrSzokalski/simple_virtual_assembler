use std::usize;

use serde::{Deserialize, Serialize};

use super::{
    connection::{self, Connection},
    port::Port,
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Ram {
    id: Option<usize>,
    index: usize,
    data: Vec<i32>,
    index_port: Port,
    data_port: Port,
    mode: Port,
}

impl Ram {
    pub fn new() -> Self {
        Self {
            id: None,
            index: 0,
            data: vec![0; 32],
            index_port: Port::new(0),
            data_port: Port::new(0),
            mode: Port::new(0),
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

    pub fn connect_index_port(&mut self, connection: &mut Connection) {
        self.index_port.connect(connection);
    }

    pub fn connect_data_port(&mut self, connection: &mut Connection) {
        self.data_port.connect(connection);
    }

    pub fn disconnect_index_port(&mut self) {
        let value = match &self.index_port {
            Port::Connected(v, e) => *v.lock().unwrap(),
            Port::Disconnected(v) => *v,
        };

        self.index_port = Port::Disconnected(value);
    }

    pub fn disconnect_data_port(&mut self) {
        let value = match &self.data_port {
            Port::Connected(v, e) => *v.lock().unwrap(),
            Port::Disconnected(v) => *v,
        };

        self.data_port = Port::Disconnected(value);
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
        self.data_port.clone()
    }

    pub fn get_data_ref(&mut self) -> &Vec<i32> {
        &self.data
    }

    pub fn refresh(&mut self) {
        self.index = self.index_port.get().try_into().unwrap_or(0);
        if self.index >= self.data.len() {
           self.index = self.data.len() - 1;
        }
        if self.mode.get() == 0 {
            self.data[self.index] = self.data_port.get();
        } else {
            self.data_port.set(self.data[self.index]);
        }
    }
}
