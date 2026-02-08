use macroquad::prelude::*;
use crate::simulation::{LorenzParams, derivatives};

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
        let sigma = params.sigma;
        let rho = params.rho;
        let beta = params.beta;

        // Evolve reference
        let ref_next = rk4_step(self.reference, sigma, rho, beta, dt);
        // Evolve shadow
        let shadow_next = rk4_step(self.shadow, sigma, rho, beta, dt);

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

// Helper RK4 step
fn rk4_step(pos: Vec3, sigma: f32, rho: f32, beta: f32, dt: f32) -> Vec3 {
    let x = pos.x;
    let y = pos.y;
    let z = pos.z;

    let (k1_x, k1_y, k1_z) = derivatives(x, y, z, sigma, rho, beta);
    let (k2_x, k2_y, k2_z) = derivatives(x + k1_x * dt * 0.5, y + k1_y * dt * 0.5, z + k1_z * dt * 0.5, sigma, rho, beta);
    let (k3_x, k3_y, k3_z) = derivatives(x + k2_x * dt * 0.5, y + k2_y * dt * 0.5, z + k2_z * dt * 0.5, sigma, rho, beta);
    let (k4_x, k4_y, k4_z) = derivatives(x + k3_x * dt, y + k3_y * dt, z + k3_z * dt, sigma, rho, beta);

    let dx = (k1_x + 2.0 * k2_x + 2.0 * k3_x + k4_x) / 6.0;
    let dy = (k1_y + 2.0 * k2_y + 2.0 * k3_y + k4_y) / 6.0;
    let dz = (k1_z + 2.0 * k2_z + 2.0 * k3_z + k4_z) / 6.0;

    vec3(x + dx * dt, y + dy * dt, z + dz * dt)
}
