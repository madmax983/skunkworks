use anyhow::{anyhow, Result};
use num_complex::Complex64;
use rand::Rng;
use std::collections::HashMap;
use std::f64::consts::FRAC_1_SQRT_2;

/// A quantum system containing N qubits in a pure state.
/// State vector size is 2^N.
#[derive(Debug, Clone)]
pub struct QubitSystem {
    pub num_qubits: usize,
    pub state: Vec<Complex64>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Gate {
    H,
    X,
    Z,
    CNOT, // Used for inventory/logic differentiation
}

impl QubitSystem {
    pub fn new(num_qubits: usize) -> Self {
        let size = 1 << num_qubits;
        let mut state = vec![Complex64::new(0.0, 0.0); size];
        // Initialize to |00...0>
        state[0] = Complex64::new(1.0, 0.0);
        Self { num_qubits, state }
    }

    /// Creates a system from a specific state (e.g., |1>)
    pub fn from_state(val: usize, num_qubits: usize) -> Self {
        let size = 1 << num_qubits;
        let mut state = vec![Complex64::new(0.0, 0.0); size];
        if val < size {
            state[val] = Complex64::new(1.0, 0.0);
        } else {
            state[0] = Complex64::new(1.0, 0.0);
        }
        Self { num_qubits, state }
    }

    pub fn apply_gate(&mut self, gate: Gate, target: usize) -> Result<()> {
        if target >= self.num_qubits {
            return Err(anyhow!("Target qubit index out of bounds"));
        }

        let mut new_state = vec![Complex64::new(0.0, 0.0); self.state.len()];

        for i in 0..self.state.len() {
            if (i & (1 << target)) == 0 {
                let zero_idx = i;
                let one_idx = i | (1 << target);

                let c0 = self.state[zero_idx];
                let c1 = self.state[one_idx];

                let (n0, n1) = match gate {
                    Gate::H => ((c0 + c1) * FRAC_1_SQRT_2, (c0 - c1) * FRAC_1_SQRT_2),
                    Gate::X => (c1, c0),
                    Gate::Z => (c0, -c1),
                    _ => (c0, c1), // CNOT not handled here
                };

                new_state[zero_idx] = n0;
                new_state[one_idx] = n1;
            }
        }
        self.state = new_state;
        Ok(())
    }

    pub fn apply_cnot(&mut self, control: usize, target: usize) -> Result<()> {
        if control >= self.num_qubits || target >= self.num_qubits {
            return Err(anyhow!("Qubit index out of bounds"));
        }
        if control == target {
            return Err(anyhow!("Control and Target cannot be the same"));
        }

        let mut new_state = self.state.clone();

        for i in 0..self.state.len() {
            if (i & (1 << control)) != 0 {
                if (i & (1 << target)) == 0 {
                    let zero_idx = i;
                    let one_idx = i | (1 << target);
                    new_state.swap(zero_idx, one_idx);
                }
            }
        }
        self.state = new_state;
        Ok(())
    }

    pub fn measure(&mut self) -> Vec<bool> {
        let mut rng = rand::thread_rng();
        let r: f64 = rng.gen();

        let mut cumulative_prob = 0.0;
        let mut collapsed_idx = 0;

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

        let mut result = Vec::new();
        for i in 0..self.num_qubits {
            result.push((collapsed_idx & (1 << i)) != 0);
        }
        result
    }

    /// Calculates marginal probability of each qubit being |1>
    pub fn get_probabilities(&self) -> Vec<f64> {
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

    /// Tensor product of self and other. Returns new system.
    /// New qubits are [self qubits ..., other qubits ...]
    pub fn tensor_product(&self, other: &QubitSystem) -> QubitSystem {
        let new_num = self.num_qubits + other.num_qubits;
        let new_size = 1 << new_num;
        let mut new_state = Vec::with_capacity(new_size);

        // State vector tensor product: v = v1 (x) v2
        // v[i * size2 + j] = v1[i] * v2[j]
        // Actually, the ordering depends on which qubits are more significant.
        // If self is qubits 0..N-1 and other is N..N+M-1.
        // Index k = (bits of other) << N | (bits of self)

        // Let's assume self is lower bits (0..N-1) and other is higher bits (N..N+M-1).

        for j in 0..other.state.len() { // High bits
            for i in 0..self.state.len() { // Low bits
                new_state.push(self.state[i] * other.state[j]);
            }
        }

        QubitSystem {
            num_qubits: new_num,
            state: new_state,
        }
    }
}

pub struct QuantumManager {
    // We use a simplified ID allocation.
    // systems store the actual quantum data.
    // keys in `systems` are SystemIDs.
    next_system_id: usize,
    pub systems: HashMap<usize, QubitSystem>,

    // Map EntityID -> (SystemID, QubitIndex)
    pub entity_map: HashMap<usize, (usize, usize)>,
}

impl QuantumManager {
    pub fn new() -> Self {
        Self {
            next_system_id: 0,
            systems: HashMap::new(),
            entity_map: HashMap::new(),
        }
    }

    pub fn add_qubit(&mut self, entity_id: usize, initial_state_one: bool) {
        let sys_id = self.next_system_id;
        self.next_system_id += 1;

        let sys = if initial_state_one {
            QubitSystem::from_state(1, 1) // |1>
        } else {
            QubitSystem::from_state(0, 1) // |0>
        };

        self.systems.insert(sys_id, sys);
        self.entity_map.insert(entity_id, (sys_id, 0));
    }

