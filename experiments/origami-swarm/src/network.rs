use crate::neuron::IzhikevichNeuron;

#[derive(Clone, Copy)]
pub struct Synapse {
    pub from: usize,
    pub to: usize,
    pub weight: f32,
    pub delay: usize,
}

#[derive(Clone)]
pub struct Network {
    pub neurons: Vec<IzhikevichNeuron>,
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
        self.neurons.push(IzhikevichNeuron::new());
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
        for (i, neuron) in self.neurons.iter_mut().enumerate() {
            let spiked = neuron.update(inputs[i]);
            self.spikes[i] = spiked;
        }
    }

    pub fn is_spiking(&self, index: usize) -> bool {
        self.spikes.get(index).cloned().unwrap_or(false)
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
            v_after < v_before_impact - 1.0,
            "Post-synaptic neuron should be inhibited by spike"
        );
    }
}
