#[cfg(feature = "nova")]
use chimera_lang::ast::{Dna, Helix, Strand};
#[cfg(feature = "nova")]
use chimera_lang::vm::{nova_linguistics, ChimeraVM, Value};

#[cfg(feature = "nova")]
fn make_vm() -> ChimeraVM {
    let genes = vec![];
    let dna = Dna {
        evolution_config: None,
        helix: Helix {
            strands: vec![Strand { genes }],
        },
    };
    ChimeraVM::new(dna)
}

#[test]
#[cfg(feature = "nova")]
fn test_levenshtein_dos_protection() {
    let mut vm = make_vm();
    // Create strings larger than the intended safety limit (1024)
    let s1 = "a".repeat(2048);
    let s2 = "b".repeat(2048);

    vm.stack.push(Value::Str(s1));
    vm.stack.push(Value::Str(s2));

    nova_linguistics::exec_levenshtein(&mut vm);

    // Should NOT put a result on the stack
    assert!(
        vm.stack.is_empty(),
        "Stack should be empty on error, but found {:?}",
        vm.stack
    );

    // Should have an error in output
    let output = vm.output.join("\n");
    assert!(
        output.contains("Error: String too long"),
        "Output: {}",
        output
    );
}

#[test]
#[cfg(feature = "nova")]
fn test_prologue_levenshtein_dos_protection() {
    let mut vm = make_vm();
    vm.prologue_state.active = true;

    let large_s1 = "a".repeat(2048);
    let large_s2 = "b".repeat(2048);

    // Place Rune (5, 6)
    vm.grid[5][6] = Value::Str("≅".to_string());

    // Set Neighbors
    vm.grid[5][5] = Value::Str(large_s1); // West
    vm.grid[4][6] = Value::Str(large_s2); // North

    // Execute Tick
    chimera_lang::vm::prologue::exec_prologue_tick(&mut vm);

    let res = &vm.prologue_state.signal_grid[5][6];

    // We expect signal to be suppressed (None) or an error indicator (not 2048)
    if let Some(Value::Int(n)) = res {
        assert_ne!(*n, 2048, "Should not compute distance for large strings");
        assert_eq!(*n, -1, "Expected -1 for error");
    } else {
        // None is fine
    }
}
