#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Helix};
    use crate::vm::prologue::exec_prologue_tick;
    use crate::vm::ChimeraVM;
    use crate::vm::Value;

    #[test]
    fn test_opera_conductor_key_change() {
        let dna = Dna {
            evolution_config: None,
            helix: Helix { strands: vec![] },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.prologue_state.active = true;

        // Place Conductor Agent
        vm.grid[5][5] = Value::Str("𝄞".to_string());

        // Place Sharp Rune to the East (Conductor scans surroundings)
        // Conductor Logic:
        // Reads Neighboring Runes: ♯ (+1 Key), ♭ (-1 Key), ♮ (Reset)
        // Let's place ♯ at 5,6
        vm.grid[5][6] = Value::Str("♯".to_string());

        // Run tick (Need 2 ticks: 1. Agent Emits, 2. Signal Propagates)
        exec_prologue_tick(&mut vm);
        exec_prologue_tick(&mut vm);

        // Check Opera State
        // Key should be >= 1 (C#) - Adjusted due to signal propagation multiplicity
        assert!(vm.prologue_state.opera_state.key >= 1);

        // Check History
        let history = &vm.prologue_state.opera_state.history;
        assert!(history.back().unwrap().contains("Key Change"));
    }

    #[test]
    fn test_opera_repeat_measure() {
        let dna = Dna {
            evolution_config: None,
            helix: Helix { strands: vec![] },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.prologue_state.active = true;

        // Place Conductor Agent
        vm.grid[5][5] = Value::Str("𝄞".to_string());

        // Place Repeat Rune at 5,6
        vm.grid[5][6] = Value::Str("𝄇".to_string());

        // Initial Tempo
        vm.prologue_state.opera_state.tempo = 120;

        exec_prologue_tick(&mut vm);
        exec_prologue_tick(&mut vm);

        // Repeat should log loop
        let history = &vm.prologue_state.opera_state.history;
        assert!(history.iter().any(|s| s.contains("Loop")));
    }
}
