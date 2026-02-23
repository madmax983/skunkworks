use chimera_lang::ast::{Dna, Helix};
use chimera_lang::vm::prologue::exec_prologue_tick;
use chimera_lang::vm::{ChimeraVM, Value};

#[test]
fn test_narrative_incipit() {
    let dna = Dna { evolution_config: None,
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // 5 -> ! -> α -> (Hero)
    vm.grid[5][3] = Value::Int(5);
    vm.grid[5][4] = Value::Str("!".to_string());
    vm.grid[5][5] = Value::Str("α".to_string());

    exec_prologue_tick(&mut vm);

    assert_eq!(
        vm.prologue_state.signal_grid[5][6],
        Some(Value::Str("Love".to_string()))
    );
}

#[test]
fn test_narrative_terminus() {
    let dna = Dna { evolution_config: None,
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // "HeroShadow" -> ! -> ω -> (Conflict)
    vm.grid[5][3] = Value::Str("HeroShadow".to_string());
    vm.grid[5][4] = Value::Str("!".to_string());
    vm.grid[5][5] = Value::Str("ω".to_string());

    exec_prologue_tick(&mut vm);

    assert_eq!(
        vm.prologue_state.signal_grid[5][6],
        Some(Value::Str("Conflict".to_string()))
    );
}

#[test]
fn test_narrative_revision() {
    let dna = Dna { evolution_config: None,
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // "hello" -> ! -> ✍ <- ! <- "up"
    vm.grid[5][3] = Value::Str("hello".to_string());
    vm.grid[5][4] = Value::Str("!".to_string());

    vm.grid[4][4] = Value::Str("up".to_string());
    vm.grid[4][5] = Value::Str("!".to_string());

    vm.grid[5][5] = Value::Str("✍".to_string());

    exec_prologue_tick(&mut vm);

    assert_eq!(
        vm.prologue_state.signal_grid[5][6],
        Some(Value::Str("HELLO".to_string()))
    );
}

#[test]
fn test_narrative_library() {
    let dna = Dna { evolution_config: None,
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // 📖 (Book) Library at 5,5

    // Step 1: Write
    // Key "key" -> ! (5,4)
    vm.grid[5][3] = Value::Str("key".to_string());
    vm.grid[5][4] = Value::Str("!".to_string());

    // Mode 1 (Write) -> ! (4,5)
    vm.grid[4][4] = Value::Int(1);
    vm.grid[4][5] = Value::Str("!".to_string());

    // Value "val" -> ! (6,5)
    vm.grid[6][4] = Value::Str("val".to_string());
    vm.grid[6][5] = Value::Str("!".to_string());

    vm.grid[5][5] = Value::Str("📖".to_string());

    exec_prologue_tick(&mut vm);

    // Check if written to library
    assert_eq!(
        vm.prologue_state.library.get("key"),
        Some(&Value::Str("val".to_string()))
    );

    // Step 2: Read
    let mut vm2 = ChimeraVM::new(Dna { evolution_config: None,
        helix: Helix { strands: vec![] },
    });
    vm2.prologue_state.active = true;
    vm2.prologue_state
        .library
        .insert("key".to_string(), Value::Str("val".to_string()));

    // Key (West): "key"
    vm2.grid[5][3] = Value::Str("key".to_string());
    vm2.grid[5][4] = Value::Str("!".to_string());

    // Mode (North): 2 (Read) - Because 0 is treated as Empty/None signal
    vm2.grid[4][4] = Value::Int(2);
    vm2.grid[4][5] = Value::Str("!".to_string());

    vm2.grid[5][5] = Value::Str("📖".to_string());

    exec_prologue_tick(&mut vm2);

    // Debugging assertions
    assert_eq!(
        vm2.prologue_state.signal_grid[5][4],
        Some(Value::Str("key".to_string())),
        "West Signal Missing"
    );
    assert_eq!(
        vm2.prologue_state.signal_grid[4][5],
        Some(Value::Int(2)),
        "North Signal Missing"
    );

    // Should emit "val" to Self (5,5)
    assert_eq!(
        vm2.prologue_state.signal_grid[5][5],
        Some(Value::Str("val".to_string()))
    );
}
