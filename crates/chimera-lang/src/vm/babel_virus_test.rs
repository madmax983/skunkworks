#[cfg(test)]
#[cfg(feature = "nova")]
mod tests {
    use crate::ast::{Dna, Gene, Helix, JunctionType, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::{ChimeraVM, Value};

    fn make_vm() -> ChimeraVM {
        let genes = vec![Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)],
        }];
        let dna = Dna { evolution_config: None,
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        };
        ChimeraVM::new(dna)
    }

    #[test]
    fn test_infect_with_grammar() {
        let mut vm = make_vm();

        // Stack: Grammar, Quorum(0,0), Payload(-1), Rate(100), Pattern("TARGET"), Name("GrammarVirus")
        // Grammar: Match("REWRITTEN")
        let grammar = Value::Junction(
            JunctionType::Any,
            vec![
                Value::Str("Match".to_string()),
                Value::Str("REWRITTEN".to_string()),
            ],
        );

        vm.stack.push(grammar);
        vm.stack.push(Value::Int(0)); // Quorum Threshold
        vm.stack.push(Value::Int(-1)); // Quorum Action
        vm.stack.push(Value::Int(-1)); // Payload
        vm.stack.push(Value::Int(100)); // Rate
        vm.stack.push(Value::Str("TARGET".to_string())); // Pattern
        vm.stack.push(Value::Str("GrammarVirus".to_string())); // Name
        vm.stack.push(Value::Int(0)); // Mode (Overwrite)

        vm.context_loc = (5, 5);
        crate::vm::memetics::exec_memetics_op(&mut vm, OpCode::Infect, &[]);

        assert_eq!(vm.virus_library.len(), 1);
        let virus = &vm.virus_library[0];
        assert!(virus.grammar.is_some());
        assert_eq!(virus.name, "GrammarVirus");
    }

    #[test]
    fn test_outbreak_rewrite() {
        let mut vm = make_vm();

        let grammar = Value::Junction(
            JunctionType::Any,
            vec![
                Value::Str("Match".to_string()),
                Value::Str("REWRITTEN".to_string()),
            ],
        );

        let virus = crate::vm::memetics::Virus {
            name: "Writer".to_string(),
            color: (0, 0, 0),
            pattern: "TARGET".to_string(),
            mutation_rate: 100,
            payload: None,
            grammar: Some(grammar),
            quorum_threshold: 0,
            quorum_action: None,
            mode: crate::vm::memetics::VirusMode::Overwrite,
        };
        vm.virus_library.push(virus);

        // Infect (5,5) at 100%
        vm.viral_grid[5][5] = Some(crate::vm::memetics::ViralState {
            infection_level: 100,
            virus_id: 0,
        });

        // Set content to "TARGET"
        vm.grid[5][5] = Value::Str("TARGET".to_string());

        // Outbreak
        crate::vm::memetics::exec_memetics_op(&mut vm, OpCode::Outbreak, &[]);

        // Check rewrite
        if let Value::Str(s) = &vm.grid[5][5] {
            assert_eq!(s, "REWRITTEN");
        } else {
            panic!("Grid cell not a string, got {:?}", vm.grid[5][5]);
        }
    }

    #[test]
    fn test_quorum_sensing() {
        let mut vm = make_vm();

        // Action: Strand 0 (Push 999)
        vm.dna.helix.strands[0].genes = vec![Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(999)],
        }];

        let virus = crate::vm::memetics::Virus {
            name: "Mob".to_string(),
            color: (0, 0, 0),
            pattern: "TARGET".to_string(),
            mutation_rate: 0,
            payload: None,
            grammar: None,
            quorum_threshold: 2,    // Need 2 neighbors
            quorum_action: Some(0), // Exec Strand 0
            mode: crate::vm::memetics::VirusMode::Overwrite,
        };
        vm.virus_library.push(virus);

        // Infect (5,5) and 2 neighbors
        vm.viral_grid[5][5] = Some(crate::vm::memetics::ViralState {
            infection_level: 100,
            virus_id: 0,
        });
        vm.viral_grid[5][6] = Some(crate::vm::memetics::ViralState {
            infection_level: 100,
            virus_id: 0,
        });
        vm.viral_grid[6][5] = Some(crate::vm::memetics::ViralState {
            infection_level: 100,
            virus_id: 0,
        });

        vm.context_loc = (5, 5);

        // Outbreak
        crate::vm::memetics::exec_memetics_op(&mut vm, OpCode::Outbreak, &[]);

        // Check if Organelle spawned at (5,5) (or others)
        // Since outbreak iterates whole grid, (5,5) sees 2 neighbors. (5,6) sees 2 (5,5 and 6,5). (6,5) sees 2.
        // So all 3 should spawn an agent if empty.
        let orgs = vm
            .organelles
            .iter()
            .filter(|o| o.name.contains("Virus 0 Agent"))
            .count();
        assert!(
            orgs >= 1,
            "Should spawn viral agent on quorum, got {}",
            orgs
        );

        let org = vm
            .organelles
            .iter()
            .find(|o| o.name.contains("Virus 0 Agent"))
            .unwrap();
        assert_eq!(org.ip.0, 0); // Check strand index
    }
}
