#[cfg(test)]
mod tests {
    use chimera_lang::vm::ChimeraVM;
    use chimera_lang::vm::Value;
    use chimera_lang::prologue_compiler;
    use std::path::Path;

    #[test]
    fn test_custom_agent_execution() {
        // We assume the test is run from repo root, so path is experiments/chimera-lang/tests/custom_agent.pro
        // But tests run with CWD as package root? No, usually package root.
        // Let's try relative path.
        let source_path = Path::new("tests/custom_agent.pro");
        let source = std::fs::read_to_string(source_path).expect("Failed to read test file");

        let (dna, grid, orca_mode, custom_runes, custom_agents) =
            prologue_compiler::compile(&source, source_path.parent()).expect("Failed to compile");

        let mut vm = ChimeraVM::new(dna);
        if let Some(g) = grid {
            vm.grid = g;
            vm.prologue_state.active = true;
        } else {
            panic!("Grid not found in compiled output");
        }
        vm.prologue_state.custom_runes = custom_runes;
        vm.prologue_state.custom_agents = custom_agents;
        if let Some(orca) = orca_mode {
            vm.prologue_state.orca_mode = orca;
        }

        // Verify Initial State
        assert_eq!(vm.grid[1][2], Value::Str("X".to_string()), "Initial placement failed");

        // Run one tick
        chimera_lang::vm::prologue::exec_prologue_tick(&mut vm);

        // Verify Move
        // Agent should move South to (2, 2)
        assert_eq!(vm.grid[1][2], Value::Int(0), "Old position not cleared");
        assert_eq!(vm.grid[2][2], Value::Str("X".to_string()), "New position not set");

        // Run another tick
        chimera_lang::vm::prologue::exec_prologue_tick(&mut vm);

        // Verify Move
        assert_eq!(vm.grid[2][2], Value::Int(0));
        assert_eq!(vm.grid[3][2], Value::Str("X".to_string()));
    }
}
