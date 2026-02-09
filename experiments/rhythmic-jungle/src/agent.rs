use macroquad::prelude::*;
use crate::rhythm::EuclideanGenerator;
use ::rand::Rng;

pub const INTERACTION_RADIUS: f32 = 30.0;
pub const INTERACTION_COOLDOWN: f32 = 2.0;

#[derive(Clone)] // Removed Copy because EuclideanGenerator contains Vec
pub struct Agent {
    pub id: usize,
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub rhythm: EuclideanGenerator,
    pub bpm: f32,
    pub step_timer: f32,
    pub current_step: usize,
    pub is_pulsing: bool,
    pub cooldown: f32,
    pub last_interaction_success: Option<bool>,
}

impl Agent {
    pub fn new(id: usize, x: f32, y: f32) -> Self {
        let mut rng = ::rand::thread_rng();
        let pulses = rng.gen_range(1..16); // Random density
        let bpm = rng.gen_range(60.0..180.0); // Random tempo

        Self {
            id,
            x,
            y,
            vx: rng.gen_range(-50.0..50.0),
            vy: rng.gen_range(-50.0..50.0),
            rhythm: EuclideanGenerator::new(16, pulses),
            bpm,
            step_timer: 0.0,
            current_step: 0,
            is_pulsing: false,
            cooldown: 0.0,
            last_interaction_success: None,
        }
    }

    pub fn update(&mut self, dt: f32, screen_w: f32, screen_h: f32) {
        // Movement
        self.x += self.vx * dt;
        self.y += self.vy * dt;

        // Bounce
        if self.x < 0.0 || self.x > screen_w {
            self.vx *= -1.0;
            self.x = self.x.clamp(0.0, screen_w);
        }
        if self.y < 0.0 || self.y > screen_h {
            self.vy *= -1.0;
            self.y = self.y.clamp(0.0, screen_h);
        }

        // Rhythm
        self.step_timer += dt;
        let step_duration = 60.0 / self.bpm / 4.0; // 16th notes

        if self.step_timer >= step_duration {
            self.step_timer -= step_duration;
            self.current_step = (self.current_step + 1) % self.rhythm.steps;

            // Pulse check
            self.is_pulsing = self.rhythm.get_beat_at(self.current_step);
        } else {
            // Decaying pulse for visual
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
        // Simple consensus: Average BPM and Pulses
        // "Entrainment"

        // Calculate consonance?
        // If beats align?

        // Let's just converge.
        // Move towards average BPM
        let avg_bpm = (self.bpm + other.bpm) / 2.0;
        self.bpm = self.bpm + (avg_bpm - self.bpm) * 0.1;
        other.bpm = other.bpm + (avg_bpm - other.bpm) * 0.1;

        // Converge Pulses (Density)
        // If I have 4 and you have 8, maybe I go to 5 and you go to 7.
        // Or we stick to integer logic.
        // 10% chance to mutate pulses towards other
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

        // Return true if they are somewhat synchronized (similar BPM or Pulses)
        let bpm_diff = (self.bpm - other.bpm).abs();
        let pulse_diff = (self.rhythm.pulses as i32 - other.rhythm.pulses as i32).abs();

        bpm_diff < 10.0 || pulse_diff == 0
    }
}
