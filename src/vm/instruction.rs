use crate::vm::opcodes::Opcode;

/// Represents instruction in SVA, either an opcode or label
#[derive(PartialEq, Eq, Clone, Debug, serde::Deserialize, serde::Serialize)]
pub enum Instruction {
    Opcode(Opcode),
    Label(String),
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

