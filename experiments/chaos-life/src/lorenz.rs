pub struct LorenzState {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub sigma: f32,
    pub rho: f32,
    pub beta: f32,
}

impl LorenzState {
    pub fn new() -> Self {
        Self {
            x: 0.1,
            y: 0.0,
            z: 0.0,
            sigma: 10.0,
            rho: 28.0,
            beta: 8.0 / 3.0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        let (dx, dy, dz) = self.rk4(dt);
        self.x += dx;
        self.y += dy;
        self.z += dz;
    }

    fn derivatives(&self, x: f32, y: f32, z: f32) -> (f32, f32, f32) {
        let dx = self.sigma * (y - x);
        let dy = x * (self.rho - z) - y;
        let dz = x * y - self.beta * z;
        (dx, dy, dz)
    }

    fn rk4(&self, dt: f32) -> (f32, f32, f32) {
        let (k1x, k1y, k1z) = self.derivatives(self.x, self.y, self.z);

        let (k2x, k2y, k2z) = self.derivatives(
            self.x + k1x * dt * 0.5,
            self.y + k1y * dt * 0.5,
            self.z + k1z * dt * 0.5,
        );

        let (k3x, k3y, k3z) = self.derivatives(
            self.x + k2x * dt * 0.5,
            self.y + k2y * dt * 0.5,
            self.z + k2z * dt * 0.5,
        );

        let (k4x, k4y, k4z) =
            self.derivatives(self.x + k3x * dt, self.y + k3y * dt, self.z + k3z * dt);

        let dx = (k1x + 2.0 * k2x + 2.0 * k3x + k4x) * (dt / 6.0);
        let dy = (k1y + 2.0 * k2y + 2.0 * k3y + k4y) * (dt / 6.0);
        let dz = (k1z + 2.0 * k2z + 2.0 * k3z + k4z) * (dt / 6.0);

        (dx, dy, dz)
    }

    /// Maps the chaotic state to Game of Life rules.
    /// Returns (survival_min, survival_max, birth_threshold).
    pub fn get_rules(&self) -> (u8, u8, u8) {
        // x typically ranges [-20, 20]. Map to [1, 4].
        let s_min = self.map_range(self.x, -20.0, 20.0, 1.0, 4.0) as u8;

        // y typically ranges [-30, 30]. Map to [3, 6].
        // Ensure s_max >= s_min
        let s_max = self.map_range(self.y, -30.0, 30.0, 3.0, 6.0) as u8;
        let s_max = s_max.max(s_min);

        // z typically ranges [0, 50]. Map to [2, 5].
        let birth = self.map_range(self.z, 0.0, 50.0, 2.0, 5.0) as u8;

        (s_min, s_max, birth)
    }

    fn map_range(&self, val: f32, in_min: f32, in_max: f32, out_min: f32, out_max: f32) -> f32 {
        let val = val.clamp(in_min, in_max);
        out_min + (val - in_min) * (out_max - out_min) / (in_max - in_min)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lorenz_update() {
        let mut lorenz = LorenzState::new();
        let initial_x = lorenz.x;
        lorenz.update(0.01);
        assert_ne!(lorenz.x, initial_x);
        assert!(!lorenz.x.is_nan());
        assert!(!lorenz.y.is_nan());
        assert!(!lorenz.z.is_nan());
    }

    #[test]
    fn test_rules_generation() {
        let lorenz = LorenzState::new();
        let (s_min, s_max, birth) = lorenz.get_rules();
        assert!(s_min <= s_max);
        assert!(s_min >= 1);
        assert!(s_max <= 6);
        assert!(birth >= 2);
    }
}
