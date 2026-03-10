use crate::boid::Boid;
use crate::blame::LineInfo;
use locus::Vec2;
use rand::Rng;
use locus::flocking::{FlockingParams, compute_force};
use std::f64::consts::PI;

pub struct World {
    pub boids: Vec<Boid>,
    pub width: f64,
    pub height: f64,
    pub blame_info: Vec<LineInfo>,
}

impl World {
    pub fn new(width: f64, height: f64, blame_info: Vec<LineInfo>) -> Self {
        let mut boids = Vec::new();
        // Create a swarm
        for _ in 0..150 {
            boids.push(Boid::new(width / 2.0, height / 2.0));
        }
        Self {
            boids,
            width,
            height,
            blame_info,
        }
    }

    pub fn update(&mut self) {
        let count = self.boids.len();

        // Extract physics states for the flocking algorithm
        let positions: Vec<Vec2> = self.boids.iter().map(|b| b.position).collect();
        let velocities: Vec<Vec2> = self.boids.iter().map(|b| b.velocity).collect();

        let mut forces = Vec::with_capacity(count);
        let mut phase_nudges = vec![0.0; count];

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

            let mut flocking_force = compute_force(&positions, &velocities, i, &params);

            // 2. Add Force from Code Age
            let pos = boid.position;
            let current_line = pos.y.floor() as usize;

            // Find nearest lines to consider for age force
            let mut age_force = Vec2::zero();
            let force_range = 5;
            let start_line = current_line.saturating_sub(force_range);
            let end_line = (current_line + force_range).min(self.blame_info.len());

            for line_idx in start_line..end_line {
                if let Some(info) = self.blame_info.get(line_idx) {
                    let score = info.age_score; // 0.0 is oldest, 1.0 is newest

                    let line_y = line_idx as f64 + 0.5;
                    let to_line = Vec2::new(pos.x, line_y) - pos;
                    let dist_sq = to_line.x * to_line.x + to_line.y * to_line.y;

                    if dist_sq > 0.1 {
                        // Attraction to new code, repulsion from old code
                        let magnitude = (score - 0.5) * 0.1 / dist_sq.sqrt();
                        age_force += to_line.normalize() * magnitude;
                    }
                }
            }

            flocking_force += age_force;
            forces.push(flocking_force);

            // Phase synchronization logic
            let mut nudge = 0.0;
            let coupling_radius_sq = boid.dna.coupling_radius.powi(2);
            let p1 = boid.position();

            for (j, other) in self.boids.iter().enumerate() {
                if i == j {
                    continue;
                }

                if p1.distance_squared(other.position()) < coupling_radius_sq {
                    if other.flash_timer == 5 {
                        nudge += boid.dna.coupling_strength;
                    }
                }
            }
            phase_nudges[i] = nudge;
        }

        // Apply updates
        for (i, boid) in self.boids.iter_mut().enumerate() {
            boid.apply_force(forces[i]);
            boid.update_physics(self.width, self.height);

            boid.phase += boid.dna.natural_freq + phase_nudges[i];
            boid.update_flash();
        }
    }

    pub fn synchronization_index(&self) -> f64 {
        let mut sum_sin = 0.0;
        let mut sum_cos = 0.0;

        for b in &self.boids {
            let theta = b.phase * 2.0 * PI;
            sum_sin += theta.sin();
            sum_cos += theta.cos();
        }

        let n = self.boids.len() as f64;
        if n == 0.0 {
            return 0.0;
        }
        ((sum_sin / n).powi(2) + (sum_cos / n).powi(2)).sqrt()
    }
}
