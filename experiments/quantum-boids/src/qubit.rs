use num_complex::Complex;
use rand::Rng;
use std::f64::consts::FRAC_1_SQRT_2;

#[derive(Clone, Copy, Debug)]
pub struct Qubit {
    pub alpha: Complex<f64>,
    pub beta: Complex<f64>,
}

impl Default for Qubit {
    fn default() -> Self {
        Self {
            alpha: Complex::new(1.0, 0.0),
            beta: Complex::new(0.0, 0.0),
        }
    }
}

impl Qubit {
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the probability of measuring |1>
    pub fn prob_one(&self) -> f64 {
        self.beta.norm_sqr()
    }

    /// Returns the phase of beta relative to alpha (simplified)
    pub fn phase(&self) -> f64 {
        self.beta.arg() - self.alpha.arg()
    }

    /// Apply Hadamard gate
    pub fn h(&mut self) {
        let a = self.alpha;
        let b = self.beta;
        self.alpha = (a + b) * FRAC_1_SQRT_2;
        self.beta = (a - b) * FRAC_1_SQRT_2;
        self.normalize();
    }

    /// Apply Pauli-X gate (NOT)
    pub fn x(&mut self) {
        let temp = self.alpha;
        self.alpha = self.beta;
        self.beta = temp;
    }

    /// Apply Pauli-Z gate (Phase Flip)
    pub fn z(&mut self) {
        self.beta = -self.beta;
    }

    /// Normalize the state vector
    fn normalize(&mut self) {
        let norm = (self.alpha.norm_sqr() + self.beta.norm_sqr()).sqrt();
        if norm > 0.0 {
            self.alpha /= norm;
            self.beta /= norm;
        }
    }

    /// Measure the qubit, collapsing it to |0> or |1>
    pub fn measure(&mut self) -> bool {
        let p = self.prob_one();
        let mut rng = rand::thread_rng();
        if rng.gen::<f64>() < p {
            self.alpha = Complex::new(0.0, 0.0);
            self.beta = Complex::new(1.0, 0.0);
            true
        } else {
            self.alpha = Complex::new(1.0, 0.0);
            self.beta = Complex::new(0.0, 0.0);
            false
        }
    }
}

/// Apply CNOT logic roughly.
/// Since we don't have a full multi-qubit state vector, we simulate entanglement
/// by correlating their changes.
/// This function updates target based on control's probability.
pub fn apply_cnot_approx(control: &Qubit, target: &mut Qubit) {
    // If control is likely |1>, flip target
    let p_control = control.prob_one();
    let mut rng = rand::thread_rng();

    // Probabilistic flip based on control's state
    if rng.gen::<f64>() < p_control {
        target.x();
    }
}
