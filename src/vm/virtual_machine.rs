//use std::collections::btree_map::Values;

use std::fmt::{self};

use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::time::Duration;
use std::{collections::HashMap, ops::IndexMut};
use std::{thread, usize};

use crate::components::connection::Connection;
use crate::components::port::Port;

use crate::vm::{
    flag::Flag,
    instruction::Instruction,
    opcodes::{JMPCondition, Opcode},
};

use super::operand::Operand;

/// Status of vm
#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq, Clone, Copy)]
pub enum VmStatus {
    Initial,
    Running,
    Stopped,
    Finished,
}

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
    p: [Port; 6],
    /// Labels used for jumps
    labels: HashMap<String, usize>,
    /// Vector of instructions to be executed
    program: Vec<Instruction>,
    /// Status of vm
    status: VmStatus,
    /// Delay between instruction in ms ( sleep between execution )
    delay_ms: u32,
    /// vm has stack
    stack_present: bool,
    /// Stack size
    stack_size: usize,
    /// Stack
    stack: Vec<i32>,
}

impl Default for VirtualMachine {
    fn default() -> Self {
        Self::new()
    }
}

impl VirtualMachine {
    /// Create an instance of VM
    ///
    pub fn new() -> VirtualMachine {
        VirtualMachine {
            pc: 0,
            acc: 0,
            flag: Flag::EQUAL,
            r: [0; 4],
            //TODO:
            p: [
                Port::new(0),
                Port::new(0),
                Port::new(0),
                Port::new(0),
                Port::new(0),
                Port::new(0),
            ],

            labels: HashMap::new(),
            program: Vec::new(),
            status: VmStatus::Initial,
            delay_ms: 0,
            stack_present: false,
            stack_size: 0,
            stack: Vec::new(),
        }
    }
    /// Create an instance of VM
    ///
    /// ### Arguments
    ///
    /// * 'program' - List of instruction to be executed
    ///
    /// ### Returns
    pub fn new_with_program(program: Vec<Instruction>) -> VirtualMachine {
        let mut vm = VirtualMachine {
            pc: 0,
            acc: 0,
            flag: Flag::EQUAL,
            r: [0; 4],
            //TODO:
            p: [
                Port::new(0),
                Port::new(0),
                Port::new(0),
                Port::new(0),
                Port::new(0),
                Port::new(0),
            ],
            program,
            labels: HashMap::new(),
            status: VmStatus::Initial,
            delay_ms: 0,
            stack_present: false,
            stack_size: 0,
            stack: Vec::new(),
        };
        vm.set_labels();
        vm
    }

    pub fn with_stack(mut self, size: usize) -> VirtualMachine {
        self.stack = Vec::with_capacity(size);
        self.stack_present = true;
        self.stack_size = size;
        self
    }

    pub fn load_program(&mut self, program: Vec<Instruction>) {
        self.labels.clear();
        self.program = program;
        self.set_labels();
    }

    pub fn set_labels(&mut self) {
        for instruction in &self.program {
            if let Instruction::Label(name, line) = instruction {
                self.labels.insert(name.clone(), line + 1);
            }
        }
    }

    pub fn set_delay(&mut self, delay_ms: u32) {
        self.delay_ms = delay_ms
    }

    pub fn get_delay(&mut self) -> u32 {
        self.delay_ms
    }

    pub fn get_acc(&self) -> i32 {
        self.acc
    }

    pub fn get_pc(&self) -> usize {
        self.pc
    }

    pub fn get_flag(&self) -> Flag {
        self.flag
    }

    pub fn get_registers(&self) -> [i32; 4] {
        self.r
    }

    pub fn get_ports(&self) -> [Port; 6] {
        self.p.clone()
    }

    pub fn get_ports_ref(&self) {}

    //TODO:
    pub fn get_ports_values(&self) -> [i32; 4] {
        [0, 0, 0, 0]
    }

    /// Gets state of all register (acc, pc, flag, r, p)
    pub fn get_registers_all(&self) -> (i32, usize, Flag, [i32; 4], [Port; 6]) {
        (self.acc, self.pc, self.flag, self.r, self.p.clone())
    }

    pub fn get_labels(&self) -> HashMap<String, usize> {
        self.labels.clone()
    }

    pub fn get_program(&self) -> Vec<Instruction> {
        self.program.clone()
    }

    pub fn get_status(&self) -> VmStatus {
        self.status
    }

    pub fn get_stack(&self) -> Vec<i32> {
        self.stack.clone()
    }

