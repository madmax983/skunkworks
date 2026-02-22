#[cfg(feature = "nova")]
#[cfg(test)]
mod tests {
    use chimera_lang::ast::{Dna, Helix};
    use chimera_lang::vm::{ChimeraVM, Value};

    #[test]
    fn test_rhythm_clock() {
        // Add dummy strand to prevent immediate halt
        let genes = vec![chimera_lang::ast::Gene {
            op: chimera_lang::opcode::OpCode::Jump,
            args: vec![chimera_lang::ast::Nucleotide::Number(0)],
        }];
        let dna = Dna {
            helix: Helix {
                strands: vec![chimera_lang::ast::Strand { genes }],
            },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.prologue_state.active = true;

        // Setup Clock at (5,5)
        vm.grid[5][5] = Value::Str("⏱️".to_string());

        // Initial state
        assert_eq!(vm.tick_counter, 0);

        // Run 4 steps (Ticks 1, 2, 3, 4)
        for _ in 0..4 {
            vm.step();
            assert!(
                vm.prologue_state.signal_grid[5][5].is_none(),
                "Should not emit yet"
            );
        }

        // Tick 5
        vm.step();
        assert_eq!(vm.tick_counter, 5);
        // Default interval is 5. 5 % 5 == 0.
        assert!(
            vm.prologue_state.signal_grid[5][5].is_some(),
            "Should emit on tick 5"
        );
    }

    #[test]
    fn test_rhythm_fader() {
        let dna = Dna {
            helix: Helix { strands: vec![] },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.prologue_state.active = true;

        // Fader at (5,5) reading (5,4)
        vm.grid[5][5] = Value::Str("🎚️".to_string());

        // Input: 12 (BPM)
        // ! Source at (5,3) reading (5,2)
        vm.grid[5][3] = Value::Str("!".to_string());
        vm.grid[5][2] = Value::Int(12);
        // Wire at (5,4)
        vm.grid[5][4] = Value::Str("~".to_string());

        // Step 1: ! emits 12 to (5,3)
        vm.step();

        // Step 2: ~ reads 12 from (5,3), emits to (5,4)
        // Fader reads 12 from (5,4).
        vm.step();

        // Check BPM
        assert_eq!(vm.prologue_state.rhythm_state.bpm, 12);
        // Interval = 600 / 12 = 50.
        assert_eq!(vm.prologue_state.rhythm_state.beat_interval, 50);
    }
}
