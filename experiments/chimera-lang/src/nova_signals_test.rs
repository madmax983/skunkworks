#[cfg(test)]
mod tests {
    use crate::vm::{ChimeraVM, Value};
    use crate::ast::{Dna, Helix, Strand, Gene, Nucleotide};
    use crate::opcode::OpCode;

    fn make_empty_vm() -> ChimeraVM {
        // Create a dummy strand so VM doesn't halt
        let genes = vec![
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] },
            Gene { op: OpCode::Drop, args: vec![] },
            Gene { op: OpCode::Jump, args: vec![Nucleotide::Number(0)] }, // Infinite loop
        ];
        let dna = Dna { helix: Helix { strands: vec![Strand { genes }] } };
        let mut vm = ChimeraVM::new(dna);
        vm.phase = crate::vm::nova::Phase::Corporeal;
        vm
    }

    #[test]
    fn test_bang_propagation() {
        let mut vm = make_empty_vm();

        // Setup Grid: * at (5,5)
        vm.grid[5][5] = Value::Str("*".to_string());

        // Inject Signal
        vm.signal_grid[5][5] = 10;

        // Step
        vm.step();

        // Neighbors should have signal
        assert!(vm.signal_grid[5][4] > 0, "West neighbor failed");
        assert!(vm.signal_grid[5][6] > 0, "East neighbor failed");
        assert!(vm.signal_grid[4][5] > 0, "North neighbor failed");
        assert!(vm.signal_grid[6][5] > 0, "South neighbor failed");

        // Original should be 0 (unless back-propagated by something, which shouldn't happen here)
        // Wait, process_signals creates next_signals from scratch.
        // So original signal dissipates if not re-triggered.
        assert_eq!(vm.signal_grid[5][5], 0, "Center signal should dissipate");
    }

    #[test]
    fn test_directional_propagation() {
        let mut vm = make_empty_vm();

        // Setup Grid: > at (5,5)
        vm.grid[5][5] = Value::Str(">".to_string());

        // Inject Signal
        vm.signal_grid[5][5] = 10;

        // Step
        vm.step();

        // East should have signal
        assert!(vm.signal_grid[5][6] > 0, "East neighbor failed");

        // West should NOT
        assert_eq!(vm.signal_grid[5][4], 0, "West neighbor shouldn't fire");
    }

    #[test]
    fn test_triggered_execution() {
        let mut vm = make_empty_vm();

        // Setup: * at (5,5), "add" at (5,6)
        vm.grid[5][5] = Value::Str("*".to_string());
        vm.grid[5][6] = Value::Str("add".to_string());

        // Stack: 10, 20
        vm.stack.push(Value::Int(10));
        vm.stack.push(Value::Int(20));

        // Inject Signal
        vm.signal_grid[5][5] = 10;

        // Step
        // Bang propagates to (5,6) next signal grid.
        // DNA executes Push(0).
        vm.step();

        // Stack should be 10, 20, 0 (dummy push)
        assert_eq!(vm.stack.len(), 3);

        // Step AGAIN
        // Signal trigger Add. Add(20, 0) -> 20. Stack: [10, 20].
        // DNA execute Drop. Stack: [10].
        vm.step();

        // Stack should be 10
        assert_eq!(vm.stack.len(), 1);
        assert_eq!(vm.stack[0], Value::Int(10));
    }
}
