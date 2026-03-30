use crate::boid::Boid;
use locus::Vec2;
use std::f64::consts::PI;

pub struct World {
    pub boids: Vec<Boid>,
    pub width: f64,
    pub height: f64,
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
        }
    }

    pub fn update(&mut self) {
        let count = self.boids.len();

        let positions: Vec<Vec2> = self.boids.iter().map(|b| b.position).collect();
        let velocities: Vec<Vec2> = self.boids.iter().map(|b| b.velocity).collect();

        for i in 0..count {
            let mut align = Vec2::zero();
            let mut cohere = Vec2::zero();
            let mut separate = Vec2::zero();

            let mut neighbors = 0;
            let boid = &self.boids[i];

            for j in 0..count {
                if i == j {
                    continue;
                }

                let dist = boid.position.distance(positions[j]);
                if dist < boid.dna.view_radius {
                    align += velocities[j];
                    cohere += positions[j];
                    if dist < boid.dna.view_radius / 2.0 {
                        separate += boid.position - positions[j];
                    }
                    neighbors += 1;
                }
            }

            if neighbors > 0 {
                align /= neighbors as f64;
                cohere = (cohere / neighbors as f64) - boid.position;
            }

            // Map distances/vectors to neural input current
            let mut inputs = vec![0.0; boid.brain.neurons.len()];

            // Align
            inputs[boid.sensory_neurons[0]] = align.x.max(0.0) as f32 * 10.0;
            inputs[boid.sensory_neurons[1]] = align.x.min(0.0).abs() as f32 * 10.0;
            inputs[boid.sensory_neurons[2]] = align.y.max(0.0) as f32 * 10.0;
            inputs[boid.sensory_neurons[3]] = align.y.min(0.0).abs() as f32 * 10.0;

            // Cohere
            inputs[boid.sensory_neurons[4]] = cohere.x.max(0.0) as f32 * 10.0;
            inputs[boid.sensory_neurons[5]] = cohere.x.min(0.0).abs() as f32 * 10.0;
            inputs[boid.sensory_neurons[6]] = cohere.y.max(0.0) as f32 * 10.0;
            inputs[boid.sensory_neurons[7]] = cohere.y.min(0.0).abs() as f32 * 10.0;

            // Separate
            inputs[boid.sensory_neurons[8]] = separate.x.max(0.0) as f32 * 10.0;
            inputs[boid.sensory_neurons[9]] = separate.x.min(0.0).abs() as f32 * 10.0;
            inputs[boid.sensory_neurons[10]] = separate.y.max(0.0) as f32 * 10.0;
            inputs[boid.sensory_neurons[11]] = separate.y.min(0.0).abs() as f32 * 10.0;

            let boid = &mut self.boids[i];

            // Step the brain
            boid.brain.step(&inputs);

            // Output steering force from motor spikes
            let mut force_x = 0.0;
            let mut force_y = 0.0;

            if boid.brain.spikes[boid.motor_neurons[0]] { force_x += 1.0; }
            if boid.brain.spikes[boid.motor_neurons[1]] { force_x -= 1.0; }
            if boid.brain.spikes[boid.motor_neurons[2]] { force_y += 1.0; }
            if boid.brain.spikes[boid.motor_neurons[3]] { force_y -= 1.0; }

            boid.apply_force(Vec2::new(force_x, force_y));
        }

        // Apply physics
        for boid in self.boids.iter_mut() {
            boid.update_physics(self.width, self.height);
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