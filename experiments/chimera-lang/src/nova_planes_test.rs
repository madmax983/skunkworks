#[cfg(test)]
#[cfg(feature = "nova")]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
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
    fn test_dimension_shift() {
        // [ push(100) push(8) push(8) g_write() push(1) dimension() g_read() ]
        // 1. Write 100 to (8,8) in Plane 0.
        // 2. Switch to Plane 1.
        // 3. Read (8,8). Should be 0 (fresh grid).
        // 4. Switch back to Plane 0.
        // 5. Read (8,8). Should be 100 (persisted).

        let genes = vec![
            // Write 100 to (8,8)
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(100)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(8)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(8)],
            },
            Gene {
                op: OpCode::GWrite,
                args: vec![],
            },
            // Switch to Plane 1
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            },
            Gene {
                op: OpCode::Dimension,
                args: vec![],
            },
            // Read (8,8) - Should be 0
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(8)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(8)],
            },
            Gene {
                op: OpCode::GRead,
                args: vec![],
            },
            // Switch back to Plane 0
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            },
            Gene {
                op: OpCode::Dimension,
                args: vec![],
            },
            // Read (8,8) - Should be 100
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(8)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(8)],
            },
            Gene {
                op: OpCode::GRead,
                args: vec![],
            },
        ];

        let mut vm = ChimeraVM::new(make_dna(genes));

        // Execute steps
        // 4 writes
        vm.step();
        vm.step();
        vm.step();
        vm.step();
        // 2 dimension
        vm.step();
        vm.step();
        // 3 read
        vm.step();
        vm.step();
        vm.step();
        // 2 dimension back
        vm.step();
        vm.step();
        // 3 read
        vm.step();
        vm.step();
        vm.step();

        assert_eq!(vm.stack.len(), 2);
        assert_eq!(vm.stack[0], Value::Int(0));
        assert_eq!(vm.stack[1], Value::Int(100));
    }

    #[test]
    fn test_dwrite_dread() {
        // [ push(50) push(5) push(5) push(2) dwrite() push(5) push(5) push(2) dread() ]
        // Write 50 to Plane 2 at (5,5) without leaving Plane 0.
        // Read it back.

        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(50)],
            }, // val
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(5)],
            }, // y
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(5)],
            }, // x
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(2)],
            }, // plane
            Gene {
                op: OpCode::DWrite,
                args: vec![],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(5)],
            }, // y
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(5)],
            }, // x
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(2)],
            }, // plane
            Gene {
                op: OpCode::DRead,
                args: vec![],
            },
        ];

        let mut vm = ChimeraVM::new(make_dna(genes));
        for _ in 0..9 {
            vm.step();
        }

        assert_eq!(vm.stack.len(), 1);
        assert_eq!(vm.stack[0], Value::Int(50));

        // Verify we are still on Plane 0
        assert_eq!(vm.current_plane, 0);
    }

    #[test]
    fn test_dmerge() {
        // Plane 0: (0,0) = 10
        // Plane 1: (0,0) = 20
        // DMerge(1, 0) -> Add -> Plane 0 (0,0) = 30.

        let genes = vec![
            // Write 10 to Plane 0 (0,0)
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(10)],
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
            // Write 20 to Plane 1 (0,0) using DWrite
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(20)],
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
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            },
            Gene {
                op: OpCode::DWrite,
                args: vec![],
            },
            // Merge Plane 1 into Plane 0 (Add)
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            }, // Method 0 (Add)
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            }, // Plane ID
            Gene {
                op: OpCode::DMerge,
                args: vec![],
            },
        ];

        let mut vm = ChimeraVM::new(make_dna(genes));
        // Execute all
        for _ in 0..12 {
            vm.step();
        }

        assert_eq!(vm.grid[0][0], Value::Int(30));
    }
}
