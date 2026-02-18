use super::normalize_coords;
use crate::ast::Dna;
use crate::vm::{Value, MAX_STRANDS};
use rand::Rng;
use crate::vm::prologue::PrologueHost;
use crate::vm::prologue::PrologueState;

pub fn apply_evolution_runes(
    rune: &str,
    y: usize,
    x: usize,
    dna: &Dna,
    current_signals: &[Vec<Option<Value>>],
    next_signals: &mut Vec<Vec<Option<Value>>>,
) -> bool {
    let mut changes = false;
    let w_sig = if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
        current_signals[wy][wx].clone()
    } else {
        None
    };
    let n_sig = if let Some((ny, nx)) = normalize_coords(y as i64 - 1, x as i64) {
        current_signals[ny][nx].clone()
    } else {
        None
    };
    let _e_sig = if let Some((ey, ex)) = normalize_coords(y as i64, x as i64 + 1) {
        current_signals[ey][ex].clone()
    } else {
        None
    };

    match rune {
        "l" => {
            // Length: West (Strand Idx) -> Self (Length)
            if let Some(Value::Int(idx)) = w_sig {
                if idx >= 0 && (idx as usize) < dna.helix.strands.len() {
                    let len = dna.helix.strands[idx as usize].genes.len() as i64;
                    if next_signals[y][x].is_none() {
                        next_signals[y][x] = Some(Value::Int(len));
                        changes = true;
                    }
                }
            }
        }
        "n" => {
            // Nucleotide: West (Strand Idx), North (Gene Idx) -> Self (String)
            if let (Some(Value::Int(s_idx)), Some(Value::Int(g_idx))) = (w_sig, n_sig) {
                if s_idx >= 0 && (s_idx as usize) < dna.helix.strands.len() {
                    let strand = &dna.helix.strands[s_idx as usize];
                    if g_idx >= 0 && (g_idx as usize) < strand.genes.len() {
                        let gene = &strand.genes[g_idx as usize];
                        let gene_str = gene.op.to_string();
                        if next_signals[y][x].is_none() {
                            next_signals[y][x] = Some(Value::Str(gene_str));
                            changes = true;
                        }
                    }
                }
            }
        }
        _ => {}
    }
    changes
}

