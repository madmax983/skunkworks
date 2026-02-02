use crate::neuron::Neuron;

pub struct Network {
    pub neurons: Vec<Neuron>,
    // adjacency[target] = vec![(source, weight)]
    pub synapses: Vec<Vec<(usize, f32)>>,
}

impl Network {
    pub fn new() -> Self {
        Self {
            neurons: Vec::new(),
            synapses: Vec::new(),
        }
    }

    pub fn add_neuron(&mut self, neuron: Neuron) -> usize {
        let id = self.neurons.len();
        self.neurons.push(neuron);
        self.synapses.push(Vec::new());
        id
    }

    pub fn add_synapse(&mut self, source: usize, target: usize, weight: f32) {
        if target < self.synapses.len() {
            self.synapses[target].push((source, weight));
        }
    }

    pub fn update(&mut self, external_currents: &[f32], dt: f32) -> Vec<bool> {
        let n = self.neurons.len();
        // Capture spikes from current state BEFORE update?
        // No, Izhikevich model updates state, then checks if v >= 30.
        // So we need to know who spiked *in the previous step* to calculate current for *this* step?
        // Usually, synaptic delay is at least one step.
        // So yes, we need to know who spiked in the PREVIOUS step.
        // But `self.neurons[i].spiked` stores the result of the LAST update.

        let mut inputs = vec![0.0; n];

        // Calculate synaptic inputs based on who spiked last frame
        for (target, sources) in self.synapses.iter().enumerate() {
            for &(source, weight) in sources {
                if self.neurons[source].spiked {
                    inputs[target] += weight;
                }
            }
        }

        // Add external currents
        for (i, &current) in external_currents.iter().enumerate() {
            if i < n {
                inputs[i] += current;
            }
        }

        // Update all neurons
        let mut spikes = Vec::with_capacity(n);
        for (i, neuron) in self.neurons.iter_mut().enumerate() {
            neuron.update(inputs[i], dt);
            spikes.push(neuron.spiked);
        }

        spikes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_synapse() {
        let mut net = Network::new();
        let n1 = net.add_neuron(Neuron::regular_spiking()); // Source
        let n2 = net.add_neuron(Neuron::regular_spiking()); // Target

        // Strong synapse
        net.add_synapse(n1, n2, 100.0);

        // Force n1 to have spiked in the "previous" state effectively
        // Actually, we can just run the simulation.
        // Stimulate n1.

        let mut n2_spiked = false;

        for _ in 0..100 {
            // Input only to n1
            let currents = vec![20.0, 0.0];
            let spikes = net.update(&currents, 1.0);

            if spikes[1] {
                n2_spiked = true;
                break;
            }
        }

        assert!(
            n2_spiked,
            "Target neuron should spike due to synaptic input"
        );
    }
}
