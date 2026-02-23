#[cfg(all(test, feature = "nova"))]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::{ChimeraVM, Value};

    #[test]
    fn test_define_and_call_macro() {
        // Strand 0 (Main):
        // [ push(1) push("square") define() push(5) square() ]
        // Strand 1 (Square):
        // [ dup() mul() ret() ]

        let strand_main = Strand {
            genes: vec![
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(1)],
                }, // Strand Index 1
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::String("square".to_string())],
                }, // Name
                Gene {
                    op: OpCode::Define,
                    args: vec![],
                }, // Define
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(5)],
                }, // Arg
                Gene {
                    op: OpCode::Unknown("square".to_string()),
                    args: vec![],
                }, // Call macro
                Gene {
                    op: OpCode::Jump,
                    args: vec![Nucleotide::Number(99)], // Jump to invalid to halt
                },
            ],
        };

        let strand_square = Strand {
            genes: vec![
                Gene {
                    op: OpCode::Dup,
                    args: vec![],
                },
                Gene {
                    op: OpCode::Mul,
                    args: vec![],
                },
                Gene {
                    op: OpCode::Ret,
                    args: vec![],
                },
            ],
        };

        let dna = Dna { evolution_config: None,
            helix: Helix {
                strands: vec![strand_main, strand_square],
            },
        };
        let mut vm = ChimeraVM::new(dna);

        // Run until halted or finished
        // define takes ~10 energy
        // call takes some steps
        for _ in 0..20 {
            vm.step();
        }

        // Stack should have result: 25
        assert_eq!(vm.stack.last(), Some(&Value::Int(25)));

        // Dictionary check
        assert_eq!(vm.dictionary.get("square"), Some(&1));
    }

    #[test]
    fn test_undefine() {
        // [ push(1) push("foo") define() push("foo") undefine() foo() ]
        // Should error on foo()
        let strand = Strand {
            genes: vec![
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(0)],
                },
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::String("foo".to_string())],
                },
                Gene {
                    op: OpCode::Define,
                    args: vec![],
                },
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::String("foo".to_string())],
                },
                Gene {
                    op: OpCode::Undefine,
                    args: vec![],
                },
                Gene {
                    op: OpCode::Unknown("foo".to_string()),
                    args: vec![],
                },
            ],
        };
        let dna = Dna { evolution_config: None,
            helix: Helix {
                strands: vec![strand],
            },
        };
        let mut vm = ChimeraVM::new(dna);

        for _ in 0..10 {
            vm.step();
        }

        assert!(vm.output.iter().any(|s| s.contains("Unknown enzyme: foo")));
    }
}
