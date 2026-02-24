#[cfg(test)]
mod tests {
    use chimera_lang::ast::{Dna, Helix};
    use chimera_lang::vm::prologue::exec_prologue_tick;
    use chimera_lang::vm::{ChimeraVM, Value};

    #[test]
    fn test_prologue_quine_cycle() {
        let dna = Dna {
            evolution_config: None,
            helix: Helix { strands: vec![] },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.prologue_state.active = true;

        // Setup:
        // Trigger: (5, 3) needs signal.
        //   (5, 2): 1
        //   (5, 3): !
        vm.grid[5][2] = Value::Int(1);
        vm.grid[5][3] = Value::Str("!".to_string());

        // Height: (4, 4) needs signal.
        //   (4, 3): 3
        //   (4, 4): !
        vm.grid[4][3] = Value::Int(3);
        vm.grid[4][4] = Value::Str("!".to_string());

        // Width: (6, 4) needs signal.
        //   (6, 3): 1
        //   (6, 4): !
        vm.grid[6][3] = Value::Int(1);
        vm.grid[6][4] = Value::Str("!".to_string());

        // B at (5, 4)
        vm.grid[5][4] = Value::Str("B".to_string());

        // Data to capture
        // B captures center (5, 5). Height 3 -> (4,5)..(6,5).
        // (4, 5): 42
        vm.grid[4][5] = Value::Int(42);

        // " at (5, 5)
        vm.grid[5][5] = Value::Str("\"".to_string());

        // Π at (5, 6)
        vm.grid[5][6] = Value::Str("Π".to_string());

        // Step 1:
        // - ! sources fire. Signals at (5,3)=1, (4,4)=3, (6,4)=1.
        // - Propagation:
        //   - B reads signals.
        //   - B emits Dish to delayed_signals[5][4].
        exec_prologue_tick(&mut vm);

        assert!(
            vm.prologue_state.delayed_signals[5][4].is_some(),
            "B did not fire"
        );

        // Step 2:
        // - prepare_signals: delayed[5][4] -> signal_grid[5][4] (Dish).
        // - Propagation:
        //   - " at (5,5) reads signal_grid[5][4] (Dish).
        //   - " emits String to signal_grid[5][5].
        //   - Π at (5,6) reads signal_grid[5][5] (String).
        // - Sinks:
        //   - Π executes. Parses JSON. Pastes to Grid (5, 7).
        //     - Center (5, 7). Height 3 -> (4,7)..(6,7).
        exec_prologue_tick(&mut vm);

        // Check Result at (4, 7)
        assert_eq!(
            vm.grid[4][7],
            Value::Int(42),
            "Failed to teleport 42 via JSON"
        );

        // Check Result at (5, 7) - should be copy of (5, 5) which is "
        assert_eq!(
            vm.grid[5][7],
            Value::Str("\"".to_string()),
            "Failed to teleport \" via JSON"
        );
    }
}
