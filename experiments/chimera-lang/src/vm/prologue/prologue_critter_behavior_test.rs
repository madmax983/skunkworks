#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Helix};
    use crate::vm::prologue::exec_prologue_tick;
    use crate::vm::{ChimeraVM, Value};

    fn setup_vm() -> ChimeraVM {
        let dna = Dna {
            helix: Helix { strands: vec![] },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.prologue_state.active = true;
        vm
    }

    #[test]
    fn test_critter_legacy_moves() {
        let mut vm = setup_vm();

        // Critter DNA: "NEW" (North, East, West)
        let genes = "NEW".to_string();
        let energy = 100;
        let critter_state = format!("C:{}:{}:0:0", energy, genes); // Start facing North (0)

        // Start at (5, 5)
        vm.grid[5][5] = Value::Str("C".to_string());
        vm.prologue_state
            .registers
            .insert((5, 5), Value::Str(critter_state));

        // Tick 1: 'N' -> Move North to (4, 5)
        exec_prologue_tick(&mut vm);
        assert_eq!(vm.grid[5][5], Value::Int(0), "Old pos empty");
        assert_eq!(vm.grid[4][5], Value::Str("C".to_string()), "Moved North");

        // Tick 2: 'E' -> Move East to (4, 6)
        exec_prologue_tick(&mut vm);
        assert_eq!(vm.grid[4][5], Value::Int(0));
        assert_eq!(vm.grid[4][6], Value::Str("C".to_string()), "Moved East");

        // Tick 3: 'W' -> Move West to (4, 5)
        exec_prologue_tick(&mut vm);
        assert_eq!(vm.grid[4][6], Value::Int(0));
        assert_eq!(vm.grid[4][5], Value::Str("C".to_string()), "Moved West");
    }

    #[test]
    fn test_critter_split() {
        let mut vm = setup_vm();

        // Critter DNA: "S" (Split)
        let genes = "S".to_string();
        let start_energy = 100;
        let critter_state = format!("C:{}:{}:0:0", start_energy, genes);

        // Place at (5, 5)
        vm.grid[5][5] = Value::Str("C".to_string());
        vm.prologue_state
            .registers
            .insert((5, 5), Value::Str(critter_state));

        exec_prologue_tick(&mut vm);

        // Parent should still be at (5, 5) but with less energy
        // Cost: 1 (metabolic) + 30 (split) = 31.
        // Energy should be 100 - 31 = 69.
        if let Value::Str(s) = vm.prologue_state.registers.get(&(5, 5)).unwrap() {
            let parts: Vec<&str> = s.split(':').collect();
            let energy: i64 = parts[1].parse().unwrap();
            assert_eq!(energy, 69, "Parent energy incorrect after split");
        } else {
            panic!("Parent missing");
        }

        // Check for child
        let neighbors = [(4, 5), (6, 5), (5, 4), (5, 6)];
        let mut child_found = false;
        for (y, x) in neighbors {
            if let Value::Str(s) = &vm.grid[y][x] {
                if s == "C" {
                    child_found = true;
                    // check child energy (starts at 50)
                    if let Value::Str(cs) = vm.prologue_state.registers.get(&(y, x)).unwrap() {
                        let c_parts: Vec<&str> = cs.split(':').collect();
                        let c_energy: i64 = c_parts[1].parse().unwrap();
                        assert_eq!(c_energy, 50, "Child energy incorrect");
                    }
                }
            }
        }
        assert!(child_found, "Child critter not spawned");
    }

    #[test]
    fn test_critter_split_fail_low_energy() {
        let mut vm = setup_vm();

        // DNA: "S"
        let start_energy = 40; // Too low (< 50)
        let critter_state = format!("C:{}:S:0:0", start_energy);

        vm.grid[5][5] = Value::Str("C".to_string());
        vm.prologue_state
            .registers
            .insert((5, 5), Value::Str(critter_state));

        exec_prologue_tick(&mut vm);

        // Should not split.
        // Energy: 40 - 1 (metabolic) = 39.
        if let Value::Str(s) = vm.prologue_state.registers.get(&(5, 5)).unwrap() {
            let parts: Vec<&str> = s.split(':').collect();
            let energy: i64 = parts[1].parse().unwrap();
            assert_eq!(energy, 39);
        }

        // Check neighbors empty
        let neighbors = [(4, 5), (6, 5), (5, 4), (5, 6)];
        for (y, x) in neighbors {
            assert_eq!(vm.grid[y][x], Value::Int(0), "Should not spawn child");
        }
    }

    #[test]
    fn test_critter_attack() {
        let mut vm = setup_vm();

        // Predator: "A" (Attack). Facing East (1).
        let pred_genes = "A".to_string();
        let pred_energy = 100;
        let pred_state = format!("C:{}:{}:0:1", pred_energy, pred_genes);

        // Place Predator at (5, 5)
        vm.grid[5][5] = Value::Str("C".to_string());
        vm.prologue_state
            .registers
            .insert((5, 5), Value::Str(pred_state));

        // Place Victim (Seeker @) at (5, 6)
        vm.grid[5][6] = Value::Str("@".to_string());
        vm.prologue_state
            .agents
            .push(crate::vm::prologue::PrologueAgent {
                x: 6,
                y: 5,
                state: Value::Int(0),
            });

        exec_prologue_tick(&mut vm);

        // Victim should be gone (0)
        assert_eq!(vm.grid[5][6], Value::Int(0), "Victim should be eaten");

        // Predator Energy: 100 - 1 (meta) + 30 (eat) = 129.
        if let Value::Str(s) = vm.prologue_state.registers.get(&(5, 5)).unwrap() {
            let parts: Vec<&str> = s.split(':').collect();
            let energy: i64 = parts[1].parse().unwrap();
            assert_eq!(energy, 129, "Predator should gain energy");
        }
    }

    #[test]
    fn test_critter_mark() {
        let mut vm = setup_vm();

        // Marker: "M" (Mark). Facing East (1).
        let genes = "M".to_string();
        let energy = 100;
        let state = format!("C:{}:{}:0:1", energy, genes);

        vm.grid[5][5] = Value::Str("C".to_string());
        vm.prologue_state
            .registers
            .insert((5, 5), Value::Str(state));

        exec_prologue_tick(&mut vm);

        // Should mark (5, 6) with "."
        if let Value::Str(s) = &vm.grid[5][6] {
            assert_eq!(s, ".", "Should place mark");
        } else {
            panic!("Mark not placed");
        }

        // Critter still at (5, 5)
        assert_eq!(vm.grid[5][5], Value::Str("C".to_string()));
    }

    #[test]
    fn test_critter_death() {
        let mut vm = setup_vm();

        // Critter with 1 energy.
        // "F" (Move). Cost 1.
        // End of tick: Energy 0. Should die.
        let genes = "F".to_string();
        let energy = 1;
        let state = format!("C:{}:{}:0:1", energy, genes); // East

        vm.grid[5][5] = Value::Str("C".to_string());
        vm.prologue_state
            .registers
            .insert((5, 5), Value::Str(state));

        exec_prologue_tick(&mut vm);

        // The critter will move and then die on the *next* tick because energy hits 0 after move logic.
        // This test ensures it dies when energy starts at 0.

        vm.grid[6][6] = Value::Str("C".to_string());
        vm.prologue_state
            .registers
            .insert((6, 6), Value::Str("C:0:F:0:0".to_string()));
        // Manually add to agents list because we skipped `scan_grid_rules`
        vm.prologue_state
            .agents
            .push(crate::vm::prologue::PrologueAgent {
                x: 6,
                y: 6,
                state: Value::Str("C:0:F:0:0".to_string()),
            });

        exec_prologue_tick(&mut vm);

        assert_eq!(
            vm.grid[6][6],
            Value::Int(0),
            "Critter with 0 energy should die"
        );
    }

    #[test]
    fn test_critter_collision_breeding() {
        let mut vm = setup_vm();

        // C1 at (5, 5) moving East ("F", Dir 1)
        let s1 = "C:100:F:0:1".to_string();
        vm.grid[5][5] = Value::Str("C".to_string());
        vm.prologue_state
            .registers
            .insert((5, 5), Value::Str(s1.clone()));

        // C2 at (5, 7) moving West ("F", Dir 3)
        let s2 = "C:100:F:0:3".to_string();
        vm.grid[5][7] = Value::Str("C".to_string());
        vm.prologue_state
            .registers
            .insert((5, 7), Value::Str(s2.clone()));

        exec_prologue_tick(&mut vm);

        // C1 moves to (5, 6).
        // C2 tries to move to (5, 6), collides, breeds, and stays at (5, 7).

        assert_eq!(vm.grid[5][6], Value::Str("C".to_string()), "C1 moved");
        assert_eq!(
            vm.grid[5][7],
            Value::Str("C".to_string()),
            "C2 blocked/stayed"
        );

        // Check for child in adjacent empty cell
        let possible_locs = [(4, 7), (6, 7), (5, 8)];
        let mut child_found = false;
        for (y, x) in possible_locs {
            if let Value::Str(s) = &vm.grid[y][x] {
                if s == "C" {
                    child_found = true;
                }
            }
        }
        assert!(child_found, "Breeding should spawn child on collision");
    }
}
