use neuro_sim::Izhikevich;

#[derive(Clone, Debug)]
pub struct Synapse {
    pub pre: usize,
    pub post: usize,
    pub weight: f32,
}

#[derive(Clone, Debug)]
pub struct Neuron {
    pub core: Izhikevich,
    pub last_spike: Option<u64>,
}

impl Neuron {
    pub fn random(rng: &mut impl rand::Rng) -> Self {
        Self {
            core: Izhikevich::random(rng),
            last_spike: None,
        }
    }

    pub fn update(&mut self, dt: f32, current: f32, tick: u64) -> bool {
        let (_v, spiked) = self.core.update(dt, current);
        if spiked {
            self.last_spike = Some(tick);
        }
        spiked
    }
}

// Deref allows access to core fields like `v`, `u`
impl std::ops::Deref for Neuron {
    type Target = Izhikevich;
    fn deref(&self) -> &Self::Target {
        &self.core
    }
}

impl std::ops::DerefMut for Neuron {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.core
    }
}

pub struct Network {
    pub neurons: Vec<Neuron>,
    pub synapses: Vec<Synapse>,
    pub traces: Vec<f32>, // Synaptic trace for each neuron
    pub tick: u64,
}

impl Network {
    pub fn new(size: usize) -> Self {
        let mut neurons = Vec::with_capacity(size);
        let mut traces = Vec::with_capacity(size);
        let mut rng = rand::thread_rng();

        for _ in 0..size {
            neurons.push(Neuron::random(&mut rng));
            traces.push(0.0);
        }

        Self {
            neurons,
            synapses: Vec::new(),
            traces,
            tick: 0,
        }
    }

    pub fn add_synapse(&mut self, pre: usize, post: usize, weight: f32) {
        self.synapses.push(Synapse { pre, post, weight });
    }

    pub fn connect_trace(&mut self) {
        // Connect neurons linearly: 0 -> 1 -> 2 ...
        // Represents the call stack flow.
        for i in 0..self.neurons.len().saturating_sub(1) {
            // Strong excitatory connection forward
            self.add_synapse(i, i + 1, 15.0);

            // Weak inhibitory connection backward (feedback)
            // self.add_synapse(i + 1, i, -2.0);
        }
    }

    pub fn step(&mut self, dt: f32, inputs: &[f32]) {
        self.tick += 1;
        let decay = 0.95;
        let a_plus = 0.1;
        // let a_minus = 0.12;

        // 1. Calculate synaptic currents
        let mut currents = vec![0.0; self.neurons.len()];

        // Add external inputs
        for (i, &input) in inputs.iter().enumerate().take(self.neurons.len()) {
            currents[i] += input;
        }

        // Add synaptic currents from *previous* step spikes
        for syn in &self.synapses {
            if let Some(t) = self.neurons[syn.pre].last_spike {
                if t == self.tick - 1 {
                    currents[syn.post] += syn.weight;
                }
            }
        }

        // 2. Update Neurons and Traces
        for (i, neuron) in self.neurons.iter_mut().enumerate() {
            let spiked = neuron.update(dt, currents[i], self.tick);

            // Update trace
            self.traces[i] *= decay;
            if spiked {
                self.traces[i] += 1.0;
            }
        }

        // 3. Simple Hebbian / STDP
        for syn in &mut self.synapses {
            let post_spiked = self.neurons[syn.post].last_spike == Some(self.tick);

            if post_spiked {
                // Pre before Post -> Potentiation
                syn.weight += a_plus * self.traces[syn.pre];
            }

            // Decay weights slowly to prevent explosion?
            // Or just clamp.
            syn.weight = syn.weight.clamp(-20.0, 30.0);
        }
    }
}
