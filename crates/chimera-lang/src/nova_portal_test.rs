#[cfg(test)]
#[cfg(feature = "nova")]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::ChimeraVM;

    fn make_dna(genes: Vec<Gene>) -> Dna {
        Dna { evolution_config: None,
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        }
    }

    #[test]
    fn test_rift_migrate() {
        // [ push(2) push(2) push(0) push(0) rift() push(1) push(0) migrate() ]
        // Rift: (0,0) -> (2,2)
        // Migrate: (0,0) + (0,1) -> (0,1)? No wait.
        // Migrate args: dy, dx.
        // Current pos: (8,8) (default).
        // Let's set pos to (0,0) first? No easy way unless we walk there.
        // Or we open rift AT (8,9) pointing to (0,0).
        // Then migrate (0,1) from (8,8) -> (8,9) -> Teleport -> (0,0).

        let genes = vec![
            // Rift: (8,9) -> (0,0)
            // stack: y1, x1, y2, x2 (top)
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(8)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(9)],
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
                op: OpCode::Rift,
                args: vec![],
            },
            // Migrate: (0, 1) -> moves from (8,8) to (8,9) -> (0,0)
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
        ];

        let mut vm = ChimeraVM::new(make_dna(genes));
        vm.energy = 1000;

        // Step 1-5: Push args and Rift
        // 1. push(0)
        // 2. push(0)
        // 3. push(8)
        // 4. push(9)
        // 5. rift
        for _ in 0..5 {
            vm.step();
        }

        // Verify portal exists
        assert!(vm.portals.contains_key(&(8, 9)));
        assert_eq!(vm.portals[&(8, 9)], (0, 0));

        // Step 6-8: Push args and Migrate
        // 6. push(0)
        // 7. push(1)
        // 8. migrate
        for _ in 0..3 {
            vm.step();
        }

        // Verify location
        assert_eq!(vm.context_loc, (0, 0));
    }

    #[test]
    fn test_seal() {
        let genes = vec![
            // Rift: (8,9) -> (0,0)
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
                args: vec![Nucleotide::Number(8)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(9)],
            },
            Gene {
                op: OpCode::Rift,
                args: vec![],
            },
            // Seal: (8,9)
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(9)],
            }, // x
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(8)],
            }, // y
            Gene {
                op: OpCode::Seal,
                args: vec![],
            },
        ];

        let mut vm = ChimeraVM::new(make_dna(genes));

        // Execute all
        for _ in 0..8 {
            vm.step();
        }

        assert!(!vm.portals.contains_key(&(8, 9)));
    }
}
