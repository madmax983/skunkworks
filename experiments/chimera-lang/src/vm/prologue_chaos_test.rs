#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Helix};
    use crate::vm::prologue::exec_prologue_tick;
    use crate::vm::{ChimeraVM, Value};

    #[test]
    fn test_chaos_source_rune() {
        let dna = Dna {
            helix: Helix { strands: vec![] },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.prologue_state.active = true;

        // Place 'k' at (5,5)
        vm.grid[5][5] = Value::Str("k".to_string());

        exec_prologue_tick(&mut vm);

        // Neighbors should receive a signal (Int)
        let n = vm.prologue_state.signal_grid[4][5].clone();
        let s = vm.prologue_state.signal_grid[6][5].clone();
        let w = vm.prologue_state.signal_grid[5][4].clone();
        let e = vm.prologue_state.signal_grid[5][6].clone();

        assert!(matches!(n, Some(Value::Int(_))));
        assert!(matches!(s, Some(Value::Int(_))));
        assert!(matches!(w, Some(Value::Int(_))));
        assert!(matches!(e, Some(Value::Int(_))));
    }

    #[test]
    fn test_glitch_rune() {
        let dna = Dna {
            helix: Helix { strands: vec![] },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.prologue_state.active = true;

        // Circuit: 1 -> ! -> ~ -> z
        vm.grid[5][4] = Value::Int(1);
        vm.grid[5][5] = Value::Str("!".to_string());
        vm.grid[5][6] = Value::Str("~".to_string());
        vm.grid[5][7] = Value::Str("z".to_string());

        // Set neighbors of 'z' to 0
        vm.grid[4][7] = Value::Int(0); // N
        vm.grid[6][7] = Value::Int(0); // S
        vm.grid[5][8] = Value::Int(0); // E

        // Run for multiple ticks to ensure signal reaches 'z' and rng triggers
        for _ in 0..5 {
            exec_prologue_tick(&mut vm);
        }

        // Check if any neighbor changed
        let n = vm.grid[4][7].clone();
        let s = vm.grid[6][7].clone();
        let e = vm.grid[5][8].clone();

        let changed = !matches!(n, Value::Int(0))
            || !matches!(s, Value::Int(0))
            || !matches!(e, Value::Int(0));
        assert!(changed, "Glitch rune should modify at least one neighbor");
    }

    #[test]
    fn test_havoc_rune() {
        let dna = Dna {
            helix: Helix { strands: vec![] },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.prologue_state.active = true;

        // Circuit: 1 -> ! -> ~ -> h
        vm.grid[5][4] = Value::Int(1);
        vm.grid[5][5] = Value::Str("!".to_string());
        vm.grid[5][6] = Value::Str("~".to_string());
        vm.grid[5][7] = Value::Str("h".to_string());

        // Run for multiple ticks
        for _ in 0..5 {
            exec_prologue_tick(&mut vm);
        }

        // Scan grid for "K" (Chaos Agent)
        let mut found_k = false;
        for y in 0..crate::vm::GRID_SIZE {
            for x in 0..crate::vm::GRID_SIZE {
                if let Value::Str(s) = &vm.grid[y][x] {
                    if s == "K" {
                        found_k = true;
                        break;
                    }
                }
            }
        }
        assert!(found_k, "Havoc rune should spawn a 'K' agent");
    }
}
