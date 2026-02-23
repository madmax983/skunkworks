#[cfg(all(test, feature = "nova"))]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::{ChimeraVM, Value};

    #[test]
    fn test_ward_trigger() {
        // Strand 0: [ Push(1) Push(1) Ward, Push(1) Push(0) Migrate, Push(-1) Push(0) Migrate ]
        // Writes Ward(1, 1) at (8,8).
        // Moves East to (8,9).
        // Moves West to (8,8). -> Should trigger Strand 1.

        let s0 = Strand {
            genes: vec![
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(1)],
                }, // Persistence
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(1)],
                }, // Strand Idx
                Gene {
                    op: OpCode::Ward,
                    args: vec![],
                },
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(0)],
                }, // dy
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(1)],
                }, // dx
                Gene {
                    op: OpCode::Migrate,
                    args: vec![],
                },
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(0)],
                }, // dy
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(-1)],
                }, // dx
                Gene {
                    op: OpCode::Migrate,
                    args: vec![],
                },
            ],
        };

        // Strand 1: [ Push(999) ]
        let s1 = Strand {
            genes: vec![Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(999)],
            }],
        };

        let mut vm = ChimeraVM::new(Dna { evolution_config: None,
            helix: Helix {
                strands: vec![s0, s1],
            },
        });

        // Run
        // 1. Ward
        vm.step(); // push
        vm.step(); // push
        vm.step(); // ward

        match &vm.grid[8][8] {
            Value::Str(s) => assert_eq!(s, "WARD:1:1"),
            _ => panic!("Ward not written"),
        }

        // 2. Migrate East
        vm.step(); // push
        vm.step(); // push
        vm.step(); // migrate
        assert_eq!(vm.context_loc, (8, 9));

        // 3. Migrate West (into Ward)
        vm.step(); // push
        vm.step(); // push
        vm.step(); // migrate -> Should trigger interrupt to Strand 1

        assert_eq!(vm.context_loc, (8, 8));

        // Check output
        assert!(vm.output.iter().any(|s| s.contains("WARD: Triggered")));

        // Next step should execute Strand 1 Gene 0 (Push 999)
        // Note: Migrate returns Some(target), so vm.ip is set to (1, 0).
        // Then loop continues? No, step() loop:
        // if let Some(target) = jump_target { self.ip = target; }
        // then loop continues if iterations > 0.
        // iterations default 1.
        // So next instruction is executed in NEXT tick.

        assert_eq!(vm.ip, (1, 0));

        vm.step(); // Push 999

        assert_eq!(vm.stack.last(), Some(&Value::Int(999)));
    }
}
