use macroquad::prelude::*;
use crate::simulation::{LorenzParams, solve_rk4};

pub struct LyapunovMonitor {
    pub reference: Vec3,
    pub shadow: Vec3,
    pub sum_log_div: f32,
    pub total_time: f32,
    pub step_count: usize,
    pub initial_dist: f32,
}

impl LyapunovMonitor {
    pub fn new(start_pos: Vec3, initial_dist: f32) -> Self {
        let shadow_pos = start_pos + vec3(initial_dist, 0.0, 0.0);
        Self {
            reference: start_pos,
            shadow: shadow_pos,
            sum_log_div: 0.0,
            total_time: 0.0,
            step_count: 0,
            initial_dist,
        }
    }

    pub fn update(&mut self, params: &LorenzParams, dt: f32) {
        // Evolve reference
        let ref_next = solve_rk4(self.reference, params, dt);
        // Evolve shadow
        let shadow_next = solve_rk4(self.shadow, params, dt);

        // Calculate new distance
        let dist = (ref_next - shadow_next).length();

        // Add to sum
        if dist > 0.0 {
            // Using a safe default if dist/initial_dist is 0 (shouldn't happen with small dt)
            let ratio = dist / self.initial_dist;
            if ratio > 0.0 {
                self.sum_log_div += ratio.ln();
            }
        }

        self.total_time += dt;
        self.step_count += 1;

        // Renormalize shadow
        // Move shadow to be exactly initial_dist away from ref_next along the direction (shadow_next - ref_next)
        let diff = shadow_next - ref_next;
        let direction = if diff.length_squared() > 0.0 {
             diff.normalize()
        } else {
             vec3(1.0, 0.0, 0.0) // Fallback
        };

        self.shadow = ref_next + direction * self.initial_dist;
        self.reference = ref_next;
    }

    pub fn get_exponent(&self) -> f32 {
        if self.total_time > 0.0 {
            self.sum_log_div / (self.total_time)
        } else {
            0.0
        }
    }
}
