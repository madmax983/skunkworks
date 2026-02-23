use chimera_lang::ast::{Dna, Helix};
use chimera_lang::opcode::OpCode;
use chimera_lang::vm::prologue::exec_prologue_tick;
use chimera_lang::vm::{ChimeraVM, Value};

#[test]
fn test_orca_mode_toggle() {
    let dna = Dna { evolution_config: None,
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    assert_eq!(vm.prologue_state.orca_mode, false);

    // Toggle ON
    chimera_lang::vm::nova::exec_nova_op(&mut vm, OpCode::Orca, &[]);
    assert_eq!(vm.prologue_state.orca_mode, true);

    // Toggle OFF
    chimera_lang::vm::nova::exec_nova_op(&mut vm, OpCode::Orca, &[]);
    assert_eq!(vm.prologue_state.orca_mode, false);
}

#[test]
fn test_orca_bang() {
    let dna = Dna { evolution_config: None,
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;
    vm.prologue_state.orca_mode = true;

    // Setup Bang Circuit
    //   1 !      (Signal at 5,5)
    //     *      (at 6,5)
    //     ?      (at 7,5)

    vm.grid[5][4] = Value::Int(1);
    vm.grid[5][5] = Value::Str("!".to_string());
    vm.grid[6][5] = Value::Str("*".to_string());
    vm.grid[7][5] = Value::Str("?".to_string()); // Sink at South
    vm.grid[6][6] = Value::Str("?".to_string()); // Sink at East

    exec_prologue_tick(&mut vm);

    // 1. ! reads 1, emits to 5,5.
    // 2. * (6,5) reads North (5,5), emits to All Neighbors (7,5), (6,6), (5,5), (6,4). And Self (6,5).
    // 3. ? (7,5) reads North (6,5).
    // 4. ? (6,6) reads West (6,5).

    let output = vm.output.join("\n");
    assert!(output.contains("PROLOGUE: Sink at 5,7 received Int(1)"));
    assert!(output.contains("PROLOGUE: Sink at 6,6 received Int(1)"));
}

#[test]
fn test_orca_directional() {
    let dna = Dna { evolution_config: None,
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;
    vm.prologue_state.orca_mode = true;

    // Test N: South -> North
    //   ?        (4,5)
    //   N        (5,5)
    //   !        (6,5) -> Emits Signal
    //   42       (6,4)
    vm.grid[4][5] = Value::Str("?".to_string());
    vm.grid[5][5] = Value::Str("N".to_string());
    vm.grid[6][5] = Value::Str("!".to_string());
    vm.grid[6][4] = Value::Int(42);

    exec_prologue_tick(&mut vm);

    let output = vm.output.join("\n");
    assert!(output.contains("PROLOGUE: Sink at 5,4 received Int(42)"));
}
