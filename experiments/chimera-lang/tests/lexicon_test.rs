use chimera_lang::vm::{ChimeraVM, Value};
use chimera_lang::ast::{Dna, Helix};

#[test]
fn test_lexicon_spell_casting() {
    let dna = Dna {
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;
    vm.orca_mode = false;
    vm.chaos_mode = false;

    // Write "FIRE" horizontally at (5,5)
    vm.grid[5][5] = Value::Str("F".to_string());
    vm.grid[5][6] = Value::Str("I".to_string());
    vm.grid[5][7] = Value::Str("R".to_string());
    vm.grid[5][8] = Value::Str("E".to_string());

    vm.step();

    // Check consumption
    assert_eq!(vm.grid[5][5], Value::Int(0), "F should be consumed");
    assert_eq!(vm.grid[5][6], Value::Int(0), "I should be consumed");
    assert_eq!(vm.grid[5][7], Value::Int(0), "R should be consumed");
    assert_eq!(vm.grid[5][8], Value::Int(0), "E should be consumed");

    // Check log
    let output = vm.output.join("\n");
    assert!(output.to_lowercase().contains("lexicon: casting fire"), "Should cast FIRE");
}

#[test]
fn test_lexicon_vertical() {
    let dna = Dna {
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;
    vm.orca_mode = false;

    // Write "HEAL" vertically at (0,0)
    vm.grid[0][0] = Value::Str("H".to_string());
    vm.grid[1][0] = Value::Str("E".to_string());
    vm.grid[2][0] = Value::Str("A".to_string());
    vm.grid[3][0] = Value::Str("L".to_string());

    vm.step();

    assert_eq!(vm.grid[0][0], Value::Int(0));
    assert_eq!(vm.grid[1][0], Value::Int(0));
    assert_eq!(vm.grid[2][0], Value::Int(0));
    assert_eq!(vm.grid[3][0], Value::Int(0));

    let output = vm.output.join("\n");
    assert!(output.to_lowercase().contains("lexicon: casting photosynthesize"), "Should cast HEAL");
}
