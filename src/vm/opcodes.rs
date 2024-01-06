use crate::vm::operand::Operand;

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
}

// instrukcje podstawowe
// nop - Instrukcja nie robiąc nic
// mov  R/I R - Kopiuje wartość pierwszego operandu do drugiego operandu
// slp R/I - Zatrzymaj procesor na ilość jednostek czasu określonych przez procesor
// Instrukcje arytmetyczne
// Rejestry mogą przechowywać wartości 16 bitowe, przepełnienie jest ignorowane.
// add R/I - Dodaje wartość pierwszego operandu do wartości w rejestrze acc i zapisuje wynik w rejestrze acc
// sub R/I - Odejmuje wartość pierwszego operandu do wartości w rejestrze acc i zapisuje wynik w rejestrze acc
// mul R/I - Mnoży wartość pierwszego operandu przez wartości w rejestrze acc i zapisuje wynik w rejestrze acc
// div R/I - Dziel całkowicie wartość przechowam w rejestrze acc przez wartość pierwszego operandu i zapisuje wynik w acc
// mod R/I - Zapisuje w rejestrze acc resztę z dzielenia wartości przechowywanej w rejestrze acc dzielonej przez pierwszy operand

// Instrukcje bitowe
// and R/I - Wykonuje operacje and na bitach wartości przechowywanej w rejestrze acc z pierwszym operandem
// or R/I - Wykonuje operacje or na bitach wartości przechowywanej w rejestrze acc z pierwszym operandem
// xor R/I - Wykonuje operację xor na bitach wartości przechowywanej w rejestrze acc z pierwszym operandem
// not - Neguje bity wartości w rejestrze acc
// Instrukcje skoku
// cmp R/I R/I - Porównuje wartości rejestrów, w zależności o wyniku ustawia flagi mniejsz, więszy, równy na 1
// je L - jeżeli flaga równy jest ustawiona na 1 kod skacze do etykiety określonej w pierwszym opadzie
// jl L - jeżeli flaga mniejszy jest ustawiona na 1 kod skacze do etykiety określonej w pierwszym opadzie
// jg L - jeżeli flaga większy jest ustawiona na 1 kod skacze do etykiety określonej w pierwszym opadzie
