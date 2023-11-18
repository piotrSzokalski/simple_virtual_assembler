use std::fmt::{Display, Formatter, Result, self};

/// Possible states of Flag register
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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