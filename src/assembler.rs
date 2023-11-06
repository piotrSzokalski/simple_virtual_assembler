use std::collections::HashMap;

use crate::{instruction::Instruction, opcodes::Opcode};

pub struct Assembler {

    translation: HashMap<&'static str, Opcode>,
    instructions: Vec<String>

}
impl Assembler {
    pub fn new(&mut self) {

        self.instructions = vec![String::from("add"), ];

        self.translation = HashMap::from([
            ("hlt", Opcode::HLT),
         //   ("add", Opcode::ADD), test
        ])
    }
    pub fn parse(program_text: String) -> Vec<Instruction> {

        let program: Vec<Instruction> = Vec::new();
        let lines = program_text.lines();
        for line in lines {
            let words: Vec<&str> = line.split_whitespace().collect();
            let instruction = words[0];
        }
        vec![]

    }


}