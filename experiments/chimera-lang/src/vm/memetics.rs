#![cfg(feature = "nova")]
use super::{ChimeraVM, Value};
#[cfg(feature = "nova")]
use crate::ast::{Gene, Nucleotide};
#[cfg(feature = "nova")]
use crate::opcode::OpCode;
#[cfg(feature = "nova")]
use rand::Rng;

#[derive(Debug, Clone, PartialEq)]
pub struct Meme {
    pub genes: Vec<Gene>,
    pub virulence: u8, // 0-100
    pub fidelity: u8,  // 0-100
    pub description: String,
}

#[derive(Debug, Clone, Default)]
pub struct MemePool {
    pub memes: Vec<Meme>,
}

impl MemePool {
    pub fn new() -> Self {
        Self { memes: Vec::new() }
    }
}

pub fn exec_memetics_op(
    vm: &mut ChimeraVM,
    op: OpCode,
    _args: &[Nucleotide],
) -> Option<(usize, usize)> {
    match op {
        OpCode::Conceive => {
            // Stack: [ ..., len, virulence, fidelity ]
            if vm.stack.len() >= 3 {
                let fid_val = vm.stack.pop().unwrap();
                let vir_val = vm.stack.pop().unwrap();
                let len_val = vm.stack.pop().unwrap();

                if let (Value::Int(l), Value::Int(v), Value::Int(f)) = (len_val, vir_val, fid_val) {
                    let len = l.max(1) as usize;
                    let virulence = v.clamp(0, 100) as u8;
                    let fidelity = f.clamp(0, 100) as u8;

                    let s_idx = vm.ip.0;
                    if s_idx < vm.dna.helix.strands.len() {
                        let strand = &vm.dna.helix.strands[s_idx];
                        let start_gene = vm.ip.1;
                        // Copy genes from current IP onwards
                        let end_gene = (start_gene + len).min(strand.genes.len());
                        if start_gene < end_gene {
                            let genes = strand.genes[start_gene..end_gene].to_vec();
                            let meme = Meme {
                                genes,
                                virulence,
                                fidelity,
                                description: format!("Meme from Strand {}", s_idx),
                            };
                            let id = vm.meme_pool.memes.len();
                            vm.meme_pool.memes.push(meme);
                            vm.stack.push(Value::Int(id as i64));
                            vm.output.push(format!(
                                "CONCEIVE: Created Meme {} (V:{} F:{})",
                                id, virulence, fidelity
                            ));
                        } else {
                            vm.stack.push(Value::Int(-1));
                            vm.output
                                .push("CONCEIVE: No genes to conceptualize".to_string());
                        }
                    }
                } else {
                    vm.output
                        .push("Error: Type mismatch for conceive".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for conceive".to_string());
            }
            None
        }
        OpCode::Propagate => {
            // Stack: [ ..., meme_id, target_strand ]
            if vm.stack.len() >= 2 {
                let target_val = vm.stack.pop().unwrap();
                let meme_val = vm.stack.pop().unwrap();

                if let (Value::Int(m_id), Value::Int(t_idx)) = (meme_val, target_val) {
                    let meme_idx = m_id as usize;
                    let target_idx = t_idx as usize;

                    if meme_idx < vm.meme_pool.memes.len()
                        && target_idx < vm.dna.helix.strands.len()
                    {
                        let meme = &vm.meme_pool.memes[meme_idx];
                        let mut rng = rand::thread_rng();

                        // Virulence check
                        if rng.gen_range(0..100) < meme.virulence {
                            let mut new_genes = meme.genes.clone();

                            // Fidelity check (Mutation)
                            if rng.gen_range(0..100) > meme.fidelity {
                                // Apply simple mutation to one gene
                                if !new_genes.is_empty() {
                                    let g_idx = rng.gen_range(0..new_genes.len());
                                    // Mutate arg if possible
                                    if !new_genes[g_idx].args.is_empty() {
                                        new_genes[g_idx].args[0] =
                                            Nucleotide::Number(rng.gen_range(0..100));
                                    }
                                }
                            }

                            // Append to target strand
                            vm.dna.helix.strands[target_idx].genes.extend(new_genes);
                            vm.output.push(format!(
                                "PROPAGATE: Infected Strand {} with Meme {}",
                                target_idx, meme_idx
                            ));
                        } else {
                            vm.output
                                .push("PROPAGATE: Infection failed (Resisted)".to_string());
                        }
                    } else {
                        vm.output
                            .push("Error: Invalid IDs for propagate".to_string());
                    }
                } else {
                    vm.output
                        .push("Error: Type mismatch for propagate".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for propagate".to_string());
            }
            None
        }
        OpCode::Forget => {
            if let Some(val) = vm.stack.pop() {
                if let Value::Int(id) = val {
                    let idx = id as usize;
                    if idx < vm.meme_pool.memes.len() {
                        vm.meme_pool.memes.remove(idx);
                        vm.output.push(format!("FORGET: Removed Meme {}", idx));
                    }
                }
            }
            None
        }
        OpCode::Shibboleth => {
            // Stack: [ ..., from_op_str, to_op_str ]
            if vm.stack.len() >= 2 {
                let to_val = vm.stack.pop().unwrap();
                let from_val = vm.stack.pop().unwrap();

                if let (Value::Str(from), Value::Str(to)) = (from_val, to_val) {
                    if let (Ok(from_op), Ok(to_op)) = (from.parse::<OpCode>(), to.parse::<OpCode>())
                    {
                        let s_idx = vm.ip.0;
                        let strand_dialect = vm.dialects.entry(s_idx).or_default();
                        strand_dialect.insert(from_op.clone(), to_op.clone());
                        vm.output.push(format!(
                            "SHIBBOLETH: Strand {} maps {} -> {}",
                            s_idx, from_op, to_op
                        ));
                    } else {
                        vm.output
                            .push("Error: Invalid OpCodes for shibboleth".to_string());
                    }
                } else {
                    vm.output
                        .push("Error: Type mismatch for shibboleth".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for shibboleth".to_string());
            }
            None
        }
        _ => None,
    }
}
