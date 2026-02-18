#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Helix};
    use crate::vm::prologue::exec_prologue_tick;
    use crate::vm::{ChimeraVM, Value};

    #[test]
    fn test_chronos_delay_line() {
        let dna = Dna {
            helix: Helix { strands: vec![] },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.prologue_state.active = true;

        // Setup Circuit: Source -> s -> g -> Sink
        // 1 2 3 -> ! -> s -> g -> ?

        // At tick 0: ! emits 1.
        // s reads 1, saves to history, outputs 1.
        // g reads s's history (1), outputs 1.

        // Wait, if g reads history, does it see it immediately?
        // s runs. pushes 1.
        // g runs. sees 1 in s's history. pops 1.
        // So no delay?

        // Let's force a delay.
        // Maybe g only outputs if history len > 1?
        // Or manually fill history first?

        vm.grid[4][5] = Value::Int(42); // North of Source
        vm.grid[5][5] = Value::Str("!".to_string()); // Source
        vm.grid[5][6] = Value::Str("s".to_string()); // Sporulate (Record)
        vm.grid[5][7] = Value::Str("g".to_string()); // Germinate (Play/Pop Front)
        vm.grid[5][8] = Value::Str("?".to_string()); // Sink

        // Tick 1: ! emits 42. s reads 42, pushes to history. outputs 42.
        // g reads history (42), pops it, outputs 42.
        exec_prologue_tick(&mut vm);

        // Check s history (should be empty if g popped it?)
        // If they run in order s then g?
        // Iteration order is based on scan?
        // Scan order is Y then X.
        // So s (5,6) runs before g (5,7).
        // s pushes. g pops. Immediate transfer.

        assert!(vm
            .prologue_state
            .history
            .get(&(5, 6))
            .map(|q| q.is_empty())
            .unwrap_or(true));

        // Output should show 42
        let output = vm.output.join("\n");
        assert!(output.contains("PROLOGUE: Sink at 8,5 received Int(42)"));
    }

    #[test]
    fn test_chronos_reverse() {
        let dna = Dna {
            helix: Helix { strands: vec![] },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.prologue_state.active = true;

        // Populate history manually
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(Value::Int(1));
        queue.push_back(Value::Int(2));
        queue.push_back(Value::Int(3));
        vm.prologue_state.history.insert((5, 6), queue);

        // Setup: s (with history) -> r -> ?
        vm.grid[5][6] = Value::Str("s".to_string());
        vm.grid[5][7] = Value::Str("r".to_string()); // Retrograde (Pop Back)
        vm.grid[5][8] = Value::Str("?".to_string());

        // Tick 1: r pops back (3)
        exec_prologue_tick(&mut vm);

        let output = vm.output.join("\n");
        assert!(output.contains("PROLOGUE: Sink at 8,5 received Int(3)"));

        // History should have 1, 2
        let h = vm.prologue_state.history.get(&(5, 6)).unwrap();
        assert_eq!(h.len(), 2);
        assert_eq!(h[0], Value::Int(1));
        assert_eq!(h[1], Value::Int(2));
    }
}
