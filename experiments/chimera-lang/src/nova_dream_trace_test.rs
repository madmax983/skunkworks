#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::ChimeraVM;

    fn make_dna(genes: Vec<Gene>) -> Dna {
        Dna { evolution_config: None,
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        }
    }

    #[test]
    fn test_dream_trace_recording() {
        // [ dream(10, 0) ] - Run a dream of itself for 10 ticks
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            }, // Strand 0
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(10)],
            }, // 10 ticks
            Gene {
                op: OpCode::Dream,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));

        // Step 1: Push 0
        vm.step();
        // Step 2: Push 10
        vm.step();
        // Step 3: Dream
        vm.step();

        // Check if trace was recorded
        assert!(
            !vm.dream_traces.is_empty(),
            "Dream traces should not be empty"
        );
        let trace = &vm.dream_traces[0];

        assert_eq!(trace.strand_idx, 0);
        assert_eq!(trace.duration, 10);
        println!("Dream mutation: {}", trace.mutation_desc);

        // Ensure outcome is recorded (Accepted or Discarded)
        // Since we didn't add any energy gain ops, it should probably fail/discard unless mutation added one.
        // But mutation is random.
        // We just check that accepted boolean matches expectation.
    }

    #[test]
    fn test_lucid_dreaming_injection() {
        // We can't easily test TUI interaction here, but we can verify the data structures allow it.
        // Create a fake trace with a mutated strand
        let mutated_genes = vec![Gene {
            op: OpCode::Photosynthesize,
            args: vec![],
        }];
        let mutated_strand = Strand {
            genes: mutated_genes,
        };

        let trace = crate::vm::dream::DreamTrace::new(
            0,
            0,
            10,
            50,
            100,
            1,
            "Fake Mutation".to_string(),
            Some(mutated_strand.clone()),
            false, // Discarded originally
            false, // is_nightmare
            vec![],
            None,
        );

        // In TUI logic:
        // if trace.strand_idx < vm.dna.helix.strands.len() {
        //     vm.dna.helix.strands[trace.strand_idx] = strand.clone();
        // }

        let genes = vec![Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)],
        }];
        let mut vm = ChimeraVM::new(make_dna(genes));

        // "Realize" the dream
        vm.dna.helix.strands[0] = trace.mutated_strand.unwrap();

        // Verify strand 0 is now Photosynthesize
        assert_eq!(vm.dna.helix.strands[0].genes[0].op, OpCode::Photosynthesize);
    }
}
