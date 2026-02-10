#[cfg(feature = "nova")]
#[cfg(test)]
mod tests {
    use chimera_lang::ast::{Dna, Gene, Helix, JunctionType, Nucleotide, Strand};
    use chimera_lang::opcode::OpCode;
    use chimera_lang::vm::{ChimeraVM, Value};

    fn make_vm() -> ChimeraVM {
        let dna = Dna {
            helix: Helix { strands: vec![] },
        };
        ChimeraVM::new(dna)
    }

    #[test]
    fn test_babel_transform_simple() {
        let mut vm = make_vm();

        // 1. Create Rules
        // Rule 1: "A" -> "Alpha"
        let rule1 = Value::Junction(
            JunctionType::All,
            vec![Value::Str("A".to_string()), Value::Str("Alpha".to_string())],
        );
        // Rule 2: "B" -> "Beta"
        let rule2 = Value::Junction(
            JunctionType::All,
            vec![Value::Str("B".to_string()), Value::Str("Beta".to_string())],
        );

        let rules = Value::Junction(JunctionType::Any, vec![rule1, rule2]);

        // 2. Create Target
        // Junction(Any, ["A", "B", "C"])
        let target = Value::Junction(
            JunctionType::Any,
            vec![
                Value::Str("A".to_string()),
                Value::Str("B".to_string()),
                Value::Str("C".to_string()),
            ],
        );

        // 3. Push to Stack
        vm.stack.push(rules);
        vm.stack.push(target);

        // 4. Execute Transform
        // Need to construct Gene because we can't call exec directly easily from integration test without exposing modules
        // But we can use execute_gene_inner if we import it, or just use vm.step() with a DNA.
        // Actually, integration tests in `tests/` treat crate as external.
        // So we should use public API. `vm.execute_gene` is not public? It's private in `lib.rs`?
        // `execute_gene_inner` is `pub(crate)`.
        // So we must use `vm.step()`.

        let transform_gene = Gene {
            op: OpCode::Transform,
            args: vec![],
        };

        // We inject the gene into a strand
        vm.dna.helix.strands.push(Strand {
            genes: vec![transform_gene],
        });

        vm.step();

        // 5. Verify
        let result = vm.stack.pop().expect("Stack underflow");
        if let Value::Junction(t, children) = result {
            assert_eq!(t, JunctionType::Any);
            assert_eq!(children.len(), 3);
            assert_eq!(children[0], Value::Str("Alpha".to_string()));
            assert_eq!(children[1], Value::Str("Beta".to_string()));
            assert_eq!(children[2], Value::Str("C".to_string()));
        } else {
            panic!("Expected Junction, got {:?}", result);
        }
    }

    #[test]
    fn test_babel_transform_glob() {
        let mut vm = make_vm();

        // Rule: "*.txt" -> "TEXT_FILE"
        let rule = Value::Junction(
            JunctionType::All,
            vec![
                Value::Str("*.txt".to_string()),
                Value::Str("TEXT_FILE".to_string())
            ],
        );
        let rules = Value::Junction(JunctionType::Any, vec![rule]);

        let target = Value::Str("document.txt".to_string());

        vm.stack.push(rules);
        vm.stack.push(target);

        let transform_gene = Gene {
            op: OpCode::Transform,
            args: vec![],
        };
        vm.dna.helix.strands.push(Strand {
            genes: vec![transform_gene],
        });

        vm.step();

        let result = vm.stack.pop().expect("Stack underflow");
        assert_eq!(result, Value::Str("TEXT_FILE".to_string()));
    }

    #[test]
    fn test_babel_transform_deep() {
        let mut vm = make_vm();

        // Rule: 0 -> 1
        let rule = Value::Junction(
            JunctionType::All,
            vec![Value::Int(0), Value::Int(1)],
        );
        let rules = Value::Junction(JunctionType::Any, vec![rule]);

        // Target: [0, [0, 2]]
        let inner = Value::Junction(
            JunctionType::All,
            vec![Value::Int(0), Value::Int(2)],
        );
        let target = Value::Junction(
            JunctionType::All,
            vec![Value::Int(0), inner],
        );

        vm.stack.push(rules);
        vm.stack.push(target);

        let transform_gene = Gene {
            op: OpCode::Transform,
            args: vec![],
        };
        vm.dna.helix.strands.push(Strand {
            genes: vec![transform_gene],
        });

        vm.step();

        let result = vm.stack.pop().expect("Stack underflow");
        // Expected: [1, [1, 2]]
        if let Value::Junction(_, children) = result {
            assert_eq!(children[0], Value::Int(1));
            if let Value::Junction(_, inner_children) = &children[1] {
                assert_eq!(inner_children[0], Value::Int(1));
                assert_eq!(inner_children[1], Value::Int(2));
            } else {
                panic!("Structure mismatch");
            }
        } else {
            panic!("Expected Junction");
        }
    }
}
