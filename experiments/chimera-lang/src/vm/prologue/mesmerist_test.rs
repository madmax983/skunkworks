#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Helix};
    use crate::vm::prologue::exec_prologue_tick;
    use crate::vm::{ChimeraVM, Value};

    #[test]
    fn test_mesmerist_gaze() {
        let dna = Dna {
            evolution_config: None,
            helix: Helix { strands: vec![] },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.prologue_state.active = true;

        // Setup:
        // (5, 4): "push(42)" (Suggestion)
        // (5, 5): 🌀 (Mesmerist)
        // (5, 7): @ (Seeker - Target)

        vm.grid[5][4] = Value::Str("push(42)".to_string());
        vm.grid[5][5] = Value::Str("🌀".to_string());
        vm.grid[5][7] = Value::Str("@".to_string());

        // Run Tick
        exec_prologue_tick(&mut vm);

        // Debug output
        println!("Agents after tick:");
        for a in &vm.prologue_state.agents {
            println!("{:?}", a);
        }

        // Find any agent with 42 on stack
        let hypnotized = vm.prologue_state.agents.iter().find(|a| {
            if let Some(Value::Int(v)) = a.stack.last() {
                *v == 42
            } else {
                false
            }
        });

        assert!(
            hypnotized.is_some(),
            "Should find an agent with 42 on stack"
        );
    }

    #[test]
    fn test_mesmerist_cone() {
        let dna = Dna {
            evolution_config: None,
            helix: Helix { strands: vec![] },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.prologue_state.active = true;

        // Setup:
        // (5, 4): "push(1)"
        // (5, 5): 🌀
        // (5, 6): @ (Hit - Direct)
        // (4, 7): @ (Hit - Diagonal)
        // (6, 7): @ (Hit - Diagonal)
        // (3, 7): @ (Miss - Too far Y)

        vm.grid[5][4] = Value::Str("push(1)".to_string());
        vm.grid[5][5] = Value::Str("🌀".to_string());

        vm.grid[5][6] = Value::Str("@".to_string());
        vm.grid[4][7] = Value::Str("@".to_string());
        vm.grid[6][7] = Value::Str("@".to_string());
        vm.grid[3][7] = Value::Str("@".to_string());

        exec_prologue_tick(&mut vm);

        // Count how many agents have [1] on stack
        let hypnotized_count = vm
            .prologue_state
            .agents
            .iter()
            .filter(|a| {
                if let Some(Value::Int(v)) = a.stack.last() {
                    *v == 1
                } else {
                    false
                }
            })
            .count();

        // We expect 3 hits
        assert_eq!(
            hypnotized_count, 3,
            "Expected 3 hypnotized agents, found {}",
            hypnotized_count
        );
    }
}
