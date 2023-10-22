//use std::collections::btree_map::Values;

use crate::{
    instruction::Instruction,
    opcodes::Opcode,
    operand::Operand,
    flag::Flag,
    register::Register
};





pub struct VirtualMachine {
    /// Program counter register
    pc: usize,
    /// Accumulator, register storing result of last mathematical or logical operation
    acc: i32,
    /// Flag register contain special states used for branching logic
    flag: Flag,
    /// General purpose registers
    pub r: [i32; 4],
    /// Ports - registers used for I/O
    p: [i32; 4],
    /// Vector of instructions to be executed
    program: Vec<Instruction>,
}

impl VirtualMachine {
    /// Create an instance of VM
    pub fn new(program: Vec<Instruction>) -> VirtualMachine {
        VirtualMachine {
            pc: 0,
            acc: 0,
            flag: Flag::ZERO,
            r: [0; 4],
            p: [0; 4],
            program,
        }
    }

    /// Ads operand to acc
    fn add(&mut self, operand: Operand) {
        match operand {
            Operand::IntegerValue(value) => {
                self.acc += value;
            }
            Operand::REGISTER(register) => {
                match register {
                    Register::GENERAL(index) => self.acc += self.r[index],
                    Register::PORT(index) =>  self.acc += self.p[index],
                }
                
            }
        }
    }

    /// Fetches next instruction from the program and increments program counter by one
    pub fn fetch(&mut self) -> Instruction {
            let opcode = self.program[self.pc].clone();
            self.pc += 1;
            opcode
    }
        /// Executes single instruction
    pub fn execute(&mut self) -> bool {
            if (self.pc >= self.program.len()) {
                return false;
            }
            let instruction = self.fetch();
            let opcode = instruction.get_opcode();
            match opcode {
                Opcode::HLT => {
                    println!("HLT encountered");
                    return false;
                }
                Opcode::NOP => todo!(),
                Opcode::MOV(_, _) => todo!(),
                Opcode::SPL(_) => todo!(),
                Opcode::ADD(operand) => self.add(operand),
                Opcode::SUB(_) => todo!(),
                Opcode::MUL(value) => todo!(),
                Opcode::DIV(_) => todo!(),
                Opcode::MOD(_) => todo!(),
                Opcode::AND(_) => todo!(),
                Opcode::OR(_) => todo!(),
                Opcode::XOR(_) => todo!(),
                Opcode::NOT => todo!(),
                Opcode::JE(_) => todo!(),
                Opcode::JL(_) => todo!(),
                Opcode::JG(_) => todo!(),
            }
            true
        }

        /// Runs all instructions in given program
    pub fn run(&mut self) {
            loop {
                self.execute();
            }
    }
}

#[cfg(test)]
mod tests {

    use std::vec;

    use crate::operand::Operand;

    use super::*;

    #[test]
    fn test_create_vm() {
        let program = vec![Instruction::new(Opcode::ADD(Operand::IntegerValue(12)))];
        let vm = VirtualMachine::new(program);
        assert_eq!(vm.r[0], 0);
    }

    #[test]
    fn test_vm_fetch_instruction() {
        let program = vec![
            Instruction::new(Opcode::ADD(Operand::IntegerValue(12))),
            Instruction::new(Opcode::SUB(Operand::IntegerValue(10))),
        ];
        let mut vm = VirtualMachine::new(program);
        let _i1 = vm.fetch();
        let i2 = vm.fetch();
        assert_eq!(i2.get_opcode(), Opcode::SUB(Operand::IntegerValue(10)));
    }

    #[test]
    fn test_vm_instruction_add() {
        let program = vec! [
            Instruction::new(Opcode::ADD(Operand::IntegerValue(10))),
        ];
        let mut vm = VirtualMachine::new(program);
        vm.execute();
        assert_eq!(vm.acc, 10);
        
    }
}

// Maszyna wirtualna posiada konfigurowalna liczbę rejestrów ogólnego użytku o domyślnych nazwach r0, r1, r2..,  oraz rejestrów pełniących rolę portów do komunikacji z zewnętrznymi peryferiami o domyślnych nazwach p0, p1, p2… .Poza tym będzie posiadać następujące rejestry specjalne:
// acc - Akumulator, rejestr używany do wykonywania operacji arytmetycznych i bitowych
// flg - Flagi, rejestr przechowujący wynik porównania, instrukcji cmp
// pc - Licznik programu, rejestr przechowujący następną linię kodu do wykonania, inkrementowany po wykonaniu każdej linii kodu
// ir - Rejestr Przerwań, przechowuje informacje od przerwaniach działania procesora
