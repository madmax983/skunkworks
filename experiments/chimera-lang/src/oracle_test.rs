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

        // Stack should contain 1 (Success)
        assert_eq!(vm.stack.len(), 1);
        assert_eq!(vm.stack[0], Value::Int(1));
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

        // Stack should contain 1 (Success)
        assert_eq!(vm.stack[0], Value::Int(1));
        // Output should contain binding
        let output = vm.output.join("\n");
        // Value::Str is printed with quotes
        assert!(output.contains("?X=\"socrates\""));
    }
}
