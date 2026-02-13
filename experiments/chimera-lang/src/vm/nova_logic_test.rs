#[cfg(test)]
mod tests {
    use crate::vm::ChimeraVM;
    use crate::vm::Value;
    use crate::vm::nova_signals::process_signals;
    use crate::opcode::OpCode;
    use crate::ast::{JunctionType, Nucleotide};
    use std::collections::HashMap;

    fn setup_vm() -> ChimeraVM {
        let mut vm = ChimeraVM::new(crate::ast::Dna {
            helix: crate::ast::Helix { strands: vec![] },
        });
        vm
    }

    #[test]
    #[cfg(all(feature = "oracle", feature = "elektra"))]
    fn test_voltage_predicate() {
        let mut vm = setup_vm();
        vm.voltage_grid[5][5] = 12.0;

        let query = Value::Junction(
            JunctionType::Any,
            vec![
                Value::Str("voltage".to_string()),
                Value::Int(5),
                Value::Int(5),
                Value::Str("?V".to_string())
            ]
        );

        let mut solutions = Vec::new();
        crate::vm::oracle::solve(
            &[query],
            HashMap::new(),
            &vm.knowledge_base,
            &vm,
            &mut solutions,
            0
        );

        assert!(!solutions.is_empty(), "Should find voltage");
        if let Some(val) = solutions[0].get("?V") {
            if let Value::Int(v) = val {
                assert_eq!(*v, 12); // Assuming int cast
            } else {
                panic!("Expected Int voltage");
            }
        }
    }

    #[test]
    #[cfg(all(feature = "oracle", feature = "nova"))]
    fn test_signal_predicate() {
        let mut vm = setup_vm();
        vm.signal_grid[3][3] = 5;

        let query = Value::Junction(
            JunctionType::Any,
            vec![
                Value::Str("signal".to_string()),
                Value::Int(3),
                Value::Int(3),
                Value::Str("?S".to_string())
            ]
        );

        let mut solutions = Vec::new();
        crate::vm::oracle::solve(
            &[query],
            HashMap::new(),
            &vm.knowledge_base,
            &vm,
            &mut solutions,
            0
        );

        assert!(!solutions.is_empty(), "Should find signal");
        if let Some(val) = solutions[0].get("?S") {
            assert_eq!(*val, Value::Int(5));
        }
    }

    #[test]
    #[cfg(all(feature = "oracle", feature = "nova"))]
    fn test_pi_operator() {
        let mut vm = setup_vm();

        // Setup Knowledge Base: Fact(1)
        vm.knowledge_base.push(Value::Int(1));

        // Setup Grid:
        // 1 (Fact ID) at (0, 0)
        // Π (Pi Operator) at (1, 0)
        // . (Empty) at (2, 0) (South)

        vm.grid[0][0] = Value::Int(1);
        vm.grid[1][0] = Value::Str("Π".to_string());

        // Activate Π with a signal (or it runs passively? Most are passive/signal triggered)
        // Let's assume it requires signal > 0 OR uppercase/special. Π is special.
        // So it runs every tick if active.

        // First tick: Process signals
        process_signals(&mut vm);

        // If Fact(1) is true (it is), Π should bang South (2,0)
        // Signal grid at (2,0) should be > 0.
        // Note: process_signals updates signal_grid for NEXT tick.

        assert!(vm.signal_grid[2][0] > 0, "Expected signal at South due to True query");

        // Test False Query
        vm.grid[0][0] = Value::Int(99); // Fact(99) does not exist
        vm.signal_grid[2][0] = 0; // Reset signal

        process_signals(&mut vm);
        assert_eq!(vm.signal_grid[2][0], 0, "Expected no signal for False query");
    }
}
