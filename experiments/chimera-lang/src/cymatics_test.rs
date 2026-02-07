#[cfg(all(test, feature = "cymatics"))]
mod tests {
    use crate::vm::{ChimeraVM, Value};
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;

    fn make_dna(genes: Vec<Gene>) -> Dna {
        Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        }
    }

    #[test]
    fn test_cymatics_strike() {
        let genes = vec![
            Gene { op: OpCode::Cymatics, args: vec![] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(100)] },
            Gene { op: OpCode::Strike, args: vec![] },
        ];

        let mut vm = ChimeraVM::new(make_dna(genes));

        // 1. Enable Cymatics
        vm.step();
        assert!(vm.cymatics_mode);
        assert!(vm.cymatics_grid.is_some());

        // 2. Push 100
        vm.step();

        // 3. Strike
        vm.step();

        // Verify wave initiated at context_loc (8,8)
        if let Some(grid) = &vm.cymatics_grid {
            let val = grid.get(8, 8);
            // 100 / 10.0 = 10.0
            assert_eq!(val, 10.0);
        } else {
            panic!("Grid missing");
        }
    }

    #[test]
    fn test_cymatics_sift() {
        let genes = vec![
            Gene { op: OpCode::Cymatics, args: vec![] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(100)] }, // Strike Str
            Gene { op: OpCode::Strike, args: vec![] },
            Gene { op: OpCode::Push, args: vec![Nucleotide::Number(10)] }, // Threshold
            Gene { op: OpCode::Sift, args: vec![] },
        ];

        let mut vm = ChimeraVM::new(make_dna(genes));

        // Place item at (8,9) - East of center
        vm.grid[9][8] = Value::Int(42);

        // Run: Cymatics -> Push -> Strike
        vm.step();
        vm.step();
        vm.step();

        // Wave is at (8,8) with amp 10.0. (9,8) is 0.0.
        // But wave hasn't propagated yet if we don't step physics.
        // vm.step() calls physics step.
        // Let's run physics for a few ticks to spread wave.
        for _ in 0..5 {
            if let Some(grid) = &mut vm.cymatics_grid {
                grid.step();
            }
        }

        // Run: Push -> Sift
        vm.step();
        vm.step();

        // We just check that Sift didn't crash and maybe moved something.
        // Exact movement depends on wave physics (PhysicsGrid implementation).
        // At (9,8), if amplitude is high, it should move to lower amplitude.

        let mut found = false;
        for y in 0..16 {
            for x in 0..16 {
                if matches!(vm.grid[y][x], Value::Int(42)) {
                    found = true;
                    // println!("Item found at {},{}", x, y);
                }
            }
        }
        assert!(found, "Item should still exist");
    }
}
