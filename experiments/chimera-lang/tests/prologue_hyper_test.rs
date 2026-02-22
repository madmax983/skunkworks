use chimera_lang::ast::{Dna, Helix};
use chimera_lang::vm::prologue::exec_prologue_tick;
use chimera_lang::vm::{ChimeraVM, Value};

#[test]
fn test_hyper_tesseract_source() {
    let dna = Dna {
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // Manually inject a hyper coord for testing source emission
    vm.prologue_state
        .hyper_state
        .extra_dims
        .insert((5, 5), (0, 42));

    // Setup Tesseract at 5,5
    vm.grid[5][5] = Value::Str("▣".to_string());

    exec_prologue_tick(&mut vm);

    // Should emit 42 to self
    assert_eq!(vm.prologue_state.signal_grid[5][5], Some(Value::Int(42)));
}

#[test]
fn test_hyper_step() {
    let dna = Dna {
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // Setup: 10 (Int) -> ! -> ~ -> ⇪ (Ascend)
    vm.grid[5][2] = Value::Int(10);
    vm.grid[5][3] = Value::Str("!".to_string());
    vm.grid[5][4] = Value::Str("~".to_string());
    vm.grid[5][5] = Value::Str("⇪".to_string());

    exec_prologue_tick(&mut vm);

    // Check if target at 6,5 (South of ⇪) has modified W coord
    let coords = vm.prologue_state.hyper_state.extra_dims.get(&(6, 5));
    assert!(coords.is_some(), "Expected hyper-coords at (6,5)");
    if let Some((_z, w)) = coords {
        assert_eq!(*w, 10);
    }
}

#[test]
fn test_hyper_rotate() {
    let dna = Dna {
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // Circuit: 90 -> ! -> ↻
    vm.grid[5][3] = Value::Int(90);
    vm.grid[5][4] = Value::Str("!".to_string());
    vm.grid[5][5] = Value::Str("↻".to_string());

    exec_prologue_tick(&mut vm);

    assert_eq!(vm.prologue_state.hyper_state.rotation, 90.0);
}
