use crate::boid::Boid;
use glam::Vec2;
use std::f32::consts::TAU;

pub struct World {
    pub boids: Vec<Boid>,
}

impl World {
    pub fn new() -> Self {
        let mut boids = Vec::new();
        let width = TAU;
        let height = TAU;

        // Spawn boids
        for _ in 0..150 {
            // Spawn somewhat centrally to avoid immediate boundary issues
            boids.push(Boid::new(width / 2.0, height / 2.0));
        }

        Self { boids }
    }

    pub fn update(&mut self) {
        let count = self.boids.len();
        let mut physics_forces = Vec::with_capacity(count);
        let mut phase_nudges = vec![0.0; count];

        // 1. Calculate forces
        for i in 0..count {
            let mut separation = Vec2::ZERO;
            let mut alignment = Vec2::ZERO;
            let mut cohesion = Vec2::ZERO;

            let mut sep_count = 0;
            let mut ali_count = 0;
            let mut coh_count = 0;

            let mut nudge = 0.0;

            let p1 = self.boids[i].position;
            let v1 = self.boids[i].velocity;
            let dna = &self.boids[i].dna;

            let view_radius_sq = dna.view_radius.powi(2);
            let coupling_radius_sq = dna.coupling_radius.powi(2);
            let separation_radius_sq = (dna.view_radius / 2.0).powi(2);

            for j in 0..count {
                if i == j {
                    continue;
                }

                let p2_orig = self.boids[j].position;
                let v2_orig = self.boids[j].velocity;
                let b2_flash = self.boids[j].flash_timer;

                // Find closest ghost of p2
                let mut best_p2 = p2_orig;
                let mut best_v2 = v2_orig;
                let mut min_dist_sq = f32::MAX;

                // Candidates:
                // 1. Direct
                let candidates = [
                    (p2_orig, v2_orig),
                    // V-Wrap (Torus style)
                    (p2_orig + Vec2::new(0.0, TAU), v2_orig),
                    (p2_orig - Vec2::new(0.0, TAU), v2_orig),
                    // U-Wrap (Klein style: Flip V)
                    // If we wrap U (+2PI), V becomes (2PI - v)
                    // Wait, standard Klein wrapping maps (u+2pi, v) -> (u, 2pi-v).
                    // So relative to p1 (u), if p2 is at (u+delta), it might be effectively at (u+delta-2pi, 2pi-v).
                    // We want to find a ghost of p2 that is close to p1.

                    // Case: p2 is "physically" to the right of p1 across the boundary
                    // Ghost pos: p2 - 2PI (in U), but V is flipped.
                    (
                        Vec2::new(p2_orig.x - TAU, TAU - p2_orig.y),
                        Vec2::new(v2_orig.x, -v2_orig.y),
                    ),
                    // Case: p2 is to the left
                    (
                        Vec2::new(p2_orig.x + TAU, TAU - p2_orig.y),
                        Vec2::new(v2_orig.x, -v2_orig.y),
                    ),
                ];

                for (cand_p, cand_v) in candidates {
                    let d_sq = p1.distance_squared(cand_p);
                    if d_sq < min_dist_sq {
                        min_dist_sq = d_sq;
                        best_p2 = cand_p;
                        best_v2 = cand_v;
                    }
                }

                if min_dist_sq < view_radius_sq {
                    // Separation
                    if min_dist_sq < separation_radius_sq {
                        let diff = p1 - best_p2;
                        // Avoid div by zero
                        if min_dist_sq > 0.00001 {
                            separation += diff / min_dist_sq;
                            sep_count += 1;
                        }
                    }

                    // Alignment
                    alignment += best_v2;
                    ali_count += 1;

                    // Cohesion
                    cohesion += best_p2;
                    coh_count += 1;
                }

                // Firefly sync
                if min_dist_sq < coupling_radius_sq && b2_flash == 5 {
                    nudge += dna.coupling_strength;
                }
            }

            // Apply weights
            let mut total_force = Vec2::ZERO;

            if sep_count > 0 {
                // separation is already normalized sort of? No.
                // separation vector sum.
                if separation.length_squared() > 0.0 {
                    separation = separation.normalize() * dna.max_speed;
                    separation -= v1;
                    separation = separation.clamp_length_max(dna.max_force);
                    total_force += separation * dna.separation_weight;
                }
            }

            if ali_count > 0 {
                alignment /= ali_count as f32;
                if alignment.length_squared() > 0.0 {
                    alignment = alignment.normalize() * dna.max_speed;
                    alignment -= v1;
                    alignment = alignment.clamp_length_max(dna.max_force);
                    total_force += alignment * dna.alignment_weight;
                }
            }

            if coh_count > 0 {
                cohesion /= coh_count as f32;
                let mut desired = cohesion - p1;
                if desired.length_squared() > 0.0 {
                    desired = desired.normalize() * dna.max_speed;
                    desired -= v1;
                    desired = desired.clamp_length_max(dna.max_force);
                    total_force += desired * dna.cohesion_weight;
                }
            }

            physics_forces.push(total_force);
            phase_nudges[i] = nudge;
        }

        // 2. Apply updates and Wrap
        for (i, boid) in self.boids.iter_mut().enumerate() {
            boid.apply_force(physics_forces[i]);
            boid.update_physics();

            // Wrappings
            let mut p = boid.position;
            let mut v = boid.velocity;

            // U Wrap (Klein)
            if p.x >= TAU {
                p.x -= TAU;
                p.y = TAU - p.y;
                v.y = -v.y;
            } else if p.x < 0.0 {
                p.x += TAU;
                p.y = TAU - p.y;
                v.y = -v.y;
            }

            // V Wrap (Torus)
            if p.y >= TAU {
                p.y -= TAU;
            } else if p.y < 0.0 {
                p.y += TAU;
            }

            boid.position = p;
            boid.velocity = v;

            // Flash update
            boid.phase += boid.dna.natural_freq + phase_nudges[i];
            boid.update_flash();
        }
    }

    pub fn synchronization_index(&self) -> f32 {
        let mut sum_sin = 0.0;
        let mut sum_cos = 0.0;
        let n = self.boids.len() as f32;

        for b in &self.boids {
            let theta = b.phase * TAU;
            sum_sin += theta.sin();
            sum_cos += theta.cos();
        }

        if n == 0.0 {
            return 0.0;
        }
        ((sum_sin / n).powi(2) + (sum_cos / n).powi(2)).sqrt()
    }
}
