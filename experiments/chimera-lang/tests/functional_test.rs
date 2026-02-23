#[cfg(feature = "nova")]
#[cfg(test)]
mod tests {
    use chimera_lang::prelude::*;

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
    fn test_quote() {
        let genes = vec![
            Gene {
                op: OpCode::Quote,
                args: vec![],
            },
            Gene {
                op: OpCode::Add,
                args: vec![],
            }, // Should be quoted, not executed
        ];
        let mut vm = make_vm(genes);
        vm.step(); // quote
                   // vm step logic executes Quote. Quote consumes next gene (Add) and pushes string "add".
                   // IP advances past Add.

        assert_eq!(vm.stack.len(), 1);
        if let Some(Value::Str(s)) = vm.stack.last() {
            assert_eq!(s, "add");
        } else {
            panic!("Expected string on stack");
        }
    }

    #[test]
    fn test_chain() {
        // Strand 0: [ push(1) ] [ push(2) add ] chain call
        // But we need strand indices.
        // Let's create helper strands.
        // Strand 1: [ push(10) ]
        // Strand 2: [ push(20) add ]
        // Strand 0: [ push(2) push(1) chain call ]

        let s1 = Strand {
            genes: vec![Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(10)],
            }],
        };
        let s2 = Strand {
            genes: vec![
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(20)],
                },
                Gene {
                    op: OpCode::Add,
                    args: vec![],
                },
            ],
        };

        let s0 = Strand {
            genes: vec![
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(2)],
                }, // Index of s2
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(1)],
                }, // Index of s1
                Gene {
                    op: OpCode::Chain,
                    args: vec![],
                },
                Gene {
                    op: OpCode::Call,
                    args: vec![],
                }, // Call the new strand
            ],
        };

        let dna = Dna {
            evolution_config: None,
            helix: Helix {
                strands: vec![s0, s1, s2],
            },
        };
        let mut vm = ChimeraVM::new(dna);

        // Step until done
        for _ in 0..10 {
            vm.step();
            if vm.halted {
                break;
            }
        }

        // Result: 10 + 20 = 30
        assert_eq!(vm.stack.last(), Some(&Value::Int(30)));
    }

    #[test]
    fn test_curry() {
        // Strand 1: [ add ]
        // Strand 0: [ push(5) push(10) push(1) curry exec ]
        // Stack state:
        // 1. Push 5 (Arg for add)
        // 2. Push 10 (Value to curry)
        // 3. Push 1 (Strand to curry)
        // 4. Curry -> Pops 1 (Strand), Pops 10 (Value) -> Creates [Push(10), Add]
        // 5. Exec -> Pops new strand index, executes it
        // Execution: Push(10), Add (pops 10, pops 5) -> 15

        let s1 = Strand {
            genes: vec![Gene {
                op: OpCode::Add,
                args: vec![],
            }],
        };
        let s0 = Strand {
            genes: vec![
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(5)],
                },
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(10)],
                }, // Value to curry
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(1)],
                }, // Index of s1
                Gene {
                    op: OpCode::Curry,
                    args: vec![],
                },
                Gene {
                    op: OpCode::Exec,
                    args: vec![],
                }, // Call the new strand (Exec pops index)
            ],
        };

        let dna = Dna {
            evolution_config: None,
            helix: Helix {
                strands: vec![s0, s1],
            },
        };
        let mut vm = ChimeraVM::new(dna);

        for _ in 0..10 {
            vm.step();
        }

        assert_eq!(vm.stack.last(), Some(&Value::Int(15)));
    }

    #[test]
    fn test_crossover() {
        // Strand 1: [ push(1) push(1) push(1) ]
        // Strand 2: [ push(2) push(2) push(2) ]
        // Strand 0: [ push(2) push(1) crossover ]

        let s1 = Strand {
            genes: vec![
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(1)]
                };
                3
            ],
        };
        let s2 = Strand {
            genes: vec![
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(2)]
                };
                3
            ],
        };

        let s0 = Strand {
            genes: vec![
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(2)],
                },
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(1)],
                },
                Gene {
                    op: OpCode::Crossover,
                    args: vec![],
                },
            ],
        };

        let dna = Dna {
            evolution_config: None,
            helix: Helix {
                strands: vec![s0, s1, s2],
            },
        };
        let mut vm = ChimeraVM::new(dna);

        vm.step();
        vm.step();
        vm.step(); // Crossover

        // Stack should have 2 new indices
        assert_eq!(vm.stack.len(), 2);

        // Verify new strands exist
        let len = vm.dna.helix.strands.len();
        assert_eq!(len, 5); // 0,1,2 + 3,4

        // Check content (probabilistic, but length should be preserved for symmetric crossover)
        let s3 = &vm.dna.helix.strands[3];
        let s4 = &vm.dna.helix.strands[4];

        assert_eq!(s3.genes.len(), 3);
        assert_eq!(s4.genes.len(), 3);
    }
}
