use crate::ast::{Dna, Helix};
use crate::vm::prologue::exec_prologue_tick;
use crate::vm::{ChimeraVM, Value};
use crate::vm::nova_void::VoidRift;

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
fn test_void_out_drains_rift() {
    // § (Void Out): If buffer empty, drains local Rift.
    let mut vm = setup_vm();

    // Setup Rift at (5,5) with severity 10
    let mut rift = VoidRift::new((5, 5));
    rift.severity = 10;
    vm.void_rifts.push(rift);

    // Place § at (5,5)
    vm.grid[5][5] = Value::Str("§".to_string());

    // Ensure buffer empty
    vm.prologue_state.void_buffer.clear();

    exec_prologue_tick(&mut vm);

    // Should have activated
    if let Some(val) = &vm.prologue_state.signal_grid[5][5] {
        // Should emit severity or something related
        // For now, let's assume implementation emits severity
        // or just checks if it emitted *something*
        println!("Emitted: {:?}", val);
    } else {
        panic!("Void Out failed to activate on Rift");
    }

    // Rift severity should decrease
    let rift = &vm.void_rifts[0];
    assert!(rift.severity < 10, "Rift severity did not decrease");
}

#[test]
fn test_infinity_rune_peek() {
    // ꝏ (Infinity): Peeks buffer non-destructively.
    let mut vm = setup_vm();

    // Place ꝏ at (5,5)
    vm.grid[5][5] = Value::Str("ꝏ".to_string());

    // Push to buffer
    vm.prologue_state.void_buffer.push_back(Value::Int(42));

    exec_prologue_tick(&mut vm);

    // Should emit 42
    if let Some(Value::Int(v)) = &vm.prologue_state.signal_grid[5][5] {
        assert_eq!(*v, 42);
    } else {
        panic!("Infinity rune failed to peek");
    }

    // Buffer should still have 42
    assert_eq!(vm.prologue_state.void_buffer.len(), 1);
    assert_eq!(vm.prologue_state.void_buffer[0], Value::Int(42));
}
