use crate::neuron::IzhikevichPopulation;
use ndarray::{azip, Array1, Array2};
use rand::Rng;

pub struct Reservoir {
    pub neurons: IzhikevichPopulation,
    pub weights: Array2<f32>,
    pub synaptic_currents: Array1<f32>,
    pub traces: Array1<f32>,
    pub last_spikes: Vec<bool>,
    pub size: usize,
}

impl Reservoir {
    pub fn new(size: usize, density: f32) -> Self {
        let neurons = IzhikevichPopulation::new(size);
        let mut weights = Array2::zeros((size, size));
        let mut rng = rand::thread_rng();

        let exc_cutoff = (size as f32 * 0.8) as usize;

        for i in 0..size {
            // Pre
            for j in 0..size {
                // Post
                if i == j {
                    continue;
                }
                if rng.gen::<f32>() < density {
                    let w = if i < exc_cutoff {
                        10.0 * rng.gen::<f32>() // Excitatory weights
                    } else {
                        -10.0 * rng.gen::<f32>() // Inhibitory weights
                    };
                    weights[[j, i]] = w;
                }
            }
        }

        Self {
            neurons,
            weights,
            synaptic_currents: Array1::zeros(size),
            traces: Array1::zeros(size),
            last_spikes: vec![false; size],
            size,
        }
    }

    pub fn step(&mut self, dt: f32, external_input: &Array1<f32>) -> Vec<bool> {
        // 1. Decay synaptic currents (tau = 5ms)
        // I = I * exp(-dt/5)
        let decay = (-dt / 5.0).exp();
        self.synaptic_currents *= decay;

        // 2. Add input from previous spikes
        for (pre_idx, &spiked) in self.last_spikes.iter().enumerate() {
            if spiked {
                // Add column pre_idx of weights to synaptic currents
                let col = self.weights.column(pre_idx);
                azip!((curr in &mut self.synaptic_currents, &w in &col) {
                    *curr += w;
                });
            }
        }

        // 3. Update Neurons
        let total_input = &self.synaptic_currents + external_input;
        let new_spikes = self.neurons.update(dt, &total_input);

        // 4. Update STDP Traces (tau = 20ms)
        let trace_decay = (-dt / 20.0).exp();
        self.traces *= trace_decay;

        for (i, &spiked) in new_spikes.iter().enumerate() {
            if spiked {
                self.traces[i] += 1.0;
            }
        }

        self.last_spikes = new_spikes.clone();
        new_spikes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reservoir_step() {
        let size = 50;
        let mut net = Reservoir::new(size, 0.2);
        let input = Array1::from_elem(size, 5.0); // Sub-threshold input

        // Run a few steps
        let mut spike_count = 0;
        for _ in 0..100 {
            let spikes = net.step(0.5, &input);
            for &s in &spikes {
                if s {
                    spike_count += 1;
                }
            }
        }
        // Just check it doesn't crash. Spike count might be 0 or >0 depending on RNG.
        println!("Spikes generated: {}", spike_count);
    }
}
