use crate::audio::SpikeEvent;
use crossbeam_channel::Sender;
use rand::Rng;
use synaptic_physics::Izhikevich;

#[derive(Clone, Copy)]
pub struct Neuron {
    pub izh: Izhikevich,
    pub x: f32,
    pub y: f32,
    pub frequency: f32,
}

pub struct Cortex {
    pub neurons: Vec<Neuron>,
    pub width: f32,
    pub height: f32,
    pub spike_tx: Sender<SpikeEvent>,
}

impl Cortex {
    pub fn new(width: f32, height: f32, neuron_count: usize, spike_tx: Sender<SpikeEvent>) -> Self {
        let mut rng = rand::thread_rng();
        let mut neurons = Vec::with_capacity(neuron_count);

        // Pentatonic scale frequencies
        let scale = [261.63, 293.66, 329.63, 392.00, 440.00];

        for _ in 0..neuron_count {
            let x = rng.gen_range(0.0..width);
            let y = rng.gen_range(0.0..height);
            let mut izh = Izhikevich::random(&mut rng);
            // Customize parameters slightly
            izh.a = 0.02 + rng.gen_range(-0.005..0.005);

            let octave = rng.gen_range(0..3); // 3 octaves
            let note = scale[rng.gen_range(0..5)];
            let frequency = note * 2.0f32.powi(octave as i32);

            neurons.push(Neuron {
                izh,
                x,
                y,
                frequency,
            });
        }

        Self {
            neurons,
            width,
            height,
            spike_tx,
        }
    }

    pub fn update(&mut self, dt: f32) {
        // Collect spikes for coupling: (index, x, y)
        let mut spikes: Vec<(usize, f32, f32)> = Vec::new();

        let mut rng = rand::thread_rng();

        for (i, neuron) in self.neurons.iter_mut().enumerate() {
            // Apply a small random noise current to keep things alive
            let noise = rng.gen_range(-2.0..2.0);
            let (_, spiked) = neuron.izh.update(dt, noise);
            if spiked {
                spikes.push((i, neuron.x, neuron.y));
                // Send to audio
                let _ = self.spike_tx.try_send(SpikeEvent {
                    frequency: neuron.frequency,
                    amplitude: 0.1,
                });
            }
        }

        // Coupling
        for (source_idx, sx, sy) in spikes {
            for (j, target) in self.neurons.iter_mut().enumerate() {
                if source_idx == j { continue; }
                let dx = target.x - sx;
                // Simple box check first
                if dx.abs() > 30.0 { continue; }
                let dy = target.y - sy;
                if dy.abs() > 30.0 { continue; }

                let dist_sq = dx*dx + dy*dy;

                // Radius of influence: 30.0
                if dist_sq < 900.0 {
                    // Weight falls off with distance
                    let weight = 50.0 / (1.0 + dist_sq * 0.1);
                    target.izh.inject(weight);
                }
            }
        }
    }

    pub fn stimulate_random(&mut self, amount: f32) {
        let mut rng = rand::thread_rng();
        // Stimulate 5% of neurons
        let count = (self.neurons.len() as f32 * 0.05).ceil() as usize;
        for _ in 0..count {
            let idx = rng.gen_range(0..self.neurons.len());
            self.neurons[idx].izh.inject(amount);
        }
    }
}
