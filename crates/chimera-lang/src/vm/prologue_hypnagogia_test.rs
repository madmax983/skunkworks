#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Helix};
    use crate::vm::prologue::exec_prologue_tick;
    use crate::vm::{ChimeraVM, Value};

    #[test]
    fn test_dream_intensity_increase() {
        let dna = Dna { evolution_config: None,
            helix: Helix { strands: vec![] },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.prologue_state.active = true;

        // Setup: 10 -> ! -> ☾
        vm.grid[5][3] = Value::Int(10);
        vm.grid[5][4] = Value::Str("!".to_string());
        vm.grid[5][5] = Value::Str("☾".to_string());

        exec_prologue_tick(&mut vm);

        // Check intensity
        // 10 * 0.5 = 5.0
        // But process_dream_logic runs at end of tick and decays by 0.1
        // So 5.0 - 0.1 = 4.9
        assert!(vm.prologue_state.dream_intensity > 0.0);
        assert!(
            (vm.prologue_state.dream_intensity - 4.9).abs() < 0.001,
            "Intensity should be approx 4.9"
        );

        // Check output signal (self)
        // Output signal is set during propagation (before decay)
        assert_eq!(vm.prologue_state.signal_grid[5][5], Some(Value::Int(5)));
    }

    #[test]
    fn test_dream_awaken() {
        let dna = Dna { evolution_config: None,
            helix: Helix { strands: vec![] },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.prologue_state.active = true;
        vm.prologue_state.dream_intensity = 50.0;

        // Setup: 1 -> ! -> ☀
        vm.grid[5][3] = Value::Int(1);
        vm.grid[5][4] = Value::Str("!".to_string());
        vm.grid[5][5] = Value::Str("☀".to_string());

        exec_prologue_tick(&mut vm);

        assert_eq!(vm.prologue_state.dream_intensity, 0.0);
    }

    #[test]
    fn test_dream_manifestation() {
        let dna = Dna { evolution_config: None,
            helix: Helix { strands: vec![] },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.prologue_state.active = true;
        vm.prologue_state.dream_intensity = 100.0; // High intensity

        // Run multiple ticks to trigger RNG effects
        let mut manifested = false;
        for _ in 0..200 {
            // Keep intensity high (it decays)
            vm.prologue_state.dream_intensity = 100.0;
            exec_prologue_tick(&mut vm);

            // Check output for logs
            let logs = vm.output.join("\n");
            if logs.contains("HYPNAGOGIA") {
                manifested = true;
                break;
            }
        }

        assert!(
            manifested,
            "High dream intensity should eventually manifest effects"
        );
    }
}
