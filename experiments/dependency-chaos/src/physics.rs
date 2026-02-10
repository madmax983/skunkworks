#[derive(Clone, Copy, Debug)]
pub struct DoublePendulumParams {
    pub m1: f32,
    pub m2: f32,
    pub l1: f32,
    pub l2: f32,
    pub g: f32,
    pub damping: f32, // Optional friction
}

impl Default for DoublePendulumParams {
    fn default() -> Self {
        Self {
            m1: 1.0,
            m2: 1.0,
            l1: 1.0,
            l2: 1.0,
            g: 9.81,
            damping: 0.0,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct State {
    pub theta1: f32,
    pub theta2: f32,
    pub omega1: f32,
    pub omega2: f32,
}

impl State {
    pub fn new(t1: f32, t2: f32, w1: f32, w2: f32) -> Self {
        Self {
            theta1: t1,
            theta2: t2,
            omega1: w1,
            omega2: w2,
        }
    }
}

pub fn rk4_step(state: &State, params: &DoublePendulumParams, dt: f32) -> State {
    let k1 = derivatives(state, params);
    let s2 = step_state(state, &k1, dt * 0.5);

    let k2 = derivatives(&s2, params);
    let s3 = step_state(state, &k2, dt * 0.5);

    let k3 = derivatives(&s3, params);
    let s4 = step_state(state, &k3, dt);

    let k4 = derivatives(&s4, params);

    State {
        theta1: state.theta1 + dt / 6.0 * (k1.theta1 + 2.0 * k2.theta1 + 2.0 * k3.theta1 + k4.theta1),
        theta2: state.theta2 + dt / 6.0 * (k1.theta2 + 2.0 * k2.theta2 + 2.0 * k3.theta2 + k4.theta2),
        omega1: state.omega1 + dt / 6.0 * (k1.omega1 + 2.0 * k2.omega1 + 2.0 * k3.omega1 + k4.omega1),
        omega2: state.omega2 + dt / 6.0 * (k1.omega2 + 2.0 * k2.omega2 + 2.0 * k3.omega2 + k4.omega2),
    }
}

// Treat State as a vector for derivatives: (dtheta1, dtheta2, domega1, domega2)
// Re-using State struct to hold derivatives is convenient.
fn derivatives(state: &State, params: &DoublePendulumParams) -> State {
    let m1 = params.m1;
    let m2 = params.m2;
    let l1 = params.l1;
    let l2 = params.l2;
    let g = params.g;

    let t1 = state.theta1;
    let t2 = state.theta2;
    let w1 = state.omega1;
    let w2 = state.omega2;

    let dt = t1 - t2; // delta theta

    // Denominator term
    let denom = 2.0 * m1 + m2 - m2 * (2.0 * t1 - 2.0 * t2).cos();

    // omega1 dot
    let num1 = -g * (2.0 * m1 + m2) * t1.sin();
    let num2 = -m2 * g * (t1 - 2.0 * t2).sin();
    let num3 = -2.0 * dt.sin() * m2 * (w2 * w2 * l2 + w1 * w1 * l1 * dt.cos());

    let d_omega1 = (num1 + num2 + num3) / (l1 * denom);

    // omega2 dot
    let num4 = 2.0 * dt.sin();
    let num5 = w1 * w1 * l1 * (m1 + m2);
    let num6 = g * (m1 + m2) * t1.cos();
    let num7 = w2 * w2 * l2 * m2 * dt.cos();

    let d_omega2 = (num4 * (num5 + num6 + num7)) / (l2 * denom);

    // Apply damping
    let d_omega1 = d_omega1 - params.damping * w1;
    let d_omega2 = d_omega2 - params.damping * w2;

    State {
        theta1: w1,
        theta2: w2,
        omega1: d_omega1,
        omega2: d_omega2,
    }
}

fn step_state(s: &State, d: &State, dt: f32) -> State {
    State {
        theta1: s.theta1 + d.theta1 * dt,
        theta2: s.theta2 + d.theta2 * dt,
        omega1: s.omega1 + d.omega1 * dt,
        omega2: s.omega2 + d.omega2 * dt,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integration_step() {
        let params = DoublePendulumParams::default();
        let state = State::new(1.0, 1.0, 0.0, 0.0);
        let next_state = rk4_step(&state, &params, 0.01);

        assert!(!next_state.theta1.is_nan());
        assert!(!next_state.theta2.is_nan());
        assert!(!next_state.omega1.is_nan());
        assert!(!next_state.omega2.is_nan());

        // Check for movement (gravity should pull it)
        assert!(next_state.omega1.abs() > 0.0);
    }
}
