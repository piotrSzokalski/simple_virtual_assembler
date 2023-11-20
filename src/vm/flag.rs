use std::fmt::{self, Display, Formatter, Result};

/// Possible states of Flag register
#[derive(Debug, PartialEq, Eq, Clone, Copy, serde::Deserialize, serde::Serialize)]

pub enum Flag {
    EQUAL = -1,
    GREATER = 0,
    LESSER = 1,
}

impl fmt::Display for Flag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Flag::EQUAL => write!(f, "EQUAL"),
            Flag::GREATER => write!(f, "GREATER"),
            Flag::LESSER => write!(f, "LESSER"),
        }
    }
}
