use std::fmt::{self, Display, Formatter};

use super::operand::Operand;

/// Conditions determine under what conditions jump should occur
#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub enum JMPCondition {
    EQ,
    NEQ,
    LST,
    GRT,
    NONE,
}

/// Opcodes used by SVA
#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize)]
pub enum Opcode {
    // ------------ Control instructions ------------
    /// Do nothing
    NOP,
    /// Halt execution
    HLT,

    // ------------ Moving operations ------------
    /// Copy value of first argument ot second
    MOV(Operand, Operand),

    // ------------  Arithmetic operations ------------

    // Add operand to acc
    ADD(Operand),
    /// Subtract operand from acc
    SUB(Operand),
    /// Multiply operand with acc
    MUL(Operand),
    /// Divide acc by operand
    DIV(Operand),
    /// Modulus acc by operand
    MOD(Operand),
    /// Increments acc by 1
    INC,
    /// Decrements acc by 1
    DEC,

    // ------------  Bit operations ------------
    /// ANDx operand with  acc
    AND(Operand),
    /// ORx operand with acc
    OR(Operand),
    /// XORx operand with acc
    XOR(Operand),
    /// NOTs acc
    NOT,
    /// Shifts bits to left
    SHL(Operand),
    /// Shifts bits to right
    SHR(Operand),

    // ------------ Jumping logic ------------
    /// Compare
    CMP(Operand, Operand),
    /// Jum to label
    JMP(String),
    /// Jump to label if flag set to equal
    JE(String),
    /// Jump to label if not flag set to equal
    JNE(String),
    /// Jump to label if flag set to lesser
    JL(String),
    /// Jump to label if flag set to greater
    JG(String),

    // ------------ Stack ------------
    /// Copies operand to stack
    PSH(Operand),
    /// Pops form a stack
    POP(Operand),
}

impl Display for Opcode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Opcode::NOP => write!(f, "NOP"),
            Opcode::HLT => write!(f, "HLT"),
            Opcode::MOV(op1, op2) => write!(f, "MOV {} {}", op1, op2),
            Opcode::ADD(op) => write!(f, "ADD {}", op),
            Opcode::SUB(op) => write!(f, "SUB {}", op),
            Opcode::MUL(op) => write!(f, "MUL {}", op),
            Opcode::DIV(op) => write!(f, "DIV {}", op),
            Opcode::MOD(op) => write!(f, "MOD {}", op),
            Opcode::INC => write!(f, "INC"),
            Opcode::DEC => write!(f, "DEC"),
            Opcode::AND(op) => write!(f, "AND {}", op),
            Opcode::OR(op) => write!(f, "OR {}", op),
            Opcode::XOR(op) => write!(f, "XOR {}", op),
            Opcode::NOT => write!(f, "NOT"),
            Opcode::SHL(op) => write!(f, "SHL {}", op),
            Opcode::SHR(op) => write!(f, "SHR {}", op),
            Opcode::CMP(op1, op2) => write!(f, "CMP {} {}", op1, op2),
            Opcode::JMP(label) => write!(f, "JMP {}", label),
            Opcode::JE(label) => write!(f, "JE {}", label),
            Opcode::JNE(label) => write!(f, "JNE {}", label),
            Opcode::JL(label) => write!(f, "JL {}", label),
            Opcode::JG(label) => write!(f, "JG {}", label),
            Opcode::PSH(op) => write!(f, "PSH {}", op),
            Opcode::POP(op) => write!(f, "POP {}", op),
        }
    }
}
