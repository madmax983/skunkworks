use crate::neuron::{HodgkinHuxley, Command};
use crate::physics::World;
use macroquad::prelude::Vec2;

pub struct Muscle {
    pub neuron_idx: usize,
    pub particle_idx: usize,
    pub force_vector: Vec2, // Direction and max magnitude of pull
}

pub struct Brain {
    pub neurons: Vec<HodgkinHuxley>,
    pub muscles: Vec<Muscle>,
}

impl Brain {
    pub fn new() -> Self {
        Self {
            neurons: Vec::new(),
            muscles: Vec::new(),
        }
    }

    pub fn add_neuron(&mut self) -> usize {
        self.neurons.push(HodgkinHuxley::new());
        self.neurons.len() - 1
    }

    pub fn add_muscle(&mut self, neuron_idx: usize, particle_idx: usize, force: Vec2) {
        self.muscles.push(Muscle {
            neuron_idx,
            particle_idx,
            force_vector: force,
        });
    }

    pub fn update(&mut self, dt: f32, world: &mut World) {
        // Update Neurons with sub-stepping for stability
        // Hodgkin-Huxley requires small timesteps (e.g., 0.05ms)
        let dt_ms = dt * 1000.0;
        let sim_dt = 0.05; // 0.05 ms
        let steps = (dt_ms / sim_dt).ceil() as usize;

        for _ in 0..steps {
            for neuron in &mut self.neurons {
                neuron.step(sim_dt);
            }
        }

        // Apply Muscle Forces
        for muscle in &self.muscles {
            if muscle.neuron_idx < self.neurons.len() && muscle.particle_idx < world.particles.len() {
                let neuron = &self.neurons[muscle.neuron_idx];

                // Activation function: Map V (-70 to +40) to 0.0 to 1.0
                // Sigmoid-like or threshold
                let activation = ((neuron.v + 50.0) / 20.0).clamp(0.0, 1.0);

                if activation > 0.0 {
                    let force = muscle.force_vector * activation * 100.0; // Scale up force
                    world.particles[muscle.particle_idx].force += force;
                }
            }
        }
    }
}
