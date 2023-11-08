//use std::collections::btree_map::Values;

use crate::{
    flag::Flag,
    instruction::Instruction,
    opcodes::Opcode,
    operand::Operand,
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
    // Labels used for looping
    //labels: Vec<(String, usize)>
}

impl VirtualMachine {
    /// Create an instance of VM
    ///
    /// ### Arguments
    ///
    /// * 'program' - List of instruction to be executed
    ///
    /// ### Returns
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
    /// Copies operand into register
    /// ### Arguments
    /// 
    /// * Operand - i32 or register
    /// * Register - register
    /// 
    fn move_operand(&mut self, operand1: Operand, operand2: Operand) {
        let index;
        let move_to_port = match operand2 {

            Operand::GeneralRegister(i) =>  {
                index = i;
                false
            }

            Operand::PortRegister(i) => {
                index = i;
                true
            }
            Operand::IntegerValue(_) => panic!(),

            // Register::PORT(i) => {
            //     index = i;
            //     true
            // }
            // Register::GENERAL(i) => {
            //     index = i;
            //     false
            // }
        };
        if index > 3 {
            panic!();
        }
        match operand1 {
            Operand::IntegerValue(value) => {
                if move_to_port {
                    self.p[index] = value;
                } else {
                    self.r[index] = value;
                }
            },
            Operand::PortRegister(i) => {
                if move_to_port {
                    self.p[index] = self.p[i];
                } else {
                    self.r[index] = self.p[i]
                }
            },
            Operand::GeneralRegister(i) => {
                if move_to_port {
                    self.p[index] = self.r[i];
                } else {
                    self.r[index] = self.r[i];
                }
            },

            // Operand::REGISTER(register) => match register {
            //     Register::GENERAL(i) => {
            //         if move_to_port {
            //             self.p[index] = self.r[i];
            //         } else {
            //             self.r[index] = self.r[i]
            //         }
            //     }
            //     Register::PORT(i) => {
            //         if move_to_port {
            //             self.p[index] = self.p[i];
            //         } else {
            //             self.r[index] = self.p[i];
            //         }
            //     }
            // },
        }
    }

