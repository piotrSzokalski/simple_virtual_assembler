use std::{collections::HashMap, error::Error};

use std::fmt;
use std::{error, result};

use crate::{
    instruction::{self, Instruction},
    opcodes::Opcode,
    operand::{self, Operand},
};

#[derive(Debug)]
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
}

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parsing error at line {}: {}", self.line, self.message)
    }
}

impl error::Error for ParsingError {}

pub struct Assembler {
    //translation: HashMap<&'static str, Opcode>,
    //instructions: Vec<String>,
}
impl Assembler {
    pub fn new() -> Assembler {
        Assembler {}
    }

    fn parse_operand(&mut self, operand_text: &str, line: usize) -> Result<Operand, ParsingError> {
        if operand_text.is_empty() {
            return Err(ParsingError::new("Empty operand", 0));
        }

        let first_char = operand_text.chars().nth(0).unwrap();
        let remaining_text = &operand_text[1..];

        match first_char {
            'r' => match remaining_text.parse::<usize>() {
                Ok(register_number) => {
                    if register_number > 3 {
                        Err(ParsingError::new("Invalid port number", line))
                    } else {
                    Ok(Operand::GeneralRegister(register_number.try_into().unwrap()))
                    }
                },
                _ => Err(ParsingError::new("Invalid register number", line)),
            },
            'p' => match remaining_text.parse::<usize>() {
                Ok(port_number) => {
                    if port_number > 3  {
                        Err(ParsingError::new("Invalid port number", line))
                    } else {
                        Ok(Operand::PortRegister(port_number))
                    }
                },
                _ => Err(ParsingError::new("Invalid port number", line)),
            },
            _ => match operand_text.parse::<i32>() {
                Ok(parsed_int) => Ok(Operand::IntegerValue(parsed_int)),
                _ => Err(ParsingError::new("Parsing error", line)),
            },
        }
    }

    // TEST
    // pub fn parse_operand2(&mut self, operand_text: String, line: usize) -> Result<Operand, ParsingError> {

    //     if let Some(first_char) = operand_text.chars().nth(0) {
    //         if first_char == 'r' { // General purpose register

    //              Err(ParsingError::new("NOT IMPLANTED", 0))
    //         } else if first_char == 'p' { // Port register

    //              Err(ParsingError::new("NOT IMPLANTED", 0))
    //         } else {
    //             // Integer value
    //             let integer_value = match operand_text.parse::<i32>() {
    //                 Ok(parsed_int) => Ok(Operand::IntegerValue(parsed_int)),
    //                 Err(_) => Err(ParsingError::new("Parsing error", 0)),
    //             };
    //             integer_value
    //         }
    //     } else {
    //         Err(ParsingError::new("Can't parse operand", 0))
    //     }
    // }

    // pub fn parse2(&mut self, program_text: String) -> Result<Vec<Instruction>, ParsingError> {
    //     let mut program: Vec<Instruction> = Vec::new();
    //     let mut error: Option<ParsingError> = None;

    //     let program_text_upper = program_text.to_ascii_uppercase();
    //     let lines = program_text_upper.lines();
    //     let mut current_line_number = 0;

    //     for line in lines {
    //         let words: Vec<&str> = line.split_whitespace().collect();
    //         let instruction_word = words[0];
    //         let operand_word = words[1].to_string();

    //         let instruction: Result<Instruction, ParsingError> = match instruction_word {
    //             "ADD" => !todo!(),
    //             "SUB" => {

    //                 let operand = self.parse_operand(operand_word, current_line_number);

    //                 match operand {
    //                     Ok(operand) => Ok(Instruction::new(Opcode::SUB(operand))),
    //                     Err(error) => Err(error),
    //                 }
    //             }
    //             "" => Err(ParsingError::new("Empty String", current_line_number)),
    //             _ => Err(ParsingError::new("Unwon error", current_line_number)),
    //         };
    //         match instruction {
    //             Ok(v) => program.push(v),
    //             Err(err) => {
    //                 error = Some(err);
    //                 break;
    //             },
    //         }
    //         current_line_number += 1;

    //     }
    //     let result = match error {
    //         Some(e) => Err(e),
    //         None => Ok(program),
    //     };
    //     result
    // }

    pub fn parse(&mut self, program_text: &str) -> Result<Vec<Instruction>, ParsingError> {
        let mut program: Vec<Instruction> = Vec::new();

        if program_text.is_empty() || program_text == "" {
            let err = ParsingError::new("Empty Program", 0);
            return Err(err);
        }

        for (current_line_number, line) in program_text.lines().enumerate() {
            let words: Vec<&str> = line.split_whitespace().collect();

            let instruction_word = words[0];
            let operand_word = words.get(1).unwrap_or(&"");

            let instruction = match instruction_word.to_ascii_uppercase().as_str() {
                "ADD" => Err(ParsingError::new(
                    "ADD not implemented",
                    current_line_number,
                )),
                "SUB" => {
                    let operand = self.parse_operand(operand_word, current_line_number)?;
                    Ok(Instruction::new(Opcode::SUB(operand)))
                }
                _ => Err(ParsingError::new("Unknown error", current_line_number)),
            };

            program.push(instruction?);
        }

        Ok(program)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parsing_operand_empty() {
        let mut assembler = Assembler::new();

        let result = assembler.parse_operand("", 0);

        assert!(result.is_err());

        let err = result.unwrap_err();
        println!("____________________________________");
        println!("{}", err);
        println!("____________________________________");
    }

    #[test]
    fn test_parsing_operand_correct_registers() {
        let mut assembler = Assembler::new();

        let registers = vec!["r0", "r1", "r2", "r3", "p0", "p1", "p2", "p3"];

        println!("_________________________________________________________");
        for register in registers.iter() {
            let result = assembler.parse_operand(&register, 0);

            assert!(result.is_ok());
            println!("{:?}", result.unwrap());

        }
        println!("_________________________________________________________");

    }

    #[test]
    fn test_parsing_operand_in_correct_registers() {
        let mut assembler = Assembler::new();

        let registers = vec!["r-1",  "r4", "rp", "pr", "p-3, p10"];

        for register in registers.iter() {
            let result = assembler.parse_operand(&register, 0);

           

            assert!(result.is_err());
        }
    }

    #[test]
    fn test_parsing_operand() {
        let mut assembler = Assembler::new();

        let registers = vec!["r0", "r1", "r2", "r3", "p0", "p1", "p2", "p3"];

        for register in registers.iter() {
            let result = assembler.parse_operand(&register, 0);

            assert!(result.is_ok());

            // let operand = result.unwrap();

            // assert_eq!(operand, Operand::GeneralRegister(0));
        }
    }

    #[test]
    fn test_parsing_empty_string() {
        let program_text = "";

        let mut assembler = Assembler::new();

        let result = assembler.parse(program_text);

        assert!(result.is_err());

        let err = result.unwrap_err();
        println!("____________________________________");
        println!("{}", err);
        println!("____________________________________");

        assert_eq!(err.message, "Empty Program")
    }
}
