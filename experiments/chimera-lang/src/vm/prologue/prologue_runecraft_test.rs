#[cfg(test)]
mod tests {
    use crate::vm::ChimeraVM;
    use crate::vm::Value;
    use crate::ast::{Dna, Helix, Strand, Gene};
    use crate::opcode::OpCode;
    use crate::vm::prologue::exec_prologue_tick;

    fn create_vm() -> ChimeraVM {
        let dna = Dna { evolution_config: None, helix: Helix { strands: vec![] } };
        let mut vm = ChimeraVM::new(dna);
        vm.prologue_state.active = true;
        vm
    }

    #[test]
    fn test_custom_rune_definition() {
        let mut vm = create_vm();

        // 1. Setup Grid for Definition
        // West: "H" (The new rune)
        // North: 1 (The strand index) - Note: 0 is considered "Empty" signal by Source (!), so use 1.
        // Center: £ (The definition rune)

        vm.grid[5][4] = Value::Str("H".to_string());
        vm.grid[5][5] = Value::Str("!".to_string()); // Signal "H" at (5,5)

        vm.grid[4][5] = Value::Int(1);
        vm.grid[4][6] = Value::Str("!".to_string()); // (4,6) emits 1.

        vm.grid[5][6] = Value::Str("£".to_string()); // The Rune Definition Rune

        // Tick 1: Definition
        exec_prologue_tick(&mut vm);

        // Verify "H" is in custom_runes
        assert!(vm.prologue_state.custom_runes.contains_key("H"));
        assert_eq!(vm.prologue_state.custom_runes.get("H"), Some(&1));
    }

    #[test]
    fn test_custom_rune_execution() {
        let mut vm = create_vm();

        // 0. Add dummy strand so we aren't at IP 0
        vm.dna.helix.strands.push(Strand { genes: vec![] });
        vm.ip = (0, 0); // Start at dummy

        // 1. Add a Strand 1: "Hello World" print
        let genes = vec![
            Gene { op: OpCode::Push, args: vec![crate::ast::Nucleotide::String("Runs!".to_string())] },
            Gene { op: OpCode::Print, args: vec![] },
        ];
        vm.dna.helix.strands.push(Strand { genes });

        // 2. Register Rune "R" -> Strand 1
        vm.prologue_state.custom_runes.insert("R".to_string(), 1);

        // 3. Place Rune "R" on Grid with a signal source
        vm.grid[5][5] = Value::Str("R".to_string());
        vm.grid[5][4] = Value::Int(1);
        vm.grid[5][4] = Value::Str("!".to_string()); // Source emitting to East (towards R)
        // Wait, ! reads from West and emits to Self.
        // So ! at 5,4 needs input at 5,3.
        vm.grid[5][3] = Value::Int(1);

        // Or simpler: put signal directly on signal_grid manually for test
        // R is at 5,5. Put signal at West neighbor (5,4).
        vm.prologue_state.signal_grid[5][4] = Some(Value::Int(1));

        // 4. Tick
        exec_prologue_tick(&mut vm);

        // 5. Verify Execution
        // Since "R" is now a registered rune, it should be picked up by scan_grid_rules
        // And executed by process_sinks (via runecraft)
        // The output should contain the print.

        let output = vm.output.join("\n");
        assert!(output.contains("RUNECRAFT: Executed Custom Rune 'R'"));
    }
}
