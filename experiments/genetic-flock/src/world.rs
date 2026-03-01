use crate::boid::{Boid, BoidDna};
use chimera_lang::prelude::*;
use locus::Vec2;
use locus::flocking::{FlockingParams, compute_force};

pub struct World {
    pub boids: Vec<Boid>,
    pub width: f64,
    pub height: f64,
    pub crossover_radius: f64,
}

impl World {
    pub fn new(width: f64, height: f64) -> Self {
        let mut boids = Vec::new();
        for _ in 0..100 {
            boids.push(Boid::new(width / 2.0, height / 2.0));
        }
        Self {
            boids,
            width,
            height,
            crossover_radius: 2.0, // Interaction distance for reproduction
        }
    }

    pub fn update(&mut self) {
        let count = self.boids.len();

        // Flocking variables
        let positions: Vec<Vec2> = self.boids.iter().map(|b| b.position).collect();
        let velocities: Vec<Vec2> = self.boids.iter().map(|b| b.velocity).collect();

        let mut forces = Vec::with_capacity(count);

        for (i, boid) in self.boids.iter().enumerate() {
            let params = FlockingParams {
                view_radius: boid.dna.view_radius,
                separation_radius: boid.dna.view_radius / 2.0,
                max_speed: boid.dna.max_speed,
                max_force: boid.dna.max_force,
                separation_weight: boid.dna.separation_weight,
                alignment_weight: boid.dna.alignment_weight,
                cohesion_weight: boid.dna.cohesion_weight,
            };

            let flocking_force = compute_force(&positions, &velocities, i, &params);
            forces.push(flocking_force);
        }

        // Apply forces
        for (i, boid) in self.boids.iter_mut().enumerate() {
            boid.apply_force(forces[i]);
            boid.update_physics(self.width, self.height);
        }

        // Handle Crossover
        self.handle_crossovers();

        // Ensure population doesn't completely die if energy depletes (just reset them)
        for boid in self.boids.iter_mut() {
            if boid.energy <= 0.0 {
                // Mutate heavily to try something new, reset energy
                boid.energy = 50.0;
                boid.dna = BoidDna::random();
                boid.generation = 1;
            }
        }
    }

    fn handle_crossovers(&mut self) {
        let count = self.boids.len();
        let mut crossover_partners: Vec<Option<(BoidDna, Dna)>> = vec![None; count];

        let crossover_radius_sq = self.crossover_radius.powi(2);

        // Look for pairs to crossover
        for i in 0..count {
            if crossover_partners[i].is_some() || self.boids[i].energy < 50.0 {
                continue; // Already partnered or too tired to mate
            }

            for j in (i + 1)..count {
                if crossover_partners[j].is_none() && self.boids[j].energy >= 50.0 {
                    let d2 = self.boids[i].position.distance_squared(self.boids[j].position);
                    if d2 < crossover_radius_sq {
                        crossover_partners[i] = Some((self.boids[j].dna.clone(), self.boids[j].chimera_dna.clone()));
                        crossover_partners[j] = Some((self.boids[i].dna.clone(), self.boids[i].chimera_dna.clone()));
                        break;
                    }
                }
            }
        }

        // Apply crossovers
        for i in 0..count {
            if let Some((partner_dna, partner_chimera)) = &crossover_partners[i] {
                self.boids[i].crossover_and_mutate(partner_dna, partner_chimera);
            }
        }
    }
}
