use chimera_lang::vm::{ChimeraVM, Value};
use chimera_lang::opcode::OpCode;
use chimera_lang::ast::{Dna, Helix};

#[test]
fn test_chimeric_recursion_limit() {
    let dna = Dna {
        helix: Helix {
            strands: vec![],
        },
    };
    let mut vm = ChimeraVM::new(dna);

    // Create deep recursion using "run" command.
    // Chimeric "run" pops a string and executes it.
    // If we stack many "run" strings, we get deep Rust recursion.
    // Each "run" execution recurses into exec_chimeric_source.

    // Push "run" 20,000 times (enough to blow standard 2MB stack)
    for _ in 0..20000 {
        vm.stack.push(Value::Str("run".to_string()));
    }

    // The script to start the chain is just "run"
    // We pass this as the argument to Chimeric OpCode?
    // No, OpCode::Chimeric expects script on stack.
    // So we push one more "run" (the script itself).
    vm.stack.push(Value::Str("run".to_string()));

    #[cfg(feature = "nova")]
    {
        // This should trigger infinite recursion on the Rust stack.
        // We expect it to eventually hit a recursion limit check (once implemented).
        // Currently, it will stack overflow (crash).
        chimera_lang::vm::nova_chimeric::exec_chimeric_op(&mut vm, OpCode::Chimeric, &[]);

        // Assert that we didn't crash and got an error message
        let error_found = vm.output.iter().any(|s| s.contains("Recursion limit exceeded"));
        assert!(error_found, "VM did not report recursion limit exceeded. Output: {:?}", vm.output);
    }
}
