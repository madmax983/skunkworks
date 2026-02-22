#[cfg(feature = "nova")]
#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::{ChimeraVM, Value};

    fn make_vm() -> ChimeraVM {
        let dna = Dna { evolution_config: None,
            helix: Helix {
                strands: vec![
                    // Strand 0: Inscribe "TestSigil" radius 1 calling Strand 1
                    Strand {
                        genes: vec![
                            // Stack order needed: [strand, radius, name] (Top)
                            // So push strand, then radius, then name.
                            Gene {
                                op: OpCode::Push,
                                args: vec![Nucleotide::Number(1)],
                            }, // Target Strand 1
                            Gene {
                                op: OpCode::Push,
                                args: vec![Nucleotide::Number(1)],
                            }, // Radius
                            Gene {
                                op: OpCode::Push,
                                args: vec![Nucleotide::String("TestSigil".to_string())],
                            }, // Name
                            Gene {
                                op: OpCode::Inscribe,
                                args: vec![],
                            },
                        ],
                    },
                    // Strand 1: Effect (Push 999)
                    Strand {
                        genes: vec![Gene {
                            op: OpCode::Push,
                            args: vec![Nucleotide::Number(999)],
                        }],
                    },
                    // Strand 2: Invoke "TestSigil"
                    Strand {
                        genes: vec![
                            Gene {
                                op: OpCode::Push,
                                args: vec![Nucleotide::String("TestSigil".to_string())],
                            },
                            Gene {
                                op: OpCode::Invoke,
                                args: vec![],
                            },
                        ],
                    },
                ],
            },
        };
        ChimeraVM::new(dna)
    }

    #[test]
    fn test_dynamic_sigil_lifecycle() {
        let mut vm = make_vm();
        vm.metamorphism_enabled = false;
        vm.energy = 1000; // Plenty of energy

        // Setup Grid Pattern for Inscription
        // Center at 8,8. Radius 1 covers 7,8 9,8 8,7 8,9 and corners?
        // get_circular_coords(1) -> checks distance <= 1.
        // (0,0) dist 0. (0,1) dist 1. (1,1) dist 2.
        // So cross shape: Center + 4 neighbors.

        // Let's set (8,9) [East] to 42.
        vm.grid[8][9] = Value::Int(42);

        // Step 1: Inscribe (Strand 0)
        // Stack args: Strand 1, Radius 1, Name "TestSigil"
        vm.step(); // Push 1 (Strand)
        vm.step(); // Push 1 (Radius)
        vm.step(); // Push "TestSigil" (Name)
        vm.step(); // Inscribe

        // Check Registry
        if !vm.sigil_registry.contains_key("TestSigil") {
            // Print output to debug if failed
            println!("VM Output: {:?}", vm.output);
        }
        assert!(
            vm.sigil_registry.contains_key("TestSigil"),
            "Sigil not registered"
        );

        let sigil = vm.sigil_registry.get("TestSigil").unwrap();
        assert_eq!(sigil.strand_idx, 1);

        // Depending on iteration order, verify content.
        assert!(!sigil.pattern.is_empty());
        let found = sigil
            .pattern
            .iter()
            .any(|(dy, dx, val)| *dy == 0 && *dx == 1 && *val == Value::Int(42));
        assert!(found, "Pattern not captured correctly: {:?}", sigil.pattern);

        // Step 2: Invoke (Strand 2)
        // Move IP to Strand 2
        vm.ip = (2, 0);

        // Invoke should succeed because grid is still set
        vm.step(); // Push "TestSigil"
        vm.step(); // Invoke

        // Check Effect: Invoke calls Strand 1.
        // Strand 1 pushes 999.
        // But Invoke creates a CALL stack frame.
        // So IP should be (1, 0).
        assert_eq!(vm.ip, (1, 0));

        // Step 3: Execute Effect
        vm.step(); // Push 999
        assert_eq!(vm.stack.last(), Some(&Value::Int(999)));

        // Check Consumption: Pattern cells should be 0.
        assert_eq!(vm.grid[8][9], Value::Int(0));
    }

    #[test]
    fn test_sigil_mismatch() {
        let mut vm = make_vm();
        vm.energy = 1000;

        // Setup Grid
        vm.grid[8][9] = Value::Int(42);

        // Inscribe
        vm.step();
        vm.step();
        vm.step();
        vm.step();

        // Change Grid (Break Pattern)
        vm.grid[8][9] = Value::Int(0);

        // Invoke (Strand 2)
        vm.ip = (2, 0);
        vm.step(); // Push
        vm.step(); // Invoke

        // Check that IP is NOT (1, 0)
        assert_ne!(vm.ip, (1, 0));
    }
}
