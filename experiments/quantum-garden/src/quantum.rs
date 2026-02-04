use anyhow::Result;
use num_complex::Complex64;
use rand::Rng;
use std::f64::consts::FRAC_1_SQRT_2;

/// A simple quantum system simulator.
/// State is represented as a vector of complex amplitudes.
/// |psi> = c_0|00...0> + ... + c_N|11...1>
pub struct QubitSystem {
    pub num_qubits: usize,
    pub state: Vec<Complex64>,
}

#[derive(Debug, Clone, Copy)]
pub enum Gate {
    H,
    X,
    Z,
}

impl QubitSystem {
    pub fn new(num_qubits: usize) -> Self {
        let size = 1 << num_qubits;
        let mut state = vec![Complex64::new(0.0, 0.0); size];
        state[0] = Complex64::new(1.0, 0.0); // Start at |00...0>
        Self { num_qubits, state }
    }

    /// Apply a single qubit gate to the target qubit.
    pub fn apply_gate(&mut self, gate: Gate, target: usize) -> Result<()> {
        if target >= self.num_qubits {
            return Err(anyhow::anyhow!("Target qubit index out of bounds"));
        }

        let mut new_state = vec![Complex64::new(0.0, 0.0); self.state.len()];

        // Iterate over state vector indices
        for i in 0..self.state.len() {
            // Determine if the target bit is 0 or 1 at this index
            // If bit is 0, we can process the pair (i, i + 2^target)
            if (i & (1 << target)) == 0 {
                let zero_idx = i;
                let one_idx = i | (1 << target);

                let c0 = self.state[zero_idx];
                let c1 = self.state[one_idx];

                let (n0, n1) = match gate {
                    Gate::H => ((c0 + c1) * FRAC_1_SQRT_2, (c0 - c1) * FRAC_1_SQRT_2),
                    Gate::X => (c1, c0),
                    Gate::Z => (c0, -c1),
                };

                new_state[zero_idx] = n0;
                new_state[one_idx] = n1;
            }
        }
        self.state = new_state;
        Ok(())
    }

    /// Apply CNOT gate. Control determines if Target is flipped (X gate).
    pub fn apply_cnot(&mut self, control: usize, target: usize) -> Result<()> {
        if control >= self.num_qubits || target >= self.num_qubits {
            return Err(anyhow::anyhow!("Qubit index out of bounds"));
        }
        if control == target {
            return Err(anyhow::anyhow!("Control and Target cannot be the same"));
        }

        let mut new_state = self.state.clone();

        for i in 0..self.state.len() {
            // If control bit is set
            if (i & (1 << control)) != 0 {
                // If this is the "zero" component of the target pair
                if (i & (1 << target)) == 0 {
                    let zero_idx = i;
                    let one_idx = i | (1 << target);

                    // Swap amplitudes (X gate logic)
                    new_state.swap(zero_idx, one_idx);
                }
            }
        }
        self.state = new_state;
        Ok(())
    }

    /// Measure all qubits, collapsing the state to a single basis state.
    /// Returns the measured values as booleans.
    pub fn measure(&mut self) -> Vec<bool> {
        let mut rng = rand::thread_rng();
        let r: f64 = rng.r#gen(); // 0.0 to 1.0

        let mut cumulative_prob = 0.0;
        let mut collapsed_idx = 0;

        // Sample from distribution
        for (idx, amplitude) in self.state.iter().enumerate() {
            let prob = amplitude.norm_sqr();
            cumulative_prob += prob;
            if r <= cumulative_prob {
                collapsed_idx = idx;
                break;
            }
        }

        // Collapse state
        for i in 0..self.state.len() {
            self.state[i] = if i == collapsed_idx {
                Complex64::new(1.0, 0.0)
            } else {
                Complex64::new(0.0, 0.0)
            };
        }

        // Convert index to bits
        let mut result = Vec::new();
        for i in 0..self.num_qubits {
            result.push((collapsed_idx & (1 << i)) != 0);
        }
        result
    }

    /// Get probabilities for each qubit being |1> (marginal probability)
    pub fn get_qubit_probabilities(&self) -> Vec<f64> {
        let mut probs = vec![0.0; self.num_qubits];

        for (idx, amplitude) in self.state.iter().enumerate() {
            let prob = amplitude.norm_sqr();
            if prob > 0.0 {
                for (i, p) in probs.iter_mut().enumerate() {
                    if (idx & (1 << i)) != 0 {
                        *p += prob;
                    }
                }
            }
        }
        probs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_state() {
        let sys = QubitSystem::new(2);
        assert_eq!(sys.state[0], Complex64::new(1.0, 0.0)); // |00>
        assert_eq!(sys.state[1], Complex64::new(0.0, 0.0)); // |01>
    }

    #[test]
    fn test_hadamard() {
        let mut sys = QubitSystem::new(1);
        sys.apply_gate(Gate::H, 0).unwrap();
        // Should be |+> = 1/sqrt(2)(|0> + |1>)
        let val = FRAC_1_SQRT_2;
        assert!((sys.state[0].re - val).abs() < 1e-9);
        assert!((sys.state[1].re - val).abs() < 1e-9);

        // H again -> |0>
        sys.apply_gate(Gate::H, 0).unwrap();
        assert!((sys.state[0].re - 1.0).abs() < 1e-9);
        assert!(sys.state[1].norm() < 1e-9);
    }

    #[test]
    fn test_pauli_x() {
        let mut sys = QubitSystem::new(1);
        sys.apply_gate(Gate::X, 0).unwrap();
        // |1>
        assert!(sys.state[0].norm() < 1e-9);
        assert!((sys.state[1].re - 1.0).abs() < 1e-9);
    }

    #[test]
    fn test_cnot() {
        // |10> (qubit 0=0, qubit 1=1... wait index 0 is LSB usually)
        // Let's use standard binary ordering convention
        // Index bits: [q_N-1 ... q_1 q_0]
        // But here I used (idx & (1 << i)) which implies q_0 is LSB.

        // Prepare |10> : q0=0, q1=1. Index = 2 (binary 10).
        let mut sys = QubitSystem::new(2);
        sys.apply_gate(Gate::X, 1).unwrap(); // q1 -> 1.

        // State should be index 2: |10>
        assert!((sys.state[2].re - 1.0).abs() < 1e-9);

        // CNOT(control=1, target=0).
        // Since q1=1, q0 should flip from 0 to 1.
        // Result: |11> (index 3).
        sys.apply_cnot(1, 0).unwrap();

        assert!(sys.state[2].norm() < 1e-9);
        assert!((sys.state[3].re - 1.0).abs() < 1e-9);
    }
}
