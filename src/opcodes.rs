use crate::{operand::Operand};

#[derive(Debug, PartialEq, Eq, Clone )]
pub enum JMPCondition {
    EQ,
    LST,
    GRT,
}

/// Opcodes used by SVA
#[derive(Debug, PartialEq, Eq, Clone )]
pub enum Opcode {
    /// Do nothing
    NOP,
    /// Copy value of first argument ot second
    MOV(Operand, Operand),
    /// Sleep
    SPL(Operand),
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
    /// AND operand to acc
    AND(Operand),
    /// OR operand to acc
    OR(Operand),
    /// XOR operand to acc
    XOR(Operand),
    /// NOT acc
    NOT,
    /// Jum to label
    JMP(String, JMPCondition),
    /// Jump to label if equal
    JE(String),
    /// Jump to label if lesser
    JL(String),
    /// Jump to label if greater
    JG(String),
    /// Halt execution
    HLT
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