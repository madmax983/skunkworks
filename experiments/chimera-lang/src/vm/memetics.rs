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

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum VirusMode {
    Overwrite,   // Current behavior: Replace cell with new content
    RewriteGrid, // Parse cell content -> Mutate -> Write back
    RewriteDNA,  // Parse Organelle DNA -> Mutate -> Compile -> Replace
}

#[derive(Debug, Clone, PartialEq)]
pub struct Virus {
    pub name: String,
    pub color: (u8, u8, u8),          // RGB
    pub pattern: String,              // Target text pattern (contains match)
    pub mutation_rate: u8,            // 0-100
    pub payload: Option<usize>,       // DNA Strand index to inject
    pub grammar: Option<Value>,       // Grammar for linguistic mutation
    pub quorum_threshold: u8,         // Neighbors needed for quorum action
    pub quorum_action: Option<usize>, // Strand to execute on quorum
    pub mode: VirusMode,
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
        OpCode::BioHack => {
            // Stack: [ ..., name_str, grammar_junction, meme_id ]
            if vm.stack.len() >= 3 {
                let meme_val = vm.stack.pop().unwrap();
                let grammar_val = vm.stack.pop().unwrap();
                let name_val = vm.stack.pop().unwrap();

                if let (Value::Int(m_id), Value::Str(name)) = (meme_val, name_val) {
                    let meme_idx = m_id as usize;
                    if meme_idx < vm.meme_pool.memes.len() {
                        if vm.dna.helix.strands.len() >= crate::vm::MAX_STRANDS {
                            vm.output.push("BIOHACK: Strand limit exceeded".to_string());
                            return None;
                        }

                        let meme = &vm.meme_pool.memes[meme_idx];

                        // Create a temporary strand for the payload
                        // In Nova, we can just push it to the helix.
                        let payload_strand = crate::ast::Strand {
                            genes: meme.genes.clone(),
                        };
                        vm.dna.helix.strands.push(payload_strand);
                        let payload_idx = vm.dna.helix.strands.len() - 1;
                        vm.telomeres.push(50); // Default telomere for new strand
                        #[cfg(feature = "cortex")]
                        {
                            vm.activation_levels.push(0);
                            vm.synapse_map.push(Vec::new());
                        }

                        let mut rng = rand::thread_rng();
                        let color = (
                            rng.gen_range(50..255),
                            rng.gen_range(50..255),
                            rng.gen_range(50..255),
                        );

                        let virus = Virus {
                            name: name.clone(),
                            color,
                            pattern: ".*".to_string(), // Default pattern matches everything? Or maybe derive from name?
                            mutation_rate: meme.virulence, // Use virulence as mutation rate
                            payload: Some(payload_idx),
                            grammar: Some(grammar_val),
                            quorum_threshold: 0,
                            quorum_action: None,
                            mode: VirusMode::Overwrite,
                        };

                        let virus_id = vm.virus_library.len();
                        vm.virus_library.push(virus);

                        let (cy, cx) = vm.context_loc;
                        vm.viral_grid[cy][cx] = Some(ViralState {
                            infection_level: 100,
                            virus_id,
                        });

                        vm.output.push(format!(
                            "BIOHACK: Synthesized Virus '{}' (ID {}) from Meme {}",
                            name, virus_id, meme_idx
                        ));
                    } else {
                        vm.output.push("BIOHACK: Invalid Meme ID".to_string());
                    }
                } else {
                    vm.output.push(
                        "BIOHACK: Type mismatch [name:Str, grammar:Junction, meme:Int]".to_string(),
                    );
                }
            } else {
                vm.output.push("BIOHACK: Stack underflow".to_string());
            }
            None
        }
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
            // Stack: [ ..., (grammar), (quorum_action, quorum_threshold), payload_idx, mutation_rate, pattern_str, name_str, mode ]
            if vm.stack.len() >= 5 {
                let mode_val = vm.stack.pop().unwrap();
                let name_val = vm.stack.pop().unwrap();
                let pattern_val = vm.stack.pop().unwrap();
                let rate_val = vm.stack.pop().unwrap();
                let payload_val = vm.stack.pop().unwrap();

                // Optional Args
                let mut grammar = None;
                let mut quorum_threshold = 0;
                let mut quorum_action = None;

                // Check for Quorum args
                if vm.stack.len() >= 2 {
                    if let (Value::Int(_), Value::Int(_)) =
                        (&vm.stack[vm.stack.len() - 1], &vm.stack[vm.stack.len() - 2])
                    {
                        if let Value::Int(thresh) = vm.stack.pop().unwrap() {
                            quorum_threshold = thresh.clamp(0, 8) as u8;
                        }
                        if let Value::Int(action) = vm.stack.pop().unwrap() {
                            quorum_action = if action >= 0 {
                                Some(action as usize)
                            } else {
                                None
                            };
                        }
                    }
                }

                // Check for Grammar arg
                if !vm.stack.is_empty() {
                    // Check if top is Junction
                    if let Value::Junction(_, _) = &vm.stack[vm.stack.len() - 1] {
                        grammar = Some(vm.stack.pop().unwrap());
                    }
                }

                if let (
                    Value::Str(name),
                    Value::Str(pattern),
                    Value::Int(rate),
                    Value::Int(p_idx),
                    Value::Int(mode_int),
                ) = (name_val, pattern_val, rate_val, payload_val, mode_val)
                {
                    let mutation_rate = rate.clamp(0, 100) as u8;
                    let payload = if p_idx >= 0 && (p_idx as usize) < vm.dna.helix.strands.len() {
                        Some(p_idx as usize)
                    } else {
                        None
                    };

                    let mode = match mode_int {
                        1 => VirusMode::RewriteGrid,
                        2 => VirusMode::RewriteDNA,
                        _ => VirusMode::Overwrite,
                    };

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
                        payload,
                        grammar,
                        quorum_threshold,
                        quorum_action,
                        mode,
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
                        let virus = vm.virus_library[state.virus_id].clone();

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

                            // Transduction (Payload Injection)
                            if let Some(payload_idx) = virus.payload {
                                // Check if an organelle is here
                                let target_org_indices: Vec<usize> = vm
                                    .organelles
                                    .iter()
                                    .enumerate()
                                    .filter(|(_, o)| o.context_loc == (y, x))
                                    .map(|(i, _)| i)
                                    .collect();

                                if !target_org_indices.is_empty()
                                    && payload_idx < vm.dna.helix.strands.len()
                                {
                                    let payload_genes =
                                        vm.dna.helix.strands[payload_idx].genes.clone();

                                    for idx in target_org_indices {
                                        let org = &vm.organelles[idx];
                                        // Inject into the genome referenced by the organelle
                                        // Note: multiple organelles might share a genome. This affects all of them.
                                        let g_id = org.genome_id as usize;
                                        if g_id < vm.dna.helix.strands.len() {
                                            vm.dna.helix.strands[g_id]
                                                .genes
                                                .extend(payload_genes.clone());
                                            mutation_count += 1;
                                            vm.output.push(format!("TRANSDUCTION: Virus {} injected Strand {} into Organelle {} (Genome {})", state.virus_id, payload_idx, org.name, g_id));
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
                                match virus.mode {
                                    VirusMode::RewriteGrid => {
                                        if let Some(grammar) = &virus.grammar {
                                            let content = match &vm.grid[y][x] {
                                                Value::Str(s) => s.clone(),
                                                _ => String::new(),
                                            };
                                            if !content.is_empty() {
                                                if let Ok((cst, _)) =
                                                    crate::vm::babel::run_parser(grammar, &content)
                                                {
                                                    let mutated_cst = crate::vm::babel::mutate_cst(
                                                        &cst,
                                                        virus.mutation_rate as f64 / 100.0,
                                                    );
                                                    let new_content =
                                                        crate::vm::babel::flatten_cst(&mutated_cst);
                                                    vm.grid[y][x] = Value::Str(new_content);
                                                    mutation_count += 1;
                                                }
                                            }
                                        }
                                    }
                                    VirusMode::RewriteDNA => {
                                        if let Some(grammar) = &virus.grammar {
                                            // Find organelle at this location
                                            let target_org_indices: Vec<usize> = vm
                                                .organelles
                                                .iter()
                                                .enumerate()
                                                .filter(|(_, o)| o.context_loc == (y, x))
                                                .map(|(i, _)| i)
                                                .collect();

                                            for idx in target_org_indices {
                                                let strand_idx = vm.organelles[idx].ip.0;
                                                if strand_idx < vm.dna.helix.strands.len() {
                                                    let source =
                                                        crate::vm::nova_genetics::strand_to_string(
                                                            &vm.dna.helix.strands[strand_idx],
                                                        );
                                                    if let Ok((cst, _)) =
                                                        crate::vm::babel::run_parser(
                                                            grammar, &source,
                                                        )
                                                    {
                                                        let mutated_cst =
                                                            crate::vm::babel::mutate_cst(
                                                                &cst,
                                                                virus.mutation_rate as f64 / 100.0,
                                                            );
                                                        // This pushes a new strand
                                                        if let Some(new_idx) = crate::vm::babel::compile_cst(
                                                            vm,
                                                            mutated_cst,
                                                            strand_idx,
                                                        ) {
                                                            // Update Organelle
                                                            if idx < vm.organelles.len() {
                                                                vm.organelles[idx].ip = (new_idx, 0);
                                                                vm.output.push(format!("REWRITE: Virus {} rewrote Organelle {} DNA to Strand {}", state.virus_id, vm.organelles[idx].name, new_idx));
                                                                mutation_count += 1;
                                                            }
                                                        } else {
                                                            vm.output.push("REWRITE: Compilation failed (Recursion limit)".to_string());
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    VirusMode::Overwrite => {
                                        // Grammar Rewrite or Scramble
                                        if let Some(grammar) = &virus.grammar {
                                            let new_content =
                                                crate::vm::babel::generate_string(grammar);
                                            vm.grid[y][x] = Value::Str(new_content);
                                            mutation_count += 1;
                                        } else if let Value::Str(s) = &mut vm.grid[y][x] {
                                            if !s.is_empty() {
                                                // Simple mutation: bitflip a char
                                                let _idx = rng.gen_range(0..s.len());
                                                *s = format!("GLITCH_{}", rng.gen_range(0..999));
                                                mutation_count += 1;
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        // Quorum Sensing
                        if virus.quorum_threshold > 0 && virus.quorum_action.is_some() {
                            let neighbors = [
                                (-1, 0),
                                (1, 0),
                                (0, -1),
                                (0, 1),
                                (-1, -1),
                                (-1, 1),
                                (1, -1),
                                (1, 1),
                            ];
                            let mut count = 0;
                            for (dy, dx) in neighbors {
                                if let Some((ny, nx)) =
                                    vm.normalize_coords(y as i64 + dy, x as i64 + dx)
                                {
                                    if let Some(n_state) = &vm.viral_grid[ny][nx] {
                                        if n_state.virus_id == state.virus_id {
                                            count += 1;
                                        }
                                    }
                                }
                            }

                            if count >= virus.quorum_threshold {
                                if let Some(action_idx) = virus.quorum_action {
                                    // Check if we should trigger (to avoid spamming, maybe check entropy or energy?)
                                    // Spawn a temporary Worker organelle to run the payload
                                    if vm.organelles.len() < crate::vm::MAX_ORGANELLES {
                                        // Don't spawn if already occupied by an organelle?
                                        if vm.organelles.iter().all(|o| o.context_loc != (y, x)) {
                                            vm.organelle_id_counter += 1;
                                            let organelle = crate::vm::nova::Organelle {
                                                stack: Vec::new(),
                                                ip: (action_idx, 0),
                                                context_loc: (y, x),
                                                call_stack: Vec::new(),
                                                recursion_depth: 0,
                                                halted: false,
                                                kind: crate::vm::nova::OrganelleType::Worker,
                                                direction: (0, 0),
                                                ttl: Some(1), // Ephemeral
                                                name: format!("Virus {} Agent", state.virus_id),
                                                traits: vec!["Viral".to_string()],
                                                id: vm.organelle_id_counter,
                                                tissue_id: None,
                                                genome_id: 0,
                                                energy: 20,
                                                experience: 0,
                                                stage: 0,
                                            };
                                            vm.organelles.push(organelle);
                                            vm.output.push(format!(
                                                "QUORUM: Virus {} triggered action {} at {},{}",
                                                state.virus_id, action_idx, x, y
                                            ));
                                        }
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
