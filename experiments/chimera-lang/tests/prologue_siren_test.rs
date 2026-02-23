use chimera_lang::prelude::*;
use chimera_lang::vm::prologue::exec_prologue_tick;
use chimera_lang::vm::Value;

#[test]
fn test_siren_lookahead_sequencer() {
    let dna = Dna {
        evolution_config: None,
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // Grid Setup:
    // ♬ A B C
    //
    // The Siren (♬) starts at (0,0) moving East (default).
    // Tick 0: At (0,0). Looks East to (0,1) which is 'A'. Plays 'A'. Moves to (0,1).
    // Tick 1: At (0,1). Looks East to (0,2) which is 'B'. Plays 'B'. Moves to (0,2).
    // Tick 2: At (0,2). Looks East to (0,3) which is 'C'. Plays 'C'. Moves to (0,3).

    vm.grid[0][0] = Value::Str("♬".to_string());
    vm.grid[0][1] = Value::Str("A".to_string());
    vm.grid[0][2] = Value::Str("B".to_string());
    vm.grid[0][3] = Value::Str("C".to_string());

    // Tick 0
    exec_prologue_tick(&mut vm);

    // Verify output for 'A' (MIDI 69 - 440Hz)
    // Default octave 0 -> Base A is 69.
    // Wait, my logic: 'A' => 60 + 9 + (0*12) = 69.
    let output0 = vm.output.last().unwrap();
    println!("Output Tick 0: {}", output0);
    assert!(output0.contains("SIREN: Note 69"));

    // Verify position
    // Siren should have moved to (0,1), overwriting 'A'.
    match &vm.grid[0][1] {
        Value::Str(s) => assert_eq!(s, "♬", "Siren should be at (0,1)"),
        _ => panic!("Siren not found at (0,1)"),
    }

    // Tick 1
    exec_prologue_tick(&mut vm);

    // Verify output for 'B' (MIDI 71)
    // 'B' => 60 + 11 = 71.
    let output1 = vm.output.last().unwrap();
    println!("Output Tick 1: {}", output1);
    assert!(output1.contains("SIREN: Note 71"));

    // Verify position
    match &vm.grid[0][2] {
        Value::Str(s) => assert_eq!(s, "♬", "Siren should be at (0,2)"),
        _ => panic!("Siren not found at (0,2)"),
    }
}

#[test]
fn test_siren_direction_change() {
    let dna = Dna {
        evolution_config: None,
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // Grid Setup:
    // ♬ S
    // . A
    //
    // Tick 0: At (0,0). Looks East to (0,1) 'S'.
    //         Processes 'S': Sets Direction to South (2).
    //         Moves to (0,1).
    // Tick 1: At (0,1). Current Direction is South. Looks South to (1,1) 'A'.
    //         Plays 'A'. Moves to (1,1).

    vm.grid[0][0] = Value::Str("♬".to_string());
    vm.grid[0][1] = Value::Str("S".to_string());
    vm.grid[1][1] = Value::Str("A".to_string());

    // Tick 0
    exec_prologue_tick(&mut vm);

    // Position Check: Should be at (0,1)
    match &vm.grid[0][1] {
        Value::Str(s) => assert_eq!(s, "♬", "Siren should be at (0,1)"),
        _ => panic!("Siren not found at (0,1)"),
    }

    // State Check: Direction should be South (2)
    // We need to parse state from register or agent list.
    let agent_state = vm.prologue_state.registers.get(&(0, 1)).unwrap();
    if let Value::Str(s) = agent_state {
        // Format: "♬:BPM:Oct:Vel:Wave:Dir"
        let parts: Vec<&str> = s.split(':').collect();
        assert_eq!(parts[5], "2", "Direction should be South (2)");
    } else {
        panic!("Invalid agent state");
    }

    // Tick 1
    exec_prologue_tick(&mut vm);

    // Output Check: 'A' (69)
    let output = vm.output.last().unwrap();
    assert!(output.contains("SIREN: Note 69"));

    // Position Check: (1,1)
    match &vm.grid[1][1] {
        Value::Str(s) => assert_eq!(s, "♬", "Siren should be at (1,1)"),
        _ => panic!("Siren not found at (1,1)"),
    }
}
