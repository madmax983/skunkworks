use rand::Rng;
use synaptic_physics::Izhikevich;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NeuronType {
    Excitatory, // Regular Spiking
    Inhibitory, // Fast Spiking
}

#[derive(Clone, Copy, Debug)]
pub struct Synapse {
    pub target_index: usize,
    pub weight: f32,
    pub delay: usize,
}

#[derive(Clone, Copy, Debug)]
pub struct IncomingConnection {
    pub source_index: usize,
    pub synapse_index: usize,
}

#[derive(Clone, Copy, Debug)]
pub struct SpikeEvent {
    pub target_index: usize,
    pub weight: f32,
}

pub struct Network {
    pub neurons: Vec<Izhikevich>,
    pub neuron_types: Vec<NeuronType>,
    pub connectivity: Vec<Vec<Synapse>>,
    pub incoming_connectivity: Vec<Vec<IncomingConnection>>,
    pub traces: Vec<f32>, // Activity trace for STDP
    pub spike_queue: Vec<Vec<SpikeEvent>>,
    pub current_step: usize,
    pub spike_history: Vec<(usize, usize)>,
    pub last_spike_times: Vec<Option<usize>>,
    pub max_delay: usize,
}

const A_PLUS: f32 = 0.1;
const A_MINUS: f32 = 0.12;
const TRACE_DECAY: f32 = 0.95;
const MAX_WEIGHT: f32 = 20.0;
const MIN_WEIGHT: f32 = 0.0;

impl Network {
    pub fn new(size: usize) -> Self {
        let mut rng = rand::thread_rng();
        let mut neurons = Vec::with_capacity(size);
        let mut neuron_types = Vec::with_capacity(size);
        let mut connectivity = Vec::with_capacity(size);
        let max_delay = 20;

        for _ in 0..size {
            let r: f32 = rng.gen();
            // 80% Excitatory, 20% Inhibitory
            if r < 0.8 {
                neuron_types.push(NeuronType::Excitatory);
                // Regular Spiking (RS)
                neurons.push(Izhikevich {
                    a: 0.02,
                    b: 0.2,
                    c: -65.0,
                    d: 8.0,
                    ..Izhikevich::new()
                });
            } else {
                neuron_types.push(NeuronType::Inhibitory);
                // Fast Spiking (FS)
                neurons.push(Izhikevich {
                    a: 0.1,
                    b: 0.2,
                    c: -65.0,
                    d: 2.0,
                    ..Izhikevich::new()
                });
            }
            connectivity.push(Vec::new());
        }

        // Create random sparse connectivity
        for i in 0..size {
            for j in 0..size {
                if i == j {
                    continue;
                }
                // 10% connectivity
                if rng.gen::<f32>() < 0.1 {
                    let delay = rng.gen_range(1..=max_delay);
                    let weight = if neuron_types[i] == NeuronType::Excitatory {
                        rng.gen_range(5.0..10.0) // Excitatory strength
                    } else {
                        rng.gen_range(-15.0..-5.0) // Inhibitory strength
                    };
                    connectivity[i].push(Synapse {
                        target_index: j,
                        weight,
                        delay,
                    });
                }
            }
        }

        // Populate incoming_connectivity
        let mut incoming_connectivity: Vec<Vec<IncomingConnection>> = vec![Vec::new(); size];
        for (src_i, outgoing) in connectivity.iter().enumerate() {
            for (syn_idx, syn) in outgoing.iter().enumerate() {
                incoming_connectivity[syn.target_index].push(IncomingConnection {
                    source_index: src_i,
                    synapse_index: syn_idx,
                });
            }
        }

        let spike_queue = vec![Vec::new(); max_delay + 1];
        let traces = vec![0.0; size];
        let last_spike_times = vec![None; size];

        Self {
            neurons,
            neuron_types,
            connectivity,
            incoming_connectivity,
            traces,
            spike_queue,
            current_step: 0,
            spike_history: Vec::new(),
            last_spike_times,
            max_delay,
        }
    }

    pub fn inject(&mut self, index: usize, current: f32) {
        if index < self.neurons.len() {
            self.neurons[index].inject(current);
        }
    }

