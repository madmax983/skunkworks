#[cfg(feature = "oracle")]
#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn test_seek_op() {
        // Strand 0: [ seek("Photosynthesize") ]
        let strand0 = Strand {
            genes: vec![
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::String("photosynthesize".to_string())],
                },
                Gene {
                    op: OpCode::Seek,
                    args: vec![],
                },
            ],
        };

        // Strand 1: Empty
        let strand1 = Strand { genes: vec![] };

        // Strand 2: [ photosynthesize() ]
        let strand2 = Strand {
            genes: vec![Gene {
                op: OpCode::Photosynthesize,
                args: vec![],
            }],
        };

        let dna = Dna { evolution_config: None,
            helix: Helix {
                strands: vec![strand0, strand1, strand2],
            },
        };

        let mut vm = ChimeraVM::new(dna);

        // Step 1: Push "Photosynthesize"
        vm.step();
        assert_eq!(vm.stack.len(), 1);

        // Step 2: Seek
        vm.step();

        // Should have jumped to Strand 2
        assert_eq!(vm.ip, (2, 0));
    }

    #[test]
    fn test_seek_fail() {
        // Strand 0: [ seek("UnknownOp") ]
        let strand0 = Strand {
            genes: vec![
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::String("UnknownOp".to_string())],
                },
                Gene {
                    op: OpCode::Seek,
                    args: vec![],
                },
            ],
        };

        let dna = Dna { evolution_config: None,
            helix: Helix {
                strands: vec![strand0],
            },
        };

        let mut vm = ChimeraVM::new(dna);

        // Step 1: Push
        vm.step();

        // Step 2: Seek (should fail)
        vm.step();

        // Should stay at (0, 2) (next instruction after seek)
        assert_eq!(vm.ip, (0, 2));
        assert!(vm.output.last().unwrap().contains("No matching strand"));
    }
}