    pub fn get_next_instruction(&self) -> Option<Instruction> {
        self.program.get(self.pc).cloned()
    }
    #[allow(clippy::type_complexity)]
    /// Gets full state of virtual machine (acc, pc, flag, r, p, labels, program)
    pub fn get_state_full_old(
        &self,
    ) -> (
        i32,
        usize,
        Flag,
        [i32; 4],
        [Port; 6],
        HashMap<String, usize>,
        Vec<Instruction>,
    ) {
        (
            self.acc,
            self.pc,
            self.flag,
            self.r,
            self.p.clone(),
            self.labels.clone(),
            self.program.clone(),
        )
    }

    pub fn get_state_for_display(&self) -> (i32, usize, Flag, [i32; 4], [i32; 6], VmStatus, u32) {
        let mut port_values: [i32; 6] = [0; 6];
        for (i, p) in self.p.iter().enumerate() {
            port_values[i] = p.clone().get();
        }

        (
            self.acc,
            self.pc,
            self.flag,
            self.r,
            port_values,
            self.status,
            self.delay_ms,
        )
    }

    pub fn set_pc(&mut self, pc: usize) {
        self.pc = pc;
    }

    pub fn reset_pc(&mut self) {
        self.pc = 0;
    }

    pub fn clear_registers(&mut self) {
        self.pc = 0;
        self.acc = 0;
        self.flag = Flag::EQUAL;
        self.r.iter_mut().for_each(|item| *item = 0);
        if self.stack_present {
            self.stack.clear();
        }
    }
    /// Connects vm with connection to shared data across threads
    ///
    /// ### Arguments
    ///
    /// * index - index of port
    /// * connection - reference to connection
    ///
    pub fn connect(&mut self, index: usize, connection: &mut Connection) {
        self.p[index].connect(connection);
    }

    /// Connects vm with connection to shared data across threads with id of port
    ///
    /// ### Arguments
    ///
    /// * index - index of port
    /// * connection - reference to connection
    /// * id of port
    ///
    pub fn connect_with_id(&mut self, index: usize, connection: &mut Connection, id: String) {
        self.p[index].connect(connection);
        connection.add_port_id(id);
    }

    /// Disconnects from connection
    pub fn disconnect(&mut self, index: usize) {
        let value = match &self.p[index] {
            Port::Connected(_, _) => self.p[index].get(),
            Port::Disconnected(value) => *value,
        };
        self.p[index] = Port::Disconnected(value);
    }

    pub fn disconnect_and_unlist(&mut self, index: usize, connection: &mut Connection) {
        let value = match &self.p[index] {
            Port::Connected(_, _) => self.p[index].get(),
            Port::Disconnected(value) => *value,
        };

        let id = self.p[index].get_id();
        if let Some(id) = id {
            let id = format!("{}P{}", id, index);
            connection.remove_port_id(id);
        }

        self.p[index] = Port::Disconnected(value);
    }

