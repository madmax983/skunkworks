use crate::ast::{Dna, Helix};
use crate::vm::prologue::exec_prologue_tick;
use crate::vm::{ChimeraVM, Value};

fn setup_vm() -> ChimeraVM {
    let dna = Dna {
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;
    vm
}

#[test]
fn test_vacuum_rune() {
    // µ (Vacuum): Inverts West emptiness.
    // Case 1: West is empty (Int(0)). Should output 1.
    let mut vm = setup_vm();
    vm.grid[5][4] = Value::Int(0); // West
    vm.grid[5][5] = Value::Str("µ".to_string()); // Self

    exec_prologue_tick(&mut vm);

    if let Some(Value::Int(v)) = &vm.prologue_state.signal_grid[5][5] {
        assert_eq!(*v, 1, "Vacuum should output 1 when West is empty");
    } else {
        panic!("Vacuum failed to activate on empty West");
    }

    // Case 2: West is not empty (Int(42)). Should output None/0.
    let mut vm2 = setup_vm();
    vm2.grid[5][4] = Value::Int(42); // West
    vm2.grid[5][5] = Value::Str("µ".to_string()); // Self

    // We need a source to make 42 a signal
    // Wait, apply_void_runes checks *current_signals*.
    // So 42 must be in current_signals.
    // Signals are populated from delayed_signals OR Sources (!).
    // Let's use a Source ! at 5,4 reading from 4,4.
    vm2.grid[4][4] = Value::Int(42);
    vm2.grid[5][4] = Value::Str("!".to_string());

    exec_prologue_tick(&mut vm2);

    // Check if signal reached 5,4 (West of Vacuum)
    assert!(
        vm2.prologue_state.signal_grid[5][4].is_some(),
        "Source should emit signal"
    );

    // Check Vacuum at 5,5
    if let Some(Value::Int(v)) = &vm2.prologue_state.signal_grid[5][5] {
        // Wait, if next_signals is not set, it remains None (default).
        // My implementation only sets it to 1 if empty.
        // So it should be None.
        // BUT, process_signal_propagation copies current to next initially?
        // No: `let mut next_signals = vm.prologue_state.signal_grid.clone();`
        // So if 5,5 had a signal, it keeps it. But 5,5 starts empty.
        // So it should be None.
        assert_ne!(*v, 1, "Vacuum should NOT output 1 when West is signal");
    }
}

#[test]
fn test_void_anchor_rune() {
    // Ø (Void Anchor): Activates if N, S, E, W are empty.
    let mut vm = setup_vm();
    vm.grid[5][5] = Value::Str("Ø".to_string());

    // All neighbors empty by default.
    exec_prologue_tick(&mut vm);

    if let Some(Value::Int(v)) = &vm.prologue_state.signal_grid[5][5] {
        assert_eq!(*v, 1, "Anchor should output 1 when neighbors empty");
    } else {
        panic!("Anchor failed to activate");
    }

    // Case 2: One neighbor has signal.
    let mut vm2 = setup_vm();
    vm2.grid[5][5] = Value::Str("Ø".to_string());

    // Signal at North (4,5)
    vm2.grid[3][5] = Value::Int(99);
    vm2.grid[4][5] = Value::Str("!".to_string());

    exec_prologue_tick(&mut vm2);

    assert!(vm2.prologue_state.signal_grid[4][5].is_some());

    if let Some(Value::Int(v)) = &vm2.prologue_state.signal_grid[5][5] {
        // Should not be 1
        assert_ne!(*v, 1);
    }
}

#[test]
fn test_singularity_rune() {
    // § (Singularity): Consumes neighbor signals.
    let mut vm = setup_vm();

    // Setup signals around 5,5
    // North: 4,5
    vm.grid[3][5] = Value::Int(10);
    vm.grid[4][5] = Value::Str("!".to_string());

    // West: 5,4
    vm.grid[4][4] = Value::Int(20);
    vm.grid[5][4] = Value::Str("!".to_string());

    // Center: 5,5
    vm.grid[5][5] = Value::Str("§".to_string());

    // Exec
    exec_prologue_tick(&mut vm);

    // Singularity itself should be active
    if let Some(Value::Int(v)) = &vm.prologue_state.signal_grid[5][5] {
        assert_eq!(*v, 1, "Singularity should be active");
    } else {
        panic!("Singularity not active");
    }

    // Neighbors should be consumed (None)
    // Note: Source ! emits signal. Singularity consumes it.
    // ! runs, sets signal. Singularity runs, clears signal.
    // Order depends on iteration.
    // If Singularity runs last, it clears.
    // If Singularity runs first, it clears (but they might not be set yet? No, current_signals is fixed).
    // Wait, Singularity clears *next_signals*.
    // ! writes to *next_signals* (via prepare_signals? No, prepare_signals runs BEFORE propagation).
    // `prepare_signals` sets `vm.prologue_state.signal_grid` (which becomes `current_signals`).

    // `process_signal_propagation` loop:
    // `next_signals` starts as copy of `current_signals`.
    // Runes modify `next_signals`.

    // ! is processed in `prepare_signals`. So signals at 4,5 and 5,4 ARE in `current_signals`.
    // So `next_signals` starts with them present.

    // Singularity rune modifies `next_signals` to clear neighbors.
    // Unless another rune re-writes them?
    // ! is not a propagation rune. It's handled in prepare.
    // So nothing else writes to 4,5 or 5,4 in propagation phase (unless wires/gates).
    // Here we just have ! and §.

    // So Singularity should successfully clear them.

    assert!(
        vm.prologue_state.signal_grid[4][5].is_none(),
        "North signal should be consumed"
    );
    assert!(
        vm.prologue_state.signal_grid[5][4].is_none(),
        "West signal should be consumed"
    );
}
