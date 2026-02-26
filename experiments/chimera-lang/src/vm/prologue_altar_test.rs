use crate::ast::{Dna, Helix};
use crate::vm::prologue::exec_prologue_tick;
use crate::vm::{ChimeraVM, Value};

#[test]
fn test_altar_elemental_convergence() {
    let dna = Dna {
        evolution_config: None,
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // Center (5,5)
    //   Δ
    // ◊ ⛩ ○
    //   ∇

    vm.grid[4][5] = Value::Str("Δ".to_string()); // North - Fire
    vm.grid[6][5] = Value::Str("∇".to_string()); // South - Water
    vm.grid[5][4] = Value::Str("◊".to_string()); // West - Earth
    vm.grid[5][6] = Value::Str("○".to_string()); // East - Air
    vm.grid[5][5] = Value::Str("⛩".to_string()); // Center - Altar

    let initial_energy = vm.energy;

    exec_prologue_tick(&mut vm);

    // Check Consumption
    assert_eq!(vm.grid[4][5], Value::Int(0), "Fire should be consumed");
    assert_eq!(vm.grid[6][5], Value::Int(0), "Water should be consumed");
    assert_eq!(vm.grid[5][4], Value::Int(0), "Earth should be consumed");
    assert_eq!(vm.grid[5][6], Value::Int(0), "Air should be consumed");

    // Check Spirit Spawn
    if let Value::Str(s) = &vm.grid[5][5] {
        assert_eq!(s, "Φ", "Altar should become Spirit");
    } else {
        panic!("Altar did not transform");
    }

    // Check Energy
    assert!(vm.energy > initial_energy, "Should gain energy");

    // Check Output
    let output = vm.output.join("\n");
    assert!(output.contains("ALTAR: Elemental Convergence"));
}

#[test]
fn test_altar_void_ritual() {
    let dna = Dna {
        evolution_config: None,
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // Center (5,5)
    //   Ø
    // Ø ⛩ Ø
    //   Ø

    vm.grid[4][5] = Value::Str("Ø".to_string());
    vm.grid[6][5] = Value::Str("Ø".to_string());
    vm.grid[5][4] = Value::Str("Ø".to_string());
    vm.grid[5][6] = Value::Str("Ø".to_string());
    vm.grid[5][5] = Value::Str("⛩".to_string());

    exec_prologue_tick(&mut vm);

    // Check Consumption
    assert_eq!(vm.grid[4][5], Value::Int(0));
    assert_eq!(vm.grid[6][5], Value::Int(0));
    assert_eq!(vm.grid[5][4], Value::Int(0));
    assert_eq!(vm.grid[5][6], Value::Int(0));

    // Check Portal Spawn
    if let Value::Str(s) = &vm.grid[5][5] {
        assert_eq!(s, "ꝏ", "Altar should become Infinity Portal");
    } else {
        panic!("Altar did not transform");
    }

    // Check Output
    let output = vm.output.join("\n");
    assert!(output.contains("ALTAR: Void Ritual"));
}
