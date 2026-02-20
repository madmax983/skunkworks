use synaptic_physics::Izhikevich;

#[derive(Clone, Debug)]
pub struct Synapse {
    pub from: usize,
    pub to: usize,
    pub weight: f32,
    pub delay: usize,
    pub spikes_in_transit: Vec<usize>,
    pub active: bool, // For visualization (did it fire this step?)
}

#[derive(Clone, Debug)]
pub struct Network {
    pub neurons: Vec<Izhikevich>,
    pub synapses: Vec<Synapse>,
    pub spikes: Vec<bool>,
}

impl Network {
    pub fn new() -> Self {
        Self {
            neurons: Vec::new(),
            synapses: Vec::new(),
            spikes: Vec::new(),
        }
    }

    pub fn add_neuron(&mut self) -> usize {
        self.neurons.push(Izhikevich::new());
        self.spikes.push(false);
        self.neurons.len() - 1
    }

    pub fn add_synapse(&mut self, from: usize, to: usize, weight: f32) {
        self.add_synapse_with_delay(from, to, weight, 0);
    }

    pub fn add_synapse_with_delay(&mut self, from: usize, to: usize, weight: f32, delay: usize) {
        self.synapses.push(Synapse {
            from,
            to,
            weight,
            delay,
            spikes_in_transit: Vec::new(),
            active: false,
        });
    }

    pub fn step(&mut self, external_inputs: &[f32]) {
        // 1. Collect inputs for this step
        let mut inputs = vec![0.0; self.neurons.len()];

        // Add external inputs
        for (i, val) in external_inputs.iter().enumerate() {
            if i < inputs.len() {
                inputs[i] += val;
            }
        }

        // 2. Process Synapses (Propagate spikes)
        for syn in &mut self.synapses {
            syn.active = false;

            // Check if source neuron spiked in previous step
            if let Some(&spiked) = self.spikes.get(syn.from) {
                if spiked {
                    syn.spikes_in_transit.push(syn.delay);
                }
            }

            // Advance spikes in transit
            let mut weight_to_add = 0.0;
            syn.spikes_in_transit.retain_mut(|t| {
                if *t == 0 {
                    weight_to_add += syn.weight;
                    false // Remove from queue
                } else {
                    *t -= 1;
                    true // Keep in queue
                }
            });

            if weight_to_add != 0.0 {
                if let Some(input) = inputs.get_mut(syn.to) {
                    *input += weight_to_add;
                    syn.active = true;
                }
            }
        }

        // 3. Update Neurons
        // We assume 1.0ms time step to match previous local implementation behavior
        let dt = 1.0;

        for (i, neuron) in self.neurons.iter_mut().enumerate() {
            let (_, spiked) = neuron.update(dt, inputs[i]);
            self.spikes[i] = spiked;
        }
    }

    pub fn is_spiking(&self, index: usize) -> bool {
        self.spikes.get(index).cloned().unwrap_or(false)
    }

    pub fn get_synapse_activity(&self, index: usize) -> bool {
        self.synapses.get(index).map(|s| s.active).unwrap_or(false)
    }
}

impl Default for Network {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_synapse_is_ignored() {
        let mut network = Network::new();
        let a = network.add_neuron();
        // Add synapse pointing to invalid neuron index 99
        network.add_synapse(a, 99, 1.0);

        // Force a spike
        network.neurons[a].v = 40.0;

        // First step: neuron spikes, `spikes[a]` becomes true
        network.step(&[]);

        // Second step: synapse sees spike (from prev step), pushes to transit (delay 0).
        // Since delay is 0, it processes immediately in THIS step (step 2).
        // It tries to add weight to neuron 99, but ignores it safely.
        network.step(&[]);

        // If we reached here without panic, the test passes.
    }

    #[test]
    fn test_inhibition() {
        let mut network = Network::new();
        let a = network.add_neuron();
        let b = network.add_neuron();

        // Add inhibitory synapse A -> B
        network.add_synapse(a, b, -10.0);

        // Set A to spike threshold
        network.neurons[a].v = 35.0;

        // Step 1: A spikes. Spike is recorded in `self.spikes`.
        // B receives 0 input yet because synapse reads `self.spikes` from *start* of step 1 (which was empty/false).
        network.step(&[]);
        assert!(network.spikes[a], "A should have spiked");

        let v_before_impact = network.neurons[b].v;

        // Step 2: Synapse reads `self.spikes` (true). Pushes delay 0.
        // Processes delay 0 -> adds weight to B's input.
        // B updates with negative input.
        network.step(&[]);

        let v_after = network.neurons[b].v;

        // B should be inhibited
        assert!(
            v_after < v_before_impact - 0.1,
            "Post-synaptic neuron should be inhibited by spike (Before: {}, After: {})",
            v_before_impact, v_after
        );
    }

    #[test]
    fn test_delay() {
        let mut network = Network::new();
        let a = network.add_neuron();
        let b = network.add_neuron();

        // Add synapse with delay 2
        network.add_synapse_with_delay(a, b, 10.0, 2);

        // Force spike
        network.neurons[a].v = 35.0;

        // Step 1: A spikes.
        network.step(&[]);
        assert!(network.spikes[a]);

        // Step 2: Synapse sees spike. Pushes delay 2.
        // Transit: [2]. Decrement? No, wait.
        // Logic: Push 2. Then retain_mut:
        // If *t == 0 -> fire. Else *t -= 1.
        // So 2 becomes 1.
        network.step(&[]);

        // Step 3: Transit: [1]. Decrement -> 0.
        network.step(&[]);

        // Step 4: Transit: [0]. Fire! Remove.
        let _v_start = network.neurons[b].v;
        network.step(&[]);
        let _v_end = network.neurons[b].v;

        // Wait, let's trace carefully.
        // Step 1: A spikes (recorded in `spikes`).
        // Step 2: Synapse reads `spikes[a]`. Pushes 2.
        //         Loop over transit: 2 -> 1.
        //         No fire.
        // Step 3: Synapse reads `spikes[a]` (false).
        //         Loop over transit: 1 -> 0.
        //         No fire.
        // Step 4: Synapse reads `spikes[a]` (false).
        //         Loop over transit: 0 -> fire!
        //         Add weight to B. B updates.

        // So delay 2 means:
        // T=1 (Spike)
        // T=2 (Transit 1)
        // T=3 (Transit 0)
        // T=4 (Impact)

        // Delay 0:
        // T=1 (Spike)
        // T=2 (Transit 0 -> Impact immediately)
        // This matches "next step" behavior (since T=1 spike affects T=2 input).

        // So delay 2 adds 2 EXTRA steps of delay beyond the implicit 1-step delay.
        // Effectively 3 steps total from spike to impact.

        // Let's verify Step 4 had impact.
        // In Step 4, `v_end` should be higher than `v_start` (if no decay/drift masking it).
        // Actually, let's just check `active`.
        assert!(network.get_synapse_activity(0));
    }
}
