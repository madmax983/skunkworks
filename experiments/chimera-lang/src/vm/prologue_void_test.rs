use crate::ast::{Dna, Helix};
use crate::vm::prologue::exec_prologue_tick;
use crate::vm::{ChimeraVM, Value};

fn setup_vm() -> ChimeraVM {
    let dna = Dna {
        evolution_config: None,
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;
    vm
}

#[test]
fn test_vacuum_rune() {
    // µ (Vacuum): Inverts West emptiness.
    let mut vm = setup_vm();
    vm.grid[5][4] = Value::Int(0); // West empty
    vm.grid[5][5] = Value::Str("µ".to_string()); // Self

    exec_prologue_tick(&mut vm);

    if let Some(Value::Int(v)) = &vm.prologue_state.signal_grid[5][5] {
        assert_eq!(*v, 1, "Vacuum should output 1 when West is empty");
    } else {
        panic!("Vacuum failed to activate on empty West");
    }
}

#[test]
fn test_void_in_rune() {
    // Ø (Void In): West (Value) -> Push to Void Buffer.
    let mut vm = setup_vm();
    vm.grid[5][5] = Value::Str("Ø".to_string());

    // Signal at West (5,4).
    // Use Source ! at 5,4 reading 5,3
    vm.grid[5][3] = Value::Int(42);
    vm.grid[5][4] = Value::Str("!".to_string());

    exec_prologue_tick(&mut vm);

    // Buffer should contain 42
    assert_eq!(vm.prologue_state.void_buffer.len(), 1);
    assert_eq!(vm.prologue_state.void_buffer[0], Value::Int(42));

    // Should activate
    if let Some(Value::Int(v)) = &vm.prologue_state.signal_grid[5][5] {
        assert_eq!(*v, 1);
    } else {
        panic!("Void In failed to activate");
    }
}

#[test]
fn test_void_out_rune() {
    // § (Void Out): Pop from Void Buffer -> Self.
    let mut vm = setup_vm();
    vm.grid[5][5] = Value::Str("§".to_string());

    // Pre-fill buffer
    vm.prologue_state.void_buffer.push_back(Value::Int(100));

    exec_prologue_tick(&mut vm);

    // Buffer should be empty
    assert_eq!(vm.prologue_state.void_buffer.len(), 0);

    // Should output 100
    if let Some(Value::Int(v)) = &vm.prologue_state.signal_grid[5][5] {
        assert_eq!(*v, 100);
    } else {
        panic!("Void Out failed to activate");
    }
}
