#[cfg(test)]
#[cfg(feature = "nova")]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::verbum::Rarity;
    use crate::vm::{ChimeraVM, Value};

    fn make_empty_dna() -> Dna {
        Dna {
            evolution_config: None,
            helix: Helix { strands: vec![] },
        }
    }

    #[test]
    fn test_forge_and_invoke_manual() {
        let mut vm = ChimeraVM::new(make_empty_dna());
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(42)],
            },
        ];

        let res = vm.verbum_forge.forge("Life".to_string(), genes.clone(), vec![]);
        assert!(res.is_ok());
        let id = res.unwrap();

        let word = vm.verbum_forge.get("Life").unwrap();
        assert_eq!(word.id, id);
        assert_eq!(word.cost, 5);
        assert_eq!(word.rarity, Rarity::Common);

        vm.energy = 10;
        let word_data = vm.verbum_forge.get_word_data("Life");
        assert!(word_data.is_some());
        let (genes, cost) = word_data.unwrap();

        vm.energy -= cost;
        for gene in genes {
             let _ = vm.execute_gene_inner(gene.op, &gene.args);
        }

        assert_eq!(vm.energy, 5);
        assert_eq!(vm.stack.len(), 1);
        assert_eq!(vm.stack[0], Value::Int(42));
    }

    #[test]
    fn test_opcode_forge() {
        let strand = Strand {
            genes: vec![
                Gene { op: OpCode::Push, args: vec![Nucleotide::Number(10)] },
            ]
        };

        let forge_strand = Strand {
            genes: vec![
                Gene { op: OpCode::Push, args: vec![Nucleotide::String("Magic".to_string())] },
                Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] },
                Gene { op: OpCode::Forge, args: vec![] },
            ]
        };

        let dna = Dna {
            evolution_config: None,
            helix: Helix { strands: vec![strand, forge_strand] },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.ip = (1, 0);

        vm.step(); // Push "Magic"
        vm.step(); // Push 0
        vm.step(); // Forge

        if vm.verbum_forge.get("Magic").is_none() {
            println!("VM Output: {:?}", vm.output);
            panic!("Forge failed");
        }

        assert_eq!(vm.stack.last(), Some(&Value::Int(0)));
    }

    #[test]
    fn test_opcode_speak() {
        let mut vm = ChimeraVM::new(make_empty_dna());
        let genes = vec![Gene { op: OpCode::Push, args: vec![Nucleotide::Number(123)] }];
        vm.verbum_forge.forge("Test".to_string(), genes, vec![]).unwrap();

        let invoke_strand = Strand {
            genes: vec![
                Gene { op: OpCode::Push, args: vec![Nucleotide::String("Test".to_string())] },
                Gene { op: OpCode::Speak, args: vec![] },
            ]
        };
        vm.dna.helix.strands.push(invoke_strand);
        vm.energy = 100;

        vm.step(); // Push "Test"
        vm.step(); // Speak

        if vm.stack.len() != 1 || vm.stack[0] != Value::Int(123) {
             println!("VM Output: {:?}", vm.output);
             println!("Stack: {:?}", vm.stack);
             panic!("Speak failed");
        }
    }

    #[test]
    fn test_etymology() {
        let mut vm = ChimeraVM::new(make_empty_dna());
        let genes = vec![Gene { op: OpCode::Add, args: vec![] }];
        vm.verbum_forge.forge("Sum".to_string(), genes, vec![]).unwrap();

        let etym_strand = Strand {
            genes: vec![
                Gene { op: OpCode::Push, args: vec![Nucleotide::String("Sum".to_string())] },
                Gene { op: OpCode::Etymology, args: vec![] },
            ]
        };
        vm.dna.helix.strands.push(etym_strand);

        vm.step();
        vm.step();

        if let Some(Value::Junction(_, list)) = vm.stack.pop() {
            assert!(!list.is_empty());
            println!("Etymology list: {:?}", list);
            // Case-insensitive check just in case
            if let Value::Str(s) = &list[0] {
                assert_eq!(s.to_lowercase(), "add");
            } else {
                panic!("Expected string in etymology");
            }
        } else {
            println!("VM Output: {:?}", vm.output);
            println!("Stack (popped): {:?}", vm.stack);
            panic!("Expected Junction");
        }
    }
}
