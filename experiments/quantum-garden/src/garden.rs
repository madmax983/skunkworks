use crate::quantum::{Gate, QubitSystem};
use anyhow::Result;
use ratatui::style::Color;

#[derive(Debug, Clone)]
pub struct Plant {
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
        for _ in 0..num_qubits {
            plants.push(Plant {
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

    pub fn apply_cnot(&mut self, control: usize, target: usize) -> Result<()> {
        self.system.apply_cnot(control, target)?;

        // Mark as entangled visually
        self.plants[control].is_entangled = true;
        self.plants[target].is_entangled = true;
        self.plants[control].entangled_with.push(target);
        self.plants[target].entangled_with.push(control);

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
