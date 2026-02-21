use chimera_lang::ast::{Dna, Gene, Helix, Strand};
use chimera_lang::opcode::OpCode;
use chimera_lang::vm::prologue::exec_prologue_tick;
use chimera_lang::vm::{ChimeraVM, Value};

#[test]
fn test_cuneiform_retract() {
    let dna = Dna {
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // 1. Assert "fact"
    vm.grid[5][3] = Value::Str("fact".to_string());
    vm.grid[5][4] = Value::Str("!".to_string());
    vm.grid[5][5] = Value::Str("¶".to_string());
    exec_prologue_tick(&mut vm);

    #[cfg(feature = "oracle")]
    assert!(vm.knowledge_base.contains(&Value::Str("fact".to_string())));

    // 2. Retract "fact" using ¥
    // Clean previous runes
    vm.grid[5][3] = Value::Int(0);
    vm.grid[5][4] = Value::Int(0);
    vm.grid[5][5] = Value::Int(0);

    vm.grid[6][3] = Value::Str("fact".to_string());
    vm.grid[6][4] = Value::Str("!".to_string());
    vm.grid[6][5] = Value::Str("¥".to_string());
    exec_prologue_tick(&mut vm);

    #[cfg(feature = "oracle")]
    assert!(!vm.knowledge_base.contains(&Value::Str("fact".to_string())));
}

#[test]
fn test_cuneiform_omen() {
    // Setup 2 strands to detect jump
    // Strand 0: Nop
    // Strand 1: Nop (Target)
    let strand = Strand {
        genes: vec![Gene {
            op: OpCode::Nop,
            args: vec![],
        }],
    };
    let dna = Dna {
        helix: Helix {
            strands: vec![strand.clone(), strand.clone()],
        },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // 1. Register Omen: "trigger" -> 1 (Jump to strand 1)

    // West Input: "trigger"
    vm.grid[5][3] = Value::Str("trigger".to_string());
    vm.grid[5][4] = Value::Str("!".to_string()); // Source emits to 5,4 (West of 5,5)

    // North Input: 1
    vm.grid[4][4] = Value::Int(1);
    vm.grid[4][5] = Value::Str("!".to_string()); // Source emits to 4,5 (North of 5,5)

    // Omen Rune
    vm.grid[5][5] = Value::Str("∃".to_string());

    exec_prologue_tick(&mut vm);

    #[cfg(feature = "oracle")]
    {
        assert_eq!(vm.omens.len(), 1);
        assert_eq!(vm.omens[0].condition, Value::Str("trigger".to_string()));
        assert_eq!(vm.omens[0].effect, Value::Int(1));
    }

    // 2. Trigger Omen
    // Assert "trigger" to KB
    vm.grid[8][3] = Value::Str("trigger".to_string());
    vm.grid[8][4] = Value::Str("!".to_string());
    vm.grid[8][5] = Value::Str("¶".to_string());

    exec_prologue_tick(&mut vm);

    #[cfg(feature = "oracle")]
    {
        // Check if VM jumped to strand 1
        assert_eq!(vm.ip.0, 1);
    }
}
