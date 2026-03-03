use neuro_sim::Izhikevich;
use rand::Rng;

#[derive(Clone, Debug)]
pub struct Synapse {
    pub pre: usize,
    pub post: usize,
    pub weight: f32,
    // Add delays later if needed
}

pub struct Network {
    pub neurons: Vec<Izhikevich>,
    pub last_spikes: Vec<Option<u64>>, // Track spike times locally
    pub synapses: Vec<Synapse>,
    pub traces: Vec<f32>, // Synaptic trace for each neuron (for STDP)
    pub tick: u64,
}

impl Network {
    pub fn new(size: usize) -> Self {
        let mut neurons = Vec::with_capacity(size);
        let mut last_spikes = Vec::with_capacity(size);
        let mut traces = Vec::with_capacity(size);
        let mut rng = rand::thread_rng();

        for _ in 0..size {
            neurons.push(Izhikevich::random(&mut rng));
            last_spikes.push(None);
            traces.push(0.0);
        }

        Self {
            neurons,
            last_spikes,
            synapses: Vec::new(),
            traces,
            tick: 0,
        }
    }

    pub fn inject(&mut self, index: usize, current: f32) {
        if let Some(neuron) = self.neurons.get_mut(index) {
            neuron.inject(current);
        }
    }

    pub fn add_synapse(&mut self, pre: usize, post: usize, weight: f32) {
        self.synapses.push(Synapse { pre, post, weight });
    }

    /// Fully connect the network with random weights
    pub fn connect_random(&mut self, probability: f32, max_weight: f32) {
        let mut rng = rand::thread_rng();
        let size = self.neurons.len();
        for i in 0..size {
            for j in 0..size {
                if i != j && rng.gen::<f32>() < probability {
                    self.add_synapse(i, j, rng.gen::<f32>() * max_weight);
                }
            }
        }
    }

    pub fn step(&mut self, dt: f32) {
        self.tick += 1;
        let decay = 0.95; // Trace decay
        let a_plus = 0.1; // Potentiation
        let a_minus = 0.12; // Depression

        // 1. Calculate synaptic currents
        let mut currents = vec![0.0; self.neurons.len()];

        // Note: External inputs are now handled by neuron.current_decay via inject()

        // Add synaptic currents from *previous* step spikes
        for syn in &self.synapses {
            if let Some(t) = self.last_spikes[syn.pre] {
                if t == self.tick - 1 {
                    currents[syn.post] += syn.weight;
                }
            }
        }

        // 2. Update Neurons and Traces
        for (i, neuron) in self.neurons.iter_mut().enumerate() {
            // update() returns (voltage, spiked)
            let (_, spiked) = neuron.update(dt, currents[i]);

            if spiked {
                self.last_spikes[i] = Some(self.tick);
            }

            // Update trace
            self.traces[i] *= decay;
            if spiked {
                self.traces[i] += 1.0;
            }
        }

        // 3. Apply STDP (Plasticity)
        for syn in &mut self.synapses {
            let pre_spiked = self.last_spikes[syn.pre] == Some(self.tick);
            let post_spiked = self.last_spikes[syn.post] == Some(self.tick);

            if pre_spiked {
                // Pre spiked NOW. Check POST trace.
                // If POST trace is high, it means POST spiked RECENTLY.
                // Post before Pre -> Depression (LTD)
                syn.weight -= a_minus * self.traces[syn.post];
            }

            if post_spiked {
                // Post spiked NOW. Check PRE trace.
                // If PRE trace is high, it means PRE spiked RECENTLY.
                // Pre before Post -> Potentiation (LTP)
                syn.weight += a_plus * self.traces[syn.pre];
            }

            // Clamp weights
            syn.weight = syn.weight.clamp(0.0, 20.0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_synaptic_transmission() {
        let mut net = Network::new(2);
        // Neuron 0 -> Neuron 1
        net.add_synapse(0, 1, 10.0);

        // Make neuron 0 spike
        // Step 1: Inject huge current into 0 for multiple steps until spike
        // Using inject() now instead of raw inputs vector
        net.inject(0, 50.0);

        let mut spiked_at = None;

        for _ in 0..10 {
            net.step(1.0);
            if net.last_spikes[0] == Some(net.tick) {
                spiked_at = Some(net.tick);
                break;
            }
            // Keep injecting if needed, but 50 should trigger it quickly.
            // Decay might reduce it, so re-inject if needed?
            // 50 is huge, tau is 10. Decay is slow. Should be fine.
        }

        assert!(
            spiked_at.is_some(),
            "Neuron 0 did not spike. Voltage: {}",
            net.neurons[0].v
        );

        // Step 2: Continue simulation.
        // Neuron 1 should receive current 1 tick after spike.

        let spike_tick = spiked_at.unwrap();
        // Run until tick == spike_tick + 1
        while net.tick <= spike_tick + 1 {
            let v_prev = net.neurons[1].v;
            net.step(1.0);

            if net.tick == spike_tick + 1 {
                // This step, synapse should have fired.
                // Current was added.
                // Verify v increased significantly
                let v_curr = net.neurons[1].v;
                assert!(
                    v_curr > v_prev + 5.0,
                    "Synapse transmission failed at tick {}. v_prev: {}, v_curr: {}",
                    net.tick,
                    v_prev,
                    v_curr
                );
            }
        }
    }
}
