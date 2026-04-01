#[cfg(feature = "nova")]
#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Gene, Helix, JunctionType, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::{ChimeraVM, Value};

    fn make_empty_vm() -> ChimeraVM {
        let dna = Dna {
            evolution_config: None,
            helix: Helix {
                strands: vec![Strand { genes: vec![] }],
            },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.reactor_mode = true;
        vm
    }

    #[test]
    fn test_reactor_simple_rule() {
        let mut vm = make_empty_vm();

        // Setup Grid: [1, 1]
        vm.grid[5][5] = Value::Int(1);
        vm.grid[5][6] = Value::Int(1);

        // Add Rule: reaction(1, 1, 2)
        // Fact: junction(Any, ["reaction", 1, 1, 2])
        let rule = Value::Junction(
            JunctionType::Any,
            vec![
                Value::Str("reaction".to_string()),
                Value::Int(1),
                Value::Int(1),
                Value::Int(2),
            ],
        );
        vm.knowledge_base.push(rule);

        // Step
        vm.step();

        // Check result
        // Cell (5,5) (Val=1) has neighbor (5,6) (Val=1). Rule matches -> Becomes 2.
        // Cell (5,6) (Val=1) has neighbor (5,5) (Val=1). Rule matches -> Becomes 2.

        assert_eq!(vm.grid[5][5], Value::Int(2));
        assert_eq!(vm.grid[5][6], Value::Int(2));
    }

    #[test]
    fn test_reactor_exec_opcode() {
        let mut vm = make_empty_vm();

        // Setup OpCodes
        // Strand 0: [ Push(1), Push(1), Push(2), Reaction, Reactor ]
        // Should assert reaction(1, 1, 2) and toggle reactor off (since it started on in helper)

        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(2)],
            },
            Gene {
                op: OpCode::Reaction,
                args: vec![],
            },
            Gene {
                op: OpCode::Reactor,
                args: vec![],
            },
        ];

        vm.dna.helix.strands[0].genes = genes;

        // Execute Genes
        for _ in 0..5 {
            vm.step();
        }

        // Check KB
        let expected = Value::Junction(
            JunctionType::Any,
            vec![
                Value::Str("reaction".to_string()),
                Value::Int(1),
                Value::Int(1),
                Value::Int(2),
            ],
        );
        assert!(vm.knowledge_base.contains(&expected));

        // Check Mode (Toggled off)
        assert!(!vm.reactor_mode);
    }
}
