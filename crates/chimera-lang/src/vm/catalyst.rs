use super::ChimeraVM;
use crate::opcode::OpCode;
use crate::value::Value;
use rand::Rng;

#[derive(Debug, Clone, PartialEq)]
pub struct Catalyst {
    pub id: u64,
    pub recipe: Vec<OpCode>,
    pub charge: i64,
    pub stability: f64,
}

pub fn synthesize(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // stack: strand_idx
    if let Some(val) = vm.stack.pop() {
        if let Value::Int(idx) = val {
            let s_idx = idx as usize;
            if s_idx < vm.dna.helix.strands.len() {
                let strand = &vm.dna.helix.strands[s_idx];
                let mut recipe = Vec::new();
                for gene in &strand.genes {
                    recipe.push(gene.op.clone());
                }

                vm.organelle_id_counter += 1;
                let id = vm.organelle_id_counter; // Reuse counter for unique IDs

                let catalyst = Catalyst {
                    id,
                    recipe,
                    charge: 100,    // Initial charge
                    stability: 0.9, // 90% stability
                };

                vm.catalysts.push(catalyst);
                vm.energy = vm.energy.saturating_sub(50);
                vm.output
                    .push(format!("SYNTHESIZE: Created Catalyst #{}", id));
                vm.stack.push(Value::Int(id as i64));
            } else {
                vm.output
                    .push("Error: Invalid strand index for synthesize".to_string());
            }
        } else {
            vm.output
                .push("Error: Type mismatch for synthesize".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for synthesize".to_string());
    }
    None
}

pub fn catalyze(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // stack: catalyst_id, target_strand_idx
    if vm.stack.len() >= 2 {
        let target_val = vm.stack.pop().unwrap();
        let cat_val = vm.stack.pop().unwrap();

        if let (Value::Int(cid), Value::Int(tid)) = (cat_val, target_val) {
            let t_idx = tid as usize;
            let c_id = cid as u64;

            // Find catalyst
            let mut cat_idx = None;
            for (i, c) in vm.catalysts.iter().enumerate() {
                if c.id == c_id {
                    cat_idx = Some(i);
                    break;
                }
            }

            if let Some(idx) = cat_idx {
                // We need to access catalyst fields and also modify vm.dna.
                // To avoid double borrow, we clone necessary data or indices.
                let (recipe, stability, charge_ok) = {
                    let c = &mut vm.catalysts[idx];
                    let ok = c.charge > 0;
                    if ok {
                        c.charge -= 10;
                    }
                    (c.recipe.clone(), c.stability, ok)
                };

                if charge_ok {
                    if t_idx < vm.dna.helix.strands.len() {
                        let strand = &mut vm.dna.helix.strands[t_idx];
                        let mut rng = rand::thread_rng();

                        // Apply recipe
                        let mut mutations = 0;
                        for gene in strand.genes.iter_mut() {
                            if rng.gen_bool(1.0 - stability) {
                                // Mutate based on recipe
                                if !recipe.is_empty() {
                                    let op = recipe[rng.gen_range(0..recipe.len())].clone();
                                    gene.op = op;
                                    mutations += 1;
                                }
                            }
                        }

                        vm.energy = vm.energy.saturating_sub(20);
                        vm.output.push(format!(
                            "CATALYZE: Applied #{} to Strand {} ({} mutations)",
                            c_id, t_idx, mutations
                        ));
                    } else {
                        vm.output.push("Error: Invalid target strand".to_string());
                    }
                } else {
                    vm.output
                        .push(format!("CATALYZE: Catalyst #{} depleted", c_id));
                }
            } else {
                vm.output
                    .push(format!("CATALYZE: Catalyst #{} not found", c_id));
            }
        } else {
            vm.output
                .push("Error: Type mismatch for catalyze".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for catalyze".to_string());
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::ChimeraVM;

    #[test]
    fn test_synthesize_and_catalyze() {
        // Strand 0: Template [ Push(1), Push(2) ]
        // Strand 1: Target [ Nop, Nop, Nop ]
        // Strand 2: Controller [ Push(0) Synthesize, Push(1) Push(1) Catalyze ]

        let s0 = Strand {
            genes: vec![
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(1)],
                },
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(2)],
                },
            ],
        };

        let s1 = Strand {
            genes: vec![
                Gene {
                    op: OpCode::Nop,
                    args: vec![],
                },
                Gene {
                    op: OpCode::Nop,
                    args: vec![],
                },
                Gene {
                    op: OpCode::Nop,
                    args: vec![],
                },
            ],
        };

        let s2 = Strand {
            genes: vec![
                // Synthesize(0)
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(0)],
                },
                Gene {
                    op: OpCode::Synthesize,
                    args: vec![],
                },
                // Catalyze(cat_id=1, target=1)
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(1)],
                }, // Target
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(1)],
                }, // Cat ID (starts at 1)
                Gene {
                    op: OpCode::Catalyze,
                    args: vec![],
                },
            ],
        };

        let dna = Dna { evolution_config: None,
            helix: Helix {
                strands: vec![s0, s1, s2],
            },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.energy = 1000; // Give enough energy

        vm.ip = (2, 0); // Start at controller

        vm.step(); // Push 0
        vm.step(); // Synthesize

        assert_eq!(vm.catalysts.len(), 1);
        let cat = &vm.catalysts[0];
        assert_eq!(cat.id, 1);
        assert_eq!(cat.recipe.len(), 2);
        assert_eq!(cat.recipe[0], OpCode::Push);

        // Force instability for test (so mutations happen)
        vm.catalysts[0].stability = 0.0;

        vm.step(); // Push 1 (Target)
        vm.step(); // Push 1 (Cat ID)
        vm.step(); // Catalyze

        // Check if Strand 1 was mutated
        let mutated = vm.dna.helix.strands[1]
            .genes
            .iter()
            .any(|g| g.op == OpCode::Push);
        // With stability 0.0, everything should mutate to Push
        assert!(mutated, "Target strand should be mutated");

        let push_count = vm.dna.helix.strands[1]
            .genes
            .iter()
            .filter(|g| g.op == OpCode::Push)
            .count();
        assert_eq!(
            push_count, 3,
            "All Nops should be replaced by Push (from recipe)"
        );
    }
}
