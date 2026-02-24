use chimera_lang::ast::{Dna, Helix};
use chimera_lang::vm::prologue::exec_prologue_tick;
use chimera_lang::vm::{ChimeraVM, Value, GRID_SIZE};

fn create_vm() -> ChimeraVM {
    let dna = Dna {
        helix: Helix { strands: vec![] },
        evolution_config: None,
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;
    vm
}

#[test]
fn test_pilot_movement() {
    let mut vm = create_vm();

    // Setup Pilot at (5,5)
    vm.grid[5][5] = Value::Str("⚓".to_string());

    // Step 1: Initialize (Scan)
    exec_prologue_tick(&mut vm);

    // Pilot should be registered.
    // Default velocity is (0, 1) -> East.
    // So it should move to (5, 6).

    assert_eq!(
        vm.grid[5][5],
        Value::Int(0),
        "Old position should be empty (0)"
    );
    assert_eq!(
        vm.grid[5][6],
        Value::Str("⚓".to_string()),
        "Pilot should be at (5, 6)"
    );
}

#[test]
fn test_pilot_interaction_and_restoration() {
    let mut vm = create_vm();

    // Setup Track:
    // (5,5): ⚓
    // (5,6): 3
    // (5,7): 5
    // (5,8): +
    // (5,9): !

    vm.grid[5][5] = Value::Str("⚓".to_string());
    vm.grid[5][6] = Value::Int(3);
    vm.grid[5][7] = Value::Int(5);
    vm.grid[5][8] = Value::Str("+".to_string());
    vm.grid[5][9] = Value::Str("!".to_string());

    // Tick 1: Scan. Pilot initialized. Moves to (5,6).
    exec_prologue_tick(&mut vm);
    assert_eq!(vm.grid[5][6], Value::Str("⚓".to_string()));
    assert_eq!(vm.grid[5][5], Value::Int(0)); // Restored 0

    // Tick 2: Pilot reads 'underfoot' (3). Stack [3].
    // Moves to (5,7). New underfoot is 5.
    exec_prologue_tick(&mut vm);
    assert_eq!(vm.grid[5][7], Value::Str("⚓".to_string()));
    assert_eq!(vm.grid[5][6], Value::Int(3)); // Restored 3

    // Tick 3: Pilot reads 'underfoot' (5). Stack [3, 5].
    // Moves to (5,8). New underfoot is +.
    exec_prologue_tick(&mut vm);
    assert_eq!(vm.grid[5][8], Value::Str("⚓".to_string()));
    assert_eq!(vm.grid[5][7], Value::Int(5)); // Restored 5

    // Tick 4: Pilot reads 'underfoot' (+). Stack [8].
    // Moves to (5,9). New underfoot is !.
    exec_prologue_tick(&mut vm);
    assert_eq!(vm.grid[5][9], Value::Str("⚓".to_string()));
    assert_eq!(vm.grid[5][8], Value::Str("+".to_string())); // Restored +

    // Tick 5: Pilot reads 'underfoot' (!). Emits 8 to Signal Grid at (5,9).
    // Moves to (5,10).
    exec_prologue_tick(&mut vm);
    assert_eq!(vm.grid[5][10], Value::Str("⚓".to_string()));
    assert_eq!(vm.grid[5][9], Value::Str("!".to_string())); // Restored !

    // Check Delayed Signal Grid immediately
    if let Some(Value::Int(v)) = &vm.prologue_state.delayed_signals[5][9] {
        assert_eq!(*v, 8, "Delayed signal should contain emitted 8");
    } else {
        panic!(
            "Delayed signal not found at (5,9). Found: {:?}",
            vm.prologue_state.delayed_signals[5][9]
        );
    }
}
