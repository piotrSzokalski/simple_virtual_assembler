use std::fmt::{self, Display, Formatter, Result};

/// Possible states of Flag register
#[derive(Debug, PartialEq, Eq, Clone, Copy, serde::Deserialize, serde::Serialize)]

pub enum Flag {
    ZERO,
    EQUAL,
    GREATER,
    LESSER,
}

impl fmt::Display for Flag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Flag::ZERO => write!(f, "ZERO"),
            Flag::EQUAL => write!(f, "EQUAL"),
            Flag::GREATER => write!(f, "GREATER"),
            Flag::LESSER => write!(f, "LESSER"),
        }
    }
}
