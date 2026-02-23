#[cfg(test)]
#[cfg(all(feature = "nova", feature = "oracle"))]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::{ChimeraVM, Value};

    fn make_dna(genes: Vec<Gene>) -> Dna {
        Dna {
            evolution_config: None,
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        }
    }

    #[test]
    fn test_omen_photosynthesis() {
        let genes = vec![
            // Assert "sun"
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("sun".to_string())],
            },
            Gene {
                op: OpCode::Assert,
                args: vec![],
            },
            // Register Omen: Augury("sun", "photosynthesize")
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("sun".to_string())],
            }, // condition
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("photosynthesize".to_string())],
            }, // effect
            Gene {
                op: OpCode::Augury,
                args: vec![],
            },
            // Divinate
            Gene {
                op: OpCode::Divinate,
                args: vec![],
            },
        ];

        let mut vm = ChimeraVM::new(make_dna(genes));

        // Run all steps
        // 6 instructions
        for _ in 0..6 {
            vm.step();
        }

        // Energy analysis:
        // Start: 50
        // Steps 1-6 cost 1 each: -6 -> 44
        // Omen triggers "photosynthesize" -> +5 -> 49

        assert!(
            vm.output
                .iter()
                .any(|s| s.contains("DIVINATE: Omen fulfilled!")),
            "Output: {:?}",
            vm.output
        );

        // Verify stack has count 1
        let count = vm.stack.pop();
        assert_eq!(count, Some(Value::Int(1)));

        assert_eq!(vm.energy, 49);
    }
}
