extern crate simple_virtual_assembler;

use simple_virtual_assembler::assembler;
use simple_virtual_assembler::assembler::assembler::Assembler;
use simple_virtual_assembler::assembler::parsing_err::ParsingError;
use simple_virtual_assembler::vm;
use simple_virtual_assembler::vm::opcodes;
use simple_virtual_assembler::vm::virtual_machine;
use simple_virtual_assembler::vm::virtual_machine::VirtualMachine;

/// Parses and runs program on vm
fn assembler_and_run(program_text: &str) -> Result<VirtualMachine, ParsingError> {
    let program = Assembler::new().parse(program_text)?;

    let mut vm =
        simple_virtual_assembler::vm::virtual_machine::VirtualMachine::new_with_program(program);
    vm.run();

    Ok(vm)
}

/// Parses and runs program on vm, displays vm state between each instruction
fn assembler_and_with_prints(program_text: &str) -> Result<VirtualMachine, ParsingError> {
    let program = Assembler::new().parse(program_text)?;

    let mut vm =
        simple_virtual_assembler::vm::virtual_machine::VirtualMachine::new_with_program(program);

    vm.set_delay(500);

    let mut running = true;
    let mut counter = 0;

    while running {
        println!(
            "__________________________{}__________________________",
            counter
        );
        println!("{}", vm);
        println!("______________________________________________________");
        running = vm.execute();
        counter += 1;
    }

    Ok(vm)
}

#[test]
fn assembling_and_running_simple_program1() {
    let program_text = r#"
    MOV 10 acc
    loop:
        ADD 8
        CMP acc 200
        JL loop
    HLT
    "#;

    let mut assembler = Assembler::new();

    let result = assembler.parse(program_text);

    assert!(result.is_ok());

    let program = result.unwrap();

    let mut vm = vm::virtual_machine::VirtualMachine::new_with_program(program);
    vm.run();

    println!("{}", vm);

    assert_eq!(vm.get_acc(), 202);
}

#[test]
fn assembling_and_running_invalid_code_by_subtracting() {
    // Error on first line MOV requires 2 operands
    let program = r#"
    MOV 10
    loop:
        ADD 8
        CMP acc 200
        JL loop
    HLT
    "#;

    let result = assembler_and_run(program);

    assert!(result.is_err());
}

#[test]
fn assembling_and_running_division_by_subtracting() {
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
    //      JG LOOP         # Jump if current value is grater that 0
    // hlt

    // Expected result r2 = 4

    let program = r#"
        MOV 20 r0
        MOV 5 r1
        MOV 0 r2
        loop:
            MOV r0 acc
            SUB r1
            MOV acc r0
            MOV r2 acc
            ADD 1
            MOV acc r2
            CMP r0 0
            JG loop
        HLT
        "#;
    let result = assembler_and_run(program);

    match result {
        Ok(vm) => println!("{}", vm),
        Err(e) => println!("{:?}", e),
    }
}

#[test]
fn test_stack() {
    let program_text = r#"
    push_stk:
        inc
        psh acc
        cmp acc 40
        jl push_stk
    pop acc
    pop r0
    pop r1
    pop r2
    pop r3
    HLT
    "#;
    
    let mut assembler= Assembler::new().with_stack();
    let result = assembler.parse(program_text);
    match result {
        Ok(program) => {
            let mut vm = VirtualMachine::new().with_stack(32);
            vm.load_program(program);
            vm.run();
            println!("{:?}", vm);
        }
        Err(err) => println!("{}", err),
    };
    

}
