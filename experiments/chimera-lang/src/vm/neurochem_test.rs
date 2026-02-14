#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::{ChimeraVM, Value};

    fn make_dna(genes: Vec<Gene>) -> Dna {
        Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        }
    }

    #[test]
    fn test_receptor_stimulation() {
        // [ push(5) push(5) neuro_genesis() ]
        // [ push(5) push(5) push(10) push(10) push(0) receptor() ]  (chan 0, sens 1.0, thresh 10)
        // [ push(20) push(0) secrete() ] (20 into chan 0)
        // [ push(5) push(5) dendrite() ]
        let genes = vec![
            // 1. Create Neuron at 5,5 (Stack: y, x)
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(5)],
            }, // y
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(5)],
            }, // x
            Gene {
                op: OpCode::NeuroGenesis,
                args: vec![],
            },
            // 2. Add Receptor
            // Stack expected: [ ..., channel, sensitivity, threshold, y, x ] (Top is x)
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            }, // channel
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(10)],
            }, // sensitivity (10 = 1.0)
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(10)],
            }, // threshold
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(5)],
            }, // y
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(5)],
            }, // x
            Gene {
                op: OpCode::Receptor,
                args: vec![],
            },
            // 3. Secrete Hormone
            // Stack expected: [ ..., channel, amount ] (Top is amount)
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            }, // channel
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(100)],
            }, // amount
            Gene {
                op: OpCode::Secrete,
                args: vec![],
            },
        ];

        let mut vm = ChimeraVM::new(make_dna(genes));
        // Need coordinates to be at 5,5 for Secrete to work there (it uses context_loc)
        vm.context_loc = (5, 5);

        // Run setup genes
        // We have 12 genes in the strand.
        for _ in 0..12 {
            vm.step();
        }

        // Verify Hormone Level (Should be 100 before diffusion runs in next step)
        // Note: Secrete happens at step 12. process_environment runs at start of step 12, BEFORE Secrete.
        // So grid is 100 now.
        assert_eq!(vm.hormone_grid[5][5][0], 100, "Hormone should be secreted");

        // Verify Receptor
        if let Some(neuron) = vm.neurons.get(&(5, 5)) {
            assert_eq!(neuron.receptors[0], (1.0, 10.0), "Receptor should be set");
        } else {
            panic!("Neuron not found");
        }

        // Run one step to trigger process_signals (which runs per tick)
        // vm.step() calls process_signals if orca_mode is true (default).
        // It also calls neuron.step().

        vm.step(); // This should trigger the receptor and inject current

        if let Some(neuron) = vm.neurons.get(&(5, 5)) {
            // i_inj should be increased.
            // Amount = 20 * 1.0 = 20.0.
            // i_inj decays by 0.99 per step, but we just injected it.
            // process_signals adds to ctx.neuron_stimuli.
            // process_environment -> process_signals.
            // then vm applies neuron_stimuli to i_inj.

            assert!(
                neuron.i_inj > 10.0,
                "Neuron should be stimulated by hormone. i_inj: {}",
                neuron.i_inj
            );
        } else {
            panic!("Neuron missing");
        }
    }
}
