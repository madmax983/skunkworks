use flocking::{FlockingParams, PhysicsState, compute_force};
use locus::Vec2;
use rand::Rng;
use ratatui::style::Color;
use std::f64::consts::{PI, TAU};

#[derive(Clone, Debug)]
pub struct Dna {
    pub max_speed: f64,
    pub max_force: f64,
    pub view_radius: f64,
    pub coupling_radius: f64,
    pub separation_weight: f64,
    pub alignment_weight: f64,
    pub cohesion_weight: f64,
    pub flee_weight: f64, // New trait for fleeing predator
    pub natural_freq: f64,
    pub coupling_strength: f64,
    pub color: Color,
    pub char_representation: char,
}

impl Dna {
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            max_speed: rng.gen_range(0.8..1.5),
            max_force: rng.gen_range(0.05..0.15),
            view_radius: rng.gen_range(10.0..20.0),
            coupling_radius: rng.gen_range(15.0..30.0),
            separation_weight: rng.gen_range(1.2..2.5),
            alignment_weight: rng.gen_range(0.8..1.2),
            cohesion_weight: rng.gen_range(0.8..1.2),
            flee_weight: rng.gen_range(3.0..5.0), // Strong flee instinct
            natural_freq: 0.005 + rng.gen::<f64>() * 0.02,
            coupling_strength: 0.005,
            color: Color::Indexed(rng.gen_range(20..230)),
            char_representation: if rng.gen_bool(0.5) { '✦' } else { '•' },
        }
    }
}

#[derive(Clone, Debug)]
pub struct Boid {
    pub physics: PhysicsState,
    pub dna: Dna,
    pub phase: f64,
    pub flash_timer: usize,
}

impl Boid {
    pub fn new(x: f64, y: f64) -> Self {
        let mut rng = rand::thread_rng();
        let angle = rng.gen_range(0.0..TAU);
        let dna = Dna::random();

        let mut physics = PhysicsState::new(x, y);
        physics.velocity = Vec2::new(angle.cos() * dna.max_speed, angle.sin() * dna.max_speed);

        Self {
            physics,
            dna,
            phase: rng.gen::<f64>(),
            flash_timer: 0,
        }
    }

    pub fn position(&self) -> Vec2 {
        self.physics.position
    }

    pub fn apply_force(&mut self, force: Vec2) {
        self.physics.apply_force(force);
    }

    pub fn update_physics(&mut self, width: f64, height: f64) {
        self.physics.update(self.dna.max_speed);

        // Wrap around edges
        if self.physics.position.x < 0.0 {
            self.physics.position.x += width;
        }
        if self.physics.position.x >= width {
            self.physics.position.x -= width;
        }
        if self.physics.position.y < 0.0 {
            self.physics.position.y += height;
        }
        if self.physics.position.y >= height {
            self.physics.position.y -= height;
        }
    }

    pub fn update_flash(&mut self) {
        if self.flash_timer > 0 {
            self.flash_timer -= 1;
        }
        if self.phase >= 1.0 {
            self.phase -= 1.0;
            self.flash_timer = 5;
        }
    }
}

pub struct World {
    pub boids: Vec<Boid>,
    pub width: f64,
    pub height: f64,
}

impl World {
    pub fn new(width: f64, height: f64) -> Self {
        let mut boids = Vec::new();
        for _ in 0..150 {
            boids.push(Boid::new(width / 2.0, height / 2.0));
        }
        Self {
            boids,
            width,
            height,
        }
    }

    pub fn update(&mut self, predator_pos: Option<Vec2>) {
        let count = self.boids.len();
        let physics_states: Vec<PhysicsState> = self.boids.iter().map(|b| b.physics).collect();
        let mut forces = Vec::with_capacity(count);
        let mut phase_nudges = vec![0.0; count];

        for (i, boid) in self.boids.iter().enumerate() {
            // 1. Standard Flocking Force
            let params = FlockingParams {
                view_radius: boid.dna.view_radius,
                separation_radius: boid.dna.view_radius / 2.0,
                max_speed: boid.dna.max_speed,
                max_force: boid.dna.max_force,
                separation_weight: boid.dna.separation_weight,
                alignment_weight: boid.dna.alignment_weight,
                cohesion_weight: boid.dna.cohesion_weight,
            };

            let mut force = compute_force(&physics_states, i, &params);

            // 2. Flee from Predator
            if let Some(pred_pos) = predator_pos {
                let dist_sq = boid.position().distance_squared(pred_pos);
                let flee_radius = 20.0; // Distance at which fear kicks in
                if dist_sq < flee_radius * flee_radius && dist_sq > 0.0 {
                    let diff = boid.position() - pred_pos;
                    let dist = diff.magnitude();
                    // Inverse proportional to distance
                    let strength = (flee_radius - dist) / flee_radius;
                    let flee_force = diff.normalize() * boid.dna.max_force * boid.dna.flee_weight * strength * 5.0;
                    force += flee_force;
                }
            }

            forces.push(force);

            // 3. Firefly Synchronization
            let mut nudge = 0.0;
            let coupling_radius_sq = boid.dna.coupling_radius.powi(2);
            let p1 = boid.position();

            for (j, other) in self.boids.iter().enumerate() {
                if i == j { continue; }
                if p1.distance_squared(other.position()) < coupling_radius_sq {
                    if other.flash_timer == 5 {
                        nudge += boid.dna.coupling_strength;
                    }
                }
            }
            phase_nudges[i] = nudge;
        }

        for (i, boid) in self.boids.iter_mut().enumerate() {
            boid.apply_force(forces[i]);
            boid.update_physics(self.width, self.height);
            boid.phase += boid.dna.natural_freq + phase_nudges[i];
            boid.update_flash();
        }
    }

    pub fn center_of_mass(&self) -> Vec2 {
        if self.boids.is_empty() {
            return Vec2::new(self.width / 2.0, self.height / 2.0);
        }
        let mut sum = Vec2::zero();
        for boid in &self.boids {
            sum += boid.position();
        }
        sum / (self.boids.len() as f64)
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
        if n == 0.0 { return 0.0; }
        ((sum_sin / n).powi(2) + (sum_cos / n).powi(2)).sqrt()
    }
}