    /// apply operation on acc
    ///
    /// ### Arguments
    ///
    /// * 'Operand' - i32 or Register
    /// * 'operation' - closure taking to two parameters acc and operand
    ///
    /// ### Example
    ///
    /// ```rs
    ///     apply_operation(operand, |a, b| a + b)
    /// ```
    fn apply_operation<F>(&mut self, operand: Operand, operation: F)
    where
        F: Fn(i32, i32) -> i32,
    {
        match operand {
            Operand::IntegerValue(value) => {
                self.acc = operation(self.acc, value);
            }
            Operand::GeneralRegister(index) => self.acc = operation(self.acc, self.r[index]),
            Operand::PortRegister(index) => self.acc = operation(self.acc, self.p[index]),


            // Operand::REGISTER(register) => match register {
            //     Register::GENERAL(index) => {
            //         self.acc = operation(self.acc, self.r[index]);
            //     }
            //     Register::PORT(index) => {
            //         self.acc = operation(self.acc, self.p[index]);
            //     }
            // },
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
        if self.pc >= self.program.len() {
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
            Opcode::MOV(o1, o2) => self.move_operand(o1, o2),
            Opcode::SPL(_) => todo!(),
            Opcode::ADD(operand) => self.apply_operation(operand, |a, b| a + b),
            Opcode::SUB(operand) => self.apply_operation(operand, |a, b| a - b),
            Opcode::MUL(operand) => self.apply_operation(operand, |a, b| a * b),
            Opcode::DIV(operand) => self.apply_operation(operand, |a, b| a / b),
            Opcode::MOD(operand) => self.apply_operation(operand, |a, b| a % b),
            Opcode::AND(operand) => self.apply_operation(operand, |a, b| a & b),
            Opcode::OR(operand) => self.apply_operation(operand, |a, b| a | b),
            Opcode::XOR(operand) => self.apply_operation(operand, |a, b| a ^ b),
            Opcode::NOT => { self.acc = ! self.acc },
            Opcode::JE(_) => todo!(),
            Opcode::JL(_) => todo!(),
            Opcode::JG(_) => todo!(),
        }
        true
    }

    /// Runs all instructions in given program
    pub fn run(&mut self) {
        let mut running = true;
        while running {
            running = self.execute();
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
        let program = vec![
            Instruction::new(Opcode::ADD(Operand::IntegerValue(10))),
            Instruction::new(Opcode::ADD(Operand::IntegerValue(45))),
        ];
        let mut vm = VirtualMachine::new(program);
        vm.execute();
        vm.execute();
        assert_eq!(vm.acc, 55);
    }

    #[test]
    fn test_vm_instruction_subtract() {
        let program = vec![
            Instruction::new(Opcode::ADD(Operand::IntegerValue(100))),
            Instruction::new(Opcode::SUB(Operand::IntegerValue(90))),
            Instruction::new(Opcode::SUB(Operand::IntegerValue(1))),
            Instruction::new(Opcode::SUB(Operand::IntegerValue(1))),
            Instruction::new(Opcode::HLT),
        ];
        let mut vm = VirtualMachine::new(program);
        vm.run();

        assert_eq!(vm.acc, 8);
    }

    #[test]
    fn test_vm_instruction_multiply() {
        let program = vec![
            Instruction::new(Opcode::ADD(Operand::IntegerValue(3))),
            Instruction::new(Opcode::MUL(Operand::IntegerValue(7))),
        ];
        let mut vm = VirtualMachine::new(program);
        vm.execute();
        vm.execute();
        assert_eq!(vm.acc, 21);
    }

    #[test]
    fn test_vm_instruction_divide() {
        let program = vec![
            Instruction::new(Opcode::ADD(Operand::IntegerValue(100))),
            Instruction::new(Opcode::DIV(Operand::IntegerValue(5))),
        ];
        let mut vm = VirtualMachine::new(program);
        vm.execute();
        vm.execute();
        assert_eq!(vm.acc, 20);
    }

    #[test]
    fn test_vm_instruction_move() {
        let program = vec![
            // MOV 12 R1    -> Moving integer into register
            // MOV R1 R2    -> Moving from register to register
            // MOV R1 P1    -> Moving from register to port
            // MOV 7 P2     -> Moving integer into port
            // MOV P1 R1    -> Moving from port to register


            Instruction::new(Opcode::MOV(Operand::IntegerValue(12), Operand::GeneralRegister(1))),
            //Instruction::new(Opcode::MOV(Operand::IntegerValue(12), Register::GENERAL(1))),
            
            Instruction::new(Opcode::MOV(Operand::GeneralRegister(1), Operand::GeneralRegister(2))),
            //Instruction::new(Opcode::MOV(Operand::REGISTER(Register::GENERAL(1)), Register::GENERAL(2))),
            
            Instruction::new(Opcode::MOV(Operand::GeneralRegister(1), Operand::PortRegister(1))),
            //Instruction::new(Opcode::MOV(Operand::REGISTER(Register::GENERAL(1)), Register::PORT(1))),
            
            Instruction::new(Opcode::MOV(Operand::IntegerValue(7),  Operand::PortRegister(2))),
            //Instruction::new(Opcode::MOV(Operand::IntegerValue(7), Register::PORT(2))),
            
            Instruction::new(Opcode::MOV(Operand::PortRegister(2), Operand::GeneralRegister(1))),
            //Instruction::new(Opcode::MOV(Operand::REGISTER(Register::PORT(2)), Register::GENERAL(1))),

        
            Instruction::new(Opcode::HLT),
        ];
        // expected:  r1 = 7, r2 =  12, p1 = 12, p2 = 7
        let mut vm = VirtualMachine::new(program);
        vm.run();
        assert_eq!(vm.r[1], 7);
        assert_eq!(vm.r[2], 12);
        assert_eq!(vm.p[1], 12);
        assert_eq!(vm.p[2], 7);
    }
}

// Maszyna wirtualna posiada konfigurowalna liczbę rejestrów ogólnego użytku o domyślnych nazwach r0, r1, r2..,  oraz rejestrów pełniących rolę portów do komunikacji z zewnętrznymi peryferiami o domyślnych nazwach p0, p1, p2… .Poza tym będzie posiadać następujące rejestry specjalne:
// acc - Akumulator, rejestr używany do wykonywania operacji arytmetycznych i bitowych
// flg - Flagi, rejestr przechowujący wynik porównania, instrukcji cmp
// pc - Licznik programu, rejestr przechowujący następną linię kodu do wykonania, inkrementowany po wykonaniu każdej linii kodu
// ir - Rejestr Przerwań, przechowuje informacje od przerwaniach działania procesora
