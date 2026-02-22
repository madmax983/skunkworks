#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Helix};
    use crate::vm::prologue::exec_prologue_tick;
    use crate::vm::{ChimeraVM, Value};

    #[test]
    fn test_forth_agent_math() {
        let dna = Dna { evolution_config: None,
            helix: Helix { strands: vec![] },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.prologue_state.active = true;

        // Setup Circuit:
        // ₣ -> 10 -> 20 -> + -> .
        // 5,5  5,6   5,7   5,8  5,9
        vm.grid[5][5] = Value::Str("₣".to_string());
        vm.grid[5][6] = Value::Int(10);
        vm.grid[5][7] = Value::Int(20);
        vm.grid[5][8] = Value::Str("+".to_string());
        vm.grid[5][9] = Value::Str(".".to_string());

        // Initialize Agent with NOP (_) underfoot to avoid pushing initial 0
        let state = Value::Junction(
            crate::ast::JunctionType::All,
            vec![Value::Int(0), Value::Int(1), Value::Str("_".to_string())],
        );
        // We need pack_agent_data but it is not public? It is in mod.rs but not pub.
        // It is defined as `fn pack_agent_data`. Not `pub fn`.
        // So I cannot call it from test module if it's in parent?
        // Test module is `mod tests` inside `prologue_forth_test.rs`.
        // `prologue_forth_test.rs` is `mod prologue_forth_test` in `mod.rs`.
        // So `pack_agent_data` is in `super` (mod.rs).
        // I can access `super::pack_agent_data` if I make it `pub(crate)`.

        // OR I manually construct the value.
        // pack_agent_data(state, stack) -> if stack empty, returns state.
        // So just insert `state` into registers.
        vm.prologue_state.registers.insert((5, 5), state);

        // Tick 1: Scan. ₣ found at 5,5.
        // It reads 5,5 (₣) -> No op (or push ₣ string? No, logic handles it).
        // Wait, process_forth_agent logic:
        // Reads cell at (y, x).
        // If cell is "₣", it matches `_` case -> parses as string "₣" if not number.
        // But "₣" is not number.
        // So it might push "₣" to stack if I'm not careful.
        // My logic: `if let Ok(n) = s.parse::<i64>()` ...
        // `else if s.starts_with('"')`...
        // `else`... nothing.
        // So "₣" does nothing. Good.
        // It updates state (Direction East default).
        // Moves to 5,6.
        // Logic stores "10" (underfoot) in state.
        // Updates grid: 5,5 restored (was ₣, stays ₣? No, underfoot was 0).
        // Wait, if 5,5 was ₣, process_agents sees current_type "₣".
        // It clears 5,5 to 0.
        // But process_forth_agent restored underfoot (0) to 5,5.
        // Then process_agents sets 5,6 to "₣".

        exec_prologue_tick(&mut vm);

        // After Tick 1: Agent should be at 5,6.
        let agent = vm
            .prologue_state
            .agents
            .iter()
            .find(|a| a.x == 6 && a.y == 5)
            .expect("Agent should move to 5,6");
        // Stack should be empty (it moved onto 10, but hasn't PROCESSED 10 yet. It processes what it is standing on).
        // At start of Tick 1, it stood on ¶.
        assert!(agent.stack.is_empty());

        // Tick 2: Stands on 10. Reads 10. Pushes 10. Moves to 5,7.
        exec_prologue_tick(&mut vm);
        let agent = vm
            .prologue_state
            .agents
            .iter()
            .find(|a| a.x == 7 && a.y == 5)
            .expect("Agent should move to 5,7");
        assert_eq!(agent.stack.len(), 1);
        assert_eq!(agent.stack[0], Value::Int(10));

        // Tick 3: Stands on 20. Pushes 20. Moves to 5,8.
        exec_prologue_tick(&mut vm);
        let agent = vm
            .prologue_state
            .agents
            .iter()
            .find(|a| a.x == 8 && a.y == 5)
            .expect("Agent should move to 5,8");
        assert_eq!(agent.stack.len(), 2);
        assert_eq!(agent.stack[0], Value::Int(10));
        assert_eq!(agent.stack[1], Value::Int(20));

        // Tick 4: Stands on +. Adds. Pushes 30. Moves to 5,9.
        exec_prologue_tick(&mut vm);
        let agent = vm
            .prologue_state
            .agents
            .iter()
            .find(|a| a.x == 9 && a.y == 5)
            .expect("Agent should move to 5,9");
        assert_eq!(agent.stack.len(), 1);
        assert_eq!(agent.stack[0], Value::Int(30));

        // Tick 5: Stands on .. Log 30. Moves to 5,10.
        exec_prologue_tick(&mut vm);
        let agent = vm
            .prologue_state
            .agents
            .iter()
            .find(|a| a.x == 10 && a.y == 5)
            .expect("Agent should move to 5,10");
        assert!(agent.stack.is_empty());

        // Check output
        let last_log = vm.output.last().expect("Should have output");
        // Value::Int display is just the number
        assert!(last_log.contains("₣ 0: 30"), "Actual log: '{}'", last_log);

        // Verify non-destructive (Trail)
        // 5,6 was 10. Agent moved off it. Is it 10 again?
        assert_eq!(vm.grid[5][6], Value::Int(10));
        assert_eq!(vm.grid[5][7], Value::Int(20));
        assert_eq!(vm.grid[5][8], Value::Str("+".to_string()));
    }
}
