use macroquad::prelude::Vec3;

#[derive(Clone)]
pub struct Attractor {
    pub pos: Vec3,
    pub sigma: f32,
    pub rho: f32,
    pub beta: f32,
}

impl Attractor {
    pub fn new(pos: Vec3, sigma: f32, rho: f32, beta: f32) -> Self {
        Self { pos, sigma, rho, beta }
    }

    /// Calculate the derivative at a given state
    fn derivative(&self, p: Vec3) -> Vec3 {
        let x = p.x;
        let y = p.y;
        let z = p.z;

        let dx = self.sigma * (y - x);
        let dy = x * (self.rho - z) - y;
        let dz = x * y - self.beta * z;

        Vec3::new(dx, dy, dz)
    }

    pub fn step(&mut self, dt: f32) {
        let k1 = self.derivative(self.pos);
        let k2 = self.derivative(self.pos + k1 * (dt * 0.5));
        let k3 = self.derivative(self.pos + k2 * (dt * 0.5));
        let k4 = self.derivative(self.pos + k3 * dt);

        self.pos += (k1 + k2 * 2.0 + k3 * 2.0 + k4) * (dt / 6.0);
    }
}

pub struct LyapunovMonitor {
    pub shadow_pos: Vec3,
    pub initial_distance: f32,
    pub total_log_divergence: f32,
    pub steps: usize,
    pub current_divergence: f32,
}

impl LyapunovMonitor {
    pub fn new(start_pos: Vec3, epsilon: f32) -> Self {
        // Offset in X direction arbitrarily
        let shadow_pos = start_pos + Vec3::new(epsilon, 0.0, 0.0);
        Self {
            shadow_pos,
            initial_distance: epsilon,
            total_log_divergence: 0.0,
            steps: 0,
            current_divergence: 0.0,
        }
    }

    pub fn step(&mut self, attractor: &Attractor, dt: f32) {
        // Create a temporary attractor for the shadow particle
        // We assume the shadow particle follows the SAME parameters
        let mut shadow_attractor = attractor.clone();
        shadow_attractor.pos = self.shadow_pos;

        // Step the shadow particle
        shadow_attractor.step(dt);
        self.shadow_pos = shadow_attractor.pos;

        // Calculate divergence
        let diff = self.shadow_pos - attractor.pos;
        let dist = diff.length();

        // Avoid division by zero
        if dist > 0.0 {
            // Calculate local exponent contribution
            // The divergence over time T is d(T)/d(0) = e^(lambda * T)
            // ln(d(T)/d(0)) = lambda * T
            // lambda = ln(dist / initial_distance) / dt
            // However, we accumulate the log divergence and divide by total time at the end.

            self.total_log_divergence += (dist / self.initial_distance).ln();
            self.current_divergence = (dist / self.initial_distance).ln() / dt;
            self.steps += 1;

            // Rescale shadow particle
            // Move shadow particle back towards the main particle so distance is initial_distance
            // This prevents it from drifting too far (where linear approximation fails)
            // or collapsing (due to floating point errors if converging)
            self.shadow_pos = attractor.pos + diff * (self.initial_distance / dist);
        }
    }

    pub fn get_lle(&self, dt: f32) -> f32 {
        if self.steps == 0 {
            return 0.0;
        }
        // Average LLE = Sum(ln(d_i / d_0)) / (N * dt)
        self.total_log_divergence / (self.steps as f32 * dt)
    }

    /// Reset the accumulator to get a "fresh" reading if parameters change drastically
    #[allow(dead_code)]
    pub fn reset_stats(&mut self) {
        self.total_log_divergence = 0.0;
        self.steps = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lorenz_divergence() {
        // Standard chaotic parameters
        let mut attractor = Attractor::new(Vec3::new(1.0, 1.0, 1.0), 10.0, 28.0, 8.0 / 3.0);
        let mut monitor = LyapunovMonitor::new(attractor.pos, 1e-4);

        let dt = 0.01;
        // Transient
        for _ in 0..1000 {
            attractor.step(dt);
        }

        // Reset monitor to track on the attractor
        monitor = LyapunovMonitor::new(attractor.pos, 1e-4);

        // Run for 5000 steps (50 seconds)
        for _ in 0..5000 {
            attractor.step(dt);
            monitor.step(&attractor, dt);
        }

        let lle = monitor.get_lle(dt);
        println!("LLE: {}", lle);

        assert!(lle > 0.8, "LLE should be close to 0.9 for Lorenz, got {}", lle);
        assert!(lle < 1.0, "LLE should be close to 0.9 for Lorenz, got {}", lle);
    }

    #[test]
    fn test_stable_point_divergence() {
        // Stable parameters (should converge or cycle)
        // Rho = 14 is stable? No, Rho < 1 is stable point.
        // Let's try very low Rho.
        let mut attractor = Attractor::new(Vec3::new(1.0, 1.0, 1.0), 10.0, 0.5, 8.0 / 3.0);
        let mut monitor = LyapunovMonitor::new(attractor.pos, 1e-4);

        let dt = 0.01;
        for _ in 0..1000 {
            attractor.step(dt);
            monitor.step(&attractor, dt);
        }

        let lle = monitor.get_lle(dt);
        println!("Stable LLE: {}", lle);

        assert!(lle < 0.1, "LLE should be small or negative for stable parameters, got {}", lle);
    }
}
