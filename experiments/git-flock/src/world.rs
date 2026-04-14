use crate::boid::Boid;
use crate::git::Commit;
use flocking::{compute_force, FlockingParams};
use locus::Vec2;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub struct World {
    pub boids: Vec<Boid>,
    pub width: f64,
    pub height: f64,
    pub commit_targets: Vec<Vec2>,
    pub current_commit: usize,
}

impl World {
    pub fn new(width: f64, height: f64, commits: &[Commit]) -> Self {
        let mut boids = Vec::new();
        // Create a flock of 100 agents
        for _ in 0..100 {
            boids.push(Boid::new(width / 2.0, height / 2.0));
        }

        // Map commits to target positions
        let mut commit_targets = Vec::new();
        for commit in commits {
            let mut hasher = DefaultHasher::new();
            commit.hash.hash(&mut hasher);
            let hash = hasher.finish();

            // Map the hash to x, y coordinates
            let x = (hash % 1000) as f64 / 1000.0 * width;
            let y = ((hash / 1000) % 1000) as f64 / 1000.0 * height;
            commit_targets.push(Vec2::new(x, y));
        }

        Self {
            boids,
            width,
            height,
            commit_targets,
            current_commit: 0,
        }
    }

    pub fn update(&mut self) {
        let count = self.boids.len();

        // Extract physics states for the flocking algorithm
        let positions: Vec<Vec2> = self.boids.iter().map(|b| b.position).collect();
        let velocities: Vec<Vec2> = self.boids.iter().map(|b| b.velocity).collect();

        let mut forces = Vec::with_capacity(count);

        let target = if self.commit_targets.is_empty() {
            Vec2::new(self.width / 2.0, self.height / 2.0)
        } else {
            self.commit_targets[self.current_commit]
        };

        for (i, boid) in self.boids.iter().enumerate() {
            // 1. Calculate Flocking Force
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

            // 2. Attraction to the current commit hotspot
            let mut seek_force = target - boid.position;
            if seek_force.magnitude_squared() > 0.0 {
                seek_force = seek_force.normalize() * boid.dna.max_speed;
                seek_force -= boid.velocity;
                seek_force = seek_force.limit(boid.dna.max_force * 1.5); // stronger pull towards target
            }

            forces.push(flocking_force + seek_force);
        }

        // Apply updates
        for (i, boid) in self.boids.iter_mut().enumerate() {
            boid.apply_force(forces[i]);
            boid.update_physics(self.width, self.height);
        }
    }
}
