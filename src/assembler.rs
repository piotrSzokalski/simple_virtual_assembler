use std::error;
use std::fmt;

use crate::{instruction::Instruction, opcodes::Opcode, operand::Operand};

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

    fn parse_operand(
        &mut self,
        operand_text: &str,
        line: usize,
        register_only: bool,
    ) -> Result<Operand, ParsingError> {
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
                        Ok(Operand::GeneralRegister(
                            register_number.try_into().unwrap(),
                        ))
                    }
                }
                _ => Err(ParsingError::new("Invalid register number", line)),
            },
            'p' => match remaining_text.parse::<usize>() {
                Ok(port_number) => {
                    if port_number > 3 {
                        Err(ParsingError::new("Invalid port number", line))
                    } else {
                        Ok(Operand::PortRegister(port_number))
                    }
                }
                _ => Err(ParsingError::new("Invalid port number", line)),
            },
            _ => match operand_text.parse::<i32>() {
                Ok(parsed_int) => {
                    if register_only {
                        Err(ParsingError::new("This operand can't be integer ", line))
                    } else {
                        Ok(Operand::IntegerValue(parsed_int))
                    }
                }
                Err(v) => Err(ParsingError::new("Can't parse integer", line)),
            },
        }
    }

    pub fn parse(&mut self, program_text: &str) -> Result<Vec<Instruction>, ParsingError> {
        let program_text = program_text.trim();

        if program_text.is_empty() {
            return Err(ParsingError::new("Empty Program", 0));
        }

        let program: Result<Vec<Instruction>, ParsingError> = program_text
            .lines()
            .enumerate()
            .map(|(current_line_number, line)| self.parse_instruction(line, current_line_number))
            .collect();

        program
    }

    fn parse_label(&mut self, label: &str, line: usize) -> Result<Instruction, ParsingError> {
        Err(ParsingError::new("Unknown error", line))
    }

    fn parse_instruction(
        &mut self,
        line: &str,
        current_line_number: usize,
    ) -> Result<Instruction, ParsingError> {
        let words: Vec<&str> = line.split_whitespace().collect();
        let instruction_word = words[0];
        let operands = &words[1..];

        let instruction = match instruction_word.to_ascii_uppercase().as_str() {
            "MOV" => self.parse_binary_instruction(Opcode::MOV, operands, current_line_number),

            "ADD" => self.parse_unary_instruction(Opcode::ADD, operands, current_line_number),
            "SUB" => self.parse_unary_instruction(Opcode::SUB, operands, current_line_number),
            "MUL" => self.parse_unary_instruction(Opcode::MUL, operands, current_line_number),
            "DIV" => self.parse_unary_instruction(Opcode::DIV, operands, current_line_number),
            "MOD" => self.parse_unary_instruction(Opcode::MOD, operands, current_line_number),

            "AND" => self.parse_unary_instruction(Opcode::AND, operands, current_line_number),
            "OR" => self.parse_unary_instruction(Opcode::OR, operands, current_line_number),
            "XOR" => self.parse_unary_instruction(Opcode::XOR, operands, current_line_number),
            "NOT" => Err(ParsingError::new("NOT IMPLEMENTED", current_line_number)),

            "JE" => self.parse_unary_instruction(Opcode::SUB, operands, current_line_number),
            "JL" => self.parse_unary_instruction(Opcode::SUB, operands, current_line_number),
            "JG" => self.parse_unary_instruction(Opcode::SUB, operands, current_line_number),

            "HLT" => Ok(Instruction::new(Opcode::HLT)),
            "NOP" => Err(ParsingError::new("NOT IMPLEMENTED", current_line_number)),
            "SPL" => Err(ParsingError::new("NOT IMPLEMENTED", current_line_number)),
            label => self.parse_label(label, current_line_number), //_ => Err(ParsingError::new("Unknown error", current_line_number)),
        };

        instruction
    }

    fn parse_binary_instruction(
        &mut self,
        opcode: fn(Operand, Operand) -> Opcode,
        operands: &[&str],
        current_line_number: usize,
    ) -> Result<Instruction, ParsingError> {
        if operands.len() != 2 {
            return Err(ParsingError::new(
                "Binary instructions requires exactly 2 operands",
                current_line_number,
            ));
        }
        let operand1 = self.parse_operand(operands[0], current_line_number, false)?;
        let operand2 = self.parse_operand(operands[1], current_line_number, true)?;
        Ok(Instruction::new(opcode(operand1, operand2)))
    }

    fn parse_unary_instruction(
        &mut self,
        opcode: fn(Operand) -> Opcode,
        operands: &[&str],
        current_line_number: usize,
    ) -> Result<Instruction, ParsingError> {
        if operands.len() != 1 {
            return Err(ParsingError::new(
                "Single operand instruction requires exactly 1 operand",
                current_line_number,
            ));
        }

        let operand = self.parse_operand(operands[0], current_line_number, false)?;
        Ok(Instruction::new(opcode(operand)))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parsing_operand_empty() {
        let mut assembler = Assembler::new();

        let result = assembler.parse_operand("", 0, false);

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
            let result = assembler.parse_operand(&register, 0, false);

            assert!(result.is_ok());
            println!("{:?}", result.unwrap());
        }
        println!("_________________________________________________________");
    }

    #[test]
    fn test_parsing_operand_in_correct_registers() {
        let mut assembler = Assembler::new();

        let registers = vec!["r-1", "r4", "rp", "pr", "p-3, p10"];

        for register in registers.iter() {
            let result = assembler.parse_operand(&register, 0, false);

            assert!(result.is_err());
        }
    }

    #[test]
    fn test_parsing_operand() {
        let mut assembler = Assembler::new();

        let registers = vec!["r0", "r1", "r2", "r3", "p0", "p1", "p2", "p3"];

        for register in registers.iter() {
            let result = assembler.parse_operand(&register, 0, false);

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

    #[test]
    fn test_parsing_sub() {
        let program_text = r#"
            SUB 10
            SUB r1
            SUB p2
            SUB -100
        "#;

        let mut assembler = Assembler::new();

        let result = assembler.parse(program_text);

        assert!(result.is_ok());

        let program = result.unwrap();

        let expected = vec![
            Instruction::new(Opcode::SUB(Operand::IntegerValue(10))),
            Instruction::new(Opcode::SUB(Operand::GeneralRegister(1))),
            Instruction::new(Opcode::SUB(Operand::PortRegister(2))),
            Instruction::new(Opcode::SUB(Operand::IntegerValue(-100))),
        ];

        assert_eq!(program, expected);
    }

    #[test]
    fn test_parsing_sub_inlaid_register() {
        println!("__________HERE__________");
        let program_text = r#"
            SUB 10
            SUB r1
            SUB p4
        "#;

        let mut assembler = Assembler::new();

        let result = assembler.parse(program_text);

        assert!(result.is_err());
    }
    #[test]
    fn test_parsing_sub_invalid_num_of_parameters() {
        let program_text = r#"
        SUB 2 p1
    "#;

        let mut assembler = Assembler::new();

        let result = assembler.parse(program_text);

        assert!(result.is_err());

        let err = result.unwrap_err();

        println!("{}", err);
    }

    #[test]
    fn test_parsing_mov_invalid_num_parameters() {
        let program_text = r#"
            MOV 10
        "#;

        let mut assembler = Assembler::new();

        let result = assembler.parse(program_text);

        assert!(result.is_err());

        let err = result.unwrap_err();

        println!("{}", err);
    }
}
