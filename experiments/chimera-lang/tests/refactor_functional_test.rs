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

    fn make_vm_with_strands(strands: Vec<Strand>) -> ChimeraVM {
        let dna = Dna {
            evolution_config: None,
            helix: Helix { strands },
        };
        ChimeraVM::new(dna)
    }

    #[test]
    fn test_chain_success() {
        // Strand 0: [ push(1) push(0) chain() ]
        // Strand 1: [ push(100) ]
        // Strand 2: [ push(200) ]
        // Result should be new strand 3: [ push(200) push(100) ] (Actually genes are concatenated)
        // Wait, exec_chain concatenates f (first arg) then g (second arg).
        // Stack: [ g_idx, f_idx ] -> pop f, pop g.
        // Wait, exec_chain pops f then g.
        // Stack pushes: push(g), push(f).
        // Stack: [ g, f ]. pop() -> f. pop() -> g.
        // New genes = genes_f + genes_g.

        let strand0 = Strand {
            genes: vec![
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(2)],
                }, // g
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(1)],
                }, // f
                Gene {
                    op: OpCode::Chain,
                    args: vec![],
                },
            ],
        };
        let strand1 = Strand {
            genes: vec![Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(100)],
            }],
        };
        let strand2 = Strand {
            genes: vec![Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(200)],
            }],
        };

        let mut vm = make_vm_with_strands(vec![strand0, strand1, strand2]);

        // Execute Chain
        vm.step(); // push 2
        vm.step(); // push 1
        vm.step(); // chain

        // Check output
        assert!(vm
            .output
            .last()
            .unwrap()
            .contains("CHAIN: Created strand 3"));

        // Check new strand
        assert_eq!(vm.dna.helix.strands.len(), 4);
        let new_strand = &vm.dna.helix.strands[3];
        assert_eq!(new_strand.genes.len(), 2);
        assert_eq!(new_strand.genes[0].args[0], Nucleotide::Number(100)); // from strand 1 (f)
        assert_eq!(new_strand.genes[1].args[0], Nucleotide::Number(200)); // from strand 2 (g)
    }

    #[test]
    fn test_chain_error_stack_underflow() {
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            },
            Gene {
                op: OpCode::Chain,
                args: vec![],
            },
        ];
        let mut vm = make_vm(genes);

        vm.step();
        vm.step();

        assert!(vm
            .output
            .last()
            .unwrap()
            .contains("CHAIN ERROR: Stack underflow"));
        assert_eq!(vm.stack.len(), 1, "Stack should contain the pushed item");
    }

    #[test]
    fn test_chain_error_type_mismatch() {
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("bad".to_string())],
            },
            Gene {
                op: OpCode::Chain,
                args: vec![],
            },
        ];
        let mut vm = make_vm(genes);

        vm.step();
        vm.step();
        vm.step();

        assert!(vm
            .output
            .last()
            .unwrap()
            .contains("CHAIN ERROR: Type mismatch"));
        // Original behavior: pops both args. Current refactor behavior: pops only top arg if mismatch.
        // We want to ASSERT the original behavior (consuming both).
        assert_eq!(
            vm.stack.len(),
            0,
            "Stack should be empty after type mismatch"
        );
    }

    #[test]
    fn test_chain_error_invalid_index() {
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            }, // Valid index
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(999)],
            }, // Invalid index
            Gene {
                op: OpCode::Chain,
                args: vec![],
            },
        ];
        // Note: we need at least 2 strands for index 1 to be valid
        let strand1 = Strand { genes: vec![] };
        let mut vm = make_vm_with_strands(vec![Strand { genes }, strand1]);

        vm.step();
        vm.step();
        vm.step();

        assert!(vm
            .output
            .last()
            .unwrap()
            .contains("CHAIN ERROR: Invalid strand index"));
        assert_eq!(
            vm.stack.len(),
            0,
            "Stack should be empty after invalid index error"
        );
    }

    #[test]
    fn test_curry_success() {
        // Strand 0: [ push(100) push(1) curry() ]
        // Strand 1: [ add() ]
        // Result strand 2: [ push(100) add() ]

        let strand0 = Strand {
            genes: vec![
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(100)],
                }, // arg
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(1)],
                }, // strand
                Gene {
                    op: OpCode::Curry,
                    args: vec![],
                },
            ],
        };
        let strand1 = Strand {
            genes: vec![Gene {
                op: OpCode::Add,
                args: vec![],
            }],
        };

        let mut vm = make_vm_with_strands(vec![strand0, strand1]);

        vm.step(); // push 100
        vm.step(); // push 1
        vm.step(); // curry

        assert!(vm
            .output
            .last()
            .unwrap()
            .contains("CURRY: Created strand 2"));

        let new_strand = &vm.dna.helix.strands[2];
        assert_eq!(new_strand.genes.len(), 2);
        assert_eq!(new_strand.genes[0].op, OpCode::Push);
        assert_eq!(new_strand.genes[0].args[0], Nucleotide::Number(100));
        assert_eq!(new_strand.genes[1].op, OpCode::Add);
    }

    #[test]
    fn test_curry_error_stack_underflow() {
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            },
            Gene {
                op: OpCode::Curry,
                args: vec![],
            },
        ];
        let mut vm = make_vm(genes);

        vm.step();
        vm.step();

        assert!(vm
            .output
            .last()
            .unwrap()
            .contains("CURRY ERROR: Stack underflow"));
        assert_eq!(vm.stack.len(), 1);
    }

    #[test]
    fn test_quote_success() {
        // [ quote() add() push(1) ]
        // quote() should consume next op (add) and push "add" string to stack
        let genes = vec![
            Gene {
                op: OpCode::Quote,
                args: vec![],
            },
            Gene {
                op: OpCode::Add,
                args: vec![],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            },
        ];
        let mut vm = make_vm(genes);

        vm.step(); // execute quote

        assert!(vm.output.last().unwrap().contains("QUOTE: add"));
        assert_eq!(vm.stack.len(), 1);
        match &vm.stack[0] {
            Value::Str(s) => assert_eq!(s, "add"),
            _ => panic!("Expected string"),
        }

        // IP should have skipped Add
        // Initial IP (0,0). Quote executed.
        // exec_quote increments IP.1 by 1 -> (0, 1) (pointing to Add)
        // Main loop increments IP.1 by 1 -> (0, 2) (pointing to Push)
        // So next step should execute Push(1)

        vm.step();
        assert_eq!(vm.stack.len(), 2);
        match &vm.stack[1] {
            Value::Int(i) => assert_eq!(*i, 1),
            _ => panic!("Expected int"),
        }
    }

    #[test]
    fn test_quote_error_end_of_strand() {
        let genes = vec![Gene {
            op: OpCode::Quote,
            args: vec![],
        }];
        let mut vm = make_vm(genes);

        vm.step();

        assert!(vm
            .output
            .last()
            .unwrap()
            .contains("QUOTE ERROR: Unexpected end of strand"));
    }
}