    #[allow(dead_code)]
    fn sleep(&mut self, operand: Operand) {
        let _duration = match operand {
            Operand::IntegerValue(value) => value,
            Operand::GeneralRegister(index) => self.r[index],
            Operand::PortRegister(index) => self.p[index].get(),
            Operand::ACC => self.acc,
            Operand::PC => self.pc.try_into().unwrap_or(0),
        };
        VirtualMachine::delay(self.delay_ms)

        //thread::sleep(Duration::from_millis(duration.try_into().unwrap_or(0)));
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

            (Operand::IntegerValue(value), Operand::PortRegister(index)) => {
                self.p[index].set(value)
            }
            (Operand::IntegerValue(value), Operand::ACC) => self.acc = value,
            (Operand::IntegerValue(value), Operand::PC) => self.pc = value as usize,

            (Operand::GeneralRegister(_), Operand::IntegerValue(_)) => unreachable!(),
            (Operand::GeneralRegister(index), Operand::GeneralRegister(index2)) => {
                self.r[index2] = self.r[index]
            }

            (Operand::GeneralRegister(index), Operand::PortRegister(index2)) => {
                self.p[index2].set(self.r[index])
            }
            (Operand::GeneralRegister(index), Operand::ACC) => self.acc = self.r[index],
            (Operand::GeneralRegister(index), Operand::PC) => self.pc = self.r[index] as usize,

            (Operand::PortRegister(_), Operand::IntegerValue(_)) => unreachable!(),

            (Operand::PortRegister(index), Operand::GeneralRegister(index2)) => {
                self.r[index2] = self.p[index].get()
            }

            (Operand::PortRegister(index), Operand::PortRegister(index2)) => {
                let new_value = self.p[index].get();
                self.p[index2].set(new_value);
            }

            (Operand::PortRegister(index), Operand::ACC) => self.acc = self.p[index].get(),

            (Operand::PortRegister(index), Operand::PC) => self.pc = self.p[index].get() as usize,
            (Operand::ACC, Operand::IntegerValue(_)) => unreachable!(),
            (Operand::ACC, Operand::GeneralRegister(index)) => self.r[index] = self.acc,

            (Operand::ACC, Operand::PortRegister(index)) => self.p[index].set(self.acc),
            (Operand::ACC, Operand::ACC) => {}
            (Operand::ACC, Operand::PC) => self.pc = self.acc as usize,

            (Operand::PC, Operand::IntegerValue(_)) => unreachable!(),
            (Operand::PC, Operand::GeneralRegister(index)) => self.r[index] = self.pc as i32,

            (Operand::PC, Operand::PortRegister(index)) => self.p[index].set(self.pc as i32),
            (Operand::PC, Operand::ACC) => self.acc = self.pc as i32,
            (Operand::PC, Operand::PC) => {}
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

            Operand::PortRegister(index) => self.acc = operation(self.acc, self.p[index].get()),
            Operand::ACC => self.acc = operation(self.acc, self.acc),
            Operand::PC => self.acc = operation(self.acc, self.pc as i32),
        }
    }

    #[allow(dead_code)]
    /// Adds label unless it is already declared
    fn add_label(&mut self, name: String) {
        if let std::collections::hash_map::Entry::Vacant(e) = self.labels.entry(name) {
            e.insert(self.pc);
        }
    }

    /// Jumps to label
    fn jump_to_label(&mut self, label: &str, condition: JMPCondition) {
        if let Some(&jmp_to) = self.labels.get(label) {
            match (self.flag, condition) {
                (Flag::EQUAL, JMPCondition::EQ) => self.pc = jmp_to,
                (Flag::LESSER | Flag::GREATER, JMPCondition::NEQ) => self.pc = jmp_to,
                (Flag::GREATER, JMPCondition::GRT) => self.pc = jmp_to,
                (Flag::LESSER, JMPCondition::LST) => self.pc = jmp_to,
                (_, JMPCondition::NONE) => self.pc = jmp_to,
                _ => {}
            }
        }
    }

    /// Compares operands
    fn compare(&mut self, operand1: Operand, operand2: Operand) {
        let value1 = match operand1 {
            Operand::IntegerValue(value) => value,
            Operand::GeneralRegister(index) => self.r[index],

            Operand::PortRegister(index) => self.p[index].get(),
            Operand::ACC => self.acc,
            Operand::PC => self.pc as i32,
        };

        let value2 = match operand2 {
            Operand::IntegerValue(value) => value,
            Operand::GeneralRegister(index) => self.r[index],

            Operand::PortRegister(index) => self.p[index].get(),
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

    /// pushes operand to stack ( if stack is present)
    fn push_to_stack(&mut self, operand: Operand) {
        if !self.stack_present {
            return;
        }
        let value = match operand {
            Operand::IntegerValue(value) => value,
            Operand::GeneralRegister(index) => self.r[index],
            Operand::PortRegister(index) => self.p[index].get(),
            Operand::ACC => self.acc,
            Operand::PC => self.pc.try_into().unwrap(),
        };

        if self.stack.len() < self.stack_size {
            self.stack.push(value);
            return;
        }
        *self.stack.index_mut(self.stack_size - 1) = value;
    }

    /// pushes operand from stack ( if stack is present)
    fn pop_from_stack(&mut self, operand: Operand) {
        if !self.stack_present {
            return;
        }
        let value = self.stack.pop().unwrap_or(0);
        match operand {
            Operand::IntegerValue(_) => unreachable!(),
            Operand::GeneralRegister(index) => self.r[index] = value,
            Operand::PortRegister(index) => self.p[index].set(value),
            Operand::ACC => self.acc = value,
            Operand::PC => self.pc = value.try_into().unwrap_or(0),
        }
    }

    /// Fetches next instruction from the program and increments program counter by one
    fn fetch(&mut self) -> Instruction {
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
            Instruction::Opcode(opcode) => {
                match opcode {
                    // ------------ Control instructions ------------
                    Opcode::HLT => {
                        return false;
                    }
                    Opcode::NOP => {}
                    // ------------ Moving operations ------------
                    Opcode::MOV(operand1, operand2) => self.move_operand(operand1, operand2),

                    // ------------  Arithmetic operations ------------
                    Opcode::ADD(operand) => self.apply_operation(operand, |a, b| a.wrapping_add(b)),
                    Opcode::SUB(operand) => self.apply_operation(operand, |a, b| a.wrapping_sub(b)),
                    Opcode::MUL(operand) => self.apply_operation(operand, |a, b| a.wrapping_mul(b)),
                    Opcode::DIV(operand) => self.apply_operation(operand, |a, b| a.wrapping_div(b)),
                    Opcode::MOD(operand) => self.apply_operation(operand, |a, b| a.wrapping_rem(b)),
                    Opcode::INC => self.acc = self.acc.wrapping_add(1),
                    Opcode::DEC => self.acc = self.acc.wrapping_sub(1),

                    // ------------  Bit operations ------------
                    Opcode::OR(operand) => self.apply_operation(operand, |a, b| a | b),
                    Opcode::XOR(operand) => self.apply_operation(operand, |a, b| a ^ b),
                    Opcode::AND(operand) => self.apply_operation(operand, |a, b| a & b),
                    Opcode::NOT => self.acc = !self.acc,
                    Opcode::SHL(operand) => self
                        .apply_operation(operand, |a, b| a.wrapping_shl(b.try_into().unwrap_or(0))),
                    Opcode::SHR(operand) => self
                        .apply_operation(operand, |a, b| a.wrapping_shr(b.try_into().unwrap_or(0))),

                    // ------------ Jumping logic ------------
                    Opcode::CMP(operand1, operand2) => self.compare(operand1, operand2),
                    Opcode::JE(name) => self.jump_to_label(&name, JMPCondition::EQ),
                    Opcode::JL(name) => self.jump_to_label(&name, JMPCondition::LST),
                    Opcode::JG(name) => self.jump_to_label(&name, JMPCondition::GRT),
                    Opcode::JMP(name) => self.jump_to_label(&name, JMPCondition::NONE),

                    Opcode::JNE(name) => self.jump_to_label(&name, JMPCondition::NEQ),
                    // ------------ Stack operations ------------
                    Opcode::PSH(operand) => self.push_to_stack(operand),
                    Opcode::POP(operand) => self.pop_from_stack(operand),
                }
            }
            Instruction::Label(_, _) => {}
        }

        true
    }

    /// Used to delay execution by sleeping current thread
    ///
    /// Another solution may more appropriate but sleep will work for now
    pub fn delay(ms: u32) {
        thread::sleep(Duration::from_millis(ms.into()));
    }

    /// Runs all instructions in given program
    pub fn run(&mut self) {
        let mut running = true;
        self.status = VmStatus::Running;
        while running {
            running = self.execute();
            VirtualMachine::delay(self.delay_ms)
        }
        self.status = VmStatus::Finished;
    }

    /// Starts vm on another thread
    pub fn start(vm: Arc<Mutex<VirtualMachine>>) -> JoinHandle<()> {
        let handle = thread::spawn(move || {
            let mut running = true;
            {
                let mut vm = vm.lock().unwrap();
                vm.status = VmStatus::Running;
            }
            let mut delay = 0;
            while running {
                {
                    let mut vm: std::sync::MutexGuard<'_, VirtualMachine> = vm.lock().unwrap();
                    if vm.status == VmStatus::Running {
                        running = vm.execute();

                        delay = vm.get_delay();
                    } else if vm.status == VmStatus::Finished {
                        break;
                    }
                }
                VirtualMachine::delay(delay)
            }
            {
                let mut vm = vm.lock().unwrap();
                vm.status = VmStatus::Finished;
            }
        });
        handle
    }
    /// Stops vm running on another thread
    pub fn stop(vm: Arc<Mutex<VirtualMachine>>) {
        vm.lock().unwrap().status = VmStatus::Stopped;
    }
    /// Continues executing code
    pub fn resume(vm: Arc<Mutex<VirtualMachine>>) {
        vm.lock().unwrap().status = VmStatus::Running;
    }

    /// Halts vm
    pub fn halt(vm: Arc<Mutex<VirtualMachine>>) {
        {
            vm.lock().unwrap().status = VmStatus::Finished;
        }
        vm.lock().unwrap().clear_registers();
    }

    /// Helper function to create shared vm
    ///
    /// ### Examples
    ///
    /// ```rs
    /// let (vm2, vm2_copy) = VirtualMachine::new_shared_with_program(program2);
    /// let handel2 = VirtualMachine::start(vm2);
    /// println!("{}", vm2_copy.lock().unwrap());
    /// ```
    pub fn new_shared() -> (Arc<Mutex<VirtualMachine>>, Arc<Mutex<VirtualMachine>>) {
        let vm = Arc::new(Mutex::new(VirtualMachine::new()));

        (vm.clone(), vm)
    }

    /// Helper function to create shared vm
    pub fn new_shared_with_program(
        program: Vec<Instruction>,
    ) -> (Arc<Mutex<VirtualMachine>>, Arc<Mutex<VirtualMachine>>) {
        let vm = Arc::new(Mutex::new(VirtualMachine::new_with_program(program)));
        (vm.clone(), vm)
    }
}

