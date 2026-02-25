use super::gardener::*;
use crate::vm::Value;
use crate::vm::prologue::PrologueAgent;
use crate::ast::{Dna, Helix};
use crate::vm::ChimeraVM;
use std::collections::VecDeque;

fn make_vm() -> ChimeraVM {
    let dna = Dna {
        evolution_config: None,
        helix: Helix { strands: vec![] },
    };
    ChimeraVM::new(dna)
}

#[test]
fn test_gardener_state_serialization() {
    let mut inv = VecDeque::new();
    inv.push_back(Value::Int(1));
    inv.push_back(Value::Int(2));

    let state = GardenerState::new(100, 0, inv);
    let s = state.to_string();
    assert_eq!(s, "♣:100:0:1,2");

    let parsed: GardenerState = s.parse().expect("Failed to parse");
    assert_eq!(parsed.energy, 100);
    assert_eq!(parsed.mode, 0);
    assert_eq!(parsed.inventory.len(), 2);
    assert_eq!(parsed.inventory[0], Value::Int(1));
    assert_eq!(parsed.inventory[1], Value::Int(2));
}

#[test]
fn test_gardener_planting() {
    for _ in 0..20 {
        let mut vm = make_vm();

        // Setup Agent at 5,5 with a seed
        let mut inv = VecDeque::new();
        inv.push_back(Value::Int(5));
        let state = GardenerState::new(100, 0, inv);

        let agent = PrologueAgent {
            x: 5,
            y: 5,
            state: state.to_value(),
            stack: vec![],
        };

        // Ensure neighbor is empty
        vm.grid[5][6] = Value::Int(0);
        let grid_snapshot = vm.grid.clone();

        // Run logic
        if let Some((updated_agent, _)) = process_gardener_logic(&mut vm, &agent, &grid_snapshot) {
            if let Value::Str(s) = updated_agent.state {
                let new_state: GardenerState = s.parse().unwrap();
                if new_state.mode == 0 {
                    // Mode didn't switch, so it should have attempted to plant

                    // Check neighbors for plant
                    let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
                    let mut planted = false;
                    for (dy, dx) in neighbors {
                        let ny = (5i64 + dy) as usize;
                        let nx = (5i64 + dx) as usize;
                        if vm.grid[ny][nx] == Value::Int(5) {
                            planted = true;
                            break;
                        }
                    }

                    if planted {
                        assert_eq!(new_state.inventory.len(), 0, "Seed should be removed");
                        assert_eq!(new_state.energy, 94, "Energy should decrease");
                        return; // Success
                    }
                }
            }
        }
    }
    panic!("Failed to plant after retries");
}

#[test]
fn test_gardener_nurture() {
    for _ in 0..20 {
        let mut vm = make_vm();

        // Setup Agent at 5,5 with no seeds, Mode 1 (Tend)
        let state = GardenerState::new(100, 1, VecDeque::new());

        let agent = PrologueAgent {
            x: 5,
            y: 5,
            state: state.to_value(),
            stack: vec![],
        };

        // Setup a plant at 5,6
        vm.grid[5][6] = Value::Int(1);
        let grid_snapshot = vm.grid.clone();

        // Run logic
        if let Some((updated_agent, _)) = process_gardener_logic(&mut vm, &agent, &grid_snapshot) {
            if let Value::Str(s) = updated_agent.state {
                let new_state: GardenerState = s.parse().unwrap();

                if new_state.mode == 1 {
                    // Check if plant grew
                    if vm.grid[5][6] == Value::Int(2) {
                        assert_eq!(new_state.energy, 97, "Energy should decrease");
                        return; // Success
                    }
                }
            }
        }
    }
    panic!("Failed to nurture after retries");
}

#[test]
fn test_gardener_harvest() {
    for _ in 0..20 {
        let mut vm = make_vm();

        // Setup Agent at 5,5 with no seeds, Mode 1 (Tend)
        let state = GardenerState::new(100, 1, VecDeque::new());

        let agent = PrologueAgent {
            x: 5,
            y: 5,
            state: state.to_value(),
            stack: vec![],
        };

        // Setup a ripe plant at 5,6
        vm.grid[5][6] = Value::Int(10);
        let grid_snapshot = vm.grid.clone();

        // Run logic
        if let Some((updated_agent, _)) = process_gardener_logic(&mut vm, &agent, &grid_snapshot) {
            if let Value::Str(s) = updated_agent.state {
                let new_state: GardenerState = s.parse().unwrap();

                if new_state.mode == 1 {
                    // Check if plant harvested
                    if vm.grid[5][6] == Value::Int(0) {
                        assert_eq!(new_state.inventory.len(), 1, "Should gain seed");
                        assert_eq!(new_state.energy, 119, "Energy should increase");
                        return; // Success
                    }
                }
            }
        }
    }
    panic!("Failed to harvest after retries");
}
