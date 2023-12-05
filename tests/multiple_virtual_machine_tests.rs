use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use simple_virtual_assembler::{
    assembler::assembler::Assembler,
    components::{
        connection::{self, Connection},
        port::Port,
    },
    vm::{
        instruction::Instruction,
        opcodes::Opcode,
        operand::Operand,
        virtual_machine::{VirtualMachine, VmStatus},
    },
};

//TODO:
// Add assertions
// So far seems to be working
#[test]
fn test_basic_connection_between_vms() {
    let mut connection_to_vm2 = Connection::new();
    let mut connection_to_vm1 = connection_to_vm2.clone();
    let mut connection_to_vm2 = connection_to_vm2.clone();

    let handel1 = thread::spawn(move || {
        let mut vm1 = VirtualMachine::new();
        vm1.connect(0, &mut connection_to_vm1);

        let program1 = vec![
            Instruction::new(Opcode::MOV(
                Operand::IntegerValue(5),
                Operand::PortRegister(0),
            )),
            Instruction::new(Opcode::HLT),
        ];
        vm1.load_program(program1);
        vm1.run();
        println!("VM1: \n {vm1}");
    });

    let handel2 = thread::spawn(move || {
        let mut vm2 = VirtualMachine::new();
        vm2.connect(3, &mut connection_to_vm2);

        let program2 = vec![
            Instruction::new(Opcode::MOV(Operand::PortRegister(3), Operand::ACC)),
            Instruction::new(Opcode::HLT),
        ];
        vm2.load_program(program2);
        vm2.run();
        println!("VM2: \n {vm2}");
    });

    handel1.join().unwrap();
    handel2.join().unwrap();
}
//TODO:
// Add assertions
// So far seems to be working
#[test]
fn test_basic_communication_between_vms() {
    // tests communion between vms
    //
    // vm1 p0 is connected to vm2 p0
    // vm1 p1 is connected to vm p1
    //
    // vm1 computes some value that vm2 requires to do its work
    // vm1 uses p0 of its ports to inform vm2 weather the value is ready
    // vm1 uses p1 to send data
    //
    // vm2 awaits for ready signal from vm1 by comparing p0 to some value in an infinite loop
    // when vm1 produces value  vm2 exits infinite loop and proceeds to use that value
    //
    // for this test value 7 will used to indicate that vm1 has produced value
    //
    // at the time of wiring this test instruction to jump to label regardless of flag state
    // is not present, therefore code might be a bit cluny
    //
    // for this test vm1 counts to 200 to simulate some computation
    // vm2 awaits it and divides it by 2
    //
    let vm1_code = r#"
    sum:                #   label used to denote block of code responsible for producing input to vm2 
        MOV 0 p0        #   inform vm2 that value is not ready
        ADD 1           #   increment by one
        CMP acc 200     #   | loop until acc is equal to 200
        JL sum          #   |
    MOV 7 p0            #   inform vm2 that value is ready
    MOV acc p1          #   send value to v2
    HLT                 #
    
    "#;

    let vm2_code = r#"
    await:              #   label used to denote block of code responsible for awaiting input
        CMP p0 7        #   comparing p0 to 7 - value used to indicate that input at p1 is ready
        JE consume      #   if value is ready jump to block of code responsible for its consumption 
        JG await        #   | currently the is no JMP instruction so JG and JL are used instead 
        JL await        #   |
    consume:            #   block of code responsible for input consumption
        MOV p1 acc      #   copying value form p1 to acc
        DIV 2           #   dividing received value by 2
        HLT             #
    HLT                 #
    "#;

    // Connection to indicate if value is ready to be read
    let communication_connection = Connection::new();
    let mut communication_connection_vm1 = communication_connection.clone();
    let mut communication_connection_vm2 = communication_connection.clone();
    // Connection for sending data
    let data_connection = Connection::new();
    let mut data_connection_vm1 = data_connection.clone();
    let mut data_connection_vm2 = data_connection.clone();

    let handel1 = thread::spawn(move || {
        let mut vm1 = VirtualMachine::new();
        vm1.connect(0, &mut communication_connection_vm1);
        vm1.connect(1, &mut data_connection_vm1);
        let mut assebmer1 = Assembler::new();

        let program1 = assebmer1.parse(vm1_code).unwrap();

        vm1.load_program(program1);
        vm1.run();
        println!("VM1: \n {vm1}");
    });

    let handel2 = thread::spawn(move || {
        let mut assebmer2 = Assembler::new();

        let program2 = assebmer2.parse(vm2_code).unwrap();

        let mut vm2 = VirtualMachine::new();
        vm2.connect(0, &mut communication_connection_vm2);
        vm2.connect(1, &mut data_connection_vm2);

        vm2.load_program(program2);
        vm2.run();
        println!("VM2: \n {vm2}");
    });

    handel1.join().unwrap();
    handel2.join().unwrap();
}

