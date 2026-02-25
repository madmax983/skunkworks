use crate::ast::Dna;
use crate::ast::Helix;
use crate::vm::prologue::exec_prologue_tick;
use crate::vm::{ChimeraVM, Value};

#[test]
fn test_phonetics_analyze() {
    // We can't access private `analyze_phonetics` directly unless we make it pub or test via VM.
    // However, for unit testing logic, it's better to test via VM execution.
    // Or we can duplicate logic in test, but that's bad.
    // I'll test via VM side effects.
}

#[test]
fn test_phonetic_transmutation() {
    let dna = Dna {
        evolution_config: None,
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // Setup:
    // "Rock" -> Æ -> (Should be Earth ◊)
    vm.grid[5][4] = Value::Str("Rock".to_string());
    vm.grid[5][5] = Value::Str("Æ".to_string());
    // Use delayed_signals because prepare_signals clears signal_grid at start of tick
    vm.prologue_state.delayed_signals[5][4] = Some(Value::Str("Rock".to_string()));

    exec_prologue_tick(&mut vm);

    // Check South of Æ (5,5) -> (6,5)
    let result = &vm.grid[6][5];
    assert_eq!(*result, Value::Str("◊".to_string()), "Rock should be Earth");
}

#[test]
fn test_phonetic_transmutation_air() {
    let dna = Dna {
        evolution_config: None,
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // "Hiss" -> Air (Fricatives)
    // H (Air), I (Fire), S (Air), S (Air) -> 3 Air, 1 Fire. Air wins.
    vm.grid[5][4] = Value::Str("Hiss".to_string());
    vm.grid[5][5] = Value::Str("Æ".to_string());
    vm.prologue_state.delayed_signals[5][4] = Some(Value::Str("Hiss".to_string()));

    exec_prologue_tick(&mut vm);

    let result = &vm.grid[6][5];
    assert_eq!(*result, Value::Str("○".to_string()), "Hiss should be Air");
}

#[test]
fn test_phonetic_transmutation_fire() {
    let dna = Dna {
        evolution_config: None,
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // "Aaaa" -> Fire
    vm.grid[5][4] = Value::Str("Aaaa".to_string());
    vm.grid[5][5] = Value::Str("Æ".to_string());
    vm.prologue_state.delayed_signals[5][4] = Some(Value::Str("Aaaa".to_string()));

    exec_prologue_tick(&mut vm);

    let result = &vm.grid[6][5];
    assert_eq!(*result, Value::Str("Δ".to_string()), "Aaaa should be Fire");
}

#[test]
fn test_phonetic_transmutation_water() {
    let dna = Dna {
        evolution_config: None,
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // "Lull"
    // L (Water), U (Fire), L (Water), L (Water).
    // 3 Water, 1 Fire. Water wins.
    vm.grid[5][4] = Value::Str("Lull".to_string());
    vm.grid[5][5] = Value::Str("Æ".to_string());
    vm.prologue_state.delayed_signals[5][4] = Some(Value::Str("Lull".to_string()));

    exec_prologue_tick(&mut vm);

    let result = &vm.grid[6][5];
    assert_eq!(*result, Value::Str("∇".to_string()), "Lull should be Water");
}
