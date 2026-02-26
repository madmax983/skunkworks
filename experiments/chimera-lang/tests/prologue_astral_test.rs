#[cfg(test)]
mod tests {
    use chimera_lang::ast::{Dna, Helix};
    use chimera_lang::vm::prologue::astral::AstralState;
    use chimera_lang::vm::prologue::exec_prologue_tick;
    use chimera_lang::vm::{ChimeraVM, Value};

    #[test]
    fn test_astral_agent_spawn_and_gravity() {
        let dna = Dna {
            evolution_config: None,
            helix: Helix { strands: vec![] },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.prologue_state.active = true;

        // Spawn an Astral Probe '★' at (5, 5)
        vm.grid[5][5] = Value::Str("★".to_string());
        // Place a Gravity Well '*' at (5, 10) (High Mass = 50.0)
        vm.grid[5][10] = Value::Str("*".to_string());

        // Run a tick - Initial Scan
        exec_prologue_tick(&mut vm);

        // Verify the agent was detected
        let agent_idx = vm.prologue_state.agents.iter().position(|a| {
            if let Value::Str(s) = &vm.grid[a.y][a.x] {
                s == "★"
            } else {
                false
            }
        });
        assert!(agent_idx.is_some(), "Astral agent not found in state");

        let agent = &vm.prologue_state.agents[agent_idx.unwrap()];
        let state = AstralState::from_value(&agent.state);

        // After 1 tick (scan + move), velocity should be non-zero towards (5, 10)
        // Since y is same, vy should be ~0, vx should be positive.
        // Wait, calculate_gravity is called during process_astral_agent which happens in tick 1.

        println!(
            "Tick 1: Pos({:.2}, {:.2}) Vel({:.4}, {:.4})",
            state.px, state.py, state.vx, state.vy
        );
        assert!(
            state.vx > 0.0,
            "Agent should accelerate towards gravity well (East)"
        );

        // Run more ticks to see movement
        for _ in 0..20 {
            exec_prologue_tick(&mut vm);
        }

        // We need to re-fetch the agent from state because agents list is rebuilt every tick
        let agent_idx = vm.prologue_state.agents.iter().position(|a| {
            if let Value::Str(s) = &vm.grid[a.y][a.x] {
                s == "★"
            } else {
                false
            }
        });
        assert!(agent_idx.is_some(), "Astral agent lost?");

        let agent = &vm.prologue_state.agents[agent_idx.unwrap()];
        let state = AstralState::from_value(&agent.state);
        println!(
            "Tick 6: Pos({:.2}, {:.2}) Vel({:.4}, {:.4})",
            state.px, state.py, state.vx, state.vy
        );

        // Should have moved from x=5 towards x=10
        assert!(agent.x > 5, "Agent should have moved East");
    }
}
