use rand::Rng;
use ratatui::style::Color;
use std::f64::consts::TAU;

#[derive(Clone, Debug)]
pub struct DNA {
    pub max_speed: f64,
    pub max_force: f64,
    pub view_radius: f64,
    pub separation_weight: f64,
    pub alignment_weight: f64,
    pub cohesion_weight: f64,
    pub sonar_avoidance_weight: f64, // How much they run from loud noise
    pub ping_probability: f64,
    pub ping_strength: f32,
    pub color: Color,
    pub char_representation: char,
}

impl DNA {
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            max_speed: rng.gen_range(0.3..0.8), // Slower to navigate echo
            max_force: rng.gen_range(0.02..0.1),
            view_radius: rng.gen_range(5.0..15.0),
            separation_weight: rng.gen_range(1.5..3.0),
            alignment_weight: rng.gen_range(0.5..1.0),
            cohesion_weight: rng.gen_range(0.5..1.0),
            sonar_avoidance_weight: rng.gen_range(5.0..15.0), // High avoidance of noise
            ping_probability: 0.02,
            ping_strength: 5.0,
            color: Color::Indexed(rng.gen_range(20..230)),
            char_representation: if rng.gen_bool(0.5) { '▲' } else { '▶' },
        }
    }
}

#[derive(Clone, Debug)]
pub struct Boid {
    pub position: (f64, f64),
    pub velocity: (f64, f64),
    pub acceleration: (f64, f64),
    pub dna: DNA,
    pub ping_timer: usize, // Visual feedback
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
            ping_timer: 0,
        }
    }

    pub fn apply_force(&mut self, force: (f64, f64)) {
        self.acceleration.0 += force.0;
        self.acceleration.1 += force.1;
    }

    pub fn update_physics(&mut self, width: usize, height: usize, walls: &[bool]) {
        self.velocity.0 += self.acceleration.0;
        self.velocity.1 += self.acceleration.1;

        // Limit speed
        let speed = (self.velocity.0.powi(2) + self.velocity.1.powi(2)).sqrt();
        if speed > self.dna.max_speed {
            self.velocity.0 = (self.velocity.0 / speed) * self.dna.max_speed;
            self.velocity.1 = (self.velocity.1 / speed) * self.dna.max_speed;
        }

        let next_x = self.position.0 + self.velocity.0;
        let next_y = self.position.1 + self.velocity.1;

        // Boundary Check (Outer walls)
        if next_x < 1.0 || next_x >= (width as f64 - 1.0) || next_y < 1.0 || next_y >= (height as f64 - 1.0) {
             self.velocity.0 *= -1.0;
             self.velocity.1 *= -1.0;
             return;
        }

        // Inner Wall Check
        let gx = next_x as usize;
        let gy = next_y as usize;
        let idx = gy * width + gx;

        if idx < walls.len() && walls[idx] {
             self.velocity.0 *= -1.0;
             self.velocity.1 *= -1.0;
        } else {
             self.position.0 = next_x;
             self.position.1 = next_y;
        }

        self.acceleration = (0.0, 0.0);

        if self.ping_timer > 0 {
            self.ping_timer -= 1;
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
