#[cfg(feature = "nova")]
#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Helix};
    #[cfg(feature = "biophysics")]
    use crate::vm::neuron::Neuron;
    use crate::vm::nova_signals::process_signals;
    use crate::vm::{ChimeraVM, Value};

    fn make_vm() -> ChimeraVM {
        let dna = Dna { evolution_config: None,
            helix: Helix { strands: vec![] },
        };
        ChimeraVM::new(dna)
    }

    #[test]
    fn test_synapse_transmission() {
        let mut vm = make_vm();
        // Layout:
        // . @ .  (Neuron at 0,1)
        // * ^ 1  (Synapse at 1,1, Input from South 2,1 is '1')
        // . 1 .

        vm.grid[0][1] = Value::Str("@".to_string());
        vm.grid[1][0] = Value::Str("*".to_string()); // Signal Source (Bang) to trigger synapse
        vm.grid[1][1] = Value::Str("^".to_string()); // Synapse
        vm.grid[2][1] = Value::Str("1".to_string()); // Input Value

        // Initial signal at (1,0) to trigger Synapse at (1,1)
        // Note: process_signals scans (1,0), sees *, adds signal to neighbors ((1,1) Synapse) in NEXT step?
        // No, process_signals logic:
        // Iterate grid. If * at (1,0) [signal or char], it adds signal to neighbors in next_signals.
        // But we want Synapse to fire NOW if it receives signal NOW.
        // "active = signal > 0 || is_uppercase || is_bang || is_special"
        // If we set signal_grid[1][1] = 1, then Synapse is active.

        vm.signal_grid[1][1] = 1;

        process_signals(&mut vm);

        // Check neuron state
        if let Some(neuron) = vm.neurons.get(&(0, 1)) {
            // Input was '1' -> weight 1.0 * 5.0 = 5.0
            assert!(neuron.i_inj > 0.0, "Neuron should be stimulated");
            assert_eq!(neuron.i_inj, 5.0, "Expected injection of 5.0");
        } else {
            panic!("Neuron not created via NeuroGenesis");
        }
    }

    #[test]
    fn test_neuron_spike_propagation() {
        let mut vm = make_vm();
        // Layout:
        // . . .
        // . @ .
        // . . .

        vm.grid[1][1] = Value::Str("@".to_string());

        // Manually create and charge neuron
        let mut neuron = Neuron::new();
        neuron.v = 10.0; // Spike!
        vm.neurons.insert((1, 1), neuron);

        process_signals(&mut vm);

        // Check neighbors for signal
        assert_eq!(
            vm.signal_grid[0][1], 1,
            "North neighbor should receive signal"
        );
        assert_eq!(
            vm.signal_grid[2][1], 1,
            "South neighbor should receive signal"
        );
        assert_eq!(
            vm.signal_grid[1][0], 1,
            "West neighbor should receive signal"
        );
        assert_eq!(
            vm.signal_grid[1][2], 1,
            "East neighbor should receive signal"
        );
    }
}
