use serde::{de::SeqAccess, de::Visitor, Deserialize, Deserializer, Serialize, Serializer};
use std::sync::{Arc, Mutex};
/// Shared data used to connect vms, analogs to a wire connecting them
#[derive(Clone, Debug)]
pub struct Connection {
    data: Arc<Mutex<i32>>,
    id: usize,
    count: usize,
    drop: bool,
}

impl Connection {

    pub fn new(id: usize) -> Connection {
        Connection { data: Arc::new(Mutex::new(0)), id, count: 0, drop: false }
    }

    pub fn get(&mut self) -> (Arc<Mutex<i32>>, usize) {
        (self.data.clone(), self.id)
    }
    pub fn increment(&mut self) {
        self.count += 1;
    }
    pub fn decrement(&mut self) {
        self.count -= 1;
        if self.count == 0 {
            self.drop = true
        }
    }
}

impl Serialize for Connection {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.data.lock().unwrap().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Connection {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ConnectionVisitor;

        impl<'de> Visitor<'de> for ConnectionVisitor {
            type Value = Connection;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("an i32 inside an Arc<Mutex<>>")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let inner_value = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;

                Ok(Connection {
                    data: Arc::new(Mutex::new(inner_value)),
                    id: todo!(),
                    count: todo!(),
                    drop: todo!(),
                })
            }
        }

        deserializer.deserialize_seq(ConnectionVisitor)
    }
}
