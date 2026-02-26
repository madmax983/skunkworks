use chimera_lang::ast::{Dna, Helix};
use chimera_lang::vm::prologue::exec_prologue_tick;
use chimera_lang::vm::{ChimeraVM, Value};

#[test]
fn test_dream_weaver_movement() {
    let dna = Dna {
        evolution_config: None,
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // Setup: Dream Weaver at (5,5)
    vm.grid[5][5] = Value::Str("💤".to_string());

    // Setup: High Intensity at (5,6) (East)
    vm.prologue_state.oneiric_grid.cells[5][6] = 50.0;

    exec_prologue_tick(&mut vm);

    // Dream Weaver should move to (5,6)
    // Note: exec_prologue_tick updates agents and grid.

    // Debug output if fails
    if vm.grid[5][6] != Value::Str("💤".to_string()) {
        println!("Grid[5][5]: {:?}", vm.grid[5][5]);
        println!("Grid[5][6]: {:?}", vm.grid[5][6]);
        println!("Agents: {:?}", vm.prologue_state.agents);
    }

    assert_eq!(vm.grid[5][5], Value::Int(0));
    assert_eq!(vm.grid[5][6], Value::Str("💤".to_string()));

    // Trail should be left at (5,5)
    assert!(vm.prologue_state.oneiric_grid.cells[5][5] >= 10.0);
}

#[test]
fn test_dream_weaver_weaving() {
    let dna = Dna {
        evolution_config: None,
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // Setup: Dream Weaver at (5,5)
    vm.grid[5][5] = Value::Str("💤".to_string());

    // Setup: Very High Intensity at (5,5)
    vm.prologue_state.oneiric_grid.cells[5][5] = 90.0;

    exec_prologue_tick(&mut vm);

    // Dream Weaver should NOT move (stays to admire work)
    assert_eq!(vm.grid[5][5], Value::Str("💤".to_string()));

    // Should have consumed intensity
    assert_eq!(vm.prologue_state.oneiric_grid.cells[5][5], 0.0);

    // Should have spawned runes around (check neighbors)
    let neighbors = [(4, 5), (6, 5), (5, 4), (5, 6)];
    let mut found_rune = false;
    for (ny, nx) in neighbors {
        if let Value::Str(_) = &vm.grid[ny][nx] {
            found_rune = true;
            break;
        }
    }
    assert!(found_rune, "Dream Weaver should spawn runes");
}

#[test]
fn test_nightmare_chase() {
    let dna = Dna {
        evolution_config: None,
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // Setup: Nightmare at (5,5)
    vm.grid[5][5] = Value::Str("👹".to_string());

    // Setup: Dream Weaver at (5,6) (East)
    vm.grid[5][6] = Value::Str("💤".to_string());

    exec_prologue_tick(&mut vm);

    // Nightmare should move to (5,6) and Kill Weaver
    assert_eq!(vm.grid[5][5], Value::Int(0));
    assert_eq!(vm.grid[5][6], Value::Str("👹".to_string()));

    // Output should contain consumption message
    let output = vm.output.join("\n");
    assert!(output.contains("NIGHTMARE: Consumed Dream Weaver"));
}
