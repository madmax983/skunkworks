#[cfg(feature = "nova")]
#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::{ChimeraVM, Chirality, Value};

    fn make_dna(genes: Vec<Gene>) -> Dna {
        Dna {
            evolution_config: None,
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        }
    }

    #[test]
    fn test_isomerize_toggle() {
        let genes = vec![Gene {
            op: OpCode::Isomerize,
            args: vec![],
        }];
        let mut vm = ChimeraVM::new(make_dna(genes));

        assert_eq!(vm.chirality, Chirality::Left);
        vm.step();
        assert_eq!(vm.chirality, Chirality::Right);
    }

    #[test]
    fn test_isomer_arithmetic() {
        // Left: 10 + 20 = 30
        // Right: 10 - 20 = -10 (Sub)
        // Right: 10 - 5 = 15 (Add) - wait Sub -> Add

        let genes = vec![
            Gene {
                op: OpCode::Isomerize,
                args: vec![],
            },
            // Test Add -> Sub
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(10)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(20)],
            },
            Gene {
                op: OpCode::Add,
                args: vec![],
            },
            // Test Sub -> Add
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(10)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(5)],
            },
            Gene {
                op: OpCode::Sub,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));

        vm.step(); // Isomerize
        vm.step();
        vm.step();
        vm.step(); // 10, 20, Add(Sub) -> -10

        assert_eq!(vm.stack.pop(), Some(Value::Int(-10)));

        vm.step();
        vm.step();
        vm.step(); // 10, 5, Sub(Add) -> 15
        assert_eq!(vm.stack.pop(), Some(Value::Int(15)));
    }

    #[test]
    fn test_isomer_control_flow() {
        // Left: Push(1) Brz(3) -> No Jump (1 != 0)
        // Right: Push(1) Brz(3) -> Jump (1 != 0 is True for Brnz)
        // Note: Brz in Right mode is Brnz (Branch if Not Zero)

        // Code:
        // 0: Isomerize
        // 1: Push(1)
        // 2: Brz(1) -> Jumps to strand 1
        // 3: Push(99)

        // Strand 1:
        // 0: Push(100)

        let strand0 = Strand {
            genes: vec![
                Gene {
                    op: OpCode::Isomerize,
                    args: vec![],
                },
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(1)],
                },
                Gene {
                    op: OpCode::Brz,
                    args: vec![Nucleotide::Number(1)],
                },
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(99)],
                },
            ],
        };
        let strand1 = Strand {
            genes: vec![Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(100)],
            }],
        };

        let dna = Dna {
            evolution_config: None,
            helix: Helix {
                strands: vec![strand0, strand1],
            },
        };
        let mut vm = ChimeraVM::new(dna);

        // Isomerize
        vm.step();
        // Push 1
        vm.step();
        // Brz(1) -> Should Jump because 1 != 0
        vm.step();

        assert_eq!(vm.ip, (1, 0));
    }

    #[test]
    fn test_isomer_spatial() {
        // Migrate(1, 0) (South)
        // Right: Migrate(-1, 0) (North)

        // Start at 8,8
        // Op: Isomerize, Migrate(1, 0)
        // Expect: 7,8 (North)

        let genes = vec![
            Gene {
                op: OpCode::Isomerize,
                args: vec![],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            }, // dx
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            }, // dy
            Gene {
                op: OpCode::Migrate,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));

        vm.step(); // Isomerize
        vm.step(); // Push
        vm.step(); // Push
        vm.step(); // Migrate

        // Push(0), Push(1) -> Stack [0, 1]. Pop dx=1, dy=0.
        // Original: East (0, 1).
        // Inverted: West (0, -1).
        // Start (8, 8). West is (8, 7).
        assert_eq!(vm.context_loc, (8, 7));
    }
}
