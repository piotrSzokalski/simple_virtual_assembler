use std::error;
use std::fmt;
use std::ops::IndexMut;

use super::super::language::Language;

use crate::vm::{instruction::Instruction, opcodes::Opcode, operand::Operand};

use super::parsing_err::ParsingError;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Assembler {}
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
            return Err(ParsingError::new(
                ParsingError::EmptyOperand,
                line,
                "".to_string(),
            ));
        }
        let remaining_text = &operand_text[1..];

        match operand_text {
            "acc" => {
                return Ok(Operand::ACC);
            }
            "pc" => {
                return Ok(Operand::PC);
            }
            r if r.starts_with('r') => {
                if let Ok(index) = remaining_text.parse::<usize>() {
                    if index > 3 {
                        return Err(ParsingError::new(
                            ParsingError::InvalidPortNumber,
                            line,
                            "".to_string(),
                        ));
                    }
                    return Ok(Operand::GeneralRegister(index.try_into().unwrap()));
                }
                return Err(ParsingError::new(
                    ParsingError::InvalidPortNumber,
                    line,
                    "".to_string(),
                ));
                //return Err(ParsingError::new("Invalid port number", line));
            }
            p if p.starts_with('p') => {
                if let Ok(index) = remaining_text.parse::<usize>() {
                    if index > 3 {
                        return Err(ParsingError::new(
                            ParsingError::InvalidPortNumber,
                            line,
                            "".to_string(),
                        ));
                        //return Err(ParsingError::new("Invalid port number", line));
                    }
                    return Ok(Operand::PortRegister(index.try_into().unwrap()));
                }
                return Err(ParsingError::new(
                    ParsingError::InvalidPortNumber,
                    line,
                    "".to_string(),
                ));
                //return Err(ParsingError::new("Invalid port number", line));
            }

            decimal
                if decimal
                    .chars()
                    .all(|c| c.is_numeric() || (c == '-' && decimal.starts_with('-'))) =>
            {
                if register_only {
                    return Err(ParsingError::new(
                        ParsingError::InvalidOperandType,
                        line,
                        "".to_string(),
                    ));
                    // return Err(ParsingError::new(
                    //     "Can't parse numeric, this operand must be a register",
                    //     line,
                    // ));
                }
                if let Ok(decimal) = decimal.parse::<i32>() {
                    return Ok(Operand::IntegerValue(decimal));
                }
                return Err(ParsingError::new(
                    ParsingError::InvalidNumericLiteral,
                    line,
                    "".to_string(),
                ));
                // return Err(ParsingError::new(
                //     "Can't parse numeric literal operand",
                //     line,
                // ));
            }
            binary if binary.starts_with("0b") => {
                if register_only {
                    return Err(ParsingError::new(
                        ParsingError::InvalidBinaryLiteral,
                        line,
                        "".to_string(),
                    ));
                    // return Err(ParsingError::new(
                    //     "Can't parse binary literal, this operand must be a register",
                    //     line,
                    // ));
                }
                if let Ok(binary) = i32::from_str_radix(&binary[2..], 2) {
                    return Ok(Operand::IntegerValue(binary));
                }
                return Err(ParsingError::new(
                    ParsingError::InvalidBinaryLiteral,
                    line,
                    "".to_string(),
                ));
                //return Err(ParsingError::new("Can't binary literal operand", line));
            }

            hex if hex.starts_with("0x") => {
                if register_only {
                    return Err(ParsingError::new(
                        ParsingError::InvalidHexLiteral,
                        line,
                        "".to_string(),
                    ));
                    // return Err(ParsingError::new(
                    //     "Can't parse hexadecimal literal, this operand must be a register",
                    //     line,
                    // ));
                }
                if let Ok(hex) = i32::from_str_radix(&hex[2..], 16) {
                    return Ok(Operand::IntegerValue(hex));
                }
                return Err(ParsingError::new(
                    ParsingError::InvalidHexLiteral,
                    line,
                    "".to_string(),
                ));
                //return Err(ParsingError::new("Can't hexadecimal literal operand", line));
            }

            _ => {
                return Err(ParsingError::new(
                    ParsingError::InvalidOperandType,
                    line,
                    "".to_string(),
                ))
            }
            // Err(ParsingError::new(
            //     "Can't parse operand, unknown operand",
            //     line,
            // )),
        }
    }

    pub fn parse(&mut self, program_text: &str) -> Result<Vec<Instruction>, ParsingError> {
        let program_text = program_text.trim();

        if program_text.is_empty() {
            return Err(ParsingError::new(ParsingError::Empty, 0, "".to_string()));
            //return Err(ParsingError::new("Empty Program", 0));
        }

        let program: Result<Vec<Instruction>, ParsingError> = program_text
            .lines()
            .enumerate()
            .filter(|(_, line)| !line.trim().is_empty())
            .filter(|(_, line)| !line.trim().starts_with("#"))
            .map(|(current_line_number, line)| self.parse_instruction(line, current_line_number))
            .collect();

        program
    }
    // TODO: return error if label is already in use
    /// Parses label
    ///
    /// ### Arguments
    /// * 'name' &str - name of label
    /// * 'line' usize - line number
    fn parse_label(&mut self, name: &str, line: usize) -> Result<Instruction, ParsingError> {
        Ok(Instruction::new_label(name[0..name.len() - 1].to_string()))

        //Err(ParsingError::new("Unknown error", line))
    }

    fn parse_instruction(
        &mut self,
        line: &str,
        current_line_number: usize,
    ) -> Result<Instruction, ParsingError> {
        let line_without_comments: &str = line.split('#').next().unwrap_or("").trim();
        let words: Vec<&str> = line_without_comments.split_whitespace().collect();
        //let ;instruction_word = words[0]    // <- FIXME:
        //let operands = &words[1..];      // Panincs
        if let Some(instruction_word) = words.get(0) {
            let operands = &words[1..];
            let instruction_word = words[0];
            let instruction = match instruction_word {
                "MOV" | "mov" => self.parse_binary_instruction(
                    Opcode::MOV,
                    operands,
                    current_line_number,
                    (false, true),
                ),

                "ADD" | "add" => {
                    self.parse_unary_instruction(Opcode::ADD, operands, current_line_number)
                }
                "SUB" | "sub" => {
                    self.parse_unary_instruction(Opcode::SUB, operands, current_line_number)
                }
                "MUL" | "mul" => {
                    self.parse_unary_instruction(Opcode::MUL, operands, current_line_number)
                }
                "DIV" | "div" => {
                    self.parse_unary_instruction(Opcode::DIV, operands, current_line_number)
                }
                "MOD" | "mod" => {
                    self.parse_unary_instruction(Opcode::MOD, operands, current_line_number)
                }

                "AND" | "and" => {
                    self.parse_unary_instruction(Opcode::AND, operands, current_line_number)
                }
                "OR" | "or" => {
                    self.parse_unary_instruction(Opcode::OR, operands, current_line_number)
                }
                "XOR" | "xor" => {
                    self.parse_unary_instruction(Opcode::XOR, operands, current_line_number)
                }
                "NOT" | "not" => Err(ParsingError::new(
                    ParsingError::NotImplanted,
                    current_line_number,
                    "".to_string(),
                )), //Err(ParsingError::new("NOT IMPLEMENTED", current_line_number)),

                "CMP" | "cmp" => self.parse_binary_instruction(
                    Opcode::CMP,
                    operands,
                    current_line_number,
                    (false, false),
                ), //TODO:

                "JE" | "je" => self.parse_jump(Opcode::JE, operands, current_line_number),
                "JL" | "jl" => self.parse_jump(Opcode::JL, operands, current_line_number),
                "JG" | "jg" => self.parse_jump(Opcode::JG, operands, current_line_number),
                "JMP" | "jmp" => self.parse_jump(Opcode::JMP, operands, current_line_number),

                "HLT" | "hlt" => Ok(Instruction::new(Opcode::HLT)),
                "NOP" | "nop" => Err(ParsingError::new(
                    ParsingError::NotImplanted,
                    current_line_number,
                    "".to_string(),
                )), //Err(ParsingError::new("NOT IMPLEMENTED", current_line_number)),
                "SPL" | "spl" => Err(ParsingError::new(
                    ParsingError::NotImplanted,
                    current_line_number,
                    "".to_string(),
                )), //Err(ParsingError::new("NOT IMPLEMENTED", current_line_number)),
                label if label.ends_with(':') => self.parse_label(label, current_line_number),
                _ => Err(ParsingError::new(
                    ParsingError::Unknown,
                    current_line_number,
                    "".to_string(),
                )), //Err(ParsingError::new("Unknown error", current_line_number)),
            };
            instruction
        } else {
            return Err(ParsingError::new(
                ParsingError::Empty,
                current_line_number,
                "FIXME".to_string(),
            ));
        }
    }

    fn parse_binary_instruction(
        &mut self,
        opcode: fn(Operand, Operand) -> Opcode,
        operands: &[&str],
        line: usize,
        register_only: (bool, bool),
    ) -> Result<Instruction, ParsingError> {
        if operands.len() != 2 {
            return Err(ParsingError::new(
                ParsingError::NotEnoughOperands,
                line,
                "".to_string(),
            ));
            // return Err(ParsingError::new(ParsingError::
            //     "Binary instructions requires exactly 2 operands",
            //     line,
            // ));
        }
        let (ro1, ro2) = register_only;
        let operand1 = self.parse_operand(operands[0], line, ro1)?;
        let operand2 = self.parse_operand(operands[1], line, ro2)?;
        Ok(Instruction::new(opcode(operand1, operand2)))
    }

    fn parse_unary_instruction(
        &mut self,
        opcode: fn(Operand) -> Opcode,
        operands: &[&str],
        line: usize,
    ) -> Result<Instruction, ParsingError> {
        if operands.len() != 1 {
            return Err(ParsingError::new(
                ParsingError::TooManyOperands,
                line,
                "".to_string(),
            ));
            // return Err(ParsingError::new(
            //     "Single operand instruction requires exactly 1 operand",
            //     line,
            // ));
        }

        let operand = self.parse_operand(operands[0], line, false)?;
        Ok(Instruction::new(opcode(operand)))
    }

    fn parse_jump(
        &mut self,
        opcode: fn(String) -> Opcode,
        operands: &[&str],
        line: usize,
    ) -> Result<Instruction, ParsingError> {
        if operands.is_empty() {
            return Err(ParsingError::new(
                ParsingError::NotEnoughOperands,
                line,
                "".to_string(),
            ));
            //return Err(ParsingError::new("Can't jump to empty label", line));
        } else if operands.len() > 1 {
            return Err(ParsingError::new(
                ParsingError::TooManyOperands,
                line,
                "".to_string(),
            ));
            // return Err(ParsingError::new(
            //     "Jump instruction take only one operand",
            //     line,
            // ));
        }
        let label = operands[0];
        Ok(Instruction::new(opcode(label.to_string())))
    }
    /// Sets language for parsing error messages
    pub fn set_language(&mut self, language: Language) {
        rust_i18n::set_locale(language.string_code());
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

        // let err = result.unwrap_err();
        // println!("____________________________________");
        // println!("{}", err);
        // println!("____________________________________");
    }

    #[test]
    fn test_parsing_operand_correct_registers() {
        let mut assembler = Assembler::new();

        let registers = vec!["r0", "r1", "r2", "r3", "p0", "p1", "p2", "p3"];

        //println!("_________________________________________________________");
        for register in registers.iter() {
            let result = assembler.parse_operand(&register, 0, false);

            assert!(result.is_ok());
            //println!("{:?}", result.unwrap());
        }
        //println!("_________________________________________________________");
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

        //assert_eq!(err.get_message(), "Empty Program")
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
    fn test_parsing_different_base_integers() {
        let program_text = r#"
        ADD -7
        ADD 0xAF
        ADD 0b101

    "#;
        let mut assembler = Assembler::new();
        let result = assembler.parse(program_text);

        // match result {
        //     Ok(res) => println!("{:?}", res),
        //     Err(err) => println!("{:?}", err),
        // }

        let expected = vec![
            Instruction::new(Opcode::ADD(Operand::IntegerValue(-7))),
            Instruction::new(Opcode::ADD(Operand::IntegerValue(175))),
            Instruction::new(Opcode::ADD(Operand::IntegerValue(5))),
        ];

        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn test_parsing_invalid_different_base_integers() {
        let program_text = r#"
        ADD -7
        ADD 0xAF
        ADD 0b1013

        "#;
        let mut assembler = Assembler::new();
        let result = assembler.parse(program_text);

        assert!(result.is_err());

        let program_text = r#"
        ADD -7
        ADD AF
        "#;

        let mut assembler = Assembler::new();
        let result = assembler.parse(program_text);

        assert!(result.is_err());
    }

    #[test]
    fn test_parsing_sub_inlaid_register() {
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

        //println!("{}", err);
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

        //println!("{}", err);
    }
    //TODO
    #[test]
    fn test_parsing_mov() {
        let program_text = r#"
            MOV r1 r2
            MOV 540999 acc
            
        "#;

        let mut assembler = Assembler::new();

        let result = assembler.parse(program_text);

        println!("________________________________________________---");
        match result {
            Ok(v) => println!("{:?}", v),
            Err(e) => println!("{:?}", e),
        }
    }

    #[test]
    fn test_parsing_labels_and_jumps() {
        let program_text = r#"
        MOV 10 acc
        loop:
            ADD 8
            CMP acc 20
            JL loop
        HLT
        "#;

        let mut assembler = Assembler::new();

        let result = assembler.parse(program_text);

        println!("________________________________________________---");
        match result {
            Ok(v) => println!("{:?}", v),
            Err(e) => println!("{:?}", e),
        }
    }

    #[test]
    fn test_parsing_comments() {
        let program_text = r#"
        MOV 10 acc      # Moving 10 to accumulator
        loop:           # Setting up label
            ADD 8       # Adding 8 to accumulator
            CMP acc 20  # Comparing value in accumulator to 20
            JL loop     # Jumping to 'loop' label if accumulator is lesser
        HLT             # Stopping execution
        "#;

        let mut assembler = Assembler::new();

        let result = assembler.parse(program_text);
        assert!(result.is_ok());
        println!("________________________________________________---");
        match result {
            Ok(v) => println!("{:?}", v),
            Err(e) => println!("{:?}", e),
        }
    }

    #[test]
    fn test_parsing_empty_line() {
        let program_text = r#"
        MOV 10 acc      # Moving 10 to accumulator

        loop:           # Setting up label
            ADD 8       # Adding 8 to accumulator
            CMP acc 20  # Comparing value in accumulator to 20
            JL loop     # Jumping to 'loop' label if accumulator is lesser
        HLT             # Stopping execution
        "#;

        let mut assembler = Assembler::new();

        let result = assembler.parse(program_text);
        assert!(result.is_ok());
        println!("________________________________________________---");
        match result {
            Ok(v) => println!("{:?}", v),
            Err(e) => println!("{:?}", e),
        }
    }

    #[test]
    fn test_lower_case_code_parsing() {
        let program_text = r#"
        mov 10 acc      # Moving 10 to accumulator

        loop:           # Setting up label
            add 8       # Adding 8 to accumulator
            cmp acc 20  # Comparing value in accumulator to 20
            jl loop     # Jumping to 'loop' label if accumulator is lesser
        hlt             # Stopping execution
        "#;

        let mut assembler = Assembler::new();

        let result = assembler.parse(program_text);
        assert!(result.is_ok());
        println!("________________________________________________---");
        match result {
            Ok(v) => println!("{:?}", v),
            Err(e) => println!("{:?}", e),
        }
    }


    #[test]
    fn test_line_with_only_comment() {
        let program_text = r#"
        #   This program does some stuff
        #
        #       *___________________*
        #       |                   |
        #       |                   |
        #       |                   |
        #       *___________________*
        #

        mov 10 acc      # Moving 10 to accumulator

        # Looping

        loop:           # Setting up label
            add 8       # Adding 8 to accumulator
            cmp acc 20  # Comparing value in accumulator to 20
            jl loop     # Jumping to 'loop' label if accumulator is lesser
        hlt             # Stopping execution

        #
        #   Something Something Something
        #
        "#;

        let mut assembler = Assembler::new();

        let result = assembler.parse(program_text);
        assert!(result.is_ok());
        println!("________________________________________________---");
        match result {
            Ok(v) => println!("{:?}", v),
            Err(e) => println!("{:?}", e),
        }
    }
}
