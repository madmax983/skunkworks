use crate::neuron::IzhikevichNeuron;

pub struct Synapse {
    pub pre: usize,
    pub post: usize,
    pub weight: f64,
}

pub struct Network {
    pub neurons: Vec<IzhikevichNeuron>,
    pub synapses: Vec<Synapse>,
    pub last_spikes: Vec<bool>,
}

impl Network {
    pub fn new() -> Self {
        Self {
            neurons: Vec::new(),
            synapses: Vec::new(),
            last_spikes: Vec::new(),
        }
    }

    pub fn add_neuron(&mut self, neuron: IzhikevichNeuron) -> usize {
        let id = self.neurons.len();
        self.neurons.push(neuron);
        self.last_spikes.push(false);
        id
    }

    pub fn add_synapse(&mut self, pre: usize, post: usize, weight: f64) {
        self.synapses.push(Synapse { pre, post, weight });
    }

    pub fn step(&mut self, inputs: &[f64]) -> Vec<bool> {
        let mut currents = inputs.to_vec();
        if currents.len() < self.neurons.len() {
            currents.resize(self.neurons.len(), 0.0);
        }

        // Add synaptic currents based on LAST step's spikes
        for syn in &self.synapses {
            if self.last_spikes[syn.pre] {
                currents[syn.post] += syn.weight;
            }
        }

        // Update neurons
        let mut new_spikes = Vec::with_capacity(self.neurons.len());
        for (i, neuron) in self.neurons.iter_mut().enumerate() {
            let spiked = neuron.update(currents[i]);
            new_spikes.push(spiked);
        }

        self.last_spikes = new_spikes.clone();
        new_spikes
    }
}

pub fn construct_cpg() -> Network {
    let mut net = Network::new();
    // Two neurons
    net.add_neuron(IzhikevichNeuron::new());
    net.add_neuron(IzhikevichNeuron::new());

    // Mutual inhibition
    // When one spikes, it inhibits the other strongly
    net.add_synapse(0, 1, -20.0);
    net.add_synapse(1, 0, -20.0);

    net
}
