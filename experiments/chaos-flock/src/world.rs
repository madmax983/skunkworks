use crate::boid::Boid;
use crate::chaos::DoublePendulum;
use locus::Vec2;
use locus::flocking::{FlockingParams, compute_force};

pub struct World {
    pub boids: Vec<Boid>,
    pub width: f64,
    pub height: f64,
    pub pendulum: DoublePendulum,
}

impl World {
    pub fn new(width: f64, height: f64) -> Self {
        let mut boids = Vec::new();
        // Create a swarm
        for _ in 0..150 {
            boids.push(Boid::new(width / 2.0, height / 2.0));
        }
        let pendulum = DoublePendulum::new(Vec2::new(width / 2.0, height / 2.0));
        Self {
            boids,
            width,
            height,
            pendulum,
        }
    }

    pub fn kick_pendulum(&mut self) {
        let mut rng = rand::thread_rng();
        use rand::Rng;
        self.pendulum.a1_v += rng.gen_range(-10.0..10.0);
        self.pendulum.a2_v += rng.gen_range(-10.0..10.0);
    }

    pub fn update(&mut self) {
        let count = self.boids.len();

        // Update chaotic pendulum
        self.pendulum.update(0.1);
        let target = self.pendulum.p2(); // Boids track the outer bob

        // Extract physics states for the flocking algorithm
        let positions: Vec<Vec2> = self.boids.iter().map(|b| b.position).collect();
        let velocities: Vec<Vec2> = self.boids.iter().map(|b| b.velocity).collect();

        let mut forces = Vec::with_capacity(count);

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

            let mut force = compute_force(&positions, &velocities, i, &params);

            // 2. Attractor Force (chasing the chaotic pendulum)
            let desired = target - boid.position;
            let d = desired.magnitude();
            if d > 0.0 {
                let speed = if d < 20.0 {
                    boid.dna.max_speed * (d / 20.0) // Arrive behavior
                } else {
                    boid.dna.max_speed
                };
                let steer = (desired.normalize() * speed) - boid.velocity;
                let attractor_force = steer.limit(boid.dna.max_force) * boid.dna.attractor_weight;
                force += attractor_force;
            }

            forces.push(force);
        }

        // Apply updates
        for (i, boid) in self.boids.iter_mut().enumerate() {
            // Apply physics
            boid.apply_force(forces[i]);
            boid.update_physics(self.width, self.height);
        }
    }
}
