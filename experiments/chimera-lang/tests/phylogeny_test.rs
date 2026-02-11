#[cfg(all(test, feature = "nova"))]
mod tests {
    use chimera_lang::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use chimera_lang::opcode::OpCode;
    use chimera_lang::vm::{ChimeraVM, Value};

    fn make_dna(genes: Vec<Gene>) -> Dna {
        Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        }
    }

    #[test]
    fn test_speciation() {
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("Cambrian".to_string())],
            },
            Gene {
                op: OpCode::Speciate,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));

        // Manual registration of root
        vm.cladistics.register_strand(0, vec![], 0, "Genesis".to_string());
        let root_id = *vm.cladistics.active_map.get(&0).unwrap();

        vm.step(); // Push "Cambrian"
        vm.step(); // Speciate

        let new_id = *vm.cladistics.active_map.get(&0).unwrap();
        assert_ne!(root_id, new_id);

        let new_node = vm.cladistics.nodes.get(&new_id).unwrap();
        assert_eq!(new_node.event, "Speciate: Cambrian");
        assert_eq!(new_node.parents, vec![root_id]);
    }

    #[test]
    fn test_dag_splice() {
        // Strand 0: [ Push(0) Push(1) Push(0) Splice ] -> Splice 0 and 1 (Method 0)
        // We need a second strand 1.

        let strand0_genes = vec![
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] }, // Method 0
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] }, // Strand 1
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] }, // Strand 0
            Gene { op: OpCode::Splice, args: vec![] },
        ];

        let strand1_genes = vec![
            Gene { op: OpCode::Nop, args: vec![] },
        ];

        let dna = Dna {
            helix: Helix {
                strands: vec![
                    Strand { genes: strand0_genes },
                    Strand { genes: strand1_genes },
                ]
            }
        };

        let mut vm = ChimeraVM::new(dna);

        // Register roots
        vm.cladistics.register_strand(0, vec![], 0, "Root0".to_string());
        vm.cladistics.register_strand(1, vec![], 0, "Root1".to_string());

        let id0 = *vm.cladistics.active_map.get(&0).unwrap();
        let id1 = *vm.cladistics.active_map.get(&1).unwrap();

        // Run
        vm.step(); // Push 0
        vm.step(); // Push 1
        vm.step(); // Push 0
        vm.step(); // Splice

        // Check new strand 2
        assert_eq!(vm.dna.helix.strands.len(), 3);
        let id2 = *vm.cladistics.active_map.get(&2).unwrap();
        let node2 = vm.cladistics.nodes.get(&id2).unwrap();

        assert!(node2.parents.contains(&id0));
        assert!(node2.parents.contains(&id1));
        assert_eq!(node2.parents.len(), 2);
    }
}
