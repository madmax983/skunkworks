use num_complex::Complex;
use rand::Rng;
use std::f64::consts::FRAC_1_SQRT_2;

#[derive(Clone, Debug)]
pub struct StateVector {
    pub num_qubits: usize,
    pub amplitudes: Vec<Complex<f64>>,
}

impl StateVector {
    pub fn new(num_qubits: usize) -> Self {
        let size = 1 << num_qubits;
        let mut amplitudes = vec![Complex::new(0.0, 0.0); size];
        amplitudes[0] = Complex::new(1.0, 0.0); // Initialize to |0...0>
        Self {
            num_qubits,
            amplitudes,
        }
    }

    pub fn apply_hadamard(&mut self, target: usize) {
        let size = self.amplitudes.len();
        let step = 1 << target;
        for i in (0..size).step_by(step * 2) {
            for j in 0..step {
                let idx0 = i + j;
                let idx1 = idx0 + step;
                let a = self.amplitudes[idx0];
                let b = self.amplitudes[idx1];
                self.amplitudes[idx0] = (a + b) * FRAC_1_SQRT_2;
                self.amplitudes[idx1] = (a - b) * FRAC_1_SQRT_2;
            }
        }
    }

    pub fn apply_x(&mut self, target: usize) {
        let size = self.amplitudes.len();
        let step = 1 << target;
        for i in (0..size).step_by(step * 2) {
            for j in 0..step {
                let idx0 = i + j;
                let idx1 = idx0 + step;
                self.amplitudes.swap(idx0, idx1);
            }
        }
    }

    pub fn apply_z(&mut self, target: usize) {
        let size = self.amplitudes.len();
        let step = 1 << target;
        for i in (0..size).step_by(step * 2) {
            for j in 0..step {
                let idx1 = i + j + step;
                self.amplitudes[idx1] = -self.amplitudes[idx1];
            }
        }
    }

    pub fn apply_cnot(&mut self, control: usize, target: usize) {
        let size = self.amplitudes.len();
        for i in 0..size {
            // Check if control bit is 1
            if (i >> control) & 1 == 1 {
                // If so, flip target bit
                let pair_idx = i ^ (1 << target);
                if i < pair_idx {
                    self.amplitudes.swap(i, pair_idx);
                }
            }
        }
    }

    pub fn get_prob(&self, qubit: usize) -> f64 {
        let mut prob = 0.0;
        for (i, amp) in self.amplitudes.iter().enumerate() {
            if (i >> qubit) & 1 == 1 {
                prob += amp.norm_sqr();
            }
        }
        prob
    }

    pub fn measure(&mut self, qubit: usize) -> bool {
        let p1 = self.get_prob(qubit);
        let mut rng = rand::thread_rng();
        let collapse_to_one = rng.gen::<f64>() < p1;

        let mut normalization = 0.0;
        for (i, amp) in self.amplitudes.iter_mut().enumerate() {
            let is_one = (i >> qubit) & 1 == 1;
            if is_one != collapse_to_one {
                *amp = Complex::new(0.0, 0.0);
            } else {
                normalization += amp.norm_sqr();
            }
        }

        if normalization > 0.0 {
            let norm_factor = 1.0 / normalization.sqrt();
            for amp in self.amplitudes.iter_mut() {
                *amp *= norm_factor;
            }
        }

        collapse_to_one
    }
}
