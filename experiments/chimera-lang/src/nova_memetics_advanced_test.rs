#[cfg(test)]
#[cfg(feature = "nova")]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::{ChimeraVM, Value};
    use crate::vm::memetics::{ViralState, Virus};

    fn make_vm() -> ChimeraVM {
        let genes = vec![Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)],
        }];
        let dna = Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        };
        ChimeraVM::new(dna)
    }

    #[test]
    fn test_infect_with_quorum_args() {
        let mut vm = make_vm();
        vm.context_loc = (5, 5);

        // Stack setup for Infect with Optional Args:
        // [Grammar, Action, Threshold, Payload, Rate, Pattern, Name] (Bottom -> Top)
        // Code pops: Name, Pattern, Rate, Payload.
        // Then checks stack top for Action, Threshold.
        // Stack must be: [Action, Threshold] after first 4 pops.
        // So original stack: [Action, Threshold, Payload, Rate, Pattern, Name]

        // Push Action (Int 99)
        vm.stack.push(Value::Int(99));
        // Push Threshold (Int 2)
        vm.stack.push(Value::Int(2));

        // Push Payload (-1)
        vm.stack.push(Value::Int(-1));
        // Push Rate (100)
        vm.stack.push(Value::Int(100));
        // Push Pattern ("PAT")
        vm.stack.push(Value::Str("PAT".to_string()));
        // Push Name ("QuorumVirus")
        vm.stack.push(Value::Str("QuorumVirus".to_string()));

        let op = OpCode::Infect;
        crate::vm::memetics::exec_memetics_op(&mut vm, op, &[]);

        assert_eq!(vm.virus_library.len(), 1);
        let virus = &vm.virus_library[0];
        assert_eq!(virus.name, "QuorumVirus");
        assert_eq!(virus.quorum_threshold, 2);
        assert_eq!(virus.quorum_action, Some(99));
    }

    #[test]
    fn test_outbreak_quorum_action() {
        let mut vm = make_vm();
        // Ensure we have enough strands so Action 0 is valid.
        // make_vm creates 1 strand (index 0).

        // 1. Create Virus with Threshold 2, Action 0.
        let virus = Virus {
            name: "QVirus".to_string(),
            color: (0, 0, 0),
            pattern: "T".to_string(),
            mutation_rate: 0,
            payload: None,
            grammar: None,
            quorum_threshold: 2,
            quorum_action: Some(0),
        };
        vm.virus_library.push(virus);

        // 2. Infect Center (5,5)
        vm.viral_grid[5][5] = Some(ViralState {
            infection_level: 100,
            virus_id: 0,
        });

        // 3. Infect Neighbors (5,6) and (6,5)
        // Neighbors of (5,5): (4,5), (6,5), (5,4), (5,6) etc.
        vm.viral_grid[5][6] = Some(ViralState {
            infection_level: 100,
            virus_id: 0,
        });
        vm.viral_grid[6][5] = Some(ViralState {
            infection_level: 100,
            virus_id: 0,
        });

        // Count neighbors of (5,5):
        // (5,6) is neighbor.
        // (6,5) is neighbor.
        // Total 2 neighbors. Threshold 2. Should trigger.

        // 4. Run Outbreak
        crate::vm::memetics::exec_memetics_op(&mut vm, OpCode::Outbreak, &[]);

        // 5. Verify Action
        // "QUORUM: Virus 0 triggered action 0 at 5,5" should be in output.
        // Also check if organelle spawned.
        assert!(vm.output.iter().any(|s| s.contains("QUORUM: Virus 0 triggered action 0")));

        // Check organelle
        // Should have spawned a Worker at (5,5).
        let org = vm.organelles.iter().find(|o| o.context_loc == (5,5) && o.traits.contains(&"Viral".to_string()));
        assert!(org.is_some(), "Viral organelle should spawn");
    }
}
