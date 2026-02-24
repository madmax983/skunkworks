#[cfg(all(feature = "biophysics", feature = "elektra", feature = "cortex"))]
mod tests {
    use chimera_lang::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use chimera_lang::opcode::OpCode;
    use chimera_lang::vm::ChimeraVM;

    #[test]
    fn test_synaptic_bridge() {
        let strand0 = Strand {
            genes: vec![
                // 1. Create Neuron at (5, 5)
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(5)],
                },
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(5)],
                },
                Gene {
                    op: OpCode::NeuroGenesis,
                    args: vec![],
                },
                // 2. Couple to Grid (Weight 1 = 0.01)
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(1)],
                }, // Weight
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(5)],
                }, // y
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(5)],
                }, // x
                Gene {
                    op: OpCode::NeuroCoupling,
                    args: vec![],
                },
                // 3. Synapse to Strand 1
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(1)],
                }, // strand_idx
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(5)],
                }, // y
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(5)],
                }, // x
                Gene {
                    op: OpCode::NeuroSynapse,
                    args: vec![],
                },
                // 4. Inject Voltage into Grid at (5, 5) via Battery (Low voltage for stability)
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(10)],
                }, // Voltage
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(5)],
                }, // y
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(5)],
                }, // x
                Gene {
                    op: OpCode::Battery,
                    args: vec![],
                },
            ],
        };

        // Strand 1: Loop to keep VM alive
        let strand1 = Strand {
            genes: vec![
                Gene {
                    op: OpCode::Nop,
                    args: vec![],
                },
                Gene {
                    op: OpCode::Jump,
                    args: vec![Nucleotide::Number(1)],
                },
            ],
        };

        let dna = Dna {
            evolution_config: None,
            helix: Helix {
                strands: vec![strand0, strand1],
            },
        };
        let mut vm = ChimeraVM::new(dna);

        // Boost Energy to survive test duration
        vm.energy = 1000;

        // Step 1: Execute setup genes
        for _ in 0..20 {
            vm.step();
        }

        // Verify setup
        assert!(vm.neurons.contains_key(&(5, 5)), "Neuron not created");
        assert!(
            vm.biophysics_couplings.contains_key(&(5, 5)),
            "Coupling not set"
        );
        assert!(
            vm.neuron_to_cortex_map.contains_key(&(5, 5)),
            "Synapse not created"
        );

        // Run simulation
        for _ in 0..50 {
            vm.step();
        }

        // Check Coupling (Grid -> Neuron)
        let n = vm.neurons.get(&(5, 5)).unwrap();
        // Resting is -65.0. If coupled, it should move.
        assert!(
            n.v != -65.0,
            "Coupling failed: Neuron potential did not change."
        );

        // Force Spike to verify Synapse (Neuron -> Cortex)
        if let Some(n) = vm.neurons.get_mut(&(5, 5)) {
            // Reset state to ensure spike (avoid depolarization block issues)
            n.v = 50.0;
            n.n = 0.32;
            n.m = 0.05;
            n.h = 0.6;
            n.last_spike = 0;
        }

        // Step to trigger spike detection
        vm.step();

        let activation = vm.activation_levels[1];
        println!("Activation Level after Forced Spike: {}", activation);
        assert!(
            activation > 0,
            "Synapse failed: Cortex strand not activated after spike."
        );
    }
}
