#[cfg(test)]
mod tests {
    use chimera_lang::ast::{Dna, Helix};
    use chimera_lang::vm::prologue::exec_prologue_tick;
    use chimera_lang::vm::{ChimeraVM, Value};

    fn make_vm() -> ChimeraVM {
        let dna = Dna {
            helix: Helix { strands: vec![] },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.prologue_state.active = true;
        vm
    }

    #[test]
    fn test_zeitgeist_rune() {
        let mut vm = make_vm();
        vm.grid[5][5] = Value::Str("Z".to_string());

        exec_prologue_tick(&mut vm);

        // Z emits to all neighbors
        let neighbors = [(4, 5), (6, 5), (5, 4), (5, 6)];
        for (y, x) in neighbors {
            assert!(
                vm.prologue_state.signal_grid[y][x].is_some(),
                "Neighbor {},{} should have signal",
                x,
                y
            );
        }
    }

    #[test]
    fn test_organelle_spawn_types() {
        let mut vm = make_vm();

        // Correct Circuit:
        // [4][4] = 2
        // [5][4] = ! (Source reads from North [4][4], Emits to Self/Neighbors)
        // [5][5] = O (Organelle reads from West [5][4])

        vm.grid[4][4] = Value::Int(2);
        vm.grid[5][4] = Value::Str("!".to_string());
        vm.grid[5][5] = Value::Str("O".to_string());

        exec_prologue_tick(&mut vm);

        // ! should have signal
        assert!(vm.prologue_state.signal_grid[5][4].is_some());

        // O should have spawned K at South (6,5)
        if let Value::Str(s) = &vm.grid[6][5] {
            assert_eq!(s, "K", "Should have spawned Chaos Agent K");
        } else {
            panic!("Spawn failed");
        }
    }

    #[test]
    fn test_hunter_behavior() {
        let mut vm = make_vm();
        // Setup Hunter at 5,5. Prey @ at 5,6.
        vm.grid[5][5] = Value::Str("H".to_string());
        vm.grid[5][6] = Value::Str("@".to_string());

        // Need to ensure they are registered as agents.
        // exec_prologue_tick calls scan_grid_rules first.

        exec_prologue_tick(&mut vm);

        // H should move to 5,6 (eating @)
        // 5,5 should be empty (or 0)
        // 5,6 should be H

        assert_eq!(vm.grid[5][5], Value::Int(0));
        assert_eq!(vm.grid[5][6], Value::Str("H".to_string()));
    }
}
