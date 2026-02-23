#[cfg(feature = "nova")]
#[cfg(test)]
mod tests {
    use chimera_lang::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use chimera_lang::opcode::OpCode;
    use chimera_lang::vm::{ChimeraVM, Value};

    fn make_vm(genes: Vec<Gene>) -> ChimeraVM {
        let dna = Dna {
            evolution_config: None,
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        };
        ChimeraVM::new(dna)
    }

    #[test]
    fn test_digest_negative_offset() {
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(-100)], // Negative offset
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(10)],
            },
            Gene {
                op: OpCode::Digest,
                args: vec![],
            },
        ];

        let mut vm = make_vm(genes);
        vm.step();
        vm.step();
        vm.step();
        // Should not panic
        println!("Output: {:?}", vm.output);
        assert!(
            vm.output.iter().any(|s| s.contains("Error")),
            "Should return error for seek failure"
        );
    }

    #[test]
    fn test_scavenge_path_traversal() {
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("../Cargo.toml".to_string())],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(100)],
            },
            Gene {
                op: OpCode::Scavenge,
                args: vec![],
            },
        ];

        let mut vm = make_vm(genes);
        vm.step();
        vm.step();
        vm.step();

        // Should catch security alert or error
        let blocked = vm
            .output
            .iter()
            .any(|s| s.contains("SECURITY ALERT") || s.contains("Error"));
        assert!(blocked, "Path traversal should be blocked");
    }

    #[test]
    fn test_deep_zip_bomb() {
        // Strand 0: Init
        let s0 = Strand {
            genes: vec![
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(1)],
                },
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(1)],
                },
                Gene {
                    op: OpCode::Jump,
                    args: vec![Nucleotide::Number(1)],
                },
            ],
        };
        // Strand 1: Loop
        let s1 = Strand {
            genes: vec![
                Gene {
                    op: OpCode::Zip,
                    args: vec![],
                },
                Gene {
                    op: OpCode::Dup,
                    args: vec![],
                },
                Gene {
                    op: OpCode::Jump,
                    args: vec![Nucleotide::Number(1)],
                },
            ],
        };

        let dna = Dna {
            evolution_config: None,
            helix: Helix {
                strands: vec![s0, s1],
            },
        };
        let mut vm = ChimeraVM::new(dna);

        // Infinite energy
        vm.energy = 100000;

        // Run
        for _ in 0..2000 {
            if vm.halted {
                break;
            }
            vm.step();
        }

        // Check if we hit recursion limit or complexity limit
        let limit_hit = vm
            .output
            .iter()
            .any(|s| s.contains("depth limit exceeded") || s.contains("too complex"));
        assert!(limit_hit, "Should eventually hit depth/complexity limit");
    }
}