#[test]
fn test_vm_start() {
    let code = r#"
    sum:                #   label used to denote block of code responsible for producing input to vm2 
        MOV 0 p0        #   inform vm2 that value is not ready
        ADD 1           #   increment by one
        CMP acc 200     #   | loop until acc is equal to 200
        JL sum          #   |
    MOV 7 p0            #   inform vm2 that value is ready
    MOV acc p1          #   send value to v2
    HLT                 #
    
    "#;
    let mut assembler = Assembler::new();

    let program = assembler.parse(code).unwrap();

    let vm = VirtualMachine::new_with_program(program);
    let vm = Arc::new(Mutex::new(vm));
    let vm_copy = vm.clone();
    let handle = VirtualMachine::start(vm);
    handle.join().unwrap();
    println!("{:?}", vm_copy);
}

#[test]
fn test_vm_start_delayed() {
    let code = r#"
    sum:                #   label used to denote block of code responsible for producing input to vm2 
        MOV 0 p0        #   inform vm2 that value is not ready
        ADD 1           #   increment by one
        CMP acc 20     #   | loop until acc is equal to 200
        JL sum          #   |
    MOV 7 p0            #   inform vm2 that value is ready
    MOV acc p1          #   send value to v2
    HLT                 #
    
    "#;
    let mut assembler = Assembler::new();

    let program = assembler.parse(code).unwrap();

    let mut vm = VirtualMachine::new_with_program(program);
    vm.set_delay(200);
    let vm = Arc::new(Mutex::new(vm));
    //vm.lock().unwrap().set_delay(2000);
    let vm_copy = vm.clone();
    let handle = VirtualMachine::start(vm);

    let mut done = false;
    let mut acc = -1;
    while !done {
        println!("Reading data");
        {
            let data = vm_copy.lock().unwrap();
            acc = data.get_acc();
            if data.get_status() == VmStatus::Finished {
                done = true;
            }
        }
        println!("{}", acc);
        thread::sleep(Duration::from_millis(100))
    }

    handle.join().unwrap();
    println!("{:?}", vm_copy);
}

#[test]
fn test_basic_communication_between_vms_with_simpler_api() {
    // tests communion between vms
    //
    // vm1 p0 is connected to vm2 p0
    // vm1 p1 is connected to vm p1
    //
    // vm1 computes some value that vm2 requires to do its work
    // vm1 uses p0 of its ports to inform vm2 weather the value is ready
    // vm1 uses p1 to send data
    //
    // vm2 awaits for ready signal from vm1 by comparing p0 to some value in an infinite loop
    // when vm1 produces value  vm2 exits infinite loop and proceeds to use that value
    //
    // for this test value 7 will used to indicate that vm1 has produced value
    //
    // at the time of wiring this test instruction to jump to label regardless of flag state
    // is not present, therefore code might be a bit cluny
    //
    // for this test vm1 counts to 200 to simulate some computation
    // vm2 awaits it and divides it by 2
    //
    let vm1_code = r#"
    sum:                #   label used to denote block of code responsible for producing input to vm2 
        MOV 0 p0        #   inform vm2 that value is not ready
        ADD 1           #   increment by one
        CMP acc 200     #   | loop until acc is equal to 200
        JL sum          #   |
    MOV 7 p0            #   inform vm2 that value is ready
    MOV acc p1          #   send value to v2
    HLT                 #
    
    "#;

    let vm2_code = r#"
    await:              #   label used to denote block of code responsible for awaiting input
        CMP p0 7        #   comparing p0 to 7 - value used to indicate that input at p1 is ready
        JE consume      #   if value is ready jump to block of code responsible for its consumption 
        JG await        #   | currently the is no JMP instruction so JG and JL are used instead 
        JL await        #   |
    consume:            #   block of code responsible for input consumption
        MOV p1 acc      #   copying value form p1 to acc
        DIV 8           #   dividing received value by 8
        HLT             #
    HLT                 #
    "#;

    // assembling programs
    let mut assembler1 = Assembler::new();
    let program1 = assembler1.parse(vm1_code).unwrap();
    let mut assembler2 = Assembler::new();
    let program2 = assembler2.parse(vm2_code).unwrap();

    // creating vm's with programs
    let (vm1, vm1_copy) = VirtualMachine::new_shared_with_program(program1);
    let (vm2, vm2_copy) = VirtualMachine::new_shared_with_program(program2);

    // creating connections
    let communication_connection = Connection::new();
    let data_connection = Connection::new();

    let mut communication_coon_to_vm1 = communication_connection.clone();
    let mut communication_coon_to_vm2 = communication_connection.clone();

    let mut data_conn_vm1 = data_connection.clone();
    let mut data_conn_vm2 = data_connection.clone();

    // connecting vms

    {
        vm1.lock()
            .unwrap()
            .connect(0, &mut communication_coon_to_vm1);
    }

    {
        vm1.lock().unwrap().connect(1, &mut data_conn_vm1);
    }

    {
        vm2.lock()
            .unwrap()
            .connect(0, &mut communication_coon_to_vm2);
    }
    {
        vm2.lock().unwrap().connect(1, &mut data_conn_vm2);
    }

    let handel1 = VirtualMachine::start(vm1);
    let handel2 = VirtualMachine::start(vm2);

    handel1.join().unwrap();
    handel2.join().unwrap();

    println!("{}", vm2_copy.lock().unwrap());
}
