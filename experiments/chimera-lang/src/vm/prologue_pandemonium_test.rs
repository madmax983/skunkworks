#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Helix};
    use crate::vm::prologue::exec_prologue_tick;
    use crate::vm::{ChimeraVM, Value};

    #[test]
    fn test_chaos_source() {
        let dna = Dna {
            evolution_config: None,
            helix: Helix { strands: vec![] },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.prologue_state.active = true;

        // Circuit: 10 -> ! -> ¿
        // ¿ (Gamble) reads Signal from West. Emits South.

        // 1. Value 10 at (5,3)
        vm.grid[5][3] = Value::Int(10);

        // 2. Source ! at (5,4). Reads West (10). Emits Signal to Self (5,4).
        vm.grid[5][4] = Value::Str("!".to_string());

        // 3. Gate: ¿ at (5,5). Reads Signal from West (5,4).
        vm.grid[5][5] = Value::Str("¿".to_string());

        exec_prologue_tick(&mut vm);

        // Output: South (6,5)
        // Should be either 20 (Double) or 0 (Zero)
        assert!(
            vm.prologue_state.signal_grid[6][5].is_some(),
            "Chaos Rune did not emit signal to South"
        );
        if let Some(Value::Int(v)) = &vm.prologue_state.signal_grid[6][5] {
            assert!(*v == 20 || *v == 0);
        } else {
            panic!("Chaos Rune did not emit Int");
        }
    }

    #[test]
    fn test_noise_wire() {
        let dna = Dna {
            evolution_config: None,
            helix: Helix { strands: vec![] },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.prologue_state.active = true;

        // Circuit: 42 -> ! -> ≈ -> (Neighbors)
        // ! reads from West (5,3)
        vm.grid[5][3] = Value::Int(42);
        vm.grid[5][4] = Value::Str("!".to_string()); // Source at (5,4) reads (5,3)

        // ≈ reads from West (5,4). Note: ! emits to self (5,4).
        vm.grid[5][5] = Value::Str("≈".to_string()); // Flux at (5,5)

        exec_prologue_tick(&mut vm);

        // Check neighbors of (5,5): (4,5), (6,5), (5,4), (5,6)
        // One of them should have 42.

        let neighbors = [(4, 5), (6, 5), (5, 4), (5, 6)];
        let mut found = false;
        for (y, x) in neighbors {
            if let Some(Value::Int(v)) = &vm.prologue_state.signal_grid[y][x] {
                if *v == 42 {
                    found = true;
                    break;
                }
            }
        }
        assert!(found, "Flux rune did not emit to any neighbor");
    }

    #[test]
    fn test_chaos_sink() {
        let dna = Dna {
            evolution_config: None,
            helix: Helix { strands: vec![] },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.prologue_state.active = true;

        // Circuit: List -> ! -> ¡
        // ¡ Scramble: Reads West (List). Output South.

        let list = Value::Junction(
            crate::ast::JunctionType::Any,
            vec![Value::Int(1), Value::Int(2), Value::Int(3)],
        );

        // Input: West of !
        vm.grid[5][3] = list;
        vm.grid[5][4] = Value::Str("!".to_string()); // Source

        // Scramble at (5,5) reads West (5,4)
        vm.grid[5][5] = Value::Str("¡".to_string());

        exec_prologue_tick(&mut vm);

        // Output: South (6,5)
        assert!(vm.prologue_state.signal_grid[6][5].is_some());
        if let Some(Value::Junction(_, l)) = &vm.prologue_state.signal_grid[6][5] {
            assert_eq!(l.len(), 3);
            // It might be shuffled, or might be same order (random).
            // Just checking it exists and preserves elements is enough.
        } else {
            panic!("Scramble did not emit Junction");
        }
    }
}
