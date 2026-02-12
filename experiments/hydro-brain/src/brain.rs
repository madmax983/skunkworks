use rand::Rng;

#[derive(Clone, Debug)]
pub struct Neuron {
    pub id: usize,
    pub layer: usize,
    pub index_in_layer: usize,
    // Layout (calculated dynamically)
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

#[derive(Clone, Debug)]
pub struct Synapse {
    pub source: usize,
    pub target: usize,
    pub weight: f32,
}

#[derive(Clone, Debug)]
pub struct TravelingSpike {
    pub target_id: usize,
    pub x: f32,
    pub y: f32,
    pub start_x: f32,
    pub start_y: f32,
    pub end_x: f32,
    pub end_y: f32,
    pub progress: f32, // 0.0 to 1.0
}

pub struct Network {
    pub neurons: Vec<Neuron>,
    pub synapses: Vec<Synapse>,
    pub traveling_spikes: Vec<TravelingSpike>,
    pub layers: Vec<usize>,
}

impl Network {
    pub fn new(layers: Vec<usize>) -> Self {
        let mut neurons = Vec::new();
        let mut synapses = Vec::new();
        let mut id_counter = 0;
        let mut rng = rand::thread_rng();

        // Create Neurons
        let mut layer_offsets = Vec::new();
        for (l_idx, &count) in layers.iter().enumerate() {
            layer_offsets.push(id_counter);
            for n_idx in 0..count {
                neurons.push(Neuron {
                    id: id_counter,
                    layer: l_idx,
                    index_in_layer: n_idx,
                    x: 0.0,
                    y: 0.0,
                    w: 0.0,
                    h: 0.0,
                });
                id_counter += 1;
            }
        }

        // Create Synapses (Full connectivity between layers)
        for l in 0..layers.len() - 1 {
            let current_start = layer_offsets[l];
            let current_end = current_start + layers[l];
            let next_start = layer_offsets[l + 1];
            let next_end = next_start + layers[l + 1];

            for src in current_start..current_end {
                let mut weights = Vec::new();
                let mut total_weight = 0.0;

                for _ in next_start..next_end {
                    let w: f32 = rng.gen_range(0.1..1.0);
                    weights.push(w);
                    total_weight += w;
                }

                for (i, tgt) in (next_start..next_end).enumerate() {
                    synapses.push(Synapse {
                        source: src,
                        target: tgt,
                        weight: weights[i] / total_weight,
                    });
                }
            }
        }

        Self {
            neurons,
            synapses,
            traveling_spikes: Vec::new(),
            layers,
        }
    }

    pub fn layout(&mut self, width: f32, height: f32) {
        let layer_height = height / (self.layers.len() as f32);
        let neuron_height = layer_height * 0.4; // 40% of layer height is the bucket
        let gap_y = layer_height * 0.1;

        for n in &mut self.neurons {
            let count_in_layer = self.layers[n.layer];
            // Fix: calculate positions relative to full width
            let layer_width = width;
            let slot_width = layer_width / count_in_layer as f32;
            let neuron_width = slot_width * 0.8;
            let gap_x = slot_width * 0.1;

            let x = n.index_in_layer as f32 * slot_width + gap_x;
            let y = n.layer as f32 * layer_height + gap_y;

            n.x = x;
            n.y = y;
            n.w = neuron_width;
            n.h = neuron_height;
        }
    }

    pub fn get_target_for_spike(&self, source_id: usize) -> Option<usize> {
        let candidates: Vec<&Synapse> = self
            .synapses
            .iter()
            .filter(|s| s.source == source_id)
            .collect();

        if candidates.is_empty() {
            return None;
        }

        let mut rng = rand::thread_rng();
        let r: f32 = rng.gen();
        let mut cumulative = 0.0;

        for s in candidates {
            cumulative += s.weight;
            if r <= cumulative {
                return Some(s.target);
            }
        }

        // Fallback
        self.synapses
            .iter()
            .filter(|s| s.source == source_id)
            .last()
            .map(|s| s.target)
    }

    pub fn spawn_spike(&mut self, src_id: usize, tgt_id: usize) {
        // Need to get positions from neurons.
        // self.neurons is minimal copy type so we can index.
        let src = &self.neurons[src_id];
        let tgt = &self.neurons[tgt_id];

        let start_x = src.x + src.w / 2.0;
        let start_y = src.y + src.h;
        let end_x = tgt.x + tgt.w / 2.0;
        let end_y = tgt.y;

        self.traveling_spikes.push(TravelingSpike {
            target_id: tgt_id,
            x: start_x,
            y: start_y,
            start_x,
            start_y,
            end_x,
            end_y,
            progress: 0.0,
        });
    }

    /// Updates spikes. Returns list of (target_id, x, y) for arrivals.
    pub fn update(&mut self, dt: f32) -> Vec<(usize, f32, f32)> {
        let mut arrivals = Vec::new();
        let speed = 2.0;

        self.traveling_spikes.retain_mut(|spike| {
            spike.progress += speed * dt;

            spike.x = spike.start_x + (spike.end_x - spike.start_x) * spike.progress;
            spike.y = spike.start_y + (spike.end_y - spike.start_y) * spike.progress;

            if spike.progress >= 1.0 {
                arrivals.push((spike.target_id, spike.end_x, spike.end_y + 1.0));
                return false;
            }
            true
        });

        arrivals
    }
}
