#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Helix, JunctionType};
    use crate::vm::prologue::exec_prologue_tick;
    use crate::vm::{ChimeraVM, Value};

    #[test]
    fn test_construct_capture_and_paste() {
        let dna = Dna {
            helix: Helix { strands: vec![] },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.prologue_state.active = true;

        // Setup B at (6,5)
        vm.grid[6][5] = Value::Str("B".to_string());

        // Target to Capture: (6,6)
        vm.grid[6][6] = Value::Int(42);

        // Manually inject trigger signal for B (via delayed, so it survives prepare_signals)
        vm.prologue_state.delayed_signals[6][4] = Some(Value::Int(1)); // West Trigger
                                                                       // Default Height/Width is 1x1.
                                                                       // Capture Area: (6,6).

        // Execute Tick 1
        exec_prologue_tick(&mut vm);

        // Verify Blueprint created in delayed signals
        let blueprint = vm.prologue_state.delayed_signals[6][5].clone();
        assert!(blueprint.is_some(), "Blueprint should be generated");

        if let Some(Value::Junction(JunctionType::Dish, rows)) = &blueprint {
            assert_eq!(rows.len(), 1);
            if let Value::Junction(_, cells) = &rows[0] {
                assert_eq!(cells.len(), 1);
                assert_eq!(cells[0], Value::Int(42));
            } else {
                panic!("Invalid row structure");
            }
        } else {
            panic!("Blueprint not found or invalid type");
        }

        // Tick 2: Paste
        // Prepare signals: Move blueprint from delayed to current (happens in prepare_signals).
        // Note: exec_prologue_tick calls prepare_signals.
        // So if we call exec_prologue_tick again, delayed -> current.
        // But we want to change the grid to place Π before the sink phase of Tick 2.
        // But `exec_prologue_tick` does `scan_grid` first.
        // So we can modify grid now.

        // Place Π at (6,6) overwriting the 42.
        vm.grid[6][6] = Value::Str("Π".to_string());

        // The signal at (6,5) (Blueprint) will be read by Π at (6,6) (West).
        // Π pastes to (6,7).

        exec_prologue_tick(&mut vm);

        assert_eq!(vm.grid[6][7], Value::Int(42));
    }

    #[test]
    fn test_construct_dimensions() {
        let dna = Dna {
            helix: Helix { strands: vec![] },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.prologue_state.active = true;

        // B at (10, 10)
        vm.grid[10][10] = Value::Str("B".to_string());

        // Inject Signals (Delayed)
        vm.prologue_state.delayed_signals[10][9] = Some(Value::Int(1)); // Trigger
        vm.prologue_state.delayed_signals[9][10] = Some(Value::Int(2)); // Height
        vm.prologue_state.delayed_signals[11][10] = Some(Value::Int(2)); // Width

        // Grid Data to Capture (2x2)
        // H=2. StartY = 10 - 1 = 9. EndY = 11. (Rows 9, 10)
        // W=2. StartX = 10 + 1 = 11. EndX = 13. (Cols 11, 12)
        vm.grid[9][11] = Value::Int(1);
        vm.grid[9][12] = Value::Int(2);
        vm.grid[10][11] = Value::Int(3);
        vm.grid[10][12] = Value::Int(4);

        exec_prologue_tick(&mut vm);

        let result = &vm.prologue_state.delayed_signals[10][10];
        if let Some(Value::Junction(JunctionType::Dish, rows)) = result {
            assert_eq!(rows.len(), 2);
            // Verify Row 0
            if let Value::Junction(_, cells) = &rows[0] {
                assert_eq!(cells[0], Value::Int(1));
                assert_eq!(cells[1], Value::Int(2));
            }
            // Verify Row 1
            if let Value::Junction(_, cells) = &rows[1] {
                assert_eq!(cells[0], Value::Int(3));
                assert_eq!(cells[1], Value::Int(4));
            }
        } else {
            panic!("Capture failed");
        }
    }
}