    pub fn apply_gate(&mut self, gate: Gate, entity_id: usize) -> Result<()> {
        if let Some(&(sys_id, qubit_idx)) = self.entity_map.get(&entity_id) {
            if let Some(sys) = self.systems.get_mut(&sys_id) {
                sys.apply_gate(gate, qubit_idx)?;
            }
        }
        Ok(())
    }

    pub fn entangle(&mut self, id1: usize, id2: usize) -> Result<()> {
        let (sys1_id, idx1) = *self.entity_map.get(&id1).ok_or(anyhow!("Entity 1 not found"))?;
        let (sys2_id, idx2) = *self.entity_map.get(&id2).ok_or(anyhow!("Entity 2 not found"))?;

        if sys1_id == sys2_id {
            // Already in same system, just apply CNOT
            if let Some(sys) = self.systems.get_mut(&sys1_id) {
                sys.apply_cnot(idx1, idx2)?;
            }
        } else {
            // Merge systems
            // Remove sys2, merge into sys1 (or create new)
            // Strategy: Create new merged system, reassign all entities from sys1 and sys2 to new system.

            let sys1 = self.systems.remove(&sys1_id).unwrap();
            let sys2 = self.systems.remove(&sys2_id).unwrap();

            // Check size limit. Max 10 qubits per system to prevent explosion?
            if sys1.num_qubits + sys2.num_qubits > 10 {
                // Restore systems
                self.systems.insert(sys1_id, sys1);
                self.systems.insert(sys2_id, sys2);
                return Err(anyhow!("Quantum System too large to entangle!"));
            }

            let merged = sys1.tensor_product(&sys2);

            let new_sys_id = self.next_system_id;
            self.next_system_id += 1;

            // Re-map entities
            // Entities from sys1 keep their index (0..N-1)
            // Entities from sys2 get shifted by sys1.num_qubits (N..N+M-1)

            let offset = sys1.num_qubits;

            // We need to find all entities that pointed to sys1_id or sys2_id
            // Iterating whole map is slow but fine for now.
            for val in self.entity_map.values_mut() {
                if val.0 == sys1_id {
                    val.0 = new_sys_id;
                    // val.1 stays same
                } else if val.0 == sys2_id {
                    val.0 = new_sys_id;
                    val.1 += offset;
                }
            }

            self.systems.insert(new_sys_id, merged);

            // Now apply CNOT in the new system
            // Target indices are updated implicitly by the mapping logic
            let new_idx1 = idx1;
            let new_idx2 = idx2 + offset;

            if let Some(sys) = self.systems.get_mut(&new_sys_id) {
                sys.apply_cnot(new_idx1, new_idx2)?;
            }
        }
        Ok(())
    }

    pub fn measure(&mut self, entity_id: usize) -> Result<bool> {
        let (sys_id, qubit_idx) = *self.entity_map.get(&entity_id).ok_or(anyhow!("Entity not found"))?;

        let result_bit = if let Some(sys) = self.systems.get_mut(&sys_id) {
            let results = sys.measure();
            results[qubit_idx]
        } else {
            return Err(anyhow!("System not found"));
        };

        // Optimization: After measurement, the system is in a product state (basis state).
        // We *could* split it back into independent qubits to save performance.
        // For this game, let's do it to keep the "System too large" check from blocking us forever.
        // Splitting is easy: everyone is now independent |0> or |1>.

        self.dissolve_system(sys_id)?;

        Ok(result_bit)
    }

    /// Splits a system into individual 1-qubit systems based on current state.
    /// Should only be called after measurement (state is collapsed).
    fn dissolve_system(&mut self, sys_id: usize) -> Result<()> {
        let sys = self.systems.remove(&sys_id).ok_or(anyhow!("System not found"))?;

        // Find the collapsed value for each qubit
        let mut collapsed_idx = 0;
        for (i, val) in sys.state.iter().enumerate() {
            if val.norm_sqr() > 0.99 {
                collapsed_idx = i;
                break;
            }
        }

        // Gather all entities belonging to this system
        let mut entities_in_sys: Vec<(usize, usize)> = Vec::new(); // (entity_id, qubit_idx)
        for (eid, (sid, qid)) in &self.entity_map {
            if *sid == sys_id {
                entities_in_sys.push((*eid, *qid));
            }
        }

        // Create new systems for each entity
        for (eid, qid) in entities_in_sys {
            let bit_val = (collapsed_idx & (1 << qid)) != 0;

            let new_sys_id = self.next_system_id;
            self.next_system_id += 1;

            let new_sys = if bit_val {
                QubitSystem::from_state(1, 1) // |1>
            } else {
                QubitSystem::from_state(0, 1) // |0>
            };

            self.systems.insert(new_sys_id, new_sys);
            self.entity_map.insert(eid, (new_sys_id, 0));
        }

        Ok(())
    }

    pub fn get_probability(&self, entity_id: usize) -> f64 {
        if let Some(&(sys_id, qubit_idx)) = self.entity_map.get(&entity_id) {
            if let Some(sys) = self.systems.get(&sys_id) {
                // Determine probability
                // We can't cache this easily as it changes with gates.
                let probs = sys.get_probabilities();
                if qubit_idx < probs.len() {
                    return probs[qubit_idx];
                }
            }
        }
        0.0
    }
}
