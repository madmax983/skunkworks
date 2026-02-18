use crate::ast::{Dna, Helix};
use crate::vm::prologue::exec_prologue_tick;
use crate::vm::{ChimeraVM, Value};

fn make_vm() -> ChimeraVM {
    let dna = Dna {
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;
    vm
}

#[test]
fn test_transmute_to_string() {
    let mut vm = make_vm();
    // Setup:
    // Value 42 at (5,4).
    // ! at (6,4) -> Emits 42 to (6,4) [West of t].
    // t at (6,5) -> Reads West (42), Default Mode 0.

    vm.grid[5][4] = Value::Int(42);
    vm.grid[6][4] = Value::Str("!".to_string());
    vm.grid[6][5] = Value::Str("t".to_string());

    { let mut state = std::mem::take(&mut vm.prologue_state); exec_prologue_tick(&mut vm, &mut state); vm.prologue_state = state; }

    assert_eq!(
        vm.prologue_state.signal_grid[6][5],
        Some(Value::Str("42".to_string()))
    );
}

#[test]
fn test_transmute_to_int() {
    let mut vm = make_vm();
    // Setup:
    // Value "100" at (5,4).
    // ! at (6,4) -> Emits "100" to (6,4) [West of t].

    // Mode 1 at (4,5).
    // ! at (5,5) -> Emits 1 to (5,5) [North of t].

    // t at (6,5) -> Reads West ("100"), North (1).

    vm.grid[5][4] = Value::Str("100".to_string());
    vm.grid[6][4] = Value::Str("!".to_string());

    vm.grid[4][5] = Value::Int(1);
    vm.grid[5][5] = Value::Str("!".to_string());

    vm.grid[6][5] = Value::Str("t".to_string());

    { let mut state = std::mem::take(&mut vm.prologue_state); exec_prologue_tick(&mut vm, &mut state); vm.prologue_state = state; }

    assert_eq!(vm.prologue_state.signal_grid[6][5], Some(Value::Int(100)));
}

#[test]
fn test_fuse_string_int() {
    let mut vm = make_vm();
    // Setup:
    // West Input: "A" at (4,4).
    // ! at (5,4) -> Emits "A" to (5,4).

    // East Input: 3 at (4,6).
    // ! at (5,6) -> Emits 3 to (5,6).

    // Fuse at (5,5). Reads West (5,4) and East (5,6).

    vm.grid[4][4] = Value::Str("A".to_string());
    vm.grid[5][4] = Value::Str("!".to_string());

    vm.grid[4][6] = Value::Int(3);
    vm.grid[5][6] = Value::Str("!".to_string());

    vm.grid[5][5] = Value::Str("f".to_string());

    { let mut state = std::mem::take(&mut vm.prologue_state); exec_prologue_tick(&mut vm, &mut state); vm.prologue_state = state; }

    assert_eq!(
        vm.prologue_state.signal_grid[5][5],
        Some(Value::Str("AAA".to_string()))
    );
}

#[test]
fn test_distill_string() {
    let mut vm = make_vm();
    // Setup:
    // Input "abc" at (4,4).
    // ! at (5,4) -> Emits "abc" to (5,4).

    // Distill at (5,5). Reads West (5,4).

    vm.grid[4][4] = Value::Str("abc".to_string());
    vm.grid[5][4] = Value::Str("!".to_string());

    vm.grid[5][5] = Value::Str("d".to_string());

    { let mut state = std::mem::take(&mut vm.prologue_state); exec_prologue_tick(&mut vm, &mut state); vm.prologue_state = state; }

    // Output North (4,5) -> "a"
    assert_eq!(
        vm.prologue_state.signal_grid[4][5],
        Some(Value::Str("a".to_string()))
    );
    // Output South (6,5) -> "bc"
    assert_eq!(
        vm.prologue_state.signal_grid[6][5],
        Some(Value::Str("bc".to_string()))
    );
}
