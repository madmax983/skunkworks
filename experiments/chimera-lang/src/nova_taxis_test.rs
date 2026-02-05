#[cfg(all(test, feature = "nova"))]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::ChimeraVM;

    fn make_dna(genes: Vec<Gene>) -> Dna {
        Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        }
    }

    #[test]
    fn test_phototaxis() {
        // [ phototaxis() ]
        let genes = vec![Gene {
            op: OpCode::Phototaxis,
            args: vec![],
        }];
        let mut vm = ChimeraVM::new(make_dna(genes));

        // Set up light gradient
        // Current pos: (8,8)
        // Target: (7,8) - North
        vm.light_grid[8][8] = 10;
        vm.light_grid[7][8] = 20;

        // Execute
        vm.step();

        // Check position
        assert_eq!(vm.context_loc, (7, 8));
        // Check energy consumption
        assert_eq!(vm.energy, 44); // 50 - 1 (step) - 5 (phototaxis) = 44
    }

    #[test]
    fn test_phototaxis_stay() {
        // [ phototaxis() ]
        let genes = vec![Gene {
            op: OpCode::Phototaxis,
            args: vec![],
        }];
        let mut vm = ChimeraVM::new(make_dna(genes));

        // Set up local maximum
        vm.light_grid[8][8] = 100;
        vm.light_grid[7][8] = 20;

        // Execute
        vm.step();

        // Should stay at (8,8)
        assert_eq!(vm.context_loc, (8, 8));
    }

    #[test]
    fn test_phototaxis_diagonal() {
        // [ phototaxis() ]
        let genes = vec![Gene {
            op: OpCode::Phototaxis,
            args: vec![],
        }];
        let mut vm = ChimeraVM::new(make_dna(genes));

        // Target: (9,9) - SouthEast
        vm.light_grid[8][8] = 10;
        vm.light_grid[9][9] = 50;

        // Execute
        vm.step();

        // Check position
        assert_eq!(vm.context_loc, (9, 9));
    }

    #[test]
    fn test_chemotaxis() {
        // [ push(1) chemotaxis() ] -> seek channel 1
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            },
            Gene {
                op: OpCode::Chemotaxis,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));

        // Set up hormone gradient on channel 1
        // Current pos: (8,8)
        // Target: (8,7) - West
        vm.hormone_grid[8][8][1] = 10;
        vm.hormone_grid[8][7][1] = 30;

        // Step 1: Push
        vm.step();
        // Step 2: Chemotaxis
        vm.step();

        // Check position
        assert_eq!(vm.context_loc, (8, 7));
    }

    #[test]
    fn test_chemotaxis_wrong_channel() {
        // [ push(0) chemotaxis() ] -> seek channel 0
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            },
            Gene {
                op: OpCode::Chemotaxis,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));

        // Set up hormone gradient on channel 1 (should ignore)
        vm.hormone_grid[8][7][1] = 30;
        // Set up hormone gradient on channel 0
        vm.hormone_grid[8][9][0] = 50; // East

        // Step 1: Push
        vm.step();
        // Step 2: Chemotaxis
        vm.step();

        // Check position
        assert_eq!(vm.context_loc, (8, 9));
    }

    #[test]
    fn test_chemotaxis_wrapping() {
        // [ push(2) chemotaxis() ]
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(2)],
            },
            Gene {
                op: OpCode::Chemotaxis,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));

        // Move to edge (0,0)
        vm.context_loc = (0, 0);

        // Gradient at (15, 15) - wrap around Top-Left to Bottom-Right
        vm.hormone_grid[0][0][2] = 10;
        vm.hormone_grid[15][15][2] = 100;

        vm.step();
        vm.step();

        assert_eq!(vm.context_loc, (15, 15));
    }
}
