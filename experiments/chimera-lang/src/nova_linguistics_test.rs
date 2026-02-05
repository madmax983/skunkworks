#[cfg(all(test, feature = "nova"))]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::{ChimeraVM, Value};

    fn make_dna(genes: Vec<Gene>) -> Dna {
        Dna {
            helix: Helix {
                strands: vec![std::rc::Rc::new(Strand { genes })],
            },
        }
    }

    #[test]
    fn test_compile() {
        // [ push("[ push(100) ]") compile() ]
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("[ push(100) ]".to_string())],
            },
            Gene {
                op: OpCode::Compile,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));

        // Initial setup for Nova
        // vm.telomeres is initialized to 50 for all strands in new()
        // but we only have 1 strand initially.

        vm.step(); // push string
        vm.step(); // compile

        // Check if stack has new strand index (should be 1)
        assert_eq!(vm.stack.len(), 1);
        assert_eq!(vm.stack[0], Value::Int(1));

        // Check if strand exists
        assert_eq!(vm.dna.helix.strands.len(), 2);
        let new_strand = &vm.dna.helix.strands[1];
        assert_eq!(new_strand.genes.len(), 1);
        assert_eq!(new_strand.genes[0].op, OpCode::Push);
        assert_eq!(new_strand.genes[0].args[0], Nucleotide::Number(100));
    }

    #[test]
    fn test_decompile() {
        // Strand 0: [ push(42) ]
        // Strand 1: [ push(0) decompile() ]
        // We run strand 1.
        let strand0 = Strand {
            genes: vec![Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(42)],
            }],
        };
        let strand1 = Strand {
            genes: vec![
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(0)],
                },
                Gene {
                    op: OpCode::Decompile,
                    args: vec![],
                },
            ],
        };

        let dna = Dna {
            helix: Helix {
                strands: vec![strand0.into(), strand1.into()],
            },
        };
        let mut vm = ChimeraVM::new(dna);

        // Setup initial ip
        vm.ip = (1, 0); // Start at strand 1

        vm.step(); // push(0)
        vm.step(); // decompile

        assert_eq!(vm.stack.len(), 1);
        if let Value::Str(s) = &vm.stack[0] {
            // Should match "[ push(42) ] " or similar. spacing might vary.
            // My implementation uses "[ push(42) ] " (trailing space after gene).
            println!("Decompiled: '{}'", s);
            assert!(s.contains("push(42)"));
            assert!(s.trim().starts_with("["));
            assert!(s.trim().ends_with("]"));
        } else {
            panic!("Expected String on stack");
        }
    }

    #[test]
    fn test_roundtrip() {
        // [ push(0) decompile() compile() ]
        // Strand 0: [ add() ]
        let strand0 = Strand {
            genes: vec![Gene {
                op: OpCode::Add,
                args: vec![],
            }],
        };
        let strand1 = Strand {
            genes: vec![
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(0)],
                },
                Gene {
                    op: OpCode::Decompile,
                    args: vec![],
                },
                Gene {
                    op: OpCode::Compile,
                    args: vec![],
                },
            ],
        };

        let dna = Dna {
            helix: Helix {
                strands: vec![strand0.into(), strand1.into()],
            },
        };
        let mut vm = ChimeraVM::new(dna);

        vm.ip = (1, 0);
        vm.step(); // push(0)
        vm.step(); // decompile
        vm.step(); // compile

        assert_eq!(vm.stack.len(), 1);
        assert_eq!(vm.stack[0], Value::Int(2)); // New strand is index 2

        // Verify strand 2 is identical to strand 0
        let s0 = &vm.dna.helix.strands[0];
        let s2 = &vm.dna.helix.strands[2];
        assert_eq!(s0, s2);
    }
}
