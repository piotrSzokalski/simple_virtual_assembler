use std::fmt::{self, Display, Formatter, Result};

use serde::de::value;

/// Operand, integer or register
#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]

pub enum Operand {
    /// Integer value
    IntegerValue(i32),
    /// General register address/index r0, r1, r2, r3
    GeneralRegister(usize),
    /// Port register used for I/O p0, p1, p2, p3
    PortRegister(usize),
    /// Accumulator register
    ACC,
    /// Program counter register
    PC,
}

impl Operand {
    /// Creates an Operand variant with an IntegerValue.
    pub fn integer(value: i32) -> Operand {
        Operand::IntegerValue(value)
    }

    // Creates an Operand variant with a REGISTER.
    // pub fn register(reg_num: Register) -> Operand {
    //     Operand::REGISTER(reg_num)
    // }
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Operand::IntegerValue(value) => write!(f, "{}", value),
            Operand::GeneralRegister(register) => write!(f, "r:{}", register),
            Operand::PortRegister(port) => write!(f, "p:{}", port),
            Operand::ACC => write!(f, "acc"),
            Operand::PC => write!(f, "pc"),
        }
    }
}
