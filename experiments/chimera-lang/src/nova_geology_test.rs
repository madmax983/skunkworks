#[cfg(feature = "nova")]
#[cfg(test)]
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
    fn test_erode() {
        // [ push(1) erode() ]
        // Should decrement values in radius 1
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            },
            Gene {
                op: OpCode::Erode,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));

        // Setup grid
        // Context is at 8,8
        vm.context_loc = (8, 8);
        vm.grid[8][8] = Value::Int(10);
        vm.grid[8][9] = Value::Int(10);

        vm.step(); // push
        vm.step(); // erode

        assert_eq!(vm.grid[8][8], Value::Int(9));
        assert_eq!(vm.grid[8][9], Value::Int(9));
        // Check energy consumption
        assert!(vm.energy < 50);
    }

    #[test]
    fn test_sediment() {
        // [ push(1) sediment() ]
        // Should increment values in radius 1
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            },
            Gene {
                op: OpCode::Sediment,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));

        // Setup grid
        vm.context_loc = (8, 8);
        vm.grid[8][8] = Value::Int(10);

        vm.step(); // push
        vm.step(); // sediment

        assert_eq!(vm.grid[8][8], Value::Int(11));
    }

    #[test]
    fn test_tectonics_shift() {
        // [ push(1) push(0) push(3) push(3) tectonics() ]
        // Shift 3x3 plate by dy=1, dx=0
        // args on stack: dy, dx, h, w.
        // So push order: w, h, dx, dy?
        // Code:
        // let w_val = vm.stack.pop().unwrap();
        // let h_val = vm.stack.pop().unwrap();
        // let dx_val = vm.stack.pop().unwrap();
        // let dy_val = vm.stack.pop().unwrap();
        // So stack must be: [dy, dx, h, w] (top)

        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            }, // dy
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            }, // dx
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(3)],
            }, // h
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(3)],
            }, // w
            Gene {
                op: OpCode::Tectonics,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));

        vm.context_loc = (8, 8);
        // Plate is 3x3 centered at 8,8 -> 7..10, 7..10
        vm.grid[8][8] = Value::Int(100);
        vm.grid[7][7] = Value::Int(50);

        // Execute all
        while !vm.halted && vm.ip.0 == 0 {
            vm.step();
        }

        // 8,8 should move to 9,8 (dy=1, dx=0)
        assert_eq!(vm.grid[9][8], Value::Int(100));
        // Old pos should be 0 (if not overwritten)
        assert_eq!(vm.grid[8][8], Value::Int(0));

        // 7,7 should move to 8,7
        assert_eq!(vm.grid[8][7], Value::Int(50));
    }

    #[test]
    fn test_tectonics_large_plate_bug() {
        // Test the potential bug where large plates cause data destruction
        // If w or h >= GRID_SIZE, we iterate over the same cells multiple times.
        // 1. Visit cell X: read V, write 0.
        // 2. Wrap around, Visit cell X again: read 0, write 0.
        // Result: V is lost.

        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            }, // dy
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            }, // dx
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(20)],
            }, // h (Large Grid Height)
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(20)],
            }, // w (Large Grid Width)
            Gene {
                op: OpCode::Tectonics,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));

        vm.context_loc = (8, 8);
        // Set a value anywhere
        vm.grid[0][0] = Value::Int(123);

        while !vm.halted && vm.ip.0 == 0 {
            vm.step();
        }

        // Should have moved to 1,0
        // If bug exists, it might be 0 because it was cleared by the wrapping iteration
        assert_eq!(
            vm.grid[1][0],
            Value::Int(123),
            "Data lost due to overlapping read/clear loop!"
        );
    }

    #[test]
    fn test_volcano_safety() {
        // Volcano expects [power].
        // 1. Empty stack -> Error message, no panic.
        let genes = vec![Gene {
            op: OpCode::Volcano,
            args: vec![],
        }];
        let mut vm = ChimeraVM::new(make_dna(genes));
        let initial_energy = vm.energy;

        vm.step(); // volcano

        assert!(
            vm.output.iter().any(|s| s.contains("Error")),
            "Expected error for empty stack"
        );
        // Energy should be unchanged (except for step cost)
        assert_eq!(vm.energy, initial_energy - 1);
    }

    #[test]
    fn test_quake_safety() {
        // Quake expects [intensity].
        // 1. Empty stack -> Defaults to intensity 1.
        let genes = vec![Gene {
            op: OpCode::Quake,
            args: vec![],
        }];
        let mut vm = ChimeraVM::new(make_dna(genes));
        let initial_energy = vm.energy;

        vm.step(); // quake

        // Should NOT have error
        assert!(
            !vm.output.iter().any(|s| s.contains("Error")),
            "Quake should handle empty stack gracefully"
        );

        // Should consume energy: step cost (1) + quake cost (1 * 5 = 5) = 6
        assert_eq!(vm.energy, initial_energy - 6);
    }
}
