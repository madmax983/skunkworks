#[cfg(all(test, feature = "nova"))]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
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
    fn test_reflect_horizontal() {
        // [ push(1) push(0) push(0) g_write() push(1) reflect() ]
        // Write 1 to (0,0). Reflect Horizontal (1).
        // Should copy Top to Bottom. (0,0) -> (15,0).
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            }, // val
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            }, // y
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            }, // x
            Gene {
                op: OpCode::GWrite,
                args: vec![],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            }, // Axis 1 (Horizontal)
            Gene {
                op: OpCode::Reflect,
                args: vec![],
            },
        ];
        let mut vm = make_vm(genes);

        // Execute
        for _ in 0..6 {
            vm.step();
        }

        assert_eq!(vm.grid[0][0], Value::Int(1));
        assert_eq!(vm.grid[15][0], Value::Int(1));
    }

    #[test]
    fn test_rotate_90() {
        // Write 1 to (0,0). Rotate 1 (90 CW).
        // (0,0) -> (0, 15).
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
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
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            }, // 1 Turn
            Gene {
                op: OpCode::Rotate,
                args: vec![],
            },
        ];
        let mut vm = make_vm(genes);

        for _ in 0..6 {
            vm.step();
        }

        assert_eq!(vm.grid[0][15], Value::Int(1));
    }

    #[test]
    fn test_symmetrize_quad() {
        // Write 1 to (0,0). Symmetrize 2 (Quad).
        // Should appear at (0,0), (0,15), (15,0), (15,15).
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
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
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(2)],
            }, // Mode 2 (Quad)
            Gene {
                op: OpCode::Symmetrize,
                args: vec![],
            },
        ];
        let mut vm = make_vm(genes);

        for _ in 0..6 {
            vm.step();
        }

        assert_eq!(vm.grid[0][0], Value::Int(1));
        assert_eq!(vm.grid[0][15], Value::Int(1));
        assert_eq!(vm.grid[15][0], Value::Int(1));
        assert_eq!(vm.grid[15][15], Value::Int(1));
    }
}
