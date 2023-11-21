use std::error;
use std::fmt;
use std::ops::IndexMut;
use std::string::ParseError;

use rust_i18n::t;

use crate::vm::{instruction::Instruction, opcodes::Opcode, operand::Operand};

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct ParsingErrorData {
    line: usize,
    message: String,
}

impl ParsingErrorData {
    fn new(line: usize, message: String) -> ParsingErrorData {
        ParsingErrorData { line, message }
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum ParsingError {
    EmptyOperand(ParsingErrorData),
    InvalidPortNumber(ParsingErrorData),
    InvalidRegisterNumber(ParsingErrorData),
    InvalidOperandType(ParsingErrorData),
    InvalidNumericLiteral(ParsingErrorData),
    InvalidBinaryLiteral(ParsingErrorData),
    InvalidHexLiteral(ParsingErrorData),
    Empty(ParsingErrorData),
    Unknown(ParsingErrorData),
    NotImplanted(ParsingErrorData),
    NotEnoughOperands(ParsingErrorData),
    TooManyOperands(ParsingErrorData),
}

impl ParsingError {
    pub fn new(
        variant: fn(ParsingErrorData) -> ParsingError,
        line: usize,
        message: String,
    ) -> ParsingError {
        variant(ParsingErrorData::new(line, message))
    }

    pub fn get_data(&self) -> ParsingErrorData {
        match self {
            ParsingError::EmptyOperand(data) => data.clone(),
            ParsingError::InvalidPortNumber(data) => data.clone(),
            ParsingError::InvalidRegisterNumber(data) => data.clone(),
            ParsingError::InvalidOperandType(data) => data.clone(),
            ParsingError::InvalidNumericLiteral(data) => data.clone(),
            ParsingError::InvalidBinaryLiteral(data) => data.clone(),
            ParsingError::InvalidHexLiteral(data) => data.clone(),
            ParsingError::Empty(data) => data.clone(),
            ParsingError::Unknown(data) => data.clone(),
            ParsingError::NotImplanted(data) => data.clone(),
            ParsingError::NotEnoughOperands(data) => data.clone(),
            ParsingError::TooManyOperands(data) => data.clone(),
        }
    }

    pub fn get_message(&self) -> String {
        self.get_data().message
    }
}

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {}: {}",
            t!("error.parsing"),
            self.get_data().line,
            self.get_data().message
        )
    }
}

impl error::Error for ParsingError {}

#[cfg(test)]
mod tests {
    use super::ParsingError;

    #[test]
    fn test_parsing_error_new() {
        let parsing_error =
            ParsingError::new(ParsingError::TooManyOperands, 123, "message".to_owned());

        println!("{}", parsing_error);
    }

    #[test]
    fn test_parsing_error_localization() {
        let parsing_error =
            ParsingError::new(ParsingError::TooManyOperands, 123, "".to_owned());

        rust_i18n::set_locale("pl");

        println!("{}", parsing_error);

        rust_i18n::set_locale("en");
        println!("_____________________________________________--");
        println!("{}", parsing_error);

       
    }
}
