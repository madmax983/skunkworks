pub trait ChaoticMap {
    fn iterate(&self, x: f64) -> f64;
    fn derivative(&self, x: f64) -> f64;
}

pub struct LogisticMap {
    pub r: f64,
}

impl LogisticMap {
    pub fn new(r: f64) -> Self {
        Self { r }
    }
}

impl ChaoticMap for LogisticMap {
    fn iterate(&self, x: f64) -> f64 {
        self.r * x * (1.0 - x)
    }

    fn derivative(&self, x: f64) -> f64 {
        self.r * (1.0 - 2.0 * x)
    }
}

pub fn calculate_lyapunov(map: &impl ChaoticMap, initial_x: f64, steps: usize) -> f64 {
    let mut x = initial_x;
    let mut sum_log_deriv = 0.0;

    for _ in 0..steps {
        let deriv = map.derivative(x).abs();
        if deriv > 1e-9 {
            sum_log_deriv += deriv.ln();
        } else {
             // Handle derivative 0 case (super-stable fixed point).
             // Log(0) is -inf. We return a sufficiently large negative number to represent stability.
             return -10.0;
        }
        x = map.iterate(x);
    }

    sum_log_deriv / (steps as f64)
}
