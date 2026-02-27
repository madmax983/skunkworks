use macroquad::prelude::*;
use ::rand::{Rng, thread_rng};
use std::f32::consts::PI;

#[derive(Clone, Debug)]
pub struct Dna {
    pub max_speed: f32,
    pub max_force: f32,
    pub view_radius: f32,
    pub coupling_radius: f32,
    pub separation_weight: f32,
    pub alignment_weight: f32,
    pub cohesion_weight: f32,
    pub heal_weight: f32, // New trait: attraction to decay
    pub natural_freq: f32,
    pub coupling_strength: f32,
    pub color: Color,
    pub char_representation: char,
}

impl Dna {
    pub fn random() -> Self {
        let mut rng = thread_rng();
        // Variety in traits creates "species" within the flock
        Self {
            max_speed: rng.gen_range(2.0..4.0), // Slightly faster for macroquad pixels
            max_force: rng.gen_range(0.1..0.3),
            view_radius: rng.gen_range(30.0..60.0),
            coupling_radius: rng.gen_range(40.0..80.0),
            separation_weight: rng.gen_range(1.5..2.5),
            alignment_weight: rng.gen_range(0.8..1.2),
            cohesion_weight: rng.gen_range(0.8..1.2),
            heal_weight: rng.gen_range(2.0..4.0), // Strong attraction to duty
            natural_freq: 0.005 + rng.gen::<f32>() * 0.02,
            coupling_strength: 0.005,
            color: Color::new(
                rng.gen_range(0.5..1.0),
                rng.gen_range(0.5..1.0),
                rng.gen_range(0.5..1.0),
                1.0
            ),
            char_representation: if rng.gen_bool(0.5) { '✦' } else { '•' },
        }
    }
}

#[derive(Clone, Debug)]
pub struct Boid {
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub dna: Dna,
    // Firefly state
    pub phase: f32,
    pub flash_timer: usize,
    pub healing: bool, // Visual state
}

impl Boid {
    pub fn new(x: f32, y: f32) -> Self {
        let mut rng = thread_rng();
        let angle = rng.gen_range(0.0..2.0 * PI);
        let dna = Dna::random();

        Self {
            position: vec2(x, y),
            velocity: vec2(angle.cos() * dna.max_speed, angle.sin() * dna.max_speed),
            acceleration: vec2(0.0, 0.0),
            dna,
            phase: rng.gen::<f32>(),
            flash_timer: 0,
            healing: false,
        }
    }

    pub fn position(&self) -> Vec2 {
        self.position
    }

    pub fn apply_force(&mut self, force: Vec2) {
        self.acceleration += force;
    }

    pub fn update_physics(&mut self, width: f32, height: f32) {
        self.velocity += self.acceleration;
        // Limit speed
        if self.velocity.length() > self.dna.max_speed {
            self.velocity = self.velocity.normalize() * self.dna.max_speed;
        }

        self.position += self.velocity;
        self.acceleration = vec2(0.0, 0.0);

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
    }

    pub fn update_flash(&mut self) {
        // Flash timer
        if self.flash_timer > 0 {
            self.flash_timer -= 1;
        }

        // Phase is updated in World system, this handles wrap
        if self.phase >= 1.0 {
            self.phase -= 1.0;
            self.flash_timer = 5;
        }
    }
}
