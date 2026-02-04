use crate::quantum::{Gate, QubitSystem};
use anyhow::Result;
use ratatui::style::Color;

#[derive(Debug, Clone)]
pub struct Plant {
    #[allow(dead_code)]
    pub index: usize,
    pub height: f64, // 0.0 to 1.0 (Probability of |1>)
    pub phase_color: Color,
    pub is_entangled: bool,
    pub entangled_with: Vec<usize>,
}

pub struct Garden {
    pub system: QubitSystem,
    pub plants: Vec<Plant>,
    pub selected_index: usize,
}

impl Garden {
    pub fn new(num_qubits: usize) -> Self {
        let system = QubitSystem::new(num_qubits);
        let mut plants = Vec::new();
        for i in 0..num_qubits {
            plants.push(Plant {
                index: i,
                height: 0.0,
                phase_color: Color::Green,
                is_entangled: false,
                entangled_with: Vec::new(),
            });
        }

        Self {
            system,
            plants,
            selected_index: 0,
        }
    }

    pub fn update(&mut self) {
        let probs = self.system.get_qubit_probabilities();

        // Simple entanglement check (very heuristic for visualization)
        // If applying a gate to A affects B's probability significantly, they are linked.
        // But checking that is expensive (need to simulate).
        // For now, let's just track "entangled pairs" manually via the CNOT history if we wanted,
        // but the Plan said "is_entangled: bool".
        // Let's rely on the user knowing they CNOTed them, or just omit entanglement visual for MVP
        // if it's too hard to derive from state.
        // Actually, we can check mutual information or covariance, but that's O(2^N).
        // Let's just update height for now.

        for (i, plant) in self.plants.iter_mut().enumerate() {
            plant.height = probs[i];

            // Map height to color roughly
            if plant.height < 0.1 {
                plant.phase_color = Color::DarkGray; // Seed
            } else if plant.height > 0.9 {
                plant.phase_color = Color::Red; // Bloom
            } else {
                plant.phase_color = Color::Cyan; // Superposition
            }
        }
    }

    pub fn apply_gate(&mut self, gate: Gate) -> Result<()> {
        self.system.apply_gate(gate, self.selected_index)?;
        self.update();
        Ok(())
    }

    #[allow(dead_code)]
    pub fn apply_cnot(&mut self, target_idx: usize) -> Result<()> {
        self.system.apply_cnot(self.selected_index, target_idx)?;

        // Mark as entangled visually
        self.plants[self.selected_index].is_entangled = true;
        self.plants[target_idx].is_entangled = true;
        self.plants[self.selected_index]
            .entangled_with
            .push(target_idx);
        self.plants[target_idx]
            .entangled_with
            .push(self.selected_index);

        self.update();
        Ok(())
    }

    pub fn measure(&mut self) {
        self.system.measure();
        self.update();
        // Reset visual entanglement on measure?
        // In reality measurement breaks entanglement.
        for plant in &mut self.plants {
            plant.is_entangled = false;
            plant.entangled_with.clear();
        }
    }
}
