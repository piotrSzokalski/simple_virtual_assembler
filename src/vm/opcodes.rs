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
    /// Sleeps vm for given amount of milliseconds
    SLP(Operand),

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
