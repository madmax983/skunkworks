use crate::opcode::OpCode;
use crate::vm::ChimeraVM;
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spell {
    pub name: String,
    pub cost: i64,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Codex {
    pub spells: Vec<Spell>,
}

impl Default for Codex {
    fn default() -> Self {
        Self::new()
    }
}

impl Codex {
    pub fn new() -> Self {
        Self {
            spells: vec![
                Spell {
                    name: "Void Bind".to_string(),
                    cost: 50,
                    description: "Binds the current strand to the Void (Clears genes).".to_string(),
                },
                Spell {
                    name: "Chroma Shift".to_string(),
                    cost: 20,
                    description: "Shifts the color of the grid cells.".to_string(),
                },
                Spell {
                    name: "Time Echo".to_string(),
                    cost: 100,
                    description: "Creates a copy of the strand that executes 1 tick later."
                        .to_string(),
                },
                Spell {
                    name: "Transmute".to_string(),
                    cost: 30,
                    description: "Randomly changes OpCodes in the current strand.".to_string(),
                },
                Spell {
                    name: "Amplify".to_string(),
                    cost: 75,
                    description: "Duplicates all genes in the current strand.".to_string(),
                },
            ],
        }
    }

    pub fn get_spell(&self, id: usize) -> Option<Spell> {
        self.spells.get(id).cloned()
    }
}

pub fn exec_spell(vm: &mut ChimeraVM, spell: &Spell) {
    if vm.energy < spell.cost {
        vm.output.push(format!(
            "CODEX: Not enough energy for {} (Need {})",
            spell.name, spell.cost
        ));
        return;
    }

    vm.energy -= spell.cost;
    vm.output.push(format!("CODEX: Casting {}...", spell.name));

    match spell.name.as_str() {
        "Void Bind" => {
            if vm.ip.0 < vm.dna.helix.strands.len() {
                vm.dna.helix.strands[vm.ip.0].genes.clear();
                vm.output.push("CODEX: Strand bound to Void.".to_string());
            }
        }
        "Chroma Shift" => {
            #[cfg(feature = "nova")]
            {
                let mut rng = rand::thread_rng();
                for row in vm.chroma_grid.iter_mut() {
                    for cell in row.iter_mut() {
                        if let Some((r, g, b)) = cell.fg {
                            let shift = rng.gen_range(0..3);
                            cell.fg = Some(match shift {
                                0 => (g, b, r),
                                1 => (b, r, g),
                                _ => (r, g, b),
                            });
                        }
                    }
                }
                vm.output.push("CODEX: Reality colors shifted.".to_string());
            }
            #[cfg(not(feature = "nova"))]
            {
                vm.output
                    .push("CODEX: Chroma Shift requires Nova feature.".to_string());
            }
        }
        "Time Echo" => {
            if vm.ip.0 < vm.dna.helix.strands.len() {
                let strand = vm.dna.helix.strands[vm.ip.0].clone();
                vm.dna.helix.strands.push(strand);
                vm.output.push("CODEX: Temporal Echo created.".to_string());
            }
        }
        "Transmute" => {
            if vm.ip.0 < vm.dna.helix.strands.len() {
                let mut rng = rand::thread_rng();
                let len = vm.dna.helix.strands[vm.ip.0].genes.len();
                if len > 0 {
                    // Mutate 10% of genes
                    let count = (len / 10).max(1);
                    for _ in 0..count {
                        let idx = rng.gen_range(0..len);
                        // Simple mutation: Nop
                        vm.dna.helix.strands[vm.ip.0].genes[idx].op = OpCode::Nop;
                    }
                    vm.output.push("CODEX: Strand transmuted.".to_string());
                }
            }
        }
        "Amplify" => {
            if vm.ip.0 < vm.dna.helix.strands.len() {
                let genes = vm.dna.helix.strands[vm.ip.0].genes.clone();
                vm.dna.helix.strands[vm.ip.0].genes.extend(genes);
                vm.output.push("CODEX: Strand amplified.".to_string());
            }
        }
        _ => {}
    }
}
