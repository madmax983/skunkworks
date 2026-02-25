#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Helix};
    use crate::vm::prologue::exec_prologue_tick;
    use crate::vm::{ChimeraVM, Value};

    #[test]
    fn test_dream_entry() {
        let dna = Dna {
            evolution_config: None,
            helix: Helix { strands: vec![] },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.prologue_state.active = true;

        // Setup: 10 -> ! -> ☾
        vm.grid[5][3] = Value::Int(10);
        vm.grid[5][4] = Value::Str("!".to_string());
        vm.grid[5][5] = Value::Str("☾".to_string());

        exec_prologue_tick(&mut vm);

        // Check intensity at (5,5)
        // Input 10 -> Adds 5.0 intensity.
        // Physics tick happens at end: Diffusion + Decay.
        let val = vm.prologue_state.oneiric_grid.cells[5][5];
        assert!(val > 0.0, "Dream intensity should increase");

        // Output signal (emitted during propagation, before physics decay)
        assert_eq!(vm.prologue_state.signal_grid[5][5], Some(Value::Int(5)));
    }

    #[test]
    fn test_dream_wake() {
        let dna = Dna {
            evolution_config: None,
            helix: Helix { strands: vec![] },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.prologue_state.active = true;
        vm.prologue_state.oneiric_grid.cells[5][5] = 50.0;

        // Setup: ☀
        vm.grid[5][5] = Value::Str("☀".to_string());

        exec_prologue_tick(&mut vm);

        // Output signal should be 50
        assert_eq!(vm.prologue_state.signal_grid[5][5], Some(Value::Int(50)));

        // Check drain (x0.8) and decay (x0.9)
        // 50 * 0.8 * 0.9 = 36.0 (approx, diffusion affects it too)
        let val = vm.prologue_state.oneiric_grid.cells[5][5];
        assert!(val < 50.0, "Dream intensity should drain when read");
        assert!(val > 0.0);
    }

    #[test]
    fn test_dream_diffusion() {
        let dna = Dna {
            evolution_config: None,
            helix: Helix { strands: vec![] },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.prologue_state.active = true;
        vm.prologue_state.oneiric_grid.cells[5][5] = 100.0;

        exec_prologue_tick(&mut vm);

        // Neighbors should receive value
        let n_val = vm.prologue_state.oneiric_grid.cells[5][6];
        assert!(n_val > 0.0, "Diffusion should spread to neighbors");

        // Center should decrease
        let c_val = vm.prologue_state.oneiric_grid.cells[5][5];
        assert!(c_val < 100.0, "Center should decay/diffuse");
    }
}
