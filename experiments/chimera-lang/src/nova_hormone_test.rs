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
        // Step 3: secrete(). Hormones[1] = 100.
        vm.step();

        assert_eq!(*vm.hormones.get(&1).unwrap(), 100);

        // Step 4: End of Strand 0. ip moves.
        // Wait, step() moves ip.
        // Let's trace carefully.
        // After secrete, ip is (0, 3). step() sees ip out of bounds of genes? No genes len is 3. ip is 3.
        // next step will move to (1, 0).
        vm.step(); // Moves to Strand 1. Energy -1. Hormone decay -1 -> 99.

        assert_eq!(vm.ip, (1, 0));
        assert_eq!(*vm.hormones.get(&1).unwrap(), 99);

        // Step 5: push(1) (Strand 1)
        vm.step(); // Hormone decay -> 98.

        // Step 6: detect()
        vm.step(); // Hormone decay -> 97.

        // Stack should have 98?
        // Logic: step() -> decay() -> execute().
        // So at step 6: decay (98->97) -> detect reads 97.

        let val = vm.stack.pop().unwrap();
        if let Value::Int(n) = val {
            assert_eq!(n, 97);
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
        vm.step();
        // Hormone = 50.
        vm.step(); // Switch to Strand 1. Decay -> 49.

        // Run Strand 1: push(1), push(20)
        vm.step(); // Decay -> 48
        vm.step(); // Decay -> 47

        // absorb()
        vm.step(); // Decay -> 46. Absorb reads 46. Takes 20. Remaining 26.

        let val = vm.stack.pop().unwrap(); // Amount absorbed
        if let Value::Int(n) = val {
            assert_eq!(n, 20);
        } else {
            panic!("Expected Int");
        }

        assert_eq!(*vm.hormones.get(&1).unwrap(), 26);
    }
}
