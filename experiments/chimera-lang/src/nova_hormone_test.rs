#[cfg(feature = "nova")]
#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::vm::{ChimeraVM, Value};

    fn make_gene(name: &str, args: Vec<i64>) -> Gene {
        Gene {
            name: name.to_string(),
            args: args.into_iter().map(Nucleotide::Number).collect(),
        }
    }

    #[test]
    fn test_secrete_detect() {
        // Strand 0: push(1) [channel] push(100) [amount] secrete()
        // Strand 1: push(1) [channel] detect()
        let strand0 = Strand {
            genes: vec![
                make_gene("push", vec![1]),
                make_gene("push", vec![100]),
                make_gene("secrete", vec![]),
            ],
        };
        let strand1 = Strand {
            genes: vec![make_gene("push", vec![1]), make_gene("detect", vec![])],
        };

        let dna = Dna {
            helix: Helix {
                strands: vec![strand0, strand1],
            },
        };
        let mut vm = ChimeraVM::new(dna);

        // Step 1: push(1)
        vm.step();
        // Step 2: push(100)
        vm.step();
        // Step 3: secrete(). Grid[8][8][1] += 100.
        vm.step();

        // Check center (8,8) channel 1
        assert_eq!(vm.hormone_grid[8][8][1], 100);

        // Step 4: End of Strand 0. ip moves.
        // Diffuse happens. 100 spreads. Center drops.
        vm.step();

        assert_eq!(vm.ip, (1, 0));
        let center_val = vm.hormone_grid[8][8][1];
        assert!(center_val < 100);
        assert!(center_val > 0);

        // Step 5: push(1) (Strand 1)
        vm.step();

        // Step 6: detect()
        vm.step();

        let val = vm.stack.pop().unwrap();
        if let Value::Int(n) = val {
            assert!(n > 0);
            assert!(n < center_val); // Should have decayed/diffused more
        } else {
            panic!("Expected Int");
        }
    }

    #[test]
    fn test_absorb() {
        // Strand 0: push(1) push(50) secrete()
        // Strand 1: push(1) push(20) absorb()

        let strand0 = Strand {
            genes: vec![
                make_gene("push", vec![1]),
                make_gene("push", vec![50]),
                make_gene("secrete", vec![]),
            ],
        };

        let strand1 = Strand {
            genes: vec![
                make_gene("push", vec![1]),
                make_gene("push", vec![20]),
                make_gene("absorb", vec![]),
            ],
        };

        let dna = Dna {
            helix: Helix {
                strands: vec![strand0, strand1],
            },
        };
        let mut vm = ChimeraVM::new(dna);

        // Run Strand 0 (3 steps + 1 switch)
        vm.step();
        vm.step();
        vm.step(); // secrete 50

        let initial_val = vm.hormone_grid[8][8][1];
        assert_eq!(initial_val, 50);

        vm.step(); // Switch to Strand 1. Diffuse/Decay.

        // Run Strand 1: push(1), push(20)
        vm.step();
        vm.step();

        let val_before_absorb = vm.hormone_grid[8][8][1];
        assert!(val_before_absorb > 0);

        // absorb() - tries to take 20
        vm.step();

        let val = vm.stack.pop().unwrap(); // Amount absorbed
        if let Value::Int(n) = val {
            // It might not be 20 if diffusion reduced it below 20
            // But with 50 start, it should still have enough?
            // 50 -> 25 -> 12.5?
            // If it drops below 20, we absorb all of it.
            assert!(n > 0);
        } else {
            panic!("Expected Int");
        }

        let val_after = vm.hormone_grid[8][8][1];
        assert!(val_after < val_before_absorb);
    }

    #[test]
    fn test_diffusion() {
        // Secrete at center, check neighbor
        let strand0 = Strand {
            genes: vec![
                make_gene("push", vec![1]),
                make_gene("push", vec![100]),
                make_gene("secrete", vec![]),
                make_gene("push", vec![0]), // loop
                make_gene("jump", vec![3]),
            ],
        };
        let dna = Dna {
            helix: Helix {
                strands: vec![strand0],
            },
        };
        let mut vm = ChimeraVM::new(dna);

        // Secrete
        vm.step();
        vm.step();
        vm.step();

        assert_eq!(vm.hormone_grid[8][8][1], 100);
        assert_eq!(vm.hormone_grid[8][9][1], 0);

        // Step (Diffuse)
        vm.step();

        // Center should drop
        assert!(vm.hormone_grid[8][8][1] < 100);
        // Neighbor should gain
        assert!(vm.hormone_grid[8][9][1] > 0);
    }
}
