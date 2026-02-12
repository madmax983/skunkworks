use crate::hologram::HolographicMemory;
use rand::prelude::*;
use synaptic_physics::Izhikevich;

pub struct Brain {
    pub width: usize,
    pub height: usize,
    pub neurons: Vec<Izhikevich>,
    pub memory: HolographicMemory,
    pub learning_rate: f64,
    pub feedback_strength: f64,
    pub is_learning: bool,
}

impl Brain {
    pub fn new(width: usize, height: usize) -> Self {
        let mut rng = rand::thread_rng();
        let neurons: Vec<Izhikevich> = (0..width * height)
            .map(|_| Izhikevich::random(&mut rng))
            .collect();

        Self {
            width,
            height,
            neurons,
            memory: HolographicMemory::new(width, height),
            learning_rate: 0.1,
            feedback_strength: 0.05,
            is_learning: false,
        }
    }

    pub fn update(&mut self) {
        let dt = 1.0;
        let mut spikes = vec![0.0f64; self.width * self.height];

        // 1. Update Neurons (autonomous dynamics + external input from last step)
        // We accumulate current in `I` during `inject`.
        for (i, neuron) in self.neurons.iter_mut().enumerate() {
            let (v, spiked) = neuron.update(dt, 0.0); // 0.0 external current (handled via inject)
            if spiked {
                spikes[i] = 1.0;
            } else {
                // Use voltage as "Analog" value for hologram?
                // Or just spikes?
                // Using voltage gives more information.
                // Let's use normalized voltage (v + 70) / 100.
                spikes[i] = (((v + 70.0) / 100.0).max(0.0).min(1.0)) as f64;
            }
        }

        // 2. Holographic Interaction
        if self.is_learning {
            // Learn the current pattern
            // We just add it to the memory (Hebbian-like accumulation)
            self.memory.record(&spikes, self.learning_rate);
        }

        // 3. Reconstruction / Feedback
        // We always reconstruct from the memory to see what it "thinks"
        let reconstruction = self.memory.reconstruct();

        // 4. Feedback into Neurons
        // If the reconstruction is strong, it injects current back into the neurons.
        // This closes the loop.
        for (i, val) in reconstruction.iter().enumerate() {
            if i < self.neurons.len() {
                // Inject current proportional to reconstruction intensity
                // Scale factor needed. Reconstruction magnitude can be large.
                // But we normalized it in `reconstruct`.
                let injection = (val * self.feedback_strength * 100.0) as f32;
                self.neurons[i].inject(injection);
            }
        }
    }

    pub fn inject_noise(&mut self, amount: f32) {
        let mut rng = rand::thread_rng();
        for neuron in &mut self.neurons {
            if rng.gen::<f32>() < 0.1 {
                neuron.inject(rng.gen::<f32>() * amount);
            }
        }
    }

    pub fn clear_neurons(&mut self) {
        let mut rng = rand::thread_rng();
        for neuron in &mut self.neurons {
            *neuron = Izhikevich::random(&mut rng);
            neuron.v = -65.0;
        }
    }

    pub fn reset_memory(&mut self) {
        self.memory.clear();
    }
}
