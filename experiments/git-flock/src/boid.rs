use locus::Vec2;
use rand::Rng;
use ratatui::style::Color;
use std::f64::consts::TAU;

#[derive(Clone, Debug)]
pub struct Dna {
    pub max_speed: f64,
    pub max_force: f64,
    pub view_radius: f64,
    pub coupling_radius: f64,
    pub separation_weight: f64,
    pub alignment_weight: f64,
    pub cohesion_weight: f64,
    pub natural_freq: f64,
    pub coupling_strength: f64,
    pub color: Color,
    pub char_representation: char,
}

impl Dna {
    pub fn from_commit(commit: &crate::git::Commit) -> Self {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        commit.hash.hash(&mut hasher);
        let hash_val = hasher.finish();

        let mut author_hasher = DefaultHasher::new();
        commit.author.hash(&mut author_hasher);
        let author_hash = author_hasher.finish();

        // Extract parameters from hash
        let p1 = ((hash_val >> 16) & 0xFF) as f64 / 255.0;
        let p2 = ((hash_val >> 24) & 0xFF) as f64 / 255.0;
        let p3 = ((hash_val >> 32) & 0xFF) as f64 / 255.0;

        let max_speed = 0.5 + p1 * 1.5;
        let view_radius = 5.0 + p2 * 25.0;
        let natural_freq = 0.005 + p3 * 0.02;

        let color_index = (author_hash % 210) as u8 + 20;

        Self {
            max_speed,
            max_force: 0.1 + p1 * 0.1,
            view_radius,
            coupling_radius: view_radius * 1.5,
            separation_weight: 1.5 + p2,
            alignment_weight: 1.0 + p3,
            cohesion_weight: 1.0 + p1,
            natural_freq,
            coupling_strength: 0.005,
            color: Color::Indexed(color_index),
            char_representation: if p1 > 0.5 { '✦' } else { '•' },
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
    pub phase: f64,
    pub flash_timer: usize,
}

impl Boid {
    pub fn new(x: f64, y: f64, commit: &crate::git::Commit) -> Self {
        let mut rng = rand::thread_rng();
        let angle = rng.gen_range(0.0..TAU);
        let dna = Dna::from_commit(commit);

        Self {
            position: Vec2::new(x, y),
            velocity: Vec2::new(angle.cos() * dna.max_speed, angle.sin() * dna.max_speed),
            acceleration: Vec2::zero(),
            dna,
            phase: rng.r#gen::<f64>(),
            flash_timer: 0,
        }
    }

    pub fn position(&self) -> Vec2 {
        self.position
    }

    pub fn apply_force(&mut self, force: Vec2) {
        self.acceleration += force;
    }

    pub fn update_physics(&mut self, width: f64, height: f64) {
        self.velocity += self.acceleration;
        self.velocity = self.velocity.limit(self.dna.max_speed);
        self.position += self.velocity;
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
    }

    pub fn update_flash(&mut self) {
        // NOTE: Phase incrementing happens in World to account for nudges first

        // Handle flash timer
        if self.flash_timer > 0 {
            self.flash_timer -= 1;
        }

        // Wrap phase and trigger flash
        if self.phase >= 1.0 {
            self.phase -= 1.0;
            self.flash_timer = 5; // Flash lasts 5 ticks
        }
    }
}
