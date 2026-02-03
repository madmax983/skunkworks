use rand::Rng;
use ratatui::style::Color;
use std::f64::consts::TAU;

#[derive(Clone, Debug)]
pub struct DNA {
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

impl DNA {
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        // Variety in traits creates "species" within the flock
        Self {
            max_speed: rng.gen_range(0.8..1.5),
            max_force: rng.gen_range(0.05..0.15),
            view_radius: rng.gen_range(10.0..20.0),
            coupling_radius: rng.gen_range(15.0..30.0), // Coupling often travels further than sight
            separation_weight: rng.gen_range(1.2..2.5),
            alignment_weight: rng.gen_range(0.8..1.2),
            cohesion_weight: rng.gen_range(0.8..1.2),
            natural_freq: 0.005 + rng.r#gen::<f64>() * 0.02,
            coupling_strength: 0.005, // Small nudges prevent chaos
            color: Color::Indexed(rng.gen_range(20..230)),
            char_representation: if rng.gen_bool(0.5) { '✦' } else { '•' },
        }
    }
}

#[derive(Clone, Debug)]
pub struct Boid {
    pub position: (f64, f64),
    pub velocity: (f64, f64),
    pub acceleration: (f64, f64),
    pub dna: DNA,
    // Firefly state
    pub phase: f64,
    pub flash_timer: usize,
}

impl Boid {
    pub fn new(x: f64, y: f64) -> Self {
        let mut rng = rand::thread_rng();
        let angle = rng.gen_range(0.0..TAU);
        let dna = DNA::random();

        Self {
            position: (x, y),
            velocity: (angle.cos() * dna.max_speed, angle.sin() * dna.max_speed),
            acceleration: (0.0, 0.0),
            dna,
            phase: rng.r#gen::<f64>(),
            flash_timer: 0,
        }
    }

    pub fn apply_force(&mut self, force: (f64, f64)) {
        self.acceleration.0 += force.0;
        self.acceleration.1 += force.1;
    }

    pub fn update_physics(&mut self, width: f64, height: f64) {
        self.velocity.0 += self.acceleration.0;
        self.velocity.1 += self.acceleration.1;

        // Limit speed
        let speed = (self.velocity.0.powi(2) + self.velocity.1.powi(2)).sqrt();
        if speed > self.dna.max_speed {
            self.velocity.0 = (self.velocity.0 / speed) * self.dna.max_speed;
            self.velocity.1 = (self.velocity.1 / speed) * self.dna.max_speed;
        }

        self.position.0 += self.velocity.0;
        self.position.1 += self.velocity.1;

        // Reset acceleration
        self.acceleration = (0.0, 0.0);

        // Wrap around edges
        if self.position.0 < 0.0 {
            self.position.0 += width;
        }
        if self.position.0 >= width {
            self.position.0 -= width;
        }
        if self.position.1 < 0.0 {
            self.position.1 += height;
        }
        if self.position.1 >= height {
            self.position.1 -= height;
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

pub fn distance_squared(p1: (f64, f64), p2: (f64, f64)) -> f64 {
    (p1.0 - p2.0).powi(2) + (p1.1 - p2.1).powi(2)
}

pub fn limit(vector: (f64, f64), max: f64) -> (f64, f64) {
    let len_sq = vector.0.powi(2) + vector.1.powi(2);
    if len_sq > max.powi(2) {
        let len = len_sq.sqrt();
        ((vector.0 / len) * max, (vector.1 / len) * max)
    } else {
        vector
    }
}
