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

#[derive(Debug, Clone, PartialEq)]
pub struct Virus {
    pub name: String,
    pub color: (u8, u8, u8), // RGB
    pub pattern: String,     // Target text pattern (contains match)
    pub mutation_rate: u8,   // 0-100
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct ViralState {
    pub infection_level: u8, // 0-255
    pub virus_id: usize,     // Index in virus_library
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
        OpCode::Infect => {
            // Stack: [ ..., mutation_rate, pattern_str, name_str ]
            if vm.stack.len() >= 3 {
                let name_val = vm.stack.pop().unwrap();
                let pattern_val = vm.stack.pop().unwrap();
                let rate_val = vm.stack.pop().unwrap();

                if let (Value::Str(name), Value::Str(pattern), Value::Int(rate)) =
                    (name_val, pattern_val, rate_val)
                {
                    let mutation_rate = rate.clamp(0, 100) as u8;
                    let mut rng = rand::thread_rng();
                    let color = (
                        rng.gen_range(50..255),
                        rng.gen_range(50..255),
                        rng.gen_range(50..255),
                    );

                    let virus = Virus {
                        name: name.clone(),
                        color,
                        pattern: pattern.clone(),
                        mutation_rate,
                    };

                    let virus_id = vm.virus_library.len();
                    vm.virus_library.push(virus);

                    let (cy, cx) = vm.context_loc;
                    vm.viral_grid[cy][cx] = Some(ViralState {
                        infection_level: 100,
                        virus_id,
                    });

                    vm.output.push(format!(
                        "INFECT: Released '{}' (ID {}) at {},{}",
                        name, virus_id, cx, cy
                    ));
                } else {
                    vm.output
                        .push("Error: Type mismatch for infect".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for infect".to_string());
            }
            None
        }
        OpCode::Outbreak => {
            let size = crate::vm::GRID_SIZE;
            let mut next_viral_grid = vm.viral_grid.clone();
            let mut spread_count = 0;
            let mut mutation_count = 0;

            for y in 0..size {
                for x in 0..size {
                    if let Some(state) = vm.viral_grid[y][x] {
                        if state.virus_id >= vm.virus_library.len() {
                            continue;
                        }
                        let virus = &vm.virus_library[state.virus_id];

                        // 1. Spread to neighbors
                        // Only spread if infection level is high enough (>20)
                        if state.infection_level > 20 {
                            let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
                            for (dy, dx) in neighbors {
                                if let Some((ny, nx)) =
                                    vm.normalize_coords(y as i64 + dy, x as i64 + dx)
                                {
                                    // Check if neighbor matches pattern
                                    let content = match &vm.grid[ny][nx] {
                                        Value::Str(s) => s.clone(),
                                        Value::Int(n) => n.to_string(),
                                        _ => String::new(),
                                    };

                                    if content.contains(&virus.pattern) {
                                        // Spread!
                                        if next_viral_grid[ny][nx].is_none() {
                                            next_viral_grid[ny][nx] = Some(ViralState {
                                                infection_level: 50, // Initial load
                                                virus_id: state.virus_id,
                                            });
                                            spread_count += 1;
                                        }
                                    }
                                }
                            }
                        }

                        // 2. Mutate host cell
                        // Only if infection level is high (>80)
                        if state.infection_level > 80 {
                            let mut rng = rand::thread_rng();
                            if rng.gen_range(0..100) < virus.mutation_rate {
                                // Scramble the string
                                if let Value::Str(s) = &mut vm.grid[y][x] {
                                    if !s.is_empty() {
                                        // Simple mutation: bitflip a char
                                        let _idx = rng.gen_range(0..s.len());
                                        // Rust strings are utf8, modifying in place is hard.
                                        // Just replace with random char or "GLITCH"
                                        *s = format!("GLITCH_{}", rng.gen_range(0..999));
                                        mutation_count += 1;
                                    }
                                }
                            }
                        }

                        // 3. Decay/Growth
                        // If cell matches pattern, infection grows. Else decays.
                        let content = match &vm.grid[y][x] {
                            Value::Str(s) => s.clone(),
                            Value::Int(n) => n.to_string(),
                            _ => String::new(),
                        };

                        if let Some(new_state) = &mut next_viral_grid[y][x] {
                            if content.contains(&virus.pattern) {
                                new_state.infection_level =
                                    new_state.infection_level.saturating_add(10);
                            } else {
                                new_state.infection_level =
                                    new_state.infection_level.saturating_sub(5);
                            }

                            if new_state.infection_level == 0 {
                                next_viral_grid[y][x] = None;
                            }
                        }
                    }
                }
            }

            vm.viral_grid = next_viral_grid;
            if spread_count > 0 || mutation_count > 0 {
                vm.output.push(format!(
                    "OUTBREAK: Spread to {} cells, mutated {} items",
                    spread_count, mutation_count
                ));
            }
            None
        }
        OpCode::Sanitize => {
            // Stack: [ ..., radius ]
            if let Some(val) = vm.stack.pop() {
                if let Value::Int(r) = val {
                    let (cy, cx) = vm.context_loc;
                    let coords = vm.get_circular_coords(cx as i64, cy as i64, r);
                    let count = coords.len();
                    for (tx, ty) in coords {
                        vm.viral_grid[ty][tx] = None;
                    }
                    vm.energy = vm.energy.saturating_sub(count as i64);
                    vm.output.push(format!("SANITIZE: Cleared {} cells", count));
                } else {
                    vm.output
                        .push("Error: Type mismatch for sanitize".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for sanitize".to_string());
            }
            None
        }
        _ => None,
    }
}
