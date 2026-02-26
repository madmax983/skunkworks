use super::glitch::*;
use crate::vm::{ChimeraVM, Value};
use crate::ast::{Dna, Helix};

#[test]
fn test_glitch_agent_movement_and_corruption() {
    let dna = Dna {
        evolution_config: None,
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // Place Glitch Agent
    vm.grid[5][5] = Value::Str("👾".to_string());
    vm.prologue_state.scan_grid_rules(&vm.grid);

    // Verify Agent Spawned
    assert_eq!(vm.prologue_state.agents.len(), 1);

    // Step
    let grid_snapshot = vm.grid.clone();
    super::process_agents(&mut vm, &grid_snapshot);

    // Verify Movement (Agent moved from 5,5)
    assert!(vm.grid[5][5] != Value::Str("👾".to_string()));

    // Find new position
    let mut found = false;
    for y in 0..16 {
        for x in 0..16 {
            if let Value::Str(s) = &vm.grid[y][x] {
                if s == "👾" {
                    found = true;
                    break;
                }
            }
        }
    }
    assert!(found, "Glitch Agent disappeared!");
}

#[test]
fn test_glitch_rune_interference() {
    let dna = Dna {
        evolution_config: None,
        helix: Helix { strands: vec![] },
    };
    let mut vm = ChimeraVM::new(dna);
    vm.prologue_state.active = true;

    // Setup: 42 -> ≋ -> ?
    vm.grid[5][4] = Value::Int(42);
    vm.grid[5][5] = Value::Str("≋".to_string());
    vm.grid[5][6] = Value::Str("?".to_string()); // Just to catch output

    // Need to initialize signals manually or run tick
    // Let's manually trigger propagation
    let mut current_signals = vec![vec![None; 16]; 16];
    let mut next_signals = vec![vec![None; 16]; 16];

    // Input Signal
    current_signals[5][4] = Some(Value::Int(42));

    apply_glitch_runes("≋", 5, 5, &current_signals, &mut next_signals);

    // Expected Output: !42 (Bitwise NOT)
    if let Some(Value::Int(res)) = next_signals[5][6] {
        assert_eq!(res, !42);
    } else {
        panic!("Glitch Rune failed to propagate signal");
    }
}
