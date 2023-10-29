use crate::opcodes::Opcode;

/// Instruction used by SVM,  wrapper for Opcode
#[derive(PartialEq, Eq, Clone)]
pub struct Instruction {
     opcode: Opcode,
}

impl Instruction {
    pub fn new(opcode: Opcode) -> Instruction {
        Instruction { opcode: opcode }
    }
    pub fn get_opcode(&self) -> Opcode {
        self.opcode.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::operand::Operand;

    use super::*;

    #[test]
    fn test_create_instruction() {
        let instruction = Instruction::new(Opcode::ADD(Operand::IntegerValue(12)));
        assert_eq!(instruction.opcode, Opcode::ADD(Operand::IntegerValue(12)));
    }
}
