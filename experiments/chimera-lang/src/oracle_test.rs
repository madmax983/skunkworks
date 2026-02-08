#[cfg(all(test, feature = "oracle"))]
mod tests {
    use crate::ast::{Dna, Gene, Helix, JunctionType, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::{ChimeraVM, Value};

    fn make_dna(genes: Vec<Gene>) -> Dna {
        Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        }
    }

    #[test]
    fn test_assert_query_fact() {
        // assert(human(socrates))
        // query(human(socrates))
        let genes = vec![
            // Push "human"("socrates")
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Junction(
                    JunctionType::Any,
                    vec![
                        Nucleotide::String("human".to_string()),
                        Nucleotide::String("socrates".to_string()),
                    ],
                )],
            },
            Gene {
                op: OpCode::Assert,
                args: vec![],
            },
            // Push query "human"("socrates")
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Junction(
                    JunctionType::Any,
                    vec![
                        Nucleotide::String("human".to_string()),
                        Nucleotide::String("socrates".to_string()),
                    ],
                )],
            },
            Gene {
                op: OpCode::Query,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));
        vm.step(); // push fact
        vm.step(); // assert
        vm.step(); // push query
        vm.step(); // query

        // Stack should contain [1, Bindings]
        // bindings should be empty list if exact match
        assert!(vm.stack.len() >= 1);
        // Result (Success) is second from top or pushed before bindings?
        // Code: vm.stack.push(1); ... vm.stack.push(Bindings);
        // So Top is Bindings, Below is 1.

        let _bindings = vm.stack.pop().unwrap();
        let result = vm.stack.pop().unwrap();

        assert_eq!(result, Value::Int(1));
    }

    #[test]
    fn test_query_fail() {
        // assert(human(socrates))
        // query(human(plato))
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Junction(
                    JunctionType::Any,
                    vec![
                        Nucleotide::String("human".to_string()),
                        Nucleotide::String("socrates".to_string()),
                    ],
                )],
            },
            Gene {
                op: OpCode::Assert,
                args: vec![],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Junction(
                    JunctionType::Any,
                    vec![
                        Nucleotide::String("human".to_string()),
                        Nucleotide::String("plato".to_string()),
                    ],
                )],
            },
            Gene {
                op: OpCode::Query,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));
        while !vm.halted {
            vm.step();
        }

        // Stack should contain 0 (Fail)
        assert_eq!(vm.stack[0], Value::Int(0));
    }

    #[test]
    fn test_variable_unification() {
        // assert(human(socrates))
        // query(human(?X))
        // Should succeed
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Junction(
                    JunctionType::Any,
                    vec![
                        Nucleotide::String("human".to_string()),
                        Nucleotide::String("socrates".to_string()),
                    ],
                )],
            },
            Gene {
                op: OpCode::Assert,
                args: vec![],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Junction(
                    JunctionType::Any,
                    vec![
                        Nucleotide::String("human".to_string()),
                        Nucleotide::String("?X".to_string()),
                    ],
                )],
            },
            Gene {
                op: OpCode::Query,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));
        while !vm.halted {
            vm.step();
        }

        let bindings = vm.stack.pop().unwrap();
        let result = vm.stack.pop().unwrap();

        assert_eq!(result, Value::Int(1));

        // Check bindings
        if let Value::Junction(_, list) = bindings {
            assert!(!list.is_empty());
            // Binding: Junction(All, ["?X", "socrates"])
            let binding = &list[0];
            if let Value::Junction(_, pair) = binding {
                assert_eq!(pair[0], Value::Str("?X".to_string()));
                assert_eq!(pair[1], Value::Str("socrates".to_string()));
            } else {
                panic!("Invalid binding format");
            }
        } else {
            panic!("Expected bindings list");
        }
    }

    #[test]
    fn test_dynamic_grid_query() {
        // [ push(100) push(5) push(5) g_write() push("cell") push("?X") push(5) push(100) query() ]
        // cell(?X, 5, 100) -> Should find ?X=5.
        // Stack order for Query Junction: ["cell", "?X", 5, 100]

        let genes = vec![
            // Write 100 to (5,5)
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(100)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(5)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(5)],
            },
            Gene {
                op: OpCode::GWrite,
                args: vec![],
            },
            // Query cell(?X, 5, 100)
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Junction(
                    JunctionType::Any,
                    vec![
                        Nucleotide::String("cell".to_string()),
                        Nucleotide::String("?X".to_string()),
                        Nucleotide::Number(5),
                        Nucleotide::Number(100),
                    ],
                )],
            },
            Gene {
                op: OpCode::Query,
                args: vec![],
            },
        ];

        let len = genes.len();
        let mut vm = ChimeraVM::new(make_dna(genes));

        // Run steps
        for _ in 0..20 {
            // Enough steps
            vm.step();
            if vm.ip.0 > 0 || vm.ip.1 >= len {
                break;
            }
        }

        // Stack top: Bindings list
        let bindings = vm.stack.pop().unwrap();
        let result = vm.stack.pop().unwrap();

        assert_eq!(result, Value::Int(1)); // Success

        // Check binding ?X = 5
        if let Value::Junction(_, list) = bindings {
            let found = list.iter().any(|b| {
                if let Value::Junction(_, pair) = b {
                    pair[0] == Value::Str("?X".to_string()) && pair[1] == Value::Int(5)
                } else {
                    false
                }
            });
            assert!(found, "Binding ?X=5 not found");
        } else {
            panic!("Invalid bindings format");
        }
    }

    #[test]
    fn test_opcode_predicate() {
        // query(opcode("push", ?X))
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Junction(
                    JunctionType::Any,
                    vec![
                        Nucleotide::String("opcode".to_string()),
                        Nucleotide::String("push".to_string()),
                        Nucleotide::String("?X".to_string()),
                    ],
                )],
            },
            Gene {
                op: OpCode::Query,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));
        vm.step(); // push
        vm.step(); // query

        let _bindings = vm.stack.pop().unwrap();
        let result = vm.stack.pop().unwrap();
        assert_eq!(result, Value::Int(1)); // Should find it
    }

    #[test]
    fn test_past_cell_predicate() {
        #[cfg(feature = "nova")]
        {
            // Write 100, wait, query past
            let genes = vec![
                // Write 100 at 0,0
                Gene { op: OpCode::Push, args: vec![Nucleotide::Number(100)] },
                Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] },
                Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] },
                Gene { op: OpCode::GWrite, args: vec![] },

                // Wait (step to push to history)
                // Grid history is updated at START of step.
                // So if we write, then next step starts, history is pushed.
                // So T=0 should see the write.
                Gene { op: OpCode::Photosynthesize, args: vec![] },

                // Query past_cell(0, 0, 0, ?X)
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Junction(
                        JunctionType::Any,
                        vec![
                            Nucleotide::String("past_cell".to_string()),
                            Nucleotide::Number(0), // Ticks back
                            Nucleotide::Number(0), // X
                            Nucleotide::Number(0), // Y
                            Nucleotide::String("?X".to_string()), // Val
                        ],
                    )],
                },
                Gene { op: OpCode::Query, args: vec![] },
            ];

            let mut vm = ChimeraVM::new(make_dna(genes));
            // Run until done
            for _ in 0..10 {
                vm.step();
                if vm.stack.len() >= 2 && matches!(vm.stack.last(), Some(Value::Junction(_, _))) {
                     break;
                }
            }

            let bindings = vm.stack.pop().unwrap();
            let result = vm.stack.pop().unwrap();

            assert_eq!(result, Value::Int(1));

            // Check ?X = 100
            if let Value::Junction(_, list) = bindings {
                let found = list.iter().any(|b| {
                    if let Value::Junction(_, pair) = b {
                        pair[0] == Value::Str("?X".to_string()) && pair[1] == Value::Int(100)
                    } else {
                        false
                    }
                });
                assert!(found, "Binding ?X=100 not found in history");
            }
        }
    }
}
