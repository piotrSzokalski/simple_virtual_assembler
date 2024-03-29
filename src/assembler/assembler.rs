use super::super::language::Language;

use crate::vm::{instruction::Instruction, opcodes::Opcode, operand::Operand};

use super::parsing_err::ParsingError;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Assembler {
    stack_present: bool,
}
impl Default for Assembler {
    fn default() -> Self {
        Self::new()
    }
}

impl Assembler {
    pub fn new() -> Assembler {
        Assembler {
            stack_present: false,
        }
    }

    pub fn with_stack(mut self) -> Assembler {
        self.stack_present = true;
        self
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
            "acc" => Ok(Operand::ACC),
            "pc" => Ok(Operand::PC),
            r if r.starts_with('r') => {
                if let Ok(index) = remaining_text.parse::<usize>() {
                    if index > 3 {
                        return Err(ParsingError::new(
                            ParsingError::InvalidPortNumber,
                            line,
                            "".to_string(),
                        ));
                    }
                    return Ok(Operand::GeneralRegister(index));
                }
                Err(ParsingError::new(
                    ParsingError::InvalidPortNumber,
                    line,
                    "".to_string(),
                ))
            }
            p if p.starts_with('p') => {
                if let Ok(index) = remaining_text.parse::<usize>() {
                    if index > 5 {
                        return Err(ParsingError::new(
                            ParsingError::InvalidPortNumber,
                            line,
                            "".to_string(),
                        ));
                    }
                    return Ok(Operand::PortRegister(index));
                }
                Err(ParsingError::new(
                    ParsingError::InvalidPortNumber,
                    line,
                    "".to_string(),
                ))
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
                }
                if let Ok(decimal) = decimal.parse::<i32>() {
                    return Ok(Operand::IntegerValue(decimal));
                }
                Err(ParsingError::new(
                    ParsingError::InvalidNumericLiteral,
                    line,
                    "".to_string(),
                ))
            }
            binary if binary.starts_with("0b") => {
                if register_only {
                    return Err(ParsingError::new(
                        ParsingError::InvalidBinaryLiteral,
                        line,
                        "".to_string(),
                    ));
                }
                if let Ok(binary) = i32::from_str_radix(&binary[2..], 2) {
                    return Ok(Operand::IntegerValue(binary));
                }
                Err(ParsingError::new(
                    ParsingError::InvalidBinaryLiteral,
                    line,
                    "".to_string(),
                ))
            }

            hex if hex.starts_with("0x") => {
                if register_only {
                    return Err(ParsingError::new(
                        ParsingError::InvalidHexLiteral,
                        line,
                        "".to_string(),
                    ));
                }
                if let Ok(hex) = i32::from_str_radix(&hex[2..], 16) {
                    return Ok(Operand::IntegerValue(hex));
                }
                Err(ParsingError::new(
                    ParsingError::InvalidHexLiteral,
                    line,
                    "".to_string(),
                ))
            }
            c if c.len() == 3 && c.starts_with('\'') && c.ends_with('\'') => {
                if register_only {
                    return Err(ParsingError::new(
                        ParsingError::InvalidCharLiteral,
                        line,
                        "".to_string(),
                    ));
                }
                if let Some(value) = &c.chars().nth(1) {
                    return Ok(Operand::IntegerValue(*value as i32));
                }
                Err(ParsingError::new(
                    ParsingError::InvalidCharLiteral,
                    line,
                    "".to_string(),
                ))
            }

