use num_complex::Complex;
use poincare_disk::{mobius_add, Point, hyperbolic_dist};
use crate::rhythm::EuclideanGenerator;
use ::rand::Rng;
use std::f64::consts::PI;

// Hyperbolic distance for interaction.
// Note: As you get closer to the edge, Euclidean distance shrinks, but hyperbolic distance stays relevant.
pub const INTERACTION_RADIUS: f64 = 0.8;
pub const INTERACTION_COOLDOWN: f32 = 0.5;

#[derive(Clone)]
pub struct Agent {
    pub id: usize,
    pub pos: Point,
    pub angle: f64, // Orientation
    pub rhythm: EuclideanGenerator,
    pub bpm: f32,
    pub step_timer: f32,
    pub current_step: usize,
    pub is_pulsing: bool,
    pub cooldown: f32,
}

impl Agent {
    pub fn new(id: usize) -> Self {
        let mut rng = ::rand::thread_rng();

        // Random position in disk
        // Uniform distribution in disk requires r = sqrt(u)
        let r = rng.gen_range(0.0..0.95f64).sqrt();
        let theta = rng.gen_range(0.0..PI * 2.0);
        let pos = Complex::from_polar(r, theta);

        let angle = rng.gen_range(0.0..PI * 2.0);

        let pulses = rng.gen_range(1..16);
        let bpm = rng.gen_range(60.0..180.0);

        Self {
            id,
            pos,
            angle,
            rhythm: EuclideanGenerator::new(16, pulses),
            bpm,
            step_timer: 0.0,
            current_step: 0,
            is_pulsing: false,
            cooldown: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        let mut rng = ::rand::thread_rng();

        // 1. Move Hyperbolically
        // Move straight
        let move_dist = 0.1 * dt as f64; // Speed
        let move_vec = Complex::from_polar(move_dist, self.angle);
        let new_pos = mobius_add(self.pos, move_vec);

        // Boundary check
        if new_pos.norm_sqr() < 0.98 {
            self.pos = new_pos;
        } else {
            // Turn around
            self.angle += PI;
            // Add some noise
            self.angle += rng.gen_range(-0.5..0.5);
        }

        // Randomly turn sometimes
        if rng.gen_bool(0.02) {
             self.angle += rng.gen_range(-1.0..1.0);
        }

        // 2. Rhythm Update
        self.step_timer += dt;
        let step_duration = 60.0 / self.bpm / 4.0; // 16th notes

        if self.step_timer >= step_duration {
            self.step_timer -= step_duration;
            self.current_step = (self.current_step + 1) % self.rhythm.steps;
            self.is_pulsing = self.rhythm.get_beat_at(self.current_step);
        } else {
            if self.step_timer > 0.1 {
                self.is_pulsing = false;
            }
        }

        // Cooldown
        if self.cooldown > 0.0 {
            self.cooldown -= dt;
        }
    }

    pub fn interact(&mut self, other: &mut Agent) -> bool {
        // Calculate Hyperbolic Distance
        let dist = hyperbolic_dist(self.pos, other.pos);

        if dist < INTERACTION_RADIUS {
            // Converge BPM
            let avg_bpm = (self.bpm + other.bpm) / 2.0;
            let strength = 0.1; // Lerp strength

            self.bpm = self.bpm + (avg_bpm - self.bpm) * strength;
            other.bpm = other.bpm + (avg_bpm - other.bpm) * strength;

            // Converge Pulses (Mutation)
            let mut rng = ::rand::thread_rng();
            if rng.gen_bool(0.1) {
                 if self.rhythm.pulses < other.rhythm.pulses {
                     self.rhythm.set_params(16, self.rhythm.pulses + 1);
                 } else if self.rhythm.pulses > other.rhythm.pulses {
                     self.rhythm.set_params(16, self.rhythm.pulses - 1);
                 }
            }
            if rng.gen_bool(0.1) {
                 if other.rhythm.pulses < self.rhythm.pulses {
                     other.rhythm.set_params(16, other.rhythm.pulses + 1);
                 } else if other.rhythm.pulses > self.rhythm.pulses {
                     other.rhythm.set_params(16, other.rhythm.pulses - 1);
                 }
            }

            return true;
        }
        false
    }
}
