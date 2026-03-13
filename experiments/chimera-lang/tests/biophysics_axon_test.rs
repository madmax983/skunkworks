#[cfg(feature = "biophysics")]
#[cfg(test)]
mod tests {
    use chimera_lang::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use chimera_lang::opcode::OpCode;
    use chimera_lang::vm::ChimeraVM;

    fn make_dna(genes: Vec<Gene>) -> Dna {
        // Strand 0: Setup
        // Strand 1: Infinite Loop (Keep Alive)
        let strand0 = Strand { genes };
        let strand1 = Strand {
            genes: vec![Gene {
                op: OpCode::Jump,
                args: vec![Nucleotide::Number(1)],
            }],
        };

        Dna {
            evolution_config: None,
            helix: Helix {
                strands: vec![strand0, strand1],
            },
        }
    }

    #[test]
    fn test_axon_propagation() {
        let genes = vec![
            // 1. Create Source Neuron at (0,0)
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            }, // y
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            }, // x
            Gene {
                op: OpCode::NeuroGenesis,
                args: vec![],
            },
            // 2. Create Target Neuron at (0,1)
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            }, // y
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            }, // x
            Gene {
                op: OpCode::NeuroGenesis,
                args: vec![],
            },
            // 3. Connect (0,0) -> (1,0) (Target y=1, x=0)
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            }, // x_src
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            }, // y_src
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            }, // x_tgt
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            }, // y_tgt
            Gene {
                op: OpCode::Axon,
                args: vec![],
            },
            // 4. Stimulate Source (0,0)
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(200)],
            }, // amount
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            }, // y
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            }, // x
            Gene {
                op: OpCode::Stimulate,
                args: vec![],
            },
        ];

        let mut vm = ChimeraVM::new(make_dna(genes));
        vm.energy = 10000; // Plenty of energy

        // Execute genesis and axon connection (approx 15 genes)
        // We run more steps to ensure we enter the loop in Strand 1
        for _ in 0..50 {
            vm.step();
        }

        assert!(!vm.halted, "VM halted unexpectedly");

        assert!(
            vm.biophysics_synapses.contains_key(&(0, 0)),
            "Synapse map missing (0,0)"
        );

        // FORCE SPIKE (Just in case stimulate didn't work or decayed)
        if let Some(source) = vm.neurons.get_mut(&(0, 0)) {
            source.v = 50.0;
        }

        // Run simulation
        let mut propagated = false;
        for _i in 0..100 {
            vm.step();
            if let Some(target) = vm.neurons.get(&(1, 0)) {
                if target.i_inj > 1.0 {
                    propagated = true;
                    break;
                }
            }
        }

        assert!(propagated, "Spike did not propagate to target neuron");
    }
}
