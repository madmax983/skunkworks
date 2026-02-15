use macroquad::prelude::Vec3;

#[derive(Clone, Debug)]
pub struct RosslerSystem {
    pub pos: Vec3,
    pub a: f32, // Bifurcation parameter (chaos)
    pub b: f32, // Inflation / Growth
    pub c: f32, // Threshold
}

impl RosslerSystem {
    pub fn new(pos: Vec3, a: f32, b: f32, c: f32) -> Self {
        Self { pos, a, b, c }
    }

    pub fn default_chaotic() -> Self {
        Self {
            pos: Vec3::new(1.0, 1.0, 1.0),
            a: 0.2,
            b: 0.2,
            c: 5.7,
        }
    }

    /// Runge-Kutta 4th Order Integration
    pub fn step(&mut self, dt: f32) {
        let p = self.pos;

        let k1 = self.derivative(p);
        let k2 = self.derivative(p + k1 * (dt * 0.5));
        let k3 = self.derivative(p + k2 * (dt * 0.5));
        let k4 = self.derivative(p + k3 * dt);

        self.pos += (k1 + k2 * 2.0 + k3 * 2.0 + k4) * (dt / 6.0);
    }

    fn derivative(&self, p: Vec3) -> Vec3 {
        let dx = -p.y - p.z;
        let dy = p.x + self.a * p.y;
        let dz = self.b + p.z * (p.x - self.c);

        Vec3::new(dx, dy, dz)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step_advances_state() {
        let mut sys = RosslerSystem::default_chaotic();
        let initial_pos = sys.pos;

        sys.step(0.01);

        assert_ne!(sys.pos, initial_pos);
        // Basic check: Rössler usually moves away from 1,1,1 initially
        // dx = -1 - 1 = -2
        // dy = 1 + 0.2 = 1.2
        // dz = 0.2 + 1(1 - 5.7) = -4.5
        // So x should decrease, y increase, z decrease.

        assert!(sys.pos.x < initial_pos.x);
        assert!(sys.pos.y > initial_pos.y);
        assert!(sys.pos.z < initial_pos.z);
    }
}
