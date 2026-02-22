#[cfg(feature = "nova")]
#[cfg(test)]
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
    fn test_waste_accumulation() {
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            },
            Gene {
                op: OpCode::Drop,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));

        // Step 1
        vm.step();
        assert!(vm.waste_grid[8][8] > 0);
    }

    #[test]
    fn test_migrate() {
        // [ push(1) push(1) migrate() ] -> moves to (8+1, 8+1) = (9, 9)
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            },
            Gene {
                op: OpCode::Migrate,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));

        // Execute pushes
        vm.step();
        vm.step();

        // Execute migrate
        vm.step();

        assert_eq!(vm.context_loc, (9, 9));

        // Next step should produce waste at (9,9)
        let old_waste = vm.waste_grid[9][9];
        vm.step(); // End of strand or loop
        assert!(vm.waste_grid[9][9] > old_waste);
    }

    #[test]
    fn test_detox() {
        // Generate waste then detox
        // [ detox(2) ]
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(2)],
            },
            Gene {
                op: OpCode::Detox,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));

        // Manually inject waste
        vm.waste_grid[8][8] = 100;
        vm.waste_grid[9][9] = 100;

        vm.step(); // push
        vm.step(); // detox

        assert_eq!(vm.waste_grid[8][8], 0);
        assert_eq!(vm.waste_grid[9][9], 0);
    }

    #[test]
    fn test_diffusion_performance_and_stability() {
        // This test runs diffusion many times to check for stability/crashes
        // and provides a baseline for performance work (allocations).
        let genes = vec![];
        let mut vm = ChimeraVM::new(make_dna(genes));

        // Seed grid
        vm.waste_grid[5][5] = 10000;
        vm.light_grid[5][5] = 10000;
        vm.hormone_grid[5][5][0] = 10000;

        for _ in 0..10 {
            crate::vm::nova::diffuse_waste(&mut vm);
            crate::vm::nova::diffuse_light(&mut vm);
            crate::vm::nova::diffuse_hormones(&mut vm);
        }

        // Check values have diffused
        assert!(vm.waste_grid[5][5] < 1000);
        assert!(vm.waste_grid[5][5] > 0, "Waste decayed to zero too fast");
        // Check neighbor
        assert!(vm.waste_grid[5][6] > 0, "Neighbor did not receive waste");
    }

    #[test]
    fn test_diffusion_scalar_stack_optimization() {
        let genes = vec![];
        let mut vm = ChimeraVM::new(make_dna(genes));

        // Seed waste (uses diffuse_scalar_grid)
        vm.waste_grid[5][5] = 10000;
        vm.waste_grid[6][6] = 5000;

        // Run diffusion 100 times
        for _ in 0..100 {
            crate::vm::nova::diffuse_waste(&mut vm);
        }

        // Assert values spread but didn't explode or vanish
        assert!(
            vm.waste_grid[5][5] < 5000,
            "Should have diffused away from center"
        );

        let total_waste: i64 = vm.waste_grid.iter().flatten().sum();
        // Allow some loss due to integer division floor
        assert!(
            total_waste > 1000,
            "Mass conservation failure (too much loss)"
        );
    }
}
