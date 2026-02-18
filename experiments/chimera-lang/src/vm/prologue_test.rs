use crate::vm::{ChimeraVM, Value};
use crate::ast::{Dna, Helix};
use crate::vm::prologue::exec_prologue_tick;

fn setup_vm() -> ChimeraVM {
    let dna = Dna { helix: Helix { strands: vec![] } };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;
    vm
}

#[test]
fn test_scribe_rune() {
    let mut vm = setup_vm();
    // Setup: 42 -> ! -> $
    // ! at 6,4 (reading 5,4).
    // $ at 6,5 (reading West 6,4).
    // Scribe should write to South (7,5).
    vm.grid[5][4] = Value::Int(42);
    vm.grid[6][4] = Value::Str("!".to_string());
    vm.grid[6][5] = Value::Str("$".to_string());

    exec_prologue_tick(&mut vm);

    // Check Grid at 7,5
    assert_eq!(vm.grid[7][5], Value::Int(42));
}

#[test]
fn test_jump_rune() {
    let mut vm = setup_vm();
    // Setup: 42 -> ! -> ^ -> ~
    // ! at 6,4.
    // ^ at 6,5 (Reads West 6,4, Emits East 6,6).
    // ~ at 6,6.

    vm.grid[5][4] = Value::Int(42);
    vm.grid[6][4] = Value::Str("!".to_string());
    vm.grid[6][5] = Value::Str("^".to_string());
    vm.grid[6][6] = Value::Str("~".to_string());

    exec_prologue_tick(&mut vm);

    // Check wire at 6,6
    if let Some(Value::Int(v)) = &vm.prologue_state.signal_grid[6][6] {
        assert_eq!(*v, 42);
    } else {
        panic!("Jump did not propagate signal to East");
    }
}

#[test]
fn test_modulo_rune() {
    let mut vm = setup_vm();
    // % : West % East -> Self
    // West Input: (6,4) -> 10
    // East Input: (6,6) -> 3
    // % at (6,5)

    vm.grid[5][4] = Value::Int(10);
    vm.grid[6][4] = Value::Str("!".to_string()); // Source 10

    vm.grid[5][6] = Value::Int(3);
    vm.grid[6][6] = Value::Str("!".to_string()); // Source 3

    vm.grid[6][5] = Value::Str("%".to_string());

    exec_prologue_tick(&mut vm);

    // Check % at 6,5. Should be 10 % 3 = 1.
    if let Some(Value::Int(v)) = &vm.prologue_state.signal_grid[6][5] {
        assert_eq!(*v, 1);
    } else {
        panic!("Modulo did not output result");
    }
}

#[test]
fn test_mutate_rune() {
    let mut vm = setup_vm();
    // M reads West, writes South.
    // 42 -> ! -> M
    // ! at 6,4. M at 6,5.
    // Write random to 7,5.

    vm.grid[5][4] = Value::Int(42);
    vm.grid[6][4] = Value::Str("!".to_string());
    vm.grid[6][5] = Value::Str("M".to_string());
    vm.grid[7][5] = Value::Int(999); // Initial value

    exec_prologue_tick(&mut vm);

    // Check 7,5 is not 999
    // And is between 0..100
    if let Value::Int(v) = vm.grid[7][5] {
        assert!(v >= 0 && v < 100);
        assert_ne!(v, 999);
    } else {
        panic!("Mutate did not write integer");
    }
}

#[test]
fn test_organelle_rune() {
    let mut vm = setup_vm();
    // O reads West, Spawns Agent South.
    // ! -> O

    vm.grid[5][4] = Value::Int(1);
    vm.grid[6][4] = Value::Str("!".to_string());
    vm.grid[6][5] = Value::Str("O".to_string());
    vm.grid[7][5] = Value::Int(0);

    exec_prologue_tick(&mut vm);

    // Check grid has "@"
    if let Value::Str(s) = &vm.grid[7][5] {
        assert_eq!(s, "@");
    } else {
        panic!("Organelle did not spawn agent symbol");
    }

    // Check O lit up
    assert!(vm.prologue_state.signal_grid[6][5].is_some());
}
