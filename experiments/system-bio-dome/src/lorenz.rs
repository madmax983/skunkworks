// 🧬 Allele B: Inherited from experiments/sys-weather
// Represents the Chaotic Driver (Lorenz Attractor)

#[derive(Debug, Clone, Copy)]
pub struct LorenzState {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct LorenzParams {
    pub sigma: f64,
    pub rho: f64,
    pub beta: f64,
}

impl LorenzState {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn update(&mut self, params: &LorenzParams, dt: f64) {
        let k1 = self.derivative(params);

        let s2 = LorenzState {
            x: self.x + k1.x * dt * 0.5,
            y: self.y + k1.y * dt * 0.5,
            z: self.z + k1.z * dt * 0.5,
        };
        let k2 = s2.derivative(params);

        let s3 = LorenzState {
            x: self.x + k2.x * dt * 0.5,
            y: self.y + k2.y * dt * 0.5,
            z: self.z + k2.z * dt * 0.5,
        };
        let k3 = s3.derivative(params);

        let s4 = LorenzState {
            x: self.x + k3.x * dt,
            y: self.y + k3.y * dt,
            z: self.z + k3.z * dt,
        };
        let k4 = s4.derivative(params);

        self.x += (dt / 6.0) * (k1.x + 2.0 * k2.x + 2.0 * k3.x + k4.x);
        self.y += (dt / 6.0) * (k1.y + 2.0 * k2.y + 2.0 * k3.y + k4.y);
        self.z += (dt / 6.0) * (k1.z + 2.0 * k2.z + 2.0 * k3.z + k4.z);
    }

    fn derivative(&self, params: &LorenzParams) -> LorenzState {
        let dx = params.sigma * (self.y - self.x);
        let dy = self.x * (params.rho - self.z) - self.y;
        let dz = self.x * self.y - params.beta * self.z;
        LorenzState {
            x: dx,
            y: dy,
            z: dz,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lorenz_step() {
        let mut state = LorenzState {
            x: 10.0,
            y: 10.0,
            z: 10.0,
        };
        let params = LorenzParams {
            sigma: 10.0,
            rho: 28.0,
            beta: 8.0 / 3.0,
        };
        let dt = 0.01;

        let initial_state = state;
        state.update(&params, dt);

        // Check if state changed (it should, for these params)
        assert!(state.x != initial_state.x);
        assert!(state.y != initial_state.y);
        assert!(state.z != initial_state.z);

        // Basic check for direction.
        assert!((state.x - 10.0).abs() < 1.0); // Should be small change
        assert!(state.y > 10.0);
        assert!(state.z > 10.0);
    }
}
