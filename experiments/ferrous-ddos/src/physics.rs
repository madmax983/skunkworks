use crate::simulation::World;
use ::rand::Rng;
use macroquad::prelude::*;
use rayon::prelude::*;

pub struct Particle {
    pub position: Vec2,
    pub velocity: Vec2,
    pub mass: f32,
    pub magnetic_charge: f32,
}

pub struct ParticleSystem {
    pub particles: Vec<Particle>,
    pub max_particles: usize,
}

impl ParticleSystem {
    pub fn new() -> Self {
        Self {
            particles: Vec::with_capacity(10000),
            max_particles: 10000,
        }
    }

    pub fn add_particle(&mut self, position: Vec2) {
        if self.particles.len() < self.max_particles {
            let mut rng = ::rand::thread_rng();
            self.particles.push(Particle {
                position,
                velocity: Vec2::ZERO,
                mass: rng.gen_range(0.8..1.2),
                magnetic_charge: rng.gen_range(0.5..1.5), // Packets act like repelling magnets
            });
        }
    }

    pub fn update(&mut self, dt: f32, world: &World, server_pos: Vec2) {
        let dt = dt.min(0.05); // cap dt

        // 1. Calculate forces (parallel)
        let forces: Vec<Vec2> = self
            .particles
            .par_iter()
            .enumerate()
            .map(|(i, p)| {
                let mut force = Vec2::ZERO;

                // Target Attraction (Server pulling packets in)
                let dir = server_pos - p.position;
                let dist = dir.length();
                if dist > 0.0 {
                    // Server exerts strong magnetic pull
                    let server_pull = 1000.0 / (dist + 10.0);
                    force += (dir / dist) * server_pull;
                }

                // Firewall Repulsion
                for firewall in &world.firewalls {
                    let fw_dir = p.position - firewall.position;
                    let fw_dist = fw_dir.length();
                    if fw_dist < firewall.radius * 2.0 {
                        let repulsion = firewall.repulsion_strength / (fw_dist * fw_dist + 1.0);
                        force += (fw_dir / fw_dist) * repulsion;
                    }
                }

                // Magnetic Repulsion between particles (The Bottleneck Fluid Dynamics)
                // To keep it O(N) instead of O(N^2), we can use a simplified spatial hash or just sample neighbors.
                // For simplicity in this experiment, we sample a subset.
                // A spatial grid would be better for true fluid dynamics, but let's do a simple bounded N^2 for demonstration if N is small, or sampling if N is large.
                // If N=10000, N^2 is 100,000,000. That's too much for every frame.
                // Let's implement a rudimentary O(N^2) restricted to a smaller radius, or just sample 50 random other particles.
                let mut rng = ::rand::thread_rng();
                let num_samples = 30; // 30 random samples for local density estimation
                let particles_slice = self.particles.as_slice(); // safe slice access
                let len = self.particles.len();

                for _ in 0..num_samples {
                    let j = rng.gen_range(0..len);
                    if i != j {
                        // Safe and bounds-checked
                        let other_p = &particles_slice[j];
                        let diff = p.position - other_p.position;
                        let dist_sq = diff.length_squared();
                        if dist_sq < 400.0 && dist_sq > 0.1 {
                            // interaction radius squared (20.0)
                            let dist = dist_sq.sqrt();
                            let magnetic_repulsion =
                                (p.magnetic_charge * other_p.magnetic_charge * 50.0) / (dist_sq);
                            force += (diff / dist) * magnetic_repulsion;
                        }
                    }
                }

                // Damping (network resistance)
                force -= p.velocity * 0.5;

                force
            })
            .collect();

        // 2. Apply forces and integrate (parallel)
        self.particles
            .par_iter_mut()
            .zip(forces.into_par_iter())
            .for_each(|(p, force)| {
                p.velocity += (force / p.mass) * dt;
                p.position += p.velocity * dt;

                // Speed limit
                let max_speed = 300.0;
                if p.velocity.length_squared() > max_speed * max_speed {
                    p.velocity = p.velocity.normalize() * max_speed;
                }
            });
    }
}
