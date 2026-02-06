#[cfg(test)]
#[cfg(feature = "nova")]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::{ChimeraVM, Value, GRID_SIZE};

    fn make_dna(genes: Vec<Gene>) -> Dna {
        Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        }
    }

    #[test]
    fn test_isomer_arithmetic() {
        // [ isomerize(0) push(10) push(5) add() ]
        // Normal: 10 + 5 = 15
        // Isomer: 10 - 5 = 5 (add -> sub)
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            },
            Gene {
                op: OpCode::Isomerize,
                args: vec![],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(10)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(5)],
            },
            Gene {
                op: OpCode::Add,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));

        while !vm.halted {
            vm.step();
        }

        assert_eq!(vm.stack.last(), Some(&Value::Int(5)));
    }

    #[test]
    fn test_isomer_logic() {
        // [ isomerize(0) push(0) brz(1) push(99) ]
        // Normal: brz(1) pops 0, jumps to 1.
        // Isomer: brnz(1) pops 0, does NOT jump (0 is not non-zero).
        // Should execute push(99).
        let strand0 = Strand {
            genes: vec![
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(0)],
                },
                Gene {
                    op: OpCode::Isomerize,
                    args: vec![],
                },
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(0)],
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
            genes: vec![
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(100)],
                },
            ],
        };

        let dna = Dna {
            helix: Helix { strands: vec![strand0, strand1] },
        };
        let mut vm = ChimeraVM::new(dna);

        while !vm.halted && vm.ip.0 == 0 {
            vm.step();
        }

        // If it jumped, stack would have 100. If not, 99.
        assert_eq!(vm.stack.last(), Some(&Value::Int(99)));
    }

    #[test]
    fn test_isomer_spatial() {
        // [ isomerize(0) push(42) push(0) push(0) g_write() ]
        // Normal: writes to (0, 0)
        // Isomer: writes to (0, 15 - 0) = (0, 15)
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            },
            Gene {
                op: OpCode::Isomerize,
                args: vec![],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(42)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            },
            Gene {
                op: OpCode::GWrite,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));

        while !vm.halted {
            vm.step();
        }

        assert_eq!(vm.grid[0][GRID_SIZE - 1], Value::Int(42));
        assert_eq!(vm.grid[0][0], Value::Int(0));
    }
}
