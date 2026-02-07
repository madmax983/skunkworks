#[cfg(all(test, feature = "nova"))]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::ChimeraVM;

    fn make_dna(genes: Vec<Gene>) -> Dna {
        Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        }
    }

    #[test]
    fn test_mitosis_tracking() {
        // [ push(0) mitosis() ]
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            },
            Gene {
                op: OpCode::Mitosis,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));

        // Register strand 0 manually for test accuracy
        vm.cladistics.register_strand(0, None, 0, "Genesis".to_string());

        vm.step(); // push
        vm.step(); // mitosis

        assert_eq!(vm.dna.helix.strands.len(), 2);

        // Check cladistics
        let parent_node_id = *vm.cladistics.active_map.get(&0).expect("Parent not active");
        let child_node_id = *vm.cladistics.active_map.get(&1).expect("Child not active");

        let parent = vm.cladistics.nodes.get(&parent_node_id).unwrap();
        let child = vm.cladistics.nodes.get(&child_node_id).unwrap();

        assert_eq!(child.parent_id, Some(parent_node_id));
        assert!(parent.children.contains(&child_node_id));
        assert_eq!(child.event, "Mitosis");
    }

    #[test]
    fn test_reincarnate_tracking() {
        // [ push(0) reincarnate() ]
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            },
            Gene {
                op: OpCode::Reincarnate,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));
        vm.cladistics.register_strand(0, None, 0, "Genesis".to_string());

        let old_node_id = *vm.cladistics.active_map.get(&0).unwrap();

        vm.step(); // push
        vm.step(); // reincarnate

        // Strand 0 genes cleared. New strand 1 added.
        assert_eq!(vm.dna.helix.strands[0].genes.len(), 0);
        assert_eq!(vm.dna.helix.strands.len(), 2);

        let new_node_id = *vm.cladistics.active_map.get(&1).unwrap();

        let old_node = vm.cladistics.nodes.get(&old_node_id).unwrap();
        let new_node = vm.cladistics.nodes.get(&new_node_id).unwrap();

        assert!(old_node.death_tick.is_some());
        assert_eq!(new_node.parent_id, Some(old_node_id));
        assert_eq!(new_node.event, "Reincarnate");
    }
}
