use crate::opcodes::Opcode;

/// Instruction used by SVM,  wrapper for Opcode
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Instruction {
     Opcode(Opcode),
     Label(String)
     
}

impl Instruction {
    pub fn new(opcode: Opcode) -> Instruction {
        Instruction::Opcode(opcode)
    }

    pub fn new_label(name: String) -> Instruction {
        Instruction::Label(name)
    }

    pub fn get_opcode(&self) -> Option<Opcode> {
        match self {
            Instruction::Opcode(opcode) => Some(opcode.clone()),
            Instruction::Label(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    // use crate::operand::Operand;

    // use super::*;

    // #[test]
    // fn test_create_instruction() {
    //     let instruction = Instruction::new(Opcode::ADD(Operand::IntegerValue(12)));
    //     assert_eq!(instruction.opcode, Opcode::ADD(Operand::IntegerValue(12)));
    // }
}
