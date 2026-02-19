use chimera_lang::ast::{Dna, Gene, Helix, Strand};
use chimera_lang::opcode::OpCode;
use chimera_lang::vm::ChimeraVM;
use chimera_lang::vm::Value;

#[test]
fn test_sigil_strand_oob() {
    let dummy_strand = Strand {
        genes: vec![Gene {
            op: OpCode::Nop,
            args: vec![],
        }],
    };
    let dna = Dna {
        helix: Helix {
            strands: vec![dummy_strand],
        },
    };
    let mut vm = ChimeraVM::new(dna);

    // Setup:
    // Grid size is 16x16.
    // Context loc is default (8,8).
    // Write 100 to (8,9) to create a valid pattern for Inscribe.
    vm.grid[8][9] = Value::Int(100);

    // Push arguments for Inscribe
    vm.stack.push(Value::Int(1000)); // Strand Index (Out of Bounds)
    vm.stack.push(Value::Int(1)); // Radius
    vm.stack.push(Value::Str("CrashSigil".to_string())); // Name

    // Execute Inscribe
    let _ = chimera_lang::vm::nova_sigil::exec_inscribe(&mut vm, OpCode::Inscribe, &[]);

    // Check if Sigil was registered (if fix is NOT applied, it will be)
    // If fix IS applied, this should be None.
    if vm.sigil_registry.contains_key("CrashSigil") {
        println!("Sigil 'CrashSigil' registered (Vulnerable). Proceeding to crash.");

        // Enable AutoCast
        vm.stack.push(Value::Str("CrashSigil".to_string()));
        vm.stack.push(Value::Int(1)); // 1 = True
        let _ = chimera_lang::vm::nova_sigil::exec_auto_cast(&mut vm, OpCode::AutoCast, &[]);

        // Step 1: Process passive sigils -> Spawns organelle with IP (1000, 0)
        vm.step();

        // At this point, an organelle is spawned.
        // Step 2: Process organelles -> Executes organelle DNA -> PANIC
        vm.step();

        // Debug output if we survived
        for msg in &vm.output {
            println!("VM Output: {}", msg);
        }
    } else {
        println!("Sigil 'CrashSigil' NOT registered (Fixed).");
        // Verify output contains error
        assert!(vm
            .output
            .iter()
            .any(|s| s.contains("Error: Strand index out of bounds")));
    }
}

#[test]
fn test_babel_compile_stack_overflow() {
    let dummy_strand = Strand { genes: vec![] };
    let dna = Dna {
        helix: Helix {
            strands: vec![dummy_strand],
        },
    };
    let mut vm = ChimeraVM::new(dna);

    // Build a deeply nested structure
    vm.stack.push(Value::Str("a".to_string()));
    // ParserMatch pops 1 item
    let _ = chimera_lang::vm::babel::exec_babel_op(&mut vm, OpCode::ParserMatch, &[]);

    // Now loop to wrap (Linear depth using ParserMany)
    for _ in 0..100000 {
        // ParserMany: Pops 1 item. Pushes Junction(Many, item).
        let _ = chimera_lang::vm::babel::exec_babel_op(&mut vm, OpCode::ParserMany, &[]);
    }

    println!("Constructed huge parser. Compiling...");

    // Now compile
    vm.stack.push(Value::Int(0)); // Handler

    // This should crash with stack overflow
    let _ = chimera_lang::vm::babel::exec_babel_op(&mut vm, OpCode::BabelCompile, &[]);
    println!("Compilation finished (Survived).");
}
