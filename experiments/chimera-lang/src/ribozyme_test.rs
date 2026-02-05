#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Gene, Helix, JunctionType, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::{ChimeraVM, Value};

    fn make_vm(genes: Vec<Gene>) -> ChimeraVM {
        let dna = Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        };
        ChimeraVM::new(dna)
    }

    #[test]
    fn test_eval_simple() {
        // [ push(10) push(20) push("add") eval() ]
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(10)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(20)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("add".to_string())],
            },
            Gene {
                op: OpCode::Eval,
                args: vec![],
            },
        ];
        let mut vm = make_vm(genes);
        while !vm.halted {
            vm.step();
        }

        assert_eq!(vm.stack.pop(), Some(Value::Int(30)));
    }

    #[test]
    fn test_map_add() {
        // [ push(10) push(Junction(Any, [1, 2])) push("add") map() ]
        // Note: Map consumes the junction.
        // Iter 1: Stack [10, 1]. add -> [11].
        // Iter 2: Stack [??].
        // Wait, 'add' consumes the 10 too!
        // So Iter 2 will fail or use what's underneath.
        // Map is destructive to the context if the op consumes context.
        // Let's use Dup inside map? No, Map takes 1 op.

        // Correct usage of Map with binary op requires broadcasting the other arg?
        // Or Map is only for Unary ops.
        // Let's test Unary op: 'dup'.
        // [ push(Junction([10, 20])) push("dup") map() ]
        // Iter 1: [10] -> dup -> [10, 10]. Pop 10. Result 10. Stack [10].
        // Iter 2: [10, 20] -> dup -> [10, 20, 20]. Pop 20. Result 20. Stack [10, 20].
        // This works nicely.

        // But let's try the dangerous 'add'.
        // [ push(10) push(20) push(Junction([1, 2])) push("add") map() ]
        // Iter 1: [10, 20, 1] -> add -> [10, 21].
        // Iter 2: [10, 21, 2] -> add -> [10, 23].
        // Result: Junction([21, 23]).
        // Stack ends as [10, 23] + Junction([21, 23]).
        // Wait, map pushes result junction.
        // So Stack: [10, 23, Junction([21, 23])].

        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(10)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(20)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Junction(
                    JunctionType::Any,
                    vec![Nucleotide::Number(1), Nucleotide::Number(2)],
                )],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("add".to_string())],
            },
            Gene {
                op: OpCode::Map,
                args: vec![],
            },
        ];
        let mut vm = make_vm(genes);
        while !vm.halted {
            vm.step();
        }

        let res = vm.stack.pop().unwrap();
        match res {
            Value::Junction(_, vals) => {
                assert_eq!(vals[0], Value::Int(21));
                // Iter 2: Stack [10, 2] -> 12
                assert_eq!(vals[1], Value::Int(12));
            }
            _ => panic!("Expected Junction"),
        }
    }

    #[test]
    fn test_fold_sum() {
        // [ push(0) push(Junction([1, 2, 3])) push("add") fold() ]
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Junction(
                    JunctionType::Any,
                    vec![
                        Nucleotide::Number(1),
                        Nucleotide::Number(2),
                        Nucleotide::Number(3),
                    ],
                )],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("add".to_string())],
            },
            Gene {
                op: OpCode::Fold,
                args: vec![],
            },
        ];
        let mut vm = make_vm(genes);
        while !vm.halted {
            vm.step();
        }

        assert_eq!(vm.stack.pop(), Some(Value::Int(6)));
    }

    #[test]
    fn test_filter() {
        // [ push(Junction([0, 5, 0, 10])) push("dup") filter() ]
        // dup: [x] -> [x, x]. Pop x. If x != 0, keep x.
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Junction(
                    JunctionType::Any,
                    vec![
                        Nucleotide::Number(0),
                        Nucleotide::Number(5),
                        Nucleotide::Number(0),
                        Nucleotide::Number(10),
                    ],
                )],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("dup".to_string())],
            },
            Gene {
                op: OpCode::Filter,
                args: vec![],
            },
        ];
        let mut vm = make_vm(genes);
        while !vm.halted {
            vm.step();
        }

        let res = vm.stack.pop().unwrap();
        match res {
            Value::Junction(_, vals) => {
                assert_eq!(vals.len(), 2);
                assert_eq!(vals[0], Value::Int(5));
                assert_eq!(vals[1], Value::Int(10));
            }
            _ => panic!("Expected Junction"),
        }
    }

    #[test]
    fn test_zip_add() {
        // [ push(Junction([10, 20])) push(Junction([1, 2])) push("add") zip() ]
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Junction(
                    JunctionType::Any,
                    vec![Nucleotide::Number(10), Nucleotide::Number(20)],
                )],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Junction(
                    JunctionType::Any,
                    vec![Nucleotide::Number(1), Nucleotide::Number(2)],
                )],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("add".to_string())],
            },
            Gene {
                op: OpCode::Zip,
                args: vec![],
            },
        ];
        let mut vm = make_vm(genes);
        while !vm.halted {
            vm.step();
        }

        let res = vm.stack.pop().unwrap();
        match res {
            Value::Junction(_, vals) => {
                assert_eq!(vals[0], Value::Int(11));
                assert_eq!(vals[1], Value::Int(22));
            }
            _ => panic!("Expected Junction"),
        }
    }
}
