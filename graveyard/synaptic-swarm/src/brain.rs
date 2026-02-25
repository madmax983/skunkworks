use rand::Rng;
use synaptic_physics::Izhikevich;

pub const N_SENSORY: usize = 4;
pub const N_INTER: usize = 4;
pub const N_MOTOR: usize = 2;
pub const N_TOTAL: usize = N_SENSORY + N_INTER + N_MOTOR;

pub struct Brain {
    pub neurons: Vec<Izhikevich>,
    /// Adjacency matrix: weights[i][j] is weight from neuron i to neuron j
    pub weights: Vec<Vec<f32>>,
    /// Spike traces for STDP
    pub traces: Vec<f32>,
    pub trace_tau: f32,
    pub learning_rate: f32,
    pub spikes: Vec<usize>, // List of indices that spiked this frame
}

impl Brain {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut neurons = Vec::with_capacity(N_TOTAL);

        // 1. Sensory Neurons (Regular Spiking)
        for _ in 0..N_SENSORY {
            neurons.push(Izhikevich::new_regular_spiking());
        }

        // 2. Interneurons (Mixed types)
        for _ in 0..N_INTER {
            neurons.push(Izhikevich::random(&mut rng));
        }

        // 3. Motor Neurons (Regular Spiking or Chattering for bursts)
        for _ in 0..N_MOTOR {
            neurons.push(Izhikevich::new_regular_spiking());
        }

        // Initialize random weights
        let mut weights = vec![vec![0.0; N_TOTAL]; N_TOTAL];
        for i in 0..N_TOTAL {
            for j in 0..N_TOTAL {
                if i == j {
                    continue;
                }
                // Sparse connectivity (30%)
                if rng.gen::<f32>() < 0.3 {
                    // Small random weights
                    weights[i][j] = rng.gen_range(0.0..10.0);
                }
            }
        }

        Self {
            neurons,
            weights,
            traces: vec![0.0; N_TOTAL],
            trace_tau: 20.0, // ms
            learning_rate: 0.1,
            spikes: Vec::new(),
        }
    }

    pub fn update(&mut self, dt: f32, inputs: &[f32]) {
        self.spikes.clear();

        // Decay traces
        let trace_decay = (-dt / self.trace_tau).exp();
        for trace in &mut self.traces {
            *trace *= trace_decay;
        }

        // Calculate synaptic currents for next step
        // We sum up currents based on *previous* spikes?
        // No, Izhikevich handles current injection.
        // But if neuron i spikes, it injects current into j.
        // We need to know who spiked *this* frame to inject current.
        // Wait, Izhikevich `update` returns `spiked`.
        // So we update all neurons, collect spikes, then inject current for *next* frame?
        // Or update immediately?
        // Standard approach: Update all neurons based on current state (and inputs).
        // If a neuron spikes, it adds to a `buffer` for the next frame or immediate injection.
        // Since we process neurons in order, immediate injection biases later neurons.
        // Let's use a two-pass approach or just accept the bias (minimal for small dt).

        // Actually, `Izhikevich` has `current_decay` which persists.
        // So if I inject now, it affects the *next* update call.
        // So:
        // 1. Inject external inputs.
        // 2. Update all neurons.
        // 3. If spiked, inject into targets (for next frame).
        // 4. Update STDP.

        // 1. Update Neurons
        let mut spiked_indices = Vec::new();
        for (i, neuron) in self.neurons.iter_mut().enumerate() {
            let input = if i < inputs.len() {
                inputs[i] * 10.0
            } else {
                0.0
            };
            let (_, spiked) = neuron.update(dt, input);
            if spiked {
                spiked_indices.push(i);
            }
        }

        // 3. Handle Spikes (Synaptic Transmission + STDP)
        for &i in &spiked_indices {
            self.traces[i] += 1.0; // Update trace *after* spike? Or before? Usually peak is at spike.
                                   // Cap trace at some value?
            if self.traces[i] > 2.0 {
                self.traces[i] = 2.0;
            }

            // Synaptic Transmission
            for j in 0..N_TOTAL {
                let w = self.weights[i][j];
                if w != 0.0 {
                    self.neurons[j].inject(w);
                }

                // STDP: Pre (i) -> Post (j)
                // i just spiked. Check j's trace (Post).
                // If j spiked recently (trace[j] is high), it means j fired BEFORE i.
                // This is anti-causal for i->j. So we weaken i->j. (LTD)
                // wait...
                // Pre (i) spikes at t_i. Post (j) spikes at t_j.
                // If t_j < t_i (Post before Pre), then dt = t_j - t_i < 0. LTD.
                // Here, i just spiked (t_i = now).
                // trace[j] represents recent t_j.
                // So trace[j] > 0 means j spiked recently.
                // So j fired before i.
                // So weaken w[i][j].

                if self.traces[j] > 0.1 {
                    let change = self.learning_rate * self.traces[j];
                    self.weights[i][j] -= change;
                }
            }

            // STDP: Post (i) <- Pre (j)
            // i just spiked (Post). Check j's trace (Pre).
            // If j spiked recently (trace[j] is high), it means j fired BEFORE i.
            // This is causal for j->i. So we strengthen j->i. (LTP)
            for j in 0..N_TOTAL {
                if self.traces[j] > 0.1 {
                    let change = self.learning_rate * self.traces[j];
                    self.weights[j][i] += change; // Note index: weight FROM j TO i
                }
            }
        }

        // Clamp weights
        for row in &mut self.weights {
            for w in row {
                *w = w.clamp(-20.0, 20.0);
            }
        }

        self.spikes = spiked_indices;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_brain_update() {
        let mut brain = Brain::new();
        // Run for 100 steps
        for _ in 0..100 {
            brain.update(0.1, &[0.0; N_SENSORY]);
        }
        // Check if traces decayed
        for t in &brain.traces {
            assert!(*t >= 0.0);
            assert!(*t <= 2.0);
        }
    }

    #[test]
    fn test_stdp() {
        let mut brain = Brain::new();
        // Force a spike on neuron 0
        brain.neurons[0].v = 30.0;

        // Update
        brain.update(0.1, &[0.0; N_SENSORY]);

        // Neuron 0 should have spiked
        assert!(brain.spikes.contains(&0));
        // Trace 0 should increase
        assert!(brain.traces[0] > 0.0);
    }
}
