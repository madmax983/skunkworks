#[cfg(all(test, feature = "nova"))]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::{ChimeraVM, Value};

    #[test]
    fn test_reflex_low_energy() {
        // Strand 0: Register reflex, then drain energy using Spawn (Cost 20)
        let strand0 = Strand {
            genes: vec![
                // Reflex Setup
                Gene { op: OpCode::Push, args: vec![Nucleotide::Number(2)] }, // Event 2
                Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] }, // Strand 1
                Gene { op: OpCode::Reflex, args: vec![] },

                // Drain 1 (Cost 20 + overhead)
                Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] },
                Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] },
                Gene { op: OpCode::Spawn, args: vec![] },

                // Drain 2 (Cost 20 + overhead) -> Should Trigger Reflex here
                Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] },
                Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] },
                Gene { op: OpCode::Spawn, args: vec![] },

                // Marker
                Gene { op: OpCode::Push, args: vec![Nucleotide::Number(999)] },
            ],
        };

        // Strand 1: Emergency Photosynthesis
        let strand1 = Strand {
            genes: vec![
                Gene { op: OpCode::Photosynthesize, args: vec![] },
                Gene { op: OpCode::Ret, args: vec![] },
            ],
        };

        let dna = Dna {
            helix: Helix {
                strands: vec![strand0, strand1],
            },
        };
        let mut vm = ChimeraVM::new(dna);

        // Run enough steps to trigger reflex
        // 3 setup + 3 drain + 3 drain = 9 steps.
        // Energy analysis:
        // Start 50.
        // 3 setup: 47.
        // Drain 1 (3 steps): 47 - 3 - 20 = 24.
        // Drain 2 (3 steps): 24 - 3 - 20 = 1.
        // Energy 1 < 10. Reflex should trigger during the LAST Spawn or right after.

        let mut triggered = false;
        for _ in 0..20 {
            vm.step();
            if vm.output.iter().any(|s| s.contains("REFLEX: Triggered Event 2")) {
                triggered = true;
                break;
            }
            if vm.halted {
                break;
            }
        }

        assert!(triggered, "Reflex should have triggered");
        assert!(!vm.halted, "Organism should have survived");

        // Continue to verify return
        // We broke loop on trigger.
        // Current state: Just executed Photosynthesize (in same tick as trigger).
        // Next: Ret.
        vm.step(); // Ret
        vm.step(); // Push 999

        assert_eq!(vm.ip.0, 0, "Should have returned to strand 0");
        assert_eq!(vm.stack.last(), Some(&Value::Int(999)));
    }
}
