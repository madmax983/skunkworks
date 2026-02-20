#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::vm::prologue::exec_prologue_tick;
    use crate::vm::Value;

    #[test]
    fn test_critter_builder_gene() {
        let dna = Dna {
            helix: Helix { strands: vec![] },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.prologue_state.active = true;

        // Setup a Critter with Builder DNA
        // "F" -> Move Forward (to 5,6)
        // "+" -> Build Wire '~' (at 5,7)
        let genes = "F+".to_string();
        let energy = 100;
        // Direction 1 = East
        let critter_state = format!("C:{}:{}:0:1", energy, genes);

        // Place Critter at (5, 5)
        vm.grid[5][5] = Value::Str("C".to_string());
        vm.prologue_state
            .registers
            .insert((5, 5), Value::Str(critter_state));

        // Tick 1: Execute 'F'
        exec_prologue_tick(&mut vm);

        // Critter should have moved East to (5, 6)
        if let Value::Str(s) = &vm.grid[5][6] {
            assert_eq!(s, "C", "Critter should be at 5,6");
        } else {
            panic!("Critter not at 5,6");
        }
        assert_eq!(vm.grid[5][5], Value::Int(0), "Old position should be empty");

        // Tick 2: Execute '+'
        // Critter is at (5, 6), facing East. Target is (5, 7).
        exec_prologue_tick(&mut vm);

        // Critter stays at (5, 6)
        assert_eq!(vm.grid[5][6], Value::Str("C".to_string()));

        // (5, 7) should now contain a Wire '~'
        if let Value::Str(s) = &vm.grid[5][7] {
            assert_eq!(s, "~", "Wire should be built at 5,7");
        } else {
            panic!("Wire not found at 5,7. Found: {:?}", vm.grid[5][7]);
        }
    }

    #[test]
    fn test_critter_sense_gene() {
        let dna = Dna {
            helix: Helix { strands: vec![] },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.prologue_state.active = true;

        // Setup Critter
        // DNA: "iF"
        // Logic: 'i' checks ahead.
        // If blocked: Execute next ('F').
        // If clear: Skip next ('F').
        // Since we are moving into empty space, it should SKIP 'F'.

        let genes = "iF".to_string();
        let energy = 100;
        let critter_state = format!("C:{}:{}:0:1", energy, genes); // East

        vm.grid[5][5] = Value::Str("C".to_string());
        vm.prologue_state
            .registers
            .insert((5, 5), Value::Str(critter_state));

        // Tick 1: Execute 'i'.
        // Ahead (5, 6) is empty (0).
        // 'i' detects Clear.
        // Action: Skip next gene. IP increments by 1 (extra).
        // Standard IP increment also happens. So IP advances by 2 total.
        // Current Gene is 'i' (index 0). Next gene is 'F' (index 1).
        // Result IP should be 2 (loop back to 0).

        exec_prologue_tick(&mut vm);

        // Critter should NOT have moved, because 'F' was skipped.
        assert_eq!(vm.grid[5][5], Value::Str("C".to_string()));
        assert_eq!(vm.grid[5][6], Value::Int(0));

        // Verify internal state (IP should be back to 0)
        let state_val = vm.prologue_state.registers.get(&(5, 5)).unwrap();
        if let Value::Str(s) = state_val {
            let parts: Vec<&str> = s.split(':').collect();
            let ip: usize = parts[3].parse().unwrap();
            assert_eq!(ip, 0, "IP should wrap around to 0 after skipping F");
        }
    }
}
