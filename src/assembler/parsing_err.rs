use std::error;
use std::fmt;
use std::ops::IndexMut;
use std::string::ParseError;

use rust_i18n::t;

use crate::vm::{instruction::Instruction, opcodes::Opcode, operand::Operand};

/// Data contained in parsing error
#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq)]
pub struct ParsingErrorData {
    line: usize,
    message: String,
}

impl ParsingErrorData {
    fn new(line: usize, message: String) -> ParsingErrorData {
        ParsingErrorData { line, message }
    }
}

/// Represents errors that can occur while attempting to assemble code to SVA instruction
#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq)]
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
    StackNotPresent(ParsingErrorData),
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
            ParsingError::StackNotPresent(data) => data.clone(),
        }
    }

    pub fn get_message(&self) -> String {
        self.get_data().message
    }
}

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let error_type = match self {
            ParsingError::EmptyOperand(_) => t!("error.empty_operand"),
            ParsingError::InvalidPortNumber(_) => t!("error.invalid_port_number"),
            ParsingError::InvalidRegisterNumber(_) => t!("error.invalid_register_number"),
            ParsingError::InvalidOperandType(_) => t!("error.invalid_operand_type"),
            ParsingError::InvalidNumericLiteral(_) => t!("error.invalid_numeric_literal"),
            ParsingError::InvalidBinaryLiteral(_) => t!("error.invalid_binary_literal"),
            ParsingError::InvalidHexLiteral(_) => t!("error.invalid_hex_literal"),
            ParsingError::Empty(_) => t!("error.empty"),
            ParsingError::Unknown(_) => t!("error.unknown"),
            ParsingError::NotImplanted(_) => t!("error.not_implanted"),
            ParsingError::NotEnoughOperands(_) => t!("error.not_enough_operands"),
            ParsingError::TooManyOperands(_) => t!("error.too_many_operands"),
            ParsingError::StackNotPresent(_) => t!("error.stack_not_present"),
        };

        let error_data = self.get_data();
        // format of error message:
        // { parsing error } : { line } \t { error type } \t { details/message }
        write!(
            f,
            "{}:{}\t{}\t{}",
            t!("error.parsing"),
            error_data.line + 1,
            error_type,
            error_data.message
        )
    }
}

impl error::Error for ParsingError {}

#[cfg(test)]
mod tests {
    use crate::assembler::parsing_err::ParsingErrorData;

    use super::ParsingError;

    #[test]
    fn test_parsing_error_new() {
        let parsing_error =
            ParsingError::new(ParsingError::TooManyOperands, 123, "message".to_owned());

        assert_eq!(
            parsing_error,
            ParsingError::TooManyOperands(ParsingErrorData::new(123, "message".to_owned()))
        );
    }

    #[test]
    fn test_parsing_error_localization() {
        let parsing_error = ParsingError::new(
            ParsingError::TooManyOperands,
            123,
            "place_holder".to_owned(),
        );

        rust_i18n::set_locale("pl");

        let x = parsing_error.to_string();

        assert_eq!(
            "Błąd parsowania na lini:124\tZbyt wiele operandów\tplace_holder",
            parsing_error.to_string()
        );

        rust_i18n::set_locale("en");

        assert_eq!(
            "Parsing error at line:124\tToo many operands\tplace_holder",
            parsing_error.to_string()
        );
    }
}
