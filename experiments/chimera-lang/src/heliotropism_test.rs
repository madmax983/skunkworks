#[cfg(feature = "nova")]
#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::vm::ChimeraVM;

    fn make_gene(name: &str, args: Vec<i64>) -> Gene {
        Gene {
            op: name.parse().unwrap(),
            args: args.into_iter().map(Nucleotide::Number).collect(),
        }
    }

    #[test]
    fn test_photosynthesis_bonus() {
        // Strand 0: push(1) push(100) lumine() photosynthesize()
        // Stack: Bottom [1] Top [100] -> Lumine pops 100 (intensity), 1 (radius)
        let strand0 = Strand {
            genes: vec![
                make_gene("push", vec![1]),   // radius
                make_gene("push", vec![100]), // intensity
                make_gene("lumine", vec![]),
                make_gene("photosynthesize", vec![]),
            ],
        };

        let dna = Dna {
            helix: Helix {
                strands: vec![strand0],
            },
        };
        let mut vm = ChimeraVM::new(dna);

        // Initial energy 50.
        // Step 1: push 100.
        vm.step();
        // Step 2: push 1.
        vm.step();
        // Step 3: lumine. Cost ~2.
        vm.step();

        // Grid should have 100 light at 8,8.
        assert_eq!(vm.light_grid[8][8], 100);

        // Step 4: photosynthesize.
        // Before change: Gain 5. Net change small.
        // After change: Gain 5 + (100/10) = 15.
        // vm.energy should be noticeably higher.
        let energy_before = vm.energy;
        vm.step();
        let energy_after = vm.energy;

        // Step cost is 1.
        // If gain is 5, diff is +4.
        // If gain is 15, diff is +14.
        let gain = energy_after - energy_before + 1; // +1 to offset step cost

        assert!(
            gain > 5,
            "Photosynthesis gain {} should be > 5 given 100 light",
            gain
        );
    }

    #[test]
    fn test_phototaxis() {
        // Setup: Place light at (9,8) (one step down).
        // Use phototaxis() to move context_loc from (8,8) to (9,8).

        let strand0 = Strand {
            genes: vec![make_gene("phototaxis", vec![])],
        };

        let dna = Dna {
            helix: Helix {
                strands: vec![strand0],
            },
        };
        let mut vm = ChimeraVM::new(dna);

        // Manually inject light
        vm.light_grid[9][8] = 100; // (y,x) = (9,8) -> Down
        vm.context_loc = (8, 8);

        // Step: phototaxis
        vm.step();

        // Should have moved to (9,8)
        assert_eq!(vm.context_loc, (9, 8));
    }
}