impl fmt::Display for VirtualMachine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "_____________________________VM__________________________________"
        )?;
        write!(f, "PC {}\t", self.pc)?;
        write!(f, "ACC: {}\t", self.acc)?;
        writeln!(f, "FLG: {:?}", self.flag)?;
        write!(f, "Registers: {:?}\t", self.r)?;
        writeln!(f, "Ports: {:?}", self.p)?;

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
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use std::vec;

    use super::*;

    #[test]
    fn test_create_vm() {
        let program = vec![Instruction::new(Opcode::ADD(Operand::IntegerValue(12)))];
        let vm = VirtualMachine::new_with_program(program);
        assert_eq!(vm.r[0], 0);
    }

    #[test]
    fn test_vm_fetch_instruction() {
        let program = vec![
            Instruction::new(Opcode::ADD(Operand::IntegerValue(12))),
            Instruction::new(Opcode::SUB(Operand::IntegerValue(10))),
        ];
        let mut vm = VirtualMachine::new_with_program(program);
        let _i1 = vm.fetch();
        let i2 = vm.fetch();
        assert_eq!(
            i2.get_opcode().unwrap(),
            Opcode::SUB(Operand::IntegerValue(10))
        );
    }

    #[test]
    fn test_load_program() {
        let mut vm = VirtualMachine::new_with_program(vec![]);
        let program = vec![
            Instruction::new(Opcode::ADD(Operand::IntegerValue(10))),
            Instruction::new(Opcode::ADD(Operand::IntegerValue(45))),
        ];
        vm.load_program(program.clone());
        assert_eq!(vm.program, program);
    }

    #[test]
    fn test_vm_instruction_add() {
        let program = vec![
            Instruction::new(Opcode::ADD(Operand::IntegerValue(10))),
            Instruction::new(Opcode::ADD(Operand::IntegerValue(45))),
        ];
        let mut vm = VirtualMachine::new_with_program(program);
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
        let mut vm = VirtualMachine::new_with_program(program);
        vm.run();

        assert_eq!(vm.acc, 8);
    }

    #[test]
    fn test_vm_instruction_multiply() {
        let program = vec![
            Instruction::new(Opcode::ADD(Operand::IntegerValue(3))),
            Instruction::new(Opcode::MUL(Operand::IntegerValue(7))),
        ];
        let mut vm = VirtualMachine::new_with_program(program);
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
        let mut vm = VirtualMachine::new_with_program(program);
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
        let mut vm = VirtualMachine::new_with_program(program);
        vm.run();
        assert_eq!(vm.r[1], 7);
        assert_eq!(vm.r[2], 12);

        //FIXME:
        //assert_eq!(vm.p[1], 12);
        //assert_eq!(vm.p[2], 7);
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
        let mut vm = VirtualMachine::new_with_program(program);
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

        let mut vm = VirtualMachine::new_with_program(program);
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
            Instruction::new_label("loop".to_string(), 3),
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
        let mut vm = VirtualMachine::new_with_program(program);
        vm.run();

        assert_eq!(vm.r[2], 4);
    }

    //TODO:
    // add aspersion, for now apers to be working fine
    #[test]
    pub fn test_ports() {
        // ADD 10
        // MOV acc p0
        // MOV p0 p1
        // MOV p1 r1
        // ADD p0
        // HLT

        // Expected state:
        // acc: 20, r: [0, 10, 0, 0] p: [10, 10, 0, 0]

        let program = vec![
            Instruction::new(Opcode::ADD(Operand::IntegerValue(10))),
            Instruction::new(Opcode::MOV(Operand::ACC, Operand::PortRegister(0))),
            Instruction::new(Opcode::MOV(
                Operand::PortRegister(0),
                Operand::PortRegister(1),
            )),
            Instruction::new(Opcode::MOV(
                Operand::PortRegister(1),
                Operand::GeneralRegister(1),
            )),
            Instruction::new(Opcode::ADD(Operand::PortRegister(0))),
            Instruction::new(Opcode::HLT),
        ];

        let mut vm = VirtualMachine::new_with_program(program);

        vm.run();
        assert_eq!(vm.get_acc(), 20);
        //println!("{}", vm);
    }
}