            _ => Err(ParsingError::new(
                ParsingError::InvalidOperandType,
                line,
                "".to_string(),
            )),
        }
    }

    pub fn parse(&mut self, program_text: &str) -> Result<Vec<Instruction>, ParsingError> {
        let program_text = program_text.trim();

        if program_text.is_empty() {
            return Err(ParsingError::new(ParsingError::Empty, 0, "".to_string()));
        }

        let program: Result<Vec<Instruction>, ParsingError> = program_text
            .lines()
            .enumerate()
            .filter(|(_, line)| !line.trim().is_empty())
            .filter(|(_, line)| !line.trim().starts_with('#'))
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
        Ok(Instruction::new_label(
            name[0..name.len() - 1].to_string(),
            line,
        ))
    }

    fn parse_instruction(
        &mut self,
        line: &str,
        current_line_number: usize,
    ) -> Result<Instruction, ParsingError> {
        let line_without_comments: &str = line.split('#').next().unwrap_or("").trim();
        let words: Vec<&str> = line_without_comments.split_whitespace().collect();
        if let Some(_instruction_word) = words.first() {
            let operands = &words[1..];
            let instruction_word = words[0];

            match instruction_word {
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

                "INC" | "inc" => Ok(Instruction::new(Opcode::INC)),

                "DEC" | "dec" => Ok(Instruction::new(Opcode::DEC)),

                "AND" | "and" => {
                    self.parse_unary_instruction(Opcode::AND, operands, current_line_number)
                }
                "OR" | "or" => {
                    self.parse_unary_instruction(Opcode::OR, operands, current_line_number)
                }
                "XOR" | "xor" => {
                    self.parse_unary_instruction(Opcode::XOR, operands, current_line_number)
                }
                "NOT" | "not" => Ok(Instruction::new(Opcode::NOT)),

                "SHL" | "shl" => {
                    self.parse_unary_instruction(Opcode::SHL, operands, current_line_number)
                }
                "SHR" | "shr" => {
                    self.parse_unary_instruction(Opcode::SHR, operands, current_line_number)
                }

                "CMP" | "cmp" => self.parse_binary_instruction(
                    Opcode::CMP,
                    operands,
                    current_line_number,
                    (false, false),
                ),

                "JE" | "je" => self.parse_jump(Opcode::JE, operands, current_line_number),
                "JNE" | "jne" => self.parse_jump(Opcode::JNE, operands, current_line_number),
                "JL" | "jl" => self.parse_jump(Opcode::JL, operands, current_line_number),
                "JG" | "jg" => self.parse_jump(Opcode::JG, operands, current_line_number),
                "JMP" | "jmp" => self.parse_jump(Opcode::JMP, operands, current_line_number),

                "HLT" | "hlt" => Ok(Instruction::new(Opcode::HLT)),
                "NOP" | "nop" => Ok(Instruction::new(Opcode::NOP)),

                "PHS" | "psh" => {
                    if self.stack_present {
                        self.parse_unary_instruction(Opcode::PSH, operands, current_line_number)
                    } else {
                        Err(ParsingError::new(
                            ParsingError::StackNotPresent,
                            current_line_number,
                            "stack not present".to_owned(),
                        ))
                    }
                }
                "POP" | "pop" => {
                    if self.stack_present {
                        self.parse_unary_instruction(Opcode::POP, operands, current_line_number)
                    } else {
                        Err(ParsingError::new(
                            ParsingError::StackNotPresent,
                            current_line_number,
                            "stack not present".to_owned(),
                        ))
                    }
                }

                label if label.ends_with(':') => self.parse_label(label, current_line_number),
                _ => Err(ParsingError::new(
                    ParsingError::NoSuchInstruction,
                    current_line_number,
                    "".to_string(),
                )),
            }
        } else {
            Err(ParsingError::new(
                ParsingError::Empty,
                current_line_number,
                "FIXME".to_string(),
            ))
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
        if operands.len() > 1 {
            return Err(ParsingError::new(
                ParsingError::TooManyOperands,
                line,
                "".to_string(),
            ));
        }
        if operands.is_empty() {
            return Err(ParsingError::new(
                ParsingError::NotEnoughOperands,
                line,
                "".to_string(),
            ));
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
        } else if operands.len() > 1 {
            return Err(ParsingError::new(
                ParsingError::TooManyOperands,
                line,
                "".to_string(),
            ));
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

        let registers = ["r0", "r1", "r2", "r3", "p0", "p1", "p2", "p3"];

        //println!("_________________________________________________________");
        for register in registers.iter() {
            let result = assembler.parse_operand(register, 0, false);

            assert!(result.is_ok());
            //println!("{:?}", result.unwrap());
        }
        //println!("_________________________________________________________");
    }

    #[test]
    fn test_parsing_operand_in_correct_registers() {
        let mut assembler = Assembler::new();

        let registers = ["r-1", "r4", "rp", "pr", "p-3, p10"];

        for register in registers.iter() {
            let result = assembler.parse_operand(register, 0, false);
            println!("_______________");
            println!("{:?}", result);
            println!("_______________");
            assert!(result.is_err());
        }
    }

    #[test]
    fn test_parsing_operand() {
        let mut assembler = Assembler::new();

        let registers = ["r0", "r1", "r2", "r3", "p0", "p1", "p2", "p3"];

        for register in registers.iter() {
            let result = assembler.parse_operand(register, 0, false);

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

        // let err = result.unwrap_err();
        // println!("____________________________________");
        // println!("{}", err);
        // println!("____________________________________");

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
            SUB p6
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

        // let err = result.unwrap_err();

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

        //let err = result.unwrap_err();

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
        assert!(result.is_ok());
        // println!("________________________________________________---");
        // match result {
        //     Ok(v) => println!("{:?}", v),
        //     Err(e) => println!("{:?}", e),
        // }
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
        assert!(result.is_ok());
        // println!("________________________________________________---");
        // match result {
        //     Ok(v) => println!("{:?}", v),
        //     Err(e) => println!("{:?}", e),
        // }
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
        // println!("________________________________________________---");
        // match result {
        //     Ok(v) => println!("{:?}", v),
        //     Err(e) => println!("{:?}", e),
        // }
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
        // println!("________________________________________________---");
        // match result {
        //     Ok(v) => println!("{:?}", v),
        //     Err(e) => println!("{:?}", e),
        // }
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
        // println!("________________________________________________---");
        // match result {
        //     Ok(v) => println!("{:?}", v),
        //     Err(e) => println!("{:?}", e),
        // }
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
        // println!("________________________________________________---");
        // match result {
        //     Ok(v) => println!("{:?}", v),
        //     Err(e) => println!("{:?}", e),
        // }
    }
}
