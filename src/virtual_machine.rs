use super::flag::Flag;
pub struct VirtualMachine {
    /// Program counter register
    pc: u32,
    /// Accumulator, register storing result of last mathematical or logical operation
    acc: u32,
    /// Flag register contain special states used for branching logic
    flag: Flag,
    /// General purpose registers
    pub r: [i32; 4],
    /// Registers used for I/O
    p: [i32; 4],
}

impl VirtualMachine {
    pub fn new() -> VirtualMachine {
        VirtualMachine {
            pc: 0,
            acc: 0,
            flag: Flag::ZERO,
            r: [0; 4],
            p: [0; 4],
        }
    }
    pub fn run() {}
}

// Maszyna wirtualna posiada konfigurowalna liczbę rejestrów ogólnego użytku o domyślnych nazwach r0, r1, r2..,  oraz rejestrów pełniących rolę portów do komunikacji z zewnętrznymi peryferiami o domyślnych nazwach p0, p1, p2… .Poza tym będzie posiadać następujące rejestry specjalne:
// acc - Akumulator, rejestr używany do wykonywania operacji arytmetycznych i bitowych
// flg - Flagi, rejestr przechowujący wynik porównania, instrukcji cmp
// pc - Licznik programu, rejestr przechowujący następną linię kodu do wykonania, inkrementowany po wykonaniu każdej linii kodu
// ir - Rejestr Przerwań, przechowuje informacje od przerwaniach działania procesora
