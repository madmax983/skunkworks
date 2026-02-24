#[cfg(feature = "nova")]
#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::vm::{ChimeraVM, Value};

    fn make_gene(name: &str, args: Vec<i64>) -> Gene {
        Gene {
            op: name.parse().unwrap(),
            args: args.into_iter().map(Nucleotide::Number).collect(),
        }
    }

    #[test]
    fn test_lumine_sense() {
        // Strand 0: push(3) [radius] push(100) [intensity] lumine()
        // Strand 1: sense_light()
        let strand0 = Strand {
            genes: vec![
                make_gene("push", vec![3]),
                make_gene("push", vec![100]),
                make_gene("lumine", vec![]),
            ],
        };
        let strand1 = Strand {
            genes: vec![make_gene("sense_light", vec![])],
        };

        let dna = Dna {
            evolution_config: None,
            helix: Helix {
                strands: vec![strand0, strand1],
            },
        };
        let mut vm = ChimeraVM::new(dna);

        // Step 1: push(3)
        vm.step();
        // Step 2: push(100)
        vm.step();
        // Step 3: lumine(). Grid[8][8] += 100. Grid[8][9] += 100...
        vm.step();

        // Check center (8,8)
        assert_eq!(vm.light_grid[8][8], 100);
        // Check neighbor (8,9) - within radius 3
        assert_eq!(vm.light_grid[8][9], 100);
        // Check far away (8,12) - outside radius 3
        assert_eq!(vm.light_grid[8][12], 0);

        // Step 4: End of Strand 0. ip moves.
        // Diffuse happens. Strong decay (50%).
        vm.step();

        // Center was 100.
        // After diffusion: (100*4 + neighbors...) / count / 2
        // It should be roughly 100 / 2 = 50.
        let center_val_before = vm.light_grid[8][8];
        assert!(center_val_before < 100);
        assert!(center_val_before > 0);

        // Step 5: sense_light() (Strand 1)
        // Note: step() calls diffuse_light() BEFORE executing the instruction.
        // So the light will decay again (approx halved) before being sensed.
        vm.step();

        let val = vm.stack.pop().unwrap();
        if let Value::Int(n) = val {
            // Verify against current grid state
            assert_eq!(n, vm.light_grid[8][8]);
            // It should be roughly half of previous
            assert!(n < center_val_before);
        } else {
            panic!("Expected Int");
        }
    }

    #[test]
    fn test_light_decay() {
        // [ push(1) push(1000) lumine() push(0) jump(3) ]
        let strand0 = Strand {
            genes: vec![
                make_gene("push", vec![1]),
                make_gene("push", vec![1000]),
                make_gene("lumine", vec![]),
                make_gene("push", vec![0]),
                make_gene("jump", vec![3]),
            ],
        };
        let dna = Dna {
            evolution_config: None,
            helix: Helix {
                strands: vec![strand0],
            },
        };
        let mut vm = ChimeraVM::new(dna);

        // lumine
        vm.step();
        vm.step();
        vm.step();

        assert_eq!(vm.light_grid[8][8], 1000);

        // Step (push 0) -> Diffuse runs
        vm.step();

        // Should decay significantly
        let val1 = vm.light_grid[8][8];
        assert!(val1 < 1000); // 1000 -> ~950 (with 95% transmission)

        // Step (jump 3) -> Diffuse runs
        vm.step();

        let val2 = vm.light_grid[8][8];
        assert!(val2 < val1);
    }
}
