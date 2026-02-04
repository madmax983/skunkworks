#[derive(Debug, Clone, Copy)]
pub struct LorenzState {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl LorenzState {
    pub fn add(self, other: LorenzState) -> LorenzState {
        LorenzState {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    pub fn scale(self, scalar: f64) -> LorenzState {
        LorenzState {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
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

    fn derivative(&self, state: LorenzState) -> LorenzState {
        let x = state.x;
        let y = state.y;
        let z = state.z;

        let dx = self.sigma * (y - x);
        let dy = x * (self.rho - z) - y;
        let dz = x * y - self.beta * z;

        LorenzState { x: dx, y: dy, z: dz }
    }

    pub fn step(&self, state: LorenzState, dt: f64) -> LorenzState {
        let k1 = self.derivative(state);
        let k2 = self.derivative(state.add(k1.scale(dt * 0.5)));
        let k3 = self.derivative(state.add(k2.scale(dt * 0.5)));
        let k4 = self.derivative(state.add(k3.scale(dt)));

        let sum_k = k1.add(k2.scale(2.0)).add(k3.scale(2.0)).add(k4);
        state.add(sum_k.scale(dt / 6.0))
    }
}
