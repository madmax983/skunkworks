#[cfg(feature = "nova")]
#[cfg(test)]
mod tests {
    use chimera_lang::prelude::*;
    use chimera_lang::vm::prologue::exec_prologue_tick;
    use chimera_lang::vm::prologue::weave_reality::RealityMode;
    use chimera_lang::vm::Value;

    #[test]
    fn test_reality_weave_activation() {
        let dna = Dna {
            evolution_config: None,
            helix: Helix { strands: vec![] },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.prologue_state.active = true;

        // Setup Reality Rune: 🌐
        // West: Radius (Int 2)
        // North: Mode (Int 2 = Silicon)

        let (cy, cx) = (8, 8);
        vm.grid[cy][cx] = Value::Str("🌐".to_string());
        vm.grid[cy][cx - 1] = Value::Int(2); // Radius
        vm.grid[cy - 1][cx] = Value::Int(2); // Silicon Mode

        // Tick 1: Scan Grid
        exec_prologue_tick(&mut vm);

        // Verify Reality Map
        // Center should be Silicon
        let mode_center = vm.prologue_state.reality_state.get_mode(cy, cx);
        assert_eq!(mode_center, RealityMode::Silicon);

        // Verify Radius (cy+1, cx) should be Silicon
        let mode_adj = vm.prologue_state.reality_state.get_mode(cy + 1, cx);
        assert_eq!(mode_adj, RealityMode::Silicon);

        // Verify Outside (cy+3, cx) should be Prologue
        let mode_out = vm.prologue_state.reality_state.get_mode(cy + 3, cx);
        assert_eq!(mode_out, RealityMode::Prologue);
    }

    #[test]
    #[cfg(feature = "silicon")]
    fn test_silicon_reality_physics() {
        let dna = Dna {
            evolution_config: None,
            helix: Helix { strands: vec![] },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.prologue_state.active = true;
        vm.silicon_mode = false; // Disable global silicon so it runs via reality bubble only

        // Create Silicon Bubble at (5, 5) radius 3
        let (cy, cx) = (5, 5);
        vm.grid[cy][cx] = Value::Str("🌐".to_string());

        // Use String inputs for Radius/Mode to prevent them from being treated as
        // Wireworld components (Conductor/Head/Tail) and mutating during simulation.
        // "3" -> Radius 3
        // "S" -> Silicon Mode
        vm.grid[cy][cx - 1] = Value::Str("3".to_string());
        vm.grid[cy - 1][cx] = Value::Str("S".to_string());

        // Setup Wireworld circuit inside bubble
        // (5, 6) = Head (2) -> (5, 7) = Wire (1) -> (5, 8) = Wire (1)
        vm.grid[cy][cx + 1] = Value::Int(2);
        vm.grid[cy][cx + 2] = Value::Int(1);
        vm.grid[cy][cx + 3] = Value::Int(1);

        // Tick 1
        exec_prologue_tick(&mut vm);

        // Check Evolution
        assert_eq!(vm.grid[cy][cx + 1], Value::Int(3), "Tick 1: Head -> Tail");
        assert_eq!(vm.grid[cy][cx + 2], Value::Int(2), "Tick 1: Wire -> Head");
        assert_eq!(vm.grid[cy][cx + 3], Value::Int(1), "Tick 1: Wire -> Wire");

        // Tick 2
        exec_prologue_tick(&mut vm);

        // Tail (3) -> Conductor (1)
        assert_eq!(
            vm.grid[cy][cx + 1],
            Value::Int(1),
            "Tick 2: Tail should become Wire"
        );

        // Head (2) -> Tail (3)
        assert_eq!(
            vm.grid[cy][cx + 2],
            Value::Int(3),
            "Tick 2: Head should become Tail"
        );

        // Wire (1) -> Head (2)
        assert_eq!(
            vm.grid[cy][cx + 3],
            Value::Int(2),
            "Tick 2: Wire should become Head"
        );
    }

    #[test]
    fn test_orca_reality_physics() {
        let dna = Dna {
            evolution_config: None,
            helix: Helix { strands: vec![] },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.prologue_state.active = true;

        // Create Orca Bubble at (10, 10) radius 2
        let (cy, cx) = (10, 10);
        vm.grid[cy][cx] = Value::Str("🌐".to_string());
        vm.grid[cy][cx - 1] = Value::Int(2);
        vm.grid[cy - 1][cx] = Value::Str("O".to_string()); // Orca Mode

        // Setup Orca Op: Bang (*) at (10, 11)
        vm.grid[cy][cx + 1] = Value::Str("*".to_string());

        // Signal from West via DELAYED signals to survive prepare_signals clear
        // Inject signal at (10, 12) (East of Bang)
        vm.prologue_state.delayed_signals[cy][cx + 2] = Some(Value::Int(5));

        exec_prologue_tick(&mut vm);

        // Check if * activated (Self lighted up)
        assert!(
            vm.prologue_state.signal_grid[cy][cx + 1].is_some(),
            "Bang should activate in Orca mode from East signal"
        );
    }
}
