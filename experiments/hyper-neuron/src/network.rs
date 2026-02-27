use crate::math::Vec4D;
use crate::monitor::SystemMonitor;
use rand::Rng;
use neuro_sim::Izhikevich;

#[derive(Clone, Debug)]
pub struct Neuron4D {
    pub position: Vec4D,
    pub state: Izhikevich,
    pub index: usize,
}

#[derive(Clone, Debug)]
pub struct Synapse {
    pub target_index: usize,
    pub weight: f32,
    // Spikes in transit: (remaining_time, initial_distance)
    pub spikes: Vec<(f32, f32)>,
}

pub struct Network {
    pub neurons: Vec<Neuron4D>,
    pub synapses: Vec<Vec<Synapse>>,
}

impl Network {
    pub fn new(count: usize) -> Self {
        let mut rng = rand::thread_rng();
        let mut neurons = Vec::with_capacity(count);
        let mut synapses = vec![Vec::new(); count]; // Initialize vector of vectors!

        // Create Neurons randomly in 4D hypercube [-2, 2]
        for i in 0..count {
            let pos = Vec4D::new(
                rng.gen_range(-2.0..2.0),
                rng.gen_range(-2.0..2.0),
                rng.gen_range(-2.0..2.0),
                rng.gen_range(-2.0..2.0),
            );

            // Mix of neuron types
            let state = if rng.gen::<f32>() < 0.7 {
                Izhikevich::new_regular_spiking()
            } else {
                Izhikevich::new_fast_spiking()
            };

            neurons.push(Neuron4D {
                position: pos,
                state,
                index: i,
            });
        }

        // Connect Neurons (Small World-ish)
        // Clone positions to avoid borrow issues during generation if we needed them, but here we just use indices
        let positions: Vec<Vec4D> = neurons.iter().map(|n| n.position).collect();

        for i in 0..count {
            // Connect to nearest 3 neighbors
            let mut distances: Vec<(usize, f32)> = positions
                .iter()
                .enumerate()
                .filter(|(idx, _)| *idx != i)
                .map(|(idx, pos)| (idx, positions[i].distance_squared(*pos)))
                .collect();

            distances.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

            for j in 0..3.min(distances.len()) {
                let target = distances[j].0;
                synapses[i].push(Synapse {
                    target_index: target,
                    weight: 15.0,
                    spikes: Vec::new(),
                });
            }

            // Long range random connection
            let target = rng.gen_range(0..count);
            if target != i {
                synapses[i].push(Synapse {
                    target_index: target,
                    weight: 10.0,
                    spikes: Vec::new(),
                });
            }
        }

        Self { neurons, synapses }
    }

    pub fn update(&mut self, dt: f32, monitor: &SystemMonitor) {
        // Distortion Factors
        let sx = 1.0 + monitor.cpu_usage;
        let sy = 1.0 + monitor.mem_usage;
        let sz = 1.0 + monitor.swap_usage;
        let sw = 1.0 + monitor.load_avg;

        // Base speed of signal
        let signal_speed = 5.0;

        // 1. Process Synapse Transit & Collect Inputs for THIS frame
        let mut inputs = vec![0.0; self.neurons.len()];

        // We iterate mutably over synapses
        for synapses_vec in self.synapses.iter_mut() {
            for synapse in synapses_vec.iter_mut() {
                // Filter spikes in place
                let mut kept_spikes = Vec::new();
                for (timer, dist) in synapse.spikes.iter_mut() {
                    *timer -= dt;
                    if *timer <= 0.0 {
                        // Spike Arrived!
                        inputs[synapse.target_index] += synapse.weight;
                    } else {
                        kept_spikes.push((*timer, *dist));
                    }
                }
                synapse.spikes = kept_spikes;
            }
        }

        // 2. Inject Inputs & Update Neurons & Collect Spikes
        let mut spiked_indices = Vec::new();
        for (i, neuron) in self.neurons.iter_mut().enumerate() {
            if inputs[i] > 0.0 {
                neuron.state.inject(inputs[i]);
            }

            let noise = rand::thread_rng().gen_range(0.0..5.0);
            let stress = monitor.load_avg * 10.0; // Higher stress = more background noise/excitation

            let (_, spiked) = neuron.state.update(dt, noise + stress);
            if spiked {
                spiked_indices.push(i);
            }
        }

        // 3. Queue NEW Spikes
        // We need positions to calculate delay.
        let positions: Vec<Vec4D> = self.neurons.iter().map(|n| n.position).collect();

        for source_idx in spiked_indices {
            let source_pos = positions[source_idx];
            // Now we access synapses mutably
            for synapse in self.synapses[source_idx].iter_mut() {
                let target_pos = positions[synapse.target_index];

                // Calculate Distorted Distance
                let diff = target_pos.sub(source_pos);
                let distorted_diff = diff.scale_dim(sx, sy, sz, sw);
                let distance = distorted_diff.length();

                let delay = distance / signal_speed;

                // Store (delay, initial_distance) for visualization
                synapse.spikes.push((delay, distance));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::monitor::SystemMonitor;

    #[test]
    fn test_network_initialization() {
        let net = Network::new(10);
        assert_eq!(net.neurons.len(), 10);
        assert_eq!(net.synapses.len(), 10);
    }

    #[test]
    fn test_network_update() {
        let mut net = Network::new(10);
        let monitor = SystemMonitor::new();
        // Just verify it doesn't panic
        net.update(1.0, &monitor);
    }
}
