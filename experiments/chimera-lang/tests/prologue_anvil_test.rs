#[cfg(feature = "nova")]
#[cfg(test)]
mod tests {
    use chimera_lang::ast::{Dna, Helix, JunctionType};
    use chimera_lang::vm::prologue::exec_prologue_tick;
    use chimera_lang::vm::{ChimeraVM, Value};

    #[test]
    fn test_anvil_compilation() {
        let dna = Dna {
            evolution_config: None,
            helix: Helix { strands: vec![] },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.prologue_state.active = true;

        // Place The Anvil ⚒ at (5, 5)
        let anvil_y = 5;
        let anvil_x = 5;
        vm.grid[anvil_y][anvil_x] = Value::Str("⚒".to_string());

        // Construct a Blueprint: "push 42"
        // Row 1: ["push", "42"]
        let row1 = Value::Junction(
            JunctionType::Dish,
            vec![Value::Str("push".to_string()), Value::Int(42)],
        );
        let blueprint = Value::Junction(JunctionType::Dish, vec![row1]);

        // Inject Signal to the West (5, 4)
        // Note: exec_prologue_tick calls scan_grid_rules first, then prepare_signals.
        // prepare_signals clears signal_grid.
        // BUT we can simulate a Source (!) emitting the blueprint.

        // Setup Source at (5, 3) pointing East to (5, 4) which is West of Anvil?
        // No, simplest way is to put the blueprint in a Source (!) at (5, 4)
        // Source (!) at (5, 4) reads West (5, 3) and emits to Self (5, 4).
        // Wait, Anvil reads West (5, 4).
        // So we need signal at (5, 4).
        // If we put ! at (5, 4), it emits to (5, 4).
        // It reads from (5, 3).

        vm.grid[anvil_y][anvil_x - 1] = Value::Str("!".to_string()); // Source at (5, 4)
        vm.grid[anvil_y][anvil_x - 2] = blueprint; // Value at (5, 3)

        // Execute Tick
        exec_prologue_tick(&mut vm);

        // Check Output
        let output = vm.output.join("\n");
        println!("{}", output);
        assert!(output.contains("ANVIL: Forged strand 0"));

        // Check DNA
        assert_eq!(vm.dna.helix.strands.len(), 1);
        let genes = &vm.dna.helix.strands[0].genes;
        assert_eq!(genes.len(), 2); // push, 42 (as push(42))
                                    // Wait, "42" as string compiles to push(42) if number-like?
                                    // Let's verify the genes.
                                    // "push 42" -> push, push(42). Wait, "push 42" is two tokens.
                                    // push -> Push (no args yet? no, push takes 1 arg)
                                    // 42 -> Push(42)
                                    // The compiler parses "push 42" as:
                                    // push (takes 1 arg? No, push takes 1 arg in OpCode, but in ChimeraScript...
                                    // `push(42)` is explicit.
                                    // `42` compiles to `push(42)`.
                                    // `push` compiles to `push`?
                                    // OpCode::Push expects 1 arg. `parse_simple_op` adds empty args if not macro.
                                    // So `push` alone becomes `push()`. This is invalid execution-wise (stack underflow/error) but valid compile-wise.
                                    // `42` becomes `push(42)`.
                                    // So we get `push()`, `push(42)`.

        // Let's make the blueprint valid: "42"
        // That compiles to `push(42)`.
    }

    #[test]
    fn test_anvil_valid_source() {
        let dna = Dna {
            evolution_config: None,
            helix: Helix { strands: vec![] },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.prologue_state.active = true;

        let anvil_y = 5;
        let anvil_x = 5;
        vm.grid[anvil_y][anvil_x] = Value::Str("⚒".to_string());

        // Blueprint: "42 print" -> push(42), print()
        let row1 = Value::Junction(
            JunctionType::Dish,
            vec![Value::Int(42), Value::Str("print".to_string())],
        );
        let blueprint = Value::Junction(JunctionType::Dish, vec![row1]);

        vm.grid[anvil_y][anvil_x - 1] = Value::Str("!".to_string());
        vm.grid[anvil_y][anvil_x - 2] = blueprint;

        exec_prologue_tick(&mut vm);

        println!("{}", vm.output.join("\n"));
        assert!(vm
            .output
            .iter()
            .any(|s| s.contains("ANVIL: Forged strand 0")));

        let genes = &vm.dna.helix.strands[0].genes;
        assert_eq!(genes.len(), 2);
        // 42 -> Push(42)
        assert_eq!(genes[0].op, chimera_lang::opcode::OpCode::Push);
        // print -> Print
        assert_eq!(genes[1].op, chimera_lang::opcode::OpCode::Print);

        // Check Output Signal at South (6, 5)
        // Signal grid is cleared at start of tick, but Anvil runs in Sink phase (step 5).
        // Sinks write to signal_grid for visual feedback/chaining in SAME tick?
        // Or next tick?
        // `apply_anvil_rune` writes to `signal_grid[sy][sx]`.
        // So it should be visible.
        if let Some(Value::Int(idx)) = &vm.prologue_state.signal_grid[6][5] {
            assert_eq!(*idx, 0);
        } else {
            panic!("Expected signal at South (6, 5)");
        }
    }
}
