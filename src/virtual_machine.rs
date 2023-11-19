//use std::collections::btree_map::Values;

use std::fmt::{self, Display, Formatter, Result};

use std::{collections::HashMap, ops::IndexMut};

use crate::{
    flag::Flag,
    instruction::Instruction,
    opcodes::{JMPCondition, Opcode},
    operand::Operand,
};
#[derive(Debug, serde::Deserialize, serde::Serialize)]
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
    /// Labels used for jumps
    labels: HashMap<String, usize>,
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
            labels: HashMap::new(),
        }
    }
    pub fn get_acc(&self) -> i32 {
        self.acc
    }

    /// Copies operand into register
    /// ### Arguments
    ///
    /// * Operand - i32 or register
    /// * Register - register
    ///
    fn move_operand(&mut self, operand1: Operand, operand2: Operand) {
        match (operand1, operand2) {
            (Operand::IntegerValue(_), Operand::IntegerValue(_)) => unreachable!(),

            (Operand::IntegerValue(value), Operand::GeneralRegister(index)) => {
                self.r[index] = value
            }
            (Operand::IntegerValue(value), Operand::PortRegister(index)) => self.p[index] = value,
            (Operand::IntegerValue(value), Operand::ACC) => self.acc = value,
            (Operand::IntegerValue(value), Operand::PC) => self.pc = value as usize,

            (Operand::GeneralRegister(_), Operand::IntegerValue(_)) => unreachable!(),
            (Operand::GeneralRegister(index), Operand::GeneralRegister(index2)) => {
                self.r[index2] = self.r[index]
            }
            (Operand::GeneralRegister(index), Operand::PortRegister(index2)) => {
                self.p[index2] = self.r[index]
            }
            (Operand::GeneralRegister(index), Operand::ACC) => self.acc = self.r[index],
            (Operand::GeneralRegister(index), Operand::PC) => self.pc = self.r[index] as usize,

            (Operand::PortRegister(_), Operand::IntegerValue(_)) => unreachable!(),
            (Operand::PortRegister(index), Operand::GeneralRegister(index2)) => {
                self.r[index2] = self.p[index]
            }
            (Operand::PortRegister(index), Operand::PortRegister(index2)) => {
                self.p[index2] = self.p[index]
            }
            (Operand::PortRegister(index), Operand::ACC) => self.acc = self.p[index],
            (Operand::PortRegister(index), Operand::PC) => self.pc = self.p[index] as usize,

            (Operand::ACC, Operand::IntegerValue(_)) => unreachable!(),
            (Operand::ACC, Operand::GeneralRegister(index)) => self.r[index] = self.acc,
            (Operand::ACC, Operand::PortRegister(index)) => self.p[index] = self.acc,
            (Operand::ACC, Operand::ACC) => self.acc = self.acc,
            (Operand::ACC, Operand::PC) => self.pc = self.acc as usize,

            (Operand::PC, Operand::IntegerValue(_)) => unreachable!(),
            (Operand::PC, Operand::GeneralRegister(index)) => self.r[index] = self.pc as i32,
            (Operand::PC, Operand::PortRegister(index)) => self.p[index] = self.pc as i32,
            (Operand::PC, Operand::ACC) => self.acc = self.pc as i32,
            (Operand::PC, Operand::PC) => self.pc = self.pc,

            _ => unreachable!(),
        }
    }

    /// apply operation on acc
    ///
    /// ### Arguments
    ///
    /// * 'Operand' - i32 or Register
    /// * 'operation' - closure taking to two parameters: acc and operand
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
            Operand::ACC => self.acc = operation(self.acc, self.acc),
            Operand::PC => self.acc = operation(self.acc, self.pc as i32),
        }
    }

    /// Adds label unless it is already declared
    pub fn add_label(&mut self, name: String) {
        if !self.labels.contains_key(&name) {
            self.labels.insert(name, self.pc);
        }
    }

    /// Jumps to label
    pub fn jump_to_label(&mut self, label: &str, condition: JMPCondition) {
        if let Some(&jmp_to) = self.labels.get(label) {
            match (self.flag, condition) {
                (Flag::ZERO, JMPCondition::EQ) => {}
                (Flag::EQUAL, JMPCondition::EQ) => self.pc = jmp_to,
                (Flag::GREATER, JMPCondition::GRT) => self.pc = jmp_to,
                (Flag::LESSER, JMPCondition::LST) => self.pc = jmp_to,
                _ => {}
            }
        }
    }

    /// Compares operands
    pub fn compare(&mut self, operand1: Operand, operand2: Operand) {
        let value1 = match operand1 {
            Operand::IntegerValue(value) => value,
            Operand::GeneralRegister(index) => self.r[index],
            Operand::PortRegister(index) => self.p[index],
            Operand::ACC => self.acc,
            Operand::PC => self.pc as i32,
        };

        let value2 = match operand2 {
            Operand::IntegerValue(value) => value,
            Operand::GeneralRegister(index) => self.r[index],
            Operand::PortRegister(index) => self.p[index],
            Operand::ACC => self.acc,
            Operand::PC => self.pc as i32,
        };

        let result = value1 - value2;
        match result {
            0 => self.flag = Flag::EQUAL,
            n if n < 0 => self.flag = Flag::LESSER,
            n if n > 0 => self.flag = Flag::GREATER,
            _ => unreachable!(),
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

        match instruction {
            Instruction::Opcode(opcode) => match opcode {
                Opcode::HLT => {
                    return false;
                }
                Opcode::NOP => {}
                Opcode::MOV(operand1, operand2) => self.move_operand(operand1, operand2),
                Opcode::SPL(_) => todo!(),
                Opcode::ADD(operand) => self.apply_operation(operand, |a, b| a + b),
                Opcode::SUB(operand) => self.apply_operation(operand, |a, b| a - b),
                Opcode::MUL(operand) => self.apply_operation(operand, |a, b| a * b),
                Opcode::DIV(operand) => self.apply_operation(operand, |a, b| a / b),
                Opcode::MOD(operand) => self.apply_operation(operand, |a, b| a % b),
                Opcode::AND(operand) => self.apply_operation(operand, |a, b| a & b),
                Opcode::OR(operand) => self.apply_operation(operand, |a, b| a | b),
                Opcode::XOR(operand) => self.apply_operation(operand, |a, b| a ^ b),
                Opcode::NOT => self.acc = !self.acc,
                Opcode::CMP(operand1, operand2) => self.compare(operand1, operand2),
                Opcode::JE(name) => self.jump_to_label(&name, JMPCondition::EQ),
                Opcode::JL(name) => self.jump_to_label(&name, JMPCondition::LST),
                Opcode::JG(name) => self.jump_to_label(&name, JMPCondition::GRT),

                // ?
                Opcode::JMP(name, condition) => self.jump_to_label(&name, condition),
            },
            Instruction::Label(name) => self.add_label(name),
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

impl fmt::Display for VirtualMachine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "_____________________________VM__________________________________"
        );
        write!(f, "PC {}\t", self.pc)?;
        write!(f, "ACC: {}\t", self.acc)?;
        write!(f, "FLG: {:?}\n", self.flag)?;
        write!(f, "Registers: {:?}\t", self.r)?;
        write!(f, "Ports: {:?}\n", self.p)?;

        // Separate the vectors from the rest
        writeln!(f, "Program Instructions:")?;
        for instruction in &self.program {
            writeln!(f, "  {:?}", instruction)?;
        }

        writeln!(f, "Labels:")?;
        for (label, value) in &self.labels {
            writeln!(f, "  {}: {}", label, value)?;
        }
        writeln!(
            f,
            "_________________________________________________________________"
        );
        Ok(())
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
        assert_eq!(
            i2.get_opcode().unwrap(),
            Opcode::SUB(Operand::IntegerValue(10))
        );
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
            Instruction::new(Opcode::MOV(
                Operand::IntegerValue(12),
                Operand::GeneralRegister(1),
            )),
            Instruction::new(Opcode::MOV(
                Operand::GeneralRegister(1),
                Operand::GeneralRegister(2),
            )),
            Instruction::new(Opcode::MOV(
                Operand::GeneralRegister(1),
                Operand::PortRegister(1),
            )),
            Instruction::new(Opcode::MOV(
                Operand::IntegerValue(7),
                Operand::PortRegister(2),
            )),
            Instruction::new(Opcode::MOV(
                Operand::PortRegister(2),
                Operand::GeneralRegister(1),
            )),
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

    #[test]
    fn test_vm_cmp() {
        // MOV 10 r1
        // MOV 12 p0
        // MOV 12 p3

        // CPM r1 p0    (10, 12)
        // Flag should be set to Lesser

        // CMP p0 p3    (12, 12)
        // Flag should be set to Equal

        // CPM p0 r1    (12, 10)
        // Flag should be set to Greater

        let program = vec![
            Instruction::new(Opcode::MOV(
                Operand::IntegerValue(10),
                Operand::GeneralRegister(1),
            )),
            Instruction::new(Opcode::MOV(
                Operand::IntegerValue(12),
                Operand::PortRegister(0),
            )),
            Instruction::new(Opcode::MOV(
                Operand::IntegerValue(12),
                Operand::PortRegister(3),
            )),
            // CPM r1 p0
            Instruction::new(Opcode::CMP(
                Operand::GeneralRegister(1),
                Operand::PortRegister(0),
            )),
            // CMP p0 p3
            Instruction::new(Opcode::CMP(
                Operand::PortRegister(0),
                Operand::PortRegister(3),
            )),
            // CMP p0 r1
            Instruction::new(Opcode::CMP(
                Operand::PortRegister(0),
                Operand::GeneralRegister(1),
            )),
            Instruction::new(Opcode::HLT),
        ];
        let mut vm = VirtualMachine::new(program);
        vm.execute();
        vm.execute();
        vm.execute();

        println!("_________________________________________");
        vm.execute();
        println!("{:?}", vm.flag);
        assert_eq!(vm.flag, Flag::LESSER);

        vm.execute();
        println!("{:?}", vm.flag);
        assert_eq!(vm.flag, Flag::EQUAL);

        vm.execute();
        println!("{:?}", vm.flag);
        assert_eq!(vm.flag, Flag::GREATER);
    }

    #[test]
    fn test_vm_acc_and_pc_operations() {
        // MOV 10 acc   PC = 1
        // NOP          PC = 2
        // NOP          PC = 3
        // NOP          PC = 4
        // NOP          PC = 5
        // NOP          PC = 6
        // ADD pc       PC = 7 then  Acc = 10 + 7
        // HLT

        // Expected result acc = 17

        let program = vec![
            Instruction::new(Opcode::MOV(Operand::IntegerValue(10), Operand::ACC)),
            Instruction::new(Opcode::NOP),
            Instruction::new(Opcode::NOP),
            Instruction::new(Opcode::NOP),
            Instruction::new(Opcode::NOP),
            Instruction::new(Opcode::NOP),
            Instruction::new(Opcode::ADD(Operand::PC)),
            Instruction::new(Opcode::HLT),
        ];

        let mut vm = VirtualMachine::new(program);
        vm.run();

        assert_eq!(vm.acc, 17);
    }

    #[test]
    fn test_vm_labels_jumping_division_by_subtraction() {
        //                      # Divide 20 by 5 without div operator
        // mov 20 r0            # Set (r0) initial value to 20
        // mov 5 r1             # Set (r1) devisor to 5
        // mov 0 r2             # Set (r2) counter to 0
        // loop:                # Label for looping
        //      mov r0 acc      # Mov to acc
        //      sub r1          # Subtract devisor
        //      mov acc r0      # Copy result
        //      mov r2  acc     # Copy counter value to accumulator
        //      add 1           # Increment accumulator by 1
        //      mov acc r2      # Copy increased value back to counter
        //      cmp r0 0        # Compare current value to 0
        //      JG loop         # Jump if current value is grater that 0
        // hlt

        // Expected result r2 = 4

        let program = vec![
            Instruction::new(Opcode::MOV(
                Operand::IntegerValue(20),
                Operand::GeneralRegister(0),
            )),
            Instruction::new(Opcode::MOV(
                Operand::IntegerValue(5),
                Operand::GeneralRegister(1),
            )),
            Instruction::new(Opcode::MOV(
                Operand::IntegerValue(0),
                Operand::GeneralRegister(2),
            )),
            Instruction::new_label("loop".to_string()),
            Instruction::new(Opcode::MOV(Operand::GeneralRegister(0), Operand::ACC)),
            Instruction::new(Opcode::SUB(Operand::GeneralRegister(1))),
            Instruction::new(Opcode::MOV(Operand::ACC, Operand::GeneralRegister(0))),
            Instruction::new(Opcode::MOV(Operand::GeneralRegister(2), Operand::ACC)),
            Instruction::new(Opcode::ADD(Operand::IntegerValue(1))),
            Instruction::new(Opcode::MOV(Operand::ACC, Operand::GeneralRegister(2))),
            Instruction::new(Opcode::CMP(
                Operand::GeneralRegister(0),
                Operand::IntegerValue(0),
            )),
            Instruction::new(Opcode::JG("loop".to_string())),
            Instruction::new(Opcode::HLT),
        ];
        let mut vm = VirtualMachine::new(program);
        vm.run();
        // println!("______________________________");

        // println!("{:?}", vm);
        // println!("______________________________");

        // println!("{:?}", vm.r[2]);

        assert_eq!(vm.r[2], 4);
    }
}

// Maszyna wirtualna posiada konfigurowalna liczbę rejestrów ogólnego użytku o domyślnych nazwach r0, r1, r2..,  oraz rejestrów pełniących rolę portów do komunikacji z zewnętrznymi peryferiami o domyślnych nazwach p0, p1, p2… .Poza tym będzie posiadać następujące rejestry specjalne:
// acc - Akumulator, rejestr używany do wykonywania operacji arytmetycznych i bitowych
// flg - Flagi, rejestr przechowujący wynik porównania, instrukcji cmp
// pc - Licznik programu, rejestr przechowujący następną linię kodu do wykonania, inkrementowany po wykonaniu każdej linii kodu
// ir - Rejestr Przerwań, przechowuje informacje od przerwaniach działania procesora
