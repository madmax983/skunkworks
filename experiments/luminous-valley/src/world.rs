use crate::boid::Boid;
use macroquad::prelude::*;

pub struct World {
    pub boids: Vec<Boid>,
    pub width: f32,
    pub depth: f32,
}

impl World {
    pub fn new(width: f32, depth: f32, num_boids: usize) -> Self {
        let mut w = Self {
            boids: Vec::with_capacity(num_boids),
            width,
            depth,
        };

        for _ in 0..num_boids {
            let x = rand::gen_range(-width / 2.0, width / 2.0);
            let z = rand::gen_range(-depth / 2.0, depth / 2.0);
            w.boids.push(Boid::new(x, z));
        }
        w
    }

    pub fn update(&mut self, heightmap: &Image, chemical_map: Option<&Image>) {
        let count = self.boids.len();
        let mut forces = Vec::with_capacity(count);
        let mut phase_nudges = vec![0.0; count];

        // 1. Calculate Flocking & Coupling
        for i in 0..count {
            let mut sep = Vec3::ZERO;
            let mut ali = Vec3::ZERO;
            let mut coh = Vec3::ZERO;

            let mut sep_count = 0;
            let mut ali_count = 0;
            let mut coh_count = 0;

            let mut nudge = 0.0;

            let p1 = self.boids[i].position;
            let v1 = self.boids[i].velocity;
            let dna = self.boids[i].dna.clone();

            for j in 0..count {
                if i == j {
                    continue;
                }

                let p2 = self.boids[j].position;
                let d_sq = p1.distance_squared(p2);

                if d_sq < dna.view_radius.powi(2) {
                    // Separation
                    if d_sq < (dna.view_radius / 2.0).powi(2) {
                        let diff = p1 - p2;
                        if d_sq > 0.001 {
                            sep += diff / d_sq;
                            sep_count += 1;
                        }
                    }

                    // Alignment
                    ali += self.boids[j].velocity;
                    ali_count += 1;

                    // Cohesion
                    coh += p2;
                    coh_count += 1;
                }

                // Firefly Coupling
                if d_sq < dna.coupling_radius.powi(2) {
                    // If neighbor is flashing (timer > 5), it pulls us
                    if self.boids[j].flash_timer > 5 {
                        nudge += dna.coupling_strength;
                    }
                }
            }

            let mut total_force = Vec3::ZERO;

            if sep_count > 0 {
                sep = sep.normalize_or_zero() * dna.max_speed;
                sep -= v1;
                sep = sep.clamp_length_max(dna.max_force);
                total_force += sep * dna.separation_weight;
            }

            if ali_count > 0 {
                ali /= ali_count as f32;
                ali = ali.normalize_or_zero() * dna.max_speed;
                ali -= v1;
                ali = ali.clamp_length_max(dna.max_force);
                total_force += ali * dna.alignment_weight;
            }

            if coh_count > 0 {
                coh /= coh_count as f32;
                let desired = coh - p1;
                let desired = desired.normalize_or_zero() * dna.max_speed;
                let steer = (desired - v1).clamp_length_max(dna.max_force);
                total_force += steer * dna.cohesion_weight;
            }

            forces.push(total_force);
            phase_nudges[i] = nudge;
        }

        // 2. Apply Updates & Terrain Interaction
        for (i, boid) in self.boids.iter_mut().enumerate() {
            // Physics
            boid.apply_force(forces[i]);
            boid.update_physics(self.width, self.depth);

            // Terrain Height Following
            // Map position to UV
            let u = (boid.position.x + self.width / 2.0) / self.width;
            let v = (boid.position.z + self.depth / 2.0) / self.depth;
            let u = u.clamp(0.0, 0.999);
            let v = v.clamp(0.0, 0.999);

            let px = (u * heightmap.width as f32) as u32;
            let py = (v * heightmap.height as f32) as u32;

            let h_val = heightmap.get_pixel(px, py).r;

            // Target altitude: Height * Scale + Offset
            // Valley Forge height scale is 4.0
            let target_y = h_val * 4.0 + 2.0;

            // Smoothly move Y to target
            boid.position.y += (target_y - boid.position.y) * 0.1;

            // Chemical Sensing
            if let Some(chem) = chemical_map {
                let chem_val = chem.get_pixel(px, py).g; // Green channel is B chemical
                boid.chemical_exposure = chem_val;
            }

            // Phase Update
            // Freq increases with chemical exposure
            let freq_mult = 1.0 + boid.chemical_exposure * 2.0;
            boid.phase += (boid.dna.natural_freq * freq_mult as f64) + phase_nudges[i];

            boid.update_flash();
        }
    }
}
