#[cfg(feature = "nova")]
#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::{ChimeraVM, Value};

    fn make_gene(name: &str, args: Vec<i64>) -> Gene {
        Gene {
            op: name.parse().unwrap(),
            args: args.into_iter().map(Nucleotide::Number).collect(),
        }
    }

    #[test]
    fn test_collision_reflex() {
        // Strand 0: push(1) [strand_idx] push(0) [code] reflex() push(0) push(-1) migrate()
        // Strand 1: push(999) [marker]

        let strand0 = Strand {
            genes: vec![
                make_gene("push", vec![0]), // Code
                make_gene("push", vec![1]), // Strand
                make_gene("reflex", vec![]),
                make_gene("push", vec![0]),  // dy
                make_gene("push", vec![-1]), // dx (West)
                make_gene("migrate", vec![]),
            ],
        };
        let strand1 = Strand {
            genes: vec![make_gene("push", vec![999])],
        };

        let dna = Dna {
            helix: Helix {
                strands: vec![strand0, strand1],
            },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.context_loc = (0, 0); // At top-left
        vm.membranes[0][0] = 8; // Block West

        // Step 1: push(1)
        vm.step();
        // Step 2: push(0)
        vm.step();
        // Step 3: reflex()
        vm.step();
        assert!(vm.reflexes.contains_key(&0));

        // Step 4: push(0)
        vm.step();
        // Step 5: push(-1)
        vm.step();

        // Step 6: migrate() -> blocked -> trigger reflex -> jump to Strand 1
        vm.step();

        // Should be at Strand 1, Gene 0
        assert_eq!(vm.ip, (1, 0));

        // Step 7: push(999)
        vm.step();

        let val = vm.stack.pop().unwrap();
        assert_eq!(val, Value::Int(999));

        // Check reflex was consumed (One-Shot)
        assert!(!vm.reflexes.contains_key(&0));
    }

    #[test]
    fn test_metabolism_reflex() {
        // Strand 0: push(2) push(1) reflex()
        // Strand 1: push(888)
        let strand0 = Strand {
            genes: vec![
                make_gene("push", vec![2]),
                make_gene("push", vec![1]),
                make_gene("reflex", vec![]),
            ],
        };
        let strand1 = Strand {
            genes: vec![make_gene("push", vec![888])],
        };

        let dna = Dna {
            helix: Helix {
                strands: vec![strand0, strand1],
            },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.energy = 15;

        // Register
        vm.step();
        vm.step();
        vm.step();
        assert!(vm.reflexes.contains_key(&2));

        // Drop energy
        vm.energy = 9;

        // Step (should trigger reflex check before executing next gene)
        // Strand 0 has no more genes, so normally it would increment IP or halt.
        // But trigger_reflex happens in `step`.

        vm.step();

        // Should be at Strand 1
        assert_eq!(vm.ip, (1, 0));
        assert!(!vm.reflexes.contains_key(&2));
    }

    #[test]
    fn test_signal_reflex() {
        // Strand 0: push(105) push(1) reflex() push(5) push(77) broadcast()
        // Strand 1: push(555)
        // Code 105 corresponds to Channel 5.

        let strand0 = Strand {
            genes: vec![
                make_gene("push", vec![105]),
                make_gene("push", vec![1]),
                make_gene("reflex", vec![]),
                make_gene("push", vec![5]),
                make_gene("push", vec![77]),
                make_gene("broadcast", vec![]),
            ],
        };
        let strand1 = Strand {
            genes: vec![make_gene("push", vec![555])],
        };

        let dna = Dna {
            helix: Helix {
                strands: vec![strand0, strand1],
            },
        };
        let mut vm = ChimeraVM::new(dna);

        // Register
        vm.step();
        vm.step();
        vm.step();

        // Prepare broadcast
        vm.step(); // push 5
        vm.step(); // push 77

        // Broadcast
        vm.step();

        // Should trigger reflex
        assert_eq!(vm.ip, (1, 0));
        assert!(!vm.reflexes.contains_key(&105));
    }
}
