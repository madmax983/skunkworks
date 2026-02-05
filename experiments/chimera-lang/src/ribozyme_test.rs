#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Helix, JunctionType, Strand};
    use crate::opcode::OpCode;
    use crate::vm::{ChimeraVM, Value};

    fn make_vm() -> ChimeraVM {
        let dna = Dna {
            helix: Helix {
                strands: vec![std::rc::Rc::new(Strand { genes: vec![] })],
            },
        };
        ChimeraVM::new(dna)
    }

    #[test]
    fn test_eval() {
        let mut vm = make_vm();
        // push("[ push(42) ]") eval()
        vm.stack.push(Value::Str("[ push(42) ]".to_string()));
        let op = OpCode::Eval;
        vm.execute_gene_inner(op, &[]);

        assert_eq!(vm.stack.pop(), Some(Value::Int(42)));
    }

    #[test]
    fn test_map_junction() {
        let mut vm = make_vm();
        // Stack: Junction(1, 2, 3), "[ push(1) add() ]"
        let j = Value::Junction(
            JunctionType::Any,
            vec![Value::Int(1), Value::Int(2), Value::Int(3)],
        );
        vm.stack.push(j);
        vm.stack.push(Value::Str("[ push(1) add() ]".to_string()));

        let op = OpCode::Map;
        vm.execute_gene_inner(op, &[]);

        // Expected: Junction(2, 3, 4)
        if let Some(Value::Junction(_, vals)) = vm.stack.pop() {
            assert_eq!(vals.len(), 3);
            assert_eq!(vals[0], Value::Int(2));
            assert_eq!(vals[1], Value::Int(3));
            assert_eq!(vals[2], Value::Int(4));
        } else {
            panic!("Expected Junction result");
        }
    }

    #[test]
    fn test_fold_junction() {
        let mut vm = make_vm();
        // Stack: Junction(1, 2, 3), Init(0), "[ add() ]"
        let j = Value::Junction(
            JunctionType::Any,
            vec![Value::Int(1), Value::Int(2), Value::Int(3)],
        );
        vm.stack.push(j);
        vm.stack.push(Value::Int(0)); // Init
        vm.stack.push(Value::Str("[ add() ]".to_string())); // Function

        let op = OpCode::Fold;
        vm.execute_gene_inner(op, &[]);

        // Expected: 0 + 1 + 2 + 3 = 6
        assert_eq!(vm.stack.pop(), Some(Value::Int(6)));
    }

    #[test]
    fn test_filter_junction() {
        let mut vm = make_vm();

        let j = Value::Junction(
            JunctionType::Any,
            vec![Value::Int(1), Value::Int(2), Value::Int(3), Value::Int(4)],
        );
        vm.stack.push(j);
        vm.stack.push(Value::Str("[ push(2) sub() ]".to_string()));

        let op = OpCode::Filter;
        vm.execute_gene_inner(op, &[]);

        if let Some(Value::Junction(_, vals)) = vm.stack.pop() {
            // Should contain 1, 3, 4
            // 1-2 = -1 (true)
            // 2-2 = 0 (false)
            // 3-2 = 1 (true)
            // 4-2 = 2 (true)
            assert_eq!(vals.len(), 3);
            assert_eq!(vals[0], Value::Int(1));
            assert_eq!(vals[1], Value::Int(3));
            assert_eq!(vals[2], Value::Int(4));
        } else {
            panic!("Expected Junction");
        }
    }

    #[test]
    fn test_zip_junction() {
        let mut vm = make_vm();
        // A: (1, 2)
        // B: (3, 4)
        // Zip -> ((1, 3), (2, 4))

        let a = Value::Junction(JunctionType::Any, vec![Value::Int(1), Value::Int(2)]);
        let b = Value::Junction(JunctionType::Any, vec![Value::Int(3), Value::Int(4)]);

        vm.stack.push(a);
        vm.stack.push(b);

        let op = OpCode::Zip;
        vm.execute_gene_inner(op, &[]);

        if let Some(Value::Junction(_, vals)) = vm.stack.pop() {
            assert_eq!(vals.len(), 2);
            // Element 0: Junction(1, 3)
            if let Value::Junction(_, pair0) = &vals[0] {
                assert_eq!(pair0[0], Value::Int(1));
                assert_eq!(pair0[1], Value::Int(3));
            } else {
                panic!("Expected pair");
            }
        } else {
            panic!("Expected Junction");
        }
    }
}
