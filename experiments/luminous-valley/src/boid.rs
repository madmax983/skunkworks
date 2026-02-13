use macroquad::prelude::*;
use ::rand::Rng;
use std::f64::consts::TAU;

#[derive(Clone, Debug)]
pub struct Dna {
    pub max_speed: f32,
    pub max_force: f32,
    pub view_radius: f32,
    pub coupling_radius: f32,
    pub separation_weight: f32,
    pub alignment_weight: f32,
    pub cohesion_weight: f32,
    pub natural_freq: f64,
    pub coupling_strength: f64,
    pub color: Color,
}

impl Dna {
    pub fn random() -> Self {
        let mut rng = ::rand::thread_rng();
        Self {
            max_speed: rng.gen_range(0.05..0.15),
            max_force: rng.gen_range(0.002..0.008),
            view_radius: rng.gen_range(2.0..5.0),
            coupling_radius: rng.gen_range(3.0..6.0),
            separation_weight: rng.gen_range(1.5..2.5),
            alignment_weight: rng.gen_range(0.8..1.2),
            cohesion_weight: rng.gen_range(0.8..1.2),
            natural_freq: 0.01 + rng.gen::<f64>() * 0.03,
            coupling_strength: 0.02,
            color: Color::new(rng.gen_range(0.5..1.0), rng.gen_range(0.5..1.0), rng.gen_range(0.5..1.0), 1.0),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Boid {
    pub position: Vec3,
    pub velocity: Vec3,
    pub acceleration: Vec3,
    pub dna: Dna,
    pub phase: f64,
    pub flash_timer: usize,
    pub chemical_exposure: f32,
}

impl Boid {
    pub fn new(x: f32, z: f32) -> Self {
        let mut rng = ::rand::thread_rng();
        let angle = rng.gen_range(0.0..TAU as f32);
        let dna = Dna::random();

        Self {
            position: Vec3::new(x, 5.0, z),
            velocity: Vec3::new(angle.cos() * dna.max_speed, 0.0, angle.sin() * dna.max_speed),
            acceleration: Vec3::ZERO,
            dna,
            phase: rng.gen::<f64>(),
            flash_timer: 0,
            chemical_exposure: 0.0,
        }
    }

    pub fn apply_force(&mut self, force: Vec3) {
        self.acceleration += force;
    }

    pub fn update_physics(&mut self, width: f32, depth: f32) {
        self.velocity += self.acceleration;

        // Limit speed (XZ plane)
        let speed_sq = self.velocity.x * self.velocity.x + self.velocity.z * self.velocity.z;
        if speed_sq > self.dna.max_speed.powi(2) {
            let speed = speed_sq.sqrt();
            self.velocity.x = (self.velocity.x / speed) * self.dna.max_speed;
            self.velocity.z = (self.velocity.z / speed) * self.dna.max_speed;
        }

        self.position += self.velocity;
        self.acceleration = Vec3::ZERO;

        // Wrap around boundaries (-width/2 to width/2)
        if self.position.x < -width/2.0 { self.position.x += width; }
        if self.position.x > width/2.0 { self.position.x -= width; }
        if self.position.z < -depth/2.0 { self.position.z += depth; }
        if self.position.z > depth/2.0 { self.position.z -= depth; }
    }

    pub fn update_flash(&mut self) {
        if self.flash_timer > 0 {
            self.flash_timer -= 1;
        }

        if self.phase >= 1.0 {
            self.phase -= 1.0;
            self.flash_timer = 10;
        }
    }
}
