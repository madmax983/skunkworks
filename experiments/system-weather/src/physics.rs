#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LorenzState {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl LorenzState {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

pub struct LorenzSystem {
    pub sigma: f64,
    pub rho: f64,
    pub beta: f64,
}

impl LorenzSystem {
    pub fn new(sigma: f64, rho: f64, beta: f64) -> Self {
        Self { sigma, rho, beta }
    }

    fn derivative(&self, state: &LorenzState) -> LorenzState {
        LorenzState {
            x: self.sigma * (state.y - state.x),
            y: state.x * (self.rho - state.z) - state.y,
            z: state.x * state.y - self.beta * state.z,
        }
    }

    pub fn integrate(&self, state: &LorenzState, dt: f64) -> LorenzState {
        let k1 = self.derivative(state);
        let k2 = self.derivative(&LorenzState {
            x: state.x + k1.x * dt * 0.5,
            y: state.y + k1.y * dt * 0.5,
            z: state.z + k1.z * dt * 0.5,
        });
        let k3 = self.derivative(&LorenzState {
            x: state.x + k2.x * dt * 0.5,
            y: state.y + k2.y * dt * 0.5,
            z: state.z + k2.z * dt * 0.5,
        });
        let k4 = self.derivative(&LorenzState {
            x: state.x + k3.x * dt,
            y: state.y + k3.y * dt,
            z: state.z + k3.z * dt,
        });

        LorenzState {
            x: state.x + (k1.x + 2.0 * k2.x + 2.0 * k3.x + k4.x) * dt / 6.0,
            y: state.y + (k1.y + 2.0 * k2.y + 2.0 * k3.y + k4.y) * dt / 6.0,
            z: state.z + (k1.z + 2.0 * k2.z + 2.0 * k3.z + k4.z) * dt / 6.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integration_step() {
        let system = LorenzSystem::new(10.0, 28.0, 8.0 / 3.0);
        let start = LorenzState::new(1.0, 1.0, 1.0);
        let dt = 0.01;
        let next = system.integrate(&start, dt);

        // Just sanity check that state changes and is not NaN
        assert!(next.x != start.x);
        assert!(next.y != start.y);
        assert!(next.z != start.z);
        assert!(!next.x.is_nan());
        assert!(!next.y.is_nan());
        assert!(!next.z.is_nan());
    }

    #[test]
    fn test_fixed_point() {
        // At origin (0,0,0) derivative should be 0, so next state should be 0,0,0
        let system = LorenzSystem::new(10.0, 28.0, 8.0 / 3.0);
        let start = LorenzState::new(0.0, 0.0, 0.0);
        let dt = 0.1;
        let next = system.integrate(&start, dt);

        assert_eq!(next.x, 0.0);
        assert_eq!(next.y, 0.0);
        assert_eq!(next.z, 0.0);
    }
}
