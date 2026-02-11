#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::alchemy::transmute_crucible;
    use crate::vm::{ChimeraVM, Value};

    fn make_dna(genes: Vec<Gene>) -> Dna {
        Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        }
    }

    #[test]
    fn test_alchemy_time_reverse() {
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            },
            Gene {
                op: OpCode::Add,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));

        vm.crucible.add(Value::Int(0)); // Strand 0
        vm.crucible.add(Value::Str("Time".to_string()));

        transmute_crucible(&mut vm);

        let result = vm.crucible.contents.pop();
        assert!(matches!(result, Some(Value::Int(1))), "Should return new strand index 1");

        let new_genes = &vm.dna.helix.strands[1].genes;
        assert_eq!(new_genes[0].op, OpCode::Add);
        assert_eq!(new_genes[1].op, OpCode::Push);
    }

    #[test]
    fn test_alchemy_gravity_sort() {
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![],
            },
            Gene {
                op: OpCode::Add,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));

        vm.crucible.add(Value::Int(0));
        vm.crucible.add(Value::Str("Gravity".to_string()));

        transmute_crucible(&mut vm);

        let result = vm.crucible.contents.pop();
        assert!(matches!(result, Some(Value::Int(1))));

        let new_genes = &vm.dna.helix.strands[1].genes;
        // "add" < "push"
        assert_eq!(new_genes[0].op, OpCode::Add);
        assert_eq!(new_genes[1].op, OpCode::Push);
    }

    #[test]
    fn test_alchemy_light_clone() {
        let genes = vec![
            Gene {
                op: OpCode::Nop,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));

        vm.crucible.add(Value::Int(0));
        vm.crucible.add(Value::Str("Light".to_string()));

        transmute_crucible(&mut vm);

        let result = vm.crucible.contents.pop();
        assert!(matches!(result, Some(Value::Int(1))));

        let new_genes = &vm.dna.helix.strands[1].genes;
        assert_eq!(new_genes.len(), 2);
    }

    #[test]
    fn test_alchemy_shadow_invert() {
        let genes = vec![
            Gene {
                op: OpCode::Add,
                args: vec![],
            },
            Gene {
                op: OpCode::Photosynthesize,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));

        vm.crucible.add(Value::Int(0));
        vm.crucible.add(Value::Str("Shadow".to_string()));

        transmute_crucible(&mut vm);

        let result = vm.crucible.contents.pop();
        assert!(matches!(result, Some(Value::Int(1))));

        let new_genes = &vm.dna.helix.strands[1].genes;
        assert_eq!(new_genes[0].op, OpCode::Sub);
        assert_eq!(new_genes[1].op, OpCode::Consume);
    }

    #[test]
    fn test_alchemy_chaos_shuffle() {
        // Needs enough genes to make shuffle detectable
        let mut genes = Vec::new();
        for i in 0..10 {
            genes.push(Gene { op: OpCode::Push, args: vec![Nucleotide::Number(i)] });
        }
        let mut vm = ChimeraVM::new(make_dna(genes.clone()));

        vm.crucible.add(Value::Int(0));
        vm.crucible.add(Value::Str("Chaos".to_string()));

        transmute_crucible(&mut vm);

        let result = vm.crucible.contents.pop();
        assert!(matches!(result, Some(Value::Int(1))));

        let new_genes = &vm.dna.helix.strands[1].genes;
        assert_ne!(new_genes, &genes, "Shuffled genes should differ (probabilistic)");
        assert_eq!(new_genes.len(), 10);
    }

    #[test]
    fn test_philosophers_stone_recipe() {
        let mut vm = ChimeraVM::new(make_dna(vec![]));

        vm.crucible.add(Value::Str("Sulfur".to_string()));
        vm.crucible.add(Value::Str("Mercury".to_string()));
        vm.crucible.add(Value::Str("Salt".to_string()));

        transmute_crucible(&mut vm);

        let result = vm.crucible.contents.pop();
        match result {
            Some(Value::Str(s)) => assert_eq!(s, "Philosopher's Stone"),
            _ => panic!("Expected Philosopher's Stone, got {:?}", result),
        }
    }

    #[test]
    fn test_philosophers_stone_energy() {
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("Philosopher's Stone".to_string())],
            },
            Gene {
                op: OpCode::Consume,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));
        // Run push
        vm.step();
        // Run consume
        vm.step();

        // Initial 50. Push costs 0? Step cost 1.
        // Step 1: Energy 49. Stack: ["PS"]
        // Step 2: Energy 48. Consume (+1000). Total 1048.
        assert!(vm.energy > 1000);
    }
}
