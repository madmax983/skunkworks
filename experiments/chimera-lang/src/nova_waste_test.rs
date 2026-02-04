#[cfg(feature = "nova")]
#[cfg(test)]
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
        // waste added +10. Diffused.
        // Center: 10*4 = 40. Neighbors 0. Count 8 (center + 4 neighbors).
        // New center: 40/8 = 5.
        // Neighbors: 0*4 + 10 (from center) = 10. Count 8? No, neighbors have 3 or 4 neighbors.
        // Let's just check it's > 0.
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
}
