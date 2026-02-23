use chimera_lang::ast::{Dna, Helix};
use chimera_lang::vm::prologue::exec_prologue_tick;
use chimera_lang::vm::{ChimeraVM, Value};

#[test]
fn test_narrative_twist() {
    let dna = Dna {
        evolution_config: None,
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // "The End" -> ! -> ? -> (Twist)
    // Twists: [" suddenly died", " woke up", " found a key", " was a dream"]
    // "The End" len = 7.
    // 7 % 4 = 3. -> " was a dream"
    // Result: "The End was a dream"

    vm.grid[5][3] = Value::Str("The End".to_string());
    vm.grid[5][4] = Value::Str("!".to_string());
    vm.grid[5][5] = Value::Str("?".to_string());

    exec_prologue_tick(&mut vm);

    assert_eq!(
        vm.prologue_state.signal_grid[5][6],
        Some(Value::Str("The End was a dream".to_string()))
    );
}

#[test]
fn test_narrative_revision_len() {
    let dna = Dna {
        evolution_config: None,
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // "hello" -> ! -> ✍ <- ! <- "len"
    vm.grid[5][3] = Value::Str("hello".to_string());
    vm.grid[5][4] = Value::Str("!".to_string());

    vm.grid[4][4] = Value::Str("len".to_string());
    vm.grid[4][5] = Value::Str("!".to_string());

    vm.grid[5][5] = Value::Str("✍".to_string());

    exec_prologue_tick(&mut vm);

    assert_eq!(
        vm.prologue_state.signal_grid[5][6],
        Some(Value::Str("5".to_string()))
    );
}
