use synaptic_physics::Izhikevich;

#[derive(Clone, Copy, Debug)]
pub struct Synapse {
    pub from: usize,
    pub to: usize,
    pub weight: f32,
    pub delay: usize,
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
        self.synapses.push(Synapse {
            from,
            to,
            weight,
            delay: 0,
        });
    }

    pub fn step(&mut self, external_inputs: &[f32]) {
        // Collect synaptic inputs from previous step's spikes
        let mut inputs = vec![0.0; self.neurons.len()];

        // Add external inputs
        for (i, val) in external_inputs.iter().enumerate() {
            if i < inputs.len() {
                inputs[i] += val;
            }
        }

        for syn in &self.synapses {
            if self.spikes[syn.from] {
                inputs[syn.to] += syn.weight;
            }
        }

        // Update neurons
        // We assume 1.0ms time step to match previous local implementation behavior
        // (which ran 2 substeps of 0.5ms = 1.0ms total)
        let dt = 1.0;

        for (i, neuron) in self.neurons.iter_mut().enumerate() {
            let (_, spiked) = neuron.update(dt, inputs[i]);
            self.spikes[i] = spiked;
        }
    }

    pub fn is_spiking(&self, index: usize) -> bool {
        self.spikes.get(index).cloned().unwrap_or(false)
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
    fn test_inhibition() {
        let mut network = Network::new();
        let a = network.add_neuron();
        let b = network.add_neuron();

        // Add inhibitory synapse A -> B
        network.add_synapse(a, b, -10.0);

        // Set A to spike threshold
        network.neurons[a].v = 35.0;

        // Step 1: A spikes. Spike is recorded. B receives 0 input.
        network.step(&[]);
        assert!(network.spikes[a], "A should have spiked");

        let v_before_impact = network.neurons[b].v;

        // Step 2: A's spike propagates to B.
        network.step(&[]);

        let v_after = network.neurons[b].v;

        // B should be inhibited
        assert!(
            v_after < v_before_impact - 0.1,
            "Post-synaptic neuron should be inhibited by spike (Before: {}, After: {})",
            v_before_impact, v_after
        );
    }
}
