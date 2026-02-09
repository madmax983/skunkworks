use crate::phonology::{Evolver, SoundLaw};
use rand::Rng;
use ratatui::style::Color;
use std::f64::consts::TAU;
pub use locus::Vec2;

#[derive(Clone, Debug)]
pub struct DNA {
    pub max_speed: f64,
    pub max_force: f64,
    pub view_radius: f64,
    pub separation_weight: f64,
    pub alignment_weight: f64,
    pub cohesion_weight: f64,
    pub color: Color,
    pub word: String,
    pub original_word: String,
}

impl DNA {
    pub fn new(word: String) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            max_speed: rng.gen_range(0.5..1.5),
            max_force: rng.gen_range(0.02..0.1),
            view_radius: rng.gen_range(5.0..15.0),
            separation_weight: rng.gen_range(1.0..2.0),
            alignment_weight: rng.gen_range(0.8..1.2),
            cohesion_weight: rng.gen_range(0.8..1.2),
            color: Color::White,
            word: word.clone(),
            original_word: word,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Boid {
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub dna: DNA,
    pub energy: f64,
}

impl Boid {
    pub fn new(x: f64, y: f64, word: String) -> Self {
        let mut rng = rand::thread_rng();
        let angle = rng.gen_range(0.0..TAU);
        let dna = DNA::new(word);

        Self {
            position: Vec2::new(x, y),
            velocity: Vec2::new(angle.cos() * dna.max_speed, angle.sin() * dna.max_speed),
            acceleration: Vec2::zero(),
            dna,
            energy: 100.0,
        }
    }

    pub fn update(&mut self, width: f64, height: f64) {
        self.velocity += self.acceleration;
        self.velocity = self.velocity.limit(self.dna.max_speed);

        self.position += self.velocity;

        // Reset acceleration
        self.acceleration = Vec2::zero();

        // Wrap around edges
        if self.position.x < 0.0 {
            self.position.x += width;
        }
        if self.position.x >= width {
            self.position.x -= width;
        }
        if self.position.y < 0.0 {
            self.position.y += height;
        }
        if self.position.y >= height {
            self.position.y -= height;
        }

        // Decay energy
        self.energy -= 0.05;
    }

    pub fn apply_force(&mut self, force: Vec2) {
        self.acceleration += force;
    }

    pub fn evolve(&mut self, law: SoundLaw) {
        let mut evolver = Evolver::new();
        evolver.add_law(law);
        let new_word = evolver.evolve(&self.dna.word);
        self.dna.word = new_word;
    }

    // Returns the force vector to be applied
    pub fn calculate_flocking_force(&self, boids: &[Boid]) -> Vec2 {
        let separation = self.calculate_separation(boids);
        let alignment = self.calculate_alignment(boids);
        let cohesion = self.calculate_cohesion(boids);

        separation * self.dna.separation_weight
            + alignment * self.dna.alignment_weight
            + cohesion * self.dna.cohesion_weight
    }

    fn calculate_separation(&self, boids: &[Boid]) -> Vec2 {
        let mut steer = Vec2::zero();
        let mut count = 0;
        let separation_radius_sq = (self.dna.view_radius / 2.0).powi(2);

        for other in boids {
            let d_sq = self.position.distance_squared(other.position);
            if d_sq > 0.0 && d_sq < separation_radius_sq {
                let diff = self.position - other.position;
                steer += diff / d_sq;
                count += 1;
            }
        }

        if count > 0 && steer.magnitude_squared() > 0.0 {
            steer = steer.normalize() * self.dna.max_speed;
            steer -= self.velocity;
            steer = steer.limit(self.dna.max_force);
            steer
        } else {
            Vec2::zero()
        }
    }

    fn calculate_alignment(&self, boids: &[Boid]) -> Vec2 {
        let mut sum = Vec2::zero();
        let mut count = 0;
        let view_radius_sq = self.dna.view_radius.powi(2);

        for other in boids {
            let d_sq = self.position.distance_squared(other.position);
            if d_sq > 0.0 && d_sq < view_radius_sq {
                sum += other.velocity;
                count += 1;
            }
        }

        if count > 0 {
            sum /= count as f64;
            if sum.magnitude_squared() > 0.0 {
                sum = sum.normalize() * self.dna.max_speed;
                sum -= self.velocity;
                sum = sum.limit(self.dna.max_force);
                return sum;
            }
        }
        Vec2::zero()
    }

    fn calculate_cohesion(&self, boids: &[Boid]) -> Vec2 {
        let mut sum = Vec2::zero();
        let mut count = 0;
        let view_radius_sq = self.dna.view_radius.powi(2);

        for other in boids {
            let d_sq = self.position.distance_squared(other.position);
            if d_sq > 0.0 && d_sq < view_radius_sq {
                sum += other.position;
                count += 1;
            }
        }

        if count > 0 {
            sum /= count as f64;
            return self.seek(sum);
        }
        Vec2::zero()
    }

    fn seek(&self, target: Vec2) -> Vec2 {
        let mut desired = target - self.position;
        if desired.magnitude_squared() > 0.0 {
            desired = desired.normalize() * self.dna.max_speed;
            desired -= self.velocity;
            desired = desired.limit(self.dna.max_force);
            desired
        } else {
            Vec2::zero()
        }
    }
}

// Deprecated or wrappers
pub fn distance(p1: Vec2, p2: Vec2) -> f64 {
    p1.distance(p2)
}

pub fn distance_squared(p1: Vec2, p2: Vec2) -> f64 {
    p1.distance_squared(p2)
}
