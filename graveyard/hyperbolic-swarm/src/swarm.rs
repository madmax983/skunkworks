use crate::boid::Boid;
use chimera_lang::prelude::*;

pub struct Swarm {
    pub boids: Vec<Boid>,
}

impl Swarm {
    pub fn new(count: usize) -> Self {
        let dna = Dna {
            helix: Helix {
                strands: vec![], // Empty for now, logic is in flocking
            },
            evolution_config: None,
        };

        let mut boids = Vec::new();
        for _ in 0..count {
            boids.push(Boid::new(dna.clone()));
        }

        Self { boids }
    }

    pub fn update(&mut self) {
        // Compute new states
        let prev_state = self.boids.clone();

        for i in 0..self.boids.len() {
            // We need to calculate flocking based on prev state, but apply to self.
            // Boid::calculate_flocking mutates self.vel
            self.boids[i].calculate_flocking(&prev_state);
            self.boids[i].update_physics();
        }
    }
}
