use super::normalize_coords;
use crate::ast::Dna;
use crate::vm::{ChimeraVM, Value};

pub fn apply_sequencer_runes(
    rune: &str,
    y: usize,
    x: usize,
    dna: &Dna,
    current_signals: &[Vec<Option<Value>>],
    next_signals: &mut [Vec<Option<Value>>],
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

    if rune == "🔍" {
        // Read Gene: West (Idx), North (StrandIdx) -> Self (GeneString)
        if let (Some(w_val), Some(n_val)) = (w_sig, n_sig) {
            let idx = value_to_index(&w_val);
            let strand_idx = value_to_index(&n_val);

            if let (Some(g_idx), Some(s_idx)) = (idx, strand_idx) {
                if s_idx < dna.helix.strands.len() {
                    let strand = &dna.helix.strands[s_idx];
                    if g_idx < strand.genes.len() {
                        let gene = &strand.genes[g_idx];
                        let gene_str = gene_to_string(gene);
                        if next_signals[y][x].is_none() {
                            next_signals[y][x] = Some(Value::Str(gene_str));
                            changes = true;
                        }
                    }
                }
            }
        }
    }
    changes
}

pub fn apply_sequencer_sinks(vm: &mut ChimeraVM, rune: &str, y: usize, x: usize) {
    let w_sig = if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
        vm.prologue_state.signal_grid[wy][wx].clone()
    } else {
        None
    };
    let n_sig = if let Some((ny, nx)) = normalize_coords(y as i64 - 1, x as i64) {
        vm.prologue_state.signal_grid[ny][nx].clone()
    } else {
        None
    };
    let e_sig = if let Some((ey, ex)) = normalize_coords(y as i64, x as i64 + 1) {
        vm.prologue_state.signal_grid[ey][ex].clone()
    } else {
        None
    };

    match rune {
        "✏" => {
            // Write Gene: West (GeneString), North (Idx), East (StrandIdx)
            if let (Some(Value::Str(gene_str)), Some(n_val), Some(e_val)) = (w_sig, n_sig, e_sig) {
                let idx = value_to_index(&n_val);
                let strand_idx = value_to_index(&e_val);

                if let (Some(g_idx), Some(s_idx)) = (idx, strand_idx) {
                    // Parse gene string by wrapping in temporary strand
                    let source = format!("strand fragment {{ {} }}", gene_str);
                    match crate::compiler::compile(&source, None) {
                        Ok(new_dna) => {
                            if let Some(temp_strand) = new_dna.helix.strands.first() {
                                if let Some(new_gene) = temp_strand.genes.first() {
                                    if s_idx < vm.dna.helix.strands.len() {
                                        let strand = &mut vm.dna.helix.strands[s_idx];
                                        if g_idx < strand.genes.len() {
                                            strand.genes[g_idx] = new_gene.clone();
                                            vm.prologue_state.signal_grid[y][x] =
                                                Some(Value::Int(1)); // Success
                                        } else if g_idx == strand.genes.len() {
                                            strand.genes.push(new_gene.clone());
                                            vm.prologue_state.signal_grid[y][x] =
                                                Some(Value::Int(1));
                                        }
                                    }
                                }
                            }
                        }
                        Err(_) => {
                            // Fail silently or log
                        }
                    }
                }
            }
        }
        "➕" => {
            // Insert Gene: West (GeneString), North (Idx), East (StrandIdx)
            if let (Some(Value::Str(gene_str)), Some(n_val), Some(e_val)) = (w_sig, n_sig, e_sig) {
                let idx = value_to_index(&n_val);
                let strand_idx = value_to_index(&e_val);

                if let (Some(g_idx), Some(s_idx)) = (idx, strand_idx) {
                    let source = format!("strand fragment {{ {} }}", gene_str);
                    if let Ok(new_dna) = crate::compiler::compile(&source, None) {
                        if let Some(temp_strand) = new_dna.helix.strands.first() {
                            if let Some(new_gene) = temp_strand.genes.first() {
                                if s_idx < vm.dna.helix.strands.len() {
                                    let strand = &mut vm.dna.helix.strands[s_idx];
                                    if g_idx <= strand.genes.len() {
                                        strand.genes.insert(g_idx, new_gene.clone());
                                        vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        "🗑" => {
            // Delete Gene: West (Idx), North (StrandIdx)
            if let (Some(w_val), Some(n_val)) = (w_sig, n_sig) {
                let idx = value_to_index(&w_val);
                let strand_idx = value_to_index(&n_val);

                if let (Some(g_idx), Some(s_idx)) = (idx, strand_idx) {
                    if s_idx < vm.dna.helix.strands.len() {
                        let strand = &mut vm.dna.helix.strands[s_idx];
                        if g_idx < strand.genes.len() {
                            strand.genes.remove(g_idx);
                            vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1));
                        }
                    }
                }
            }
        }
        _ => {}
    }
}

fn value_to_index(v: &Value) -> Option<usize> {
    match v {
        Value::Int(n) if *n >= 0 => Some(*n as usize),
        Value::Str(s) => s.parse::<usize>().ok(),
        _ => None,
    }
}

fn gene_to_string(g: &crate::ast::Gene) -> String {
    let args: Vec<String> = g.args.iter().map(nucleotide_to_string).collect();
    if args.is_empty() {
        format!("{}", g.op)
    } else {
        format!("{}({})", g.op, args.join(", "))
    }
}

fn nucleotide_to_string(n: &crate::ast::Nucleotide) -> String {
    match n {
        crate::ast::Nucleotide::Number(i) => i.to_string(),
        crate::ast::Nucleotide::String(s) => format!("\"{}\"", s),
        crate::ast::Nucleotide::Identifier(s) => s.clone(),
        crate::ast::Nucleotide::Junction(_, list) => {
            let inner: Vec<String> = list.iter().map(nucleotide_to_string).collect();
            format!("[{}]", inner.join(", "))
        }
    }
}
