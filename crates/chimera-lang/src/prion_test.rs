#[cfg(all(test, feature = "nova"))]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::{ChimeraVM, Value};

    fn make_dna(genes: Vec<Gene>) -> Dna {
        Dna { evolution_config: None,
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        }
    }

    #[test]
    fn test_remap() {
        // [ push(10) push(5) push("add") push("sub") remap() add() ]
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(10)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(5)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("add".to_string())],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("sub".to_string())],
            },
            Gene {
                op: OpCode::Remap,
                args: vec![],
            },
            Gene {
                op: OpCode::Add,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));

        // Step through
        // 1. push(10)
        // 2. push(5)
        // 3. push("add")
        // 4. push("sub")
        // 5. remap() -> remap_table[Add] = Sub
        // 6. add() -> executes Sub(10, 5) -> 5

        for _ in 0..6 {
            vm.step();
        }

        assert_eq!(vm.stack.len(), 1);
        assert_eq!(vm.stack[0], Value::Int(5));

        // Verify remap table
        assert!(vm.remap_table.contains_key(&OpCode::Add));
        assert_eq!(vm.remap_table[&OpCode::Add], OpCode::Sub);
    }

    #[test]
    fn test_restore() {
        // [ push("add") push("sub") remap() push("add") restore() ]
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("add".to_string())],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("sub".to_string())],
            },
            Gene {
                op: OpCode::Remap,
                args: vec![],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("add".to_string())],
            },
            Gene {
                op: OpCode::Restore,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));

        for _ in 0..5 {
            vm.step();
        }

        assert!(!vm.remap_table.contains_key(&OpCode::Add));
    }

    #[test]
    fn test_mirror() {
        // Strand 0: [ push(1) ]
        // Strand 1: [ mirror() ]
        // Strand 2: [ push(2) ]

        // Execution:
        // 1. Strand 0: push(1). Stack [1]. Next -> Strand 1.
        // 2. Strand 1: mirror(). Dir -1. Next -> Back to Strand 0.
        // 3. Strand 0: push(1). Stack [1, 1]. Next -> Back to ... (wrap to last strand?)
        //    Logic: ip.1 > 0? No (len 1). ip.0 -= 1 -> -1 -> Wrap to Strand 2.
        //    ip.1 = len(Strand 2) - 1 = 0.
        // 4. Strand 2: push(2). Stack [1, 1, 2].

        let strand0 = Strand {
            genes: vec![Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            }],
        };
        let strand1 = Strand {
            genes: vec![Gene {
                op: OpCode::Mirror,
                args: vec![],
            }],
        };
        let strand2 = Strand {
            genes: vec![Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(2)],
            }],
        };

        let dna = Dna { evolution_config: None,
            helix: Helix {
                strands: vec![strand0, strand1, strand2],
            },
        };
        let mut vm = ChimeraVM::new(dna);

        vm.step(); // push(1)
        assert_eq!(vm.stack.last(), Some(&Value::Int(1)));
        // IP is at (0, 1) because strand transition happens at start of next step
        assert_eq!(vm.ip, (0, 1));

        vm.step(); // transitions to (1, 0)
        assert_eq!(vm.ip, (1, 0));

        vm.step(); // executes mirror()
        assert_eq!(vm.direction, -1);
        // Step logic should have moved us to (0, 0)
        assert_eq!(vm.ip, (0, 0));

        vm.step(); // push(1)
        assert_eq!(vm.stack.len(), 2);
        // Step logic: (0,0) -> back -> wrap to (2, 0)
        assert_eq!(vm.ip, (2, 0));

        vm.step(); // push(2)
        assert_eq!(vm.stack.len(), 3);
        assert_eq!(vm.stack[2], Value::Int(2));
    }
}
