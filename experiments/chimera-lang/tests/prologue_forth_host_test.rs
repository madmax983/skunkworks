#[cfg(test)]
mod tests {
    use chimera_lang::ast::{Dna, Helix};
    use chimera_lang::vm::prologue::exec_prologue_tick;
    use chimera_lang::vm::{ChimeraVM, Value};

    #[test]
    fn test_forth_host_engineering() {
        // 1. Initialize VM with empty DNA
        let dna = Dna {
            helix: Helix { strands: vec![] },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.prologue_state.active = true;

        // 2. Setup Grid
        // 0 1 2 3 4      5  6 7 8 9       10 11 12
        // ₣ n " 0 "push" 42 w " 1 "print" 0  w  x

        vm.grid[0][0] = Value::Str("₣".to_string());
        vm.grid[0][1] = Value::Str("n".to_string()); // New Strand -> [0]
        vm.grid[0][2] = Value::Str("\"".to_string()); // Dup -> [0, 0]
        vm.grid[0][3] = Value::Int(0); // Gene 0 -> [0, 0, 0]
        vm.grid[0][4] = Value::Str("\"push\"".to_string()); // Op -> [0, 0, 0, "push"]
        vm.grid[0][5] = Value::Int(42); // Arg -> [0, 0, 0, "push", 42]
        vm.grid[0][6] = Value::Str("w".to_string()); // Write -> [0]

        vm.grid[0][7] = Value::Str("\"".to_string()); // Dup -> [0, 0]
        vm.grid[0][8] = Value::Int(1); // Gene 1 -> [0, 0, 1]
        vm.grid[0][9] = Value::Str("\"print\"".to_string()); // Op -> [0, 0, 1, "print"]
        vm.grid[0][10] = Value::Int(0); // Arg -> [0, 0, 1, "print", 0]
        vm.grid[0][11] = Value::Str("w".to_string()); // Write -> [0]

        vm.grid[0][12] = Value::Str("x".to_string()); // Execute -> []

        // 3. Step Prologue enough times to execute all instructions
        // There are 13 instructions. Agent moves 1 step per tick.
        // It starts at 0,0.
        // Tick 1: Scan, Agent at 0,0 executes '₣' (Self) -> Does nothing (or moves East default).
        // Wait, '₣' moves East by default.
        // process_forth_agent:
        // 1. Extract State (Default 0,1 East, Empty Underfoot).
        // 2. Execute Underfoot ('₣' logic? No, '₣' is just the marker. Underfoot is what was there before.
        //    Initially 0.
        // 3. Move to target.
        // So at Tick 1: Agent moves to 0,1. Underfoot becomes 'n'. Grid at 0,1 becomes '₣'. Grid at 0,0 becomes 0.

        // Tick 2: Agent at 0,1. Underfoot is 'n'. Executes 'n'. Moves to 0,2.

        for _ in 0..20 {
            exec_prologue_tick(&mut vm);

            // Check if interrupt happened
            if !vm.call_stack.is_empty() || vm.ip.0 == 0 {
                // Check if execution started
                // But wait, step() executes genes. exec_prologue_tick() only does grid logic.
                // We need to run vm.step() to actually execute the genes when interrupt happens?
                // `vm.interrupt()` sets `vm.ip` and pushes to `call_stack`.
                // But `exec_prologue_tick` is called inside `vm.step()` usually.
                // Here we are calling `exec_prologue_tick` manually.
                // We should manually check if DNA was modified.
            }
        }

        // Verify DNA was written
        assert_eq!(vm.dna.helix.strands.len(), 1, "Strand should be created");
        let strand = &vm.dna.helix.strands[0];
        assert_eq!(strand.genes.len(), 2, "Should have 2 genes");
        assert_eq!(strand.genes[0].op.to_string(), "push");
        assert_eq!(strand.genes[1].op.to_string(), "print");

        // Now run VM step to process the interrupt and execute genes
        // The interrupt was set by 'x'.
        vm.step(); // Should execute push(42)
        vm.step(); // Should execute print()

        assert!(
            vm.output.contains(&"42".to_string()),
            "Output should contain 42"
        );
    }
}
