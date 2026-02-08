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

        let bindings = vm.stack.pop().unwrap();
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
}
