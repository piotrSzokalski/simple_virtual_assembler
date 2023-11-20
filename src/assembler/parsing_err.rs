use std::fmt;
use std::error;
use std::ops::IndexMut;

use crate::vm::{instruction::Instruction, opcodes::Opcode, operand::Operand};
#[derive(Debug, serde::Deserialize, serde::Serialize)]

pub struct ParsingError {
    line: usize,
    message: String,
}

impl ParsingError {
    pub fn new(message: &str, line: usize) -> ParsingError {
        ParsingError {
            line: line,
            message: message.to_string(),
        }
    }

    pub fn get_message(&self) -> String {
        self.message.clone()
    }
}

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parsing error at line {}: {}", self.line, self.message)
    }
}

impl error::Error for ParsingError {}