pub fn apply_evolution_sinks(
    host: &mut impl PrologueHost,
    state: &mut PrologueState,
    rune: &str,
    y: usize,
    x: usize
) {
    match rune {
        "G" => {
            // Genesis: North (Code), West (Config) -> Self (Strand Index)
            let code_to_compile = if let Some((ny, nx)) = normalize_coords(y as i64 - 1, x as i64) {
                if let Some(Value::Str(s)) = &state.signal_grid[ny][nx] {
                    Some(s.clone())
                } else {
                    None
                }
            } else {
                None
            };

            if let Some(code) = code_to_compile {
                // To avoid borrow checker issues with host.dna_mut(), we compile first.
                // Compile is independent of host state.
                match crate::compiler::compile(&code, None) {
                    Ok(dna_res) => {
                        if let Some(strand) = dna_res.helix.strands.first() {
                            let dna = host.dna_mut();
                            if dna.helix.strands.len() < MAX_STRANDS {
                                dna.helix.strands.push(strand.clone());
                                let idx = dna.helix.strands.len() - 1;
                                host.output_push(format!("PROLOGUE: Genesis created Strand {}", idx));
                                state.signal_grid[y][x] = Some(Value::Int(idx as i64));
                            } else {
                                host.output_push(
                                    "PROLOGUE: Genesis failed (MAX_STRANDS limit)".to_string(),
                                );
                            }
                        }
                    }
                    Err(e) => {
                        host.output_push(format!("PROLOGUE: Genesis failed: {}", e));
                    }
                }
            }
        }
        "X" => {
            // Crossover: West (Idx A), East (Idx B) -> South (New Idx)
            if let (Some((wy, wx)), Some((ey, ex))) = (
                normalize_coords(y as i64, x as i64 - 1),
                normalize_coords(y as i64, x as i64 + 1),
            ) {
                // Signals are in state, which is disjoint from host. Good.
                let w_sig = state.signal_grid[wy][wx].clone();
                let e_sig = state.signal_grid[ey][ex].clone();

                if let (Some(Value::Int(idx_a)), Some(Value::Int(idx_b))) = (w_sig, e_sig) {
                    let mut new_idx = None;
                    {
                        let dna = host.dna_mut();
                        let len = dna.helix.strands.len();
                        if idx_a >= 0 && (idx_a as usize) < len && idx_b >= 0 && (idx_b as usize) < len {
                             let strand_a = dna.helix.strands[idx_a as usize].clone();
                             let strand_b = dna.helix.strands[idx_b as usize].clone();
                             // ... logic ...
                             let split_a = strand_a.genes.len() / 2;
                             let split_b = strand_b.genes.len() / 2;
                             let mut new_genes = Vec::new();
                             new_genes.extend(strand_a.genes.iter().take(split_a).cloned());
                             new_genes.extend(strand_b.genes.iter().skip(split_b).cloned());
                             let new_strand = crate::ast::Strand { genes: new_genes };

                             if dna.helix.strands.len() < MAX_STRANDS {
                                 dna.helix.strands.push(new_strand);
                                 new_idx = Some(dna.helix.strands.len() - 1);
                             } else {
                                 // Can't push to output here easily if borrowing dna mutably from host?
                                 // Actually output_push borrows host mutably too.
                                 // So I need to scope the dna borrow.
                             }
                        }
                    } // dna borrow ends

                    if let Some(idx) = new_idx {
                        if let Some((sy, sx)) = normalize_coords(y as i64 + 1, x as i64) {
                            host.grid_write(sy, sx, Value::Int(idx as i64));
                            state.signal_grid[y][x] = Some(Value::Int(1));
                        }
                    } else if idx_a >= 0 && idx_b >= 0 { // Failed due to limits or bounds (logic is a bit loose on bounds check above inside block)
                        // If it failed due to max strands, we want to log.
                        // Ideally we check max strands before logic.
                         let len = host.dna().helix.strands.len();
                         if len >= MAX_STRANDS {
                             host.output_push("PROLOGUE: Crossover failed (MAX_STRANDS limit)".to_string());
                         }
                    }
                }
            }
        }
        "e" => {
            // Evolve: West (Idx) -> Self (New Idx)
            if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
                if let Some(Value::Int(idx)) = &state.signal_grid[wy][wx] {
                    let s_idx = *idx as usize;

                    let mut new_idx = None;
                    {
                        let dna = host.dna_mut();
                        if s_idx < dna.helix.strands.len() {
                            let mut strand = dna.helix.strands[s_idx].clone();
                            // Mutate
                            if !strand.genes.is_empty() {
                                let mut rng = rand::thread_rng();
                                let g_idx = rng.gen_range(0..strand.genes.len());
                                if rng.gen_bool(0.5) {
                                    // Deletion
                                    strand.genes.remove(g_idx);
                                } else {
                                    // Duplication
                                    let gene = strand.genes[g_idx].clone();
                                    strand.genes.insert(g_idx, gene);
                                }
                            }

                            if dna.helix.strands.len() < MAX_STRANDS {
                                dna.helix.strands.push(strand);
                                new_idx = Some(dna.helix.strands.len() - 1);
                            }
                        }
                    }

                    if let Some(idx) = new_idx {
                        state.signal_grid[y][x] = Some(Value::Int(idx as i64));
                    }
                }
            }
        }
        "b" => {
            // Breed: West (Idx A), East (Idx B) -> Self (New Idx)
            if let (Some((wy, wx)), Some((ey, ex))) = (
                normalize_coords(y as i64, x as i64 - 1),
                normalize_coords(y as i64, x as i64 + 1),
            ) {
                let w_sig = state.signal_grid[wy][wx].clone();
                let e_sig = state.signal_grid[ey][ex].clone();

                if let (Some(Value::Int(idx_a)), Some(Value::Int(idx_b))) = (w_sig, e_sig) {

                    let mut new_idx = None;
                    {
                        let dna = host.dna_mut();
                        let len = dna.helix.strands.len();

                        if idx_a >= 0 && (idx_a as usize) < len && idx_b >= 0 && (idx_b as usize) < len
                        {
                            let strand_a = dna.helix.strands[idx_a as usize].clone();
                            let strand_b = dna.helix.strands[idx_b as usize].clone();

                            let split_a = strand_a.genes.len() / 2;
                            let split_b = strand_b.genes.len() / 2;

                            let mut new_genes = Vec::new();
                            new_genes.extend(strand_a.genes.iter().take(split_a).cloned());
                            new_genes.extend(strand_b.genes.iter().skip(split_b).cloned());

                            let new_strand = crate::ast::Strand { genes: new_genes };
                            if dna.helix.strands.len() < MAX_STRANDS {
                                dna.helix.strands.push(new_strand);
                                new_idx = Some(dna.helix.strands.len() - 1);
                            }
                        }
                    }

                    if let Some(idx) = new_idx {
                        state.signal_grid[y][x] = Some(Value::Int(idx as i64));
                    }
                }
            }
        }
        _ => {}
    }
}