    pub fn update(&mut self) {
        // 1. Process incoming spikes
        let queue_idx = self.current_step % self.spike_queue.len();
        let events = std::mem::take(&mut self.spike_queue[queue_idx]);

        for event in events {
            if event.target_index < self.neurons.len() {
                self.neurons[event.target_index].inject(event.weight);
            }
        }

        // 2. Update neurons
        let mut spikes = Vec::new();
        for (i, neuron) in self.neurons.iter_mut().enumerate() {
            let noise: f32 = rand::thread_rng().gen_range(-2.0..2.0);
            let (_, spiked) = neuron.update(1.0, noise);
            if spiked {
                spikes.push(i);
            }
        }

        // 3. Update Traces and Apply STDP

        // Decay traces first (simulating decay since last step)
        for t in self.traces.iter_mut() {
            *t *= TRACE_DECAY;
        }

        // Apply STDP for spiking neurons
        // We use loops to avoid simultaneous mutable borrow of connectivity
        // Strategy: Calculate weight deltas first, then apply?
        // Or access carefully.
        // We need random access to `connectivity` (for LTD) and `connectivity[source]` (for LTP).

        // Since we are inside `Network`, we have full access.
        // But `connectivity` is `Vec<Vec<Synapse>>`.
        // We can't iterate `spikes` and mutably access `connectivity` easily if we want parallelism, but here it's serial.

        for &k in &spikes {
            // LTP: Post-synaptic spike at `k`.
            // Strengthen incoming Excitatory synapses from neurons with high trace.
            // We need to modify `connectivity[src][syn_idx].weight`.
            // We can iterate `incoming_connectivity[k]` to find these locations.

            // To satisfy borrow checker, we can't hold reference to `incoming` while mutating `connectivity`?
            // Actually, `incoming_connectivity` and `connectivity` are separate fields.
            // So we can borrow `incoming_connectivity` immutably and `connectivity` mutably.

            let incoming = &self.incoming_connectivity[k];
            for inc in incoming {
                if self.neuron_types[inc.source_index] == NeuronType::Excitatory {
                    let trace_pre = self.traces[inc.source_index];
                    // If trace_pre is high, it means pre spiked recently.
                    // LTP: w += A_PLUS * trace_pre
                    if trace_pre > 0.001 {
                         let w = &mut self.connectivity[inc.source_index][inc.synapse_index].weight;
                         *w = (*w + A_PLUS * trace_pre).clamp(MIN_WEIGHT, MAX_WEIGHT);
                    }
                }
            }

            // LTD: Pre-synaptic spike at `k`.
            // Weaken outgoing synapses to neurons with high trace (post spiked recently, so pre is late -> acausal).
            if self.neuron_types[k] == NeuronType::Excitatory {
                let outgoing = &mut self.connectivity[k];
                for syn in outgoing.iter_mut() {
                     let trace_post = self.traces[syn.target_index];
                     if trace_post > 0.001 {
                         syn.weight = (syn.weight - A_MINUS * trace_post).clamp(MIN_WEIGHT, MAX_WEIGHT);
                     }
                }
            }
        }

        // Increment traces for spiking neurons
        for &k in &spikes {
            self.traces[k] += 1.0;
        }

        // 4. Propagate spikes
        for &neuron_idx in &spikes {
            self.spike_history.push((self.current_step, neuron_idx));
            self.last_spike_times[neuron_idx] = Some(self.current_step);

            for synapse in &self.connectivity[neuron_idx] {
                let arrival_time = self.current_step + synapse.delay;
                let q_idx = arrival_time % self.spike_queue.len();
                self.spike_queue[q_idx].push(SpikeEvent {
                    target_index: synapse.target_index,
                    weight: synapse.weight,
                });
            }
        }

        // Prune history
        if self.spike_history.len() > 10_000 {
             self.spike_history.drain(0..1000);
        }

        self.current_step += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_initialization() {
        let size = 100;
        let network = Network::new(size);
        assert_eq!(network.neurons.len(), size);
        assert_eq!(network.neuron_types.len(), size);
        assert_eq!(network.connectivity.len(), size);
        assert_eq!(network.traces.len(), size);
    }

    #[test]
    fn test_spike_propagation() {
        let mut network = Network::new(10);
        // Clear random connections for deterministic test
        for c in &mut network.connectivity {
            c.clear();
        }
        network.incoming_connectivity = vec![Vec::new(); 10];
        network.spike_queue = vec![Vec::new(); network.max_delay + 1];

        // Connect 0 -> 1 with delay 5
        network.connectivity[0].push(Synapse {
            target_index: 1,
            weight: 50.0, // Strong enough to spike
            delay: 5,
        });

        // Inject current to spike 0
        network.inject(0, 100.0);

        // Update
        network.update(); // Step 0. Neuron 0 should spike.

        // Check spike history
        let spiked_0 = network.spike_history.iter().any(|(t, idx)| *t == 0 && *idx == 0);
        assert!(spiked_0, "Neuron 0 should have spiked at step 0");

        // Run until delay
        for _ in 0..4 {
            network.update();
        }

        // Step 5. Spike should arrive at 1.
        network.update();

        // Step 6. Neuron 1 might spike here if integration takes time.
        network.update();

        // Check if 1 spiked (it might take a step for voltage to rise)
        // Neuron 0 spike at t=0. Arrival t=5.
        // At t=5, `update` processes queue. `inject` happens.
        // Then `neuron.update`. Voltage rises.

        let spiked_1 = network.spike_history.iter().any(|(t, idx)| *t >= 5 && *idx == 1);
        assert!(spiked_1, "Neuron 1 should have spiked after delay. History: {:?}", network.spike_history);
    }
}
