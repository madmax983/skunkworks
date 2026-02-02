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
}

pub fn integrate(state: &LorenzState, params: &LorenzParams, dt: f64) -> LorenzState {
    let k1 = derivative(state, params);

    let s2 = LorenzState {
        x: state.x + k1.x * dt * 0.5,
        y: state.y + k1.y * dt * 0.5,
        z: state.z + k1.z * dt * 0.5,
    };
    let k2 = derivative(&s2, params);

    let s3 = LorenzState {
        x: state.x + k2.x * dt * 0.5,
        y: state.y + k2.y * dt * 0.5,
        z: state.z + k2.z * dt * 0.5,
    };
    let k3 = derivative(&s3, params);

    let s4 = LorenzState {
        x: state.x + k3.x * dt,
        y: state.y + k3.y * dt,
        z: state.z + k3.z * dt,
    };
    let k4 = derivative(&s4, params);

    LorenzState {
        x: state.x + (dt / 6.0) * (k1.x + 2.0 * k2.x + 2.0 * k3.x + k4.x),
        y: state.y + (dt / 6.0) * (k1.y + 2.0 * k2.y + 2.0 * k3.y + k4.y),
        z: state.z + (dt / 6.0) * (k1.z + 2.0 * k2.z + 2.0 * k3.z + k4.z),
    }
}

fn derivative(state: &LorenzState, params: &LorenzParams) -> LorenzState {
    let dx = params.sigma * (state.y - state.x);
    let dy = state.x * (params.rho - state.z) - state.y;
    let dz = state.x * state.y - params.beta * state.z;
    LorenzState {
        x: dx,
        y: dy,
        z: dz,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lorenz_step() {
        let state = LorenzState {
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

        let next_state = integrate(&state, &params, dt);

        // Check if state changed (it should, for these params)
        assert!(next_state.x != state.x);
        assert!(next_state.y != state.y);
        assert!(next_state.z != state.z);

        // Basic check for direction.
        assert!((next_state.x - 10.0).abs() < 1.0); // Should be small change
        assert!(next_state.y > 10.0);
        assert!(next_state.z > 10.0);
    }
}
