use super::normalize_coords;
use crate::ast::Dna;
use crate::vm::{ChimeraVM, Value, MAX_STRANDS};
use rand::Rng;

pub fn apply_evolution_runes(
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

pub fn apply_evolution_sinks(vm: &mut ChimeraVM, rune: &str, y: usize, x: usize) {
    match rune {
        "G" => {
            // Genesis:
            // Mode 1: West (Length) -> Read Grid East -> Strand (Incubation)
            // Mode 2: North (Code String) -> Strand (Compilation)

            let w_sig = if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
                vm.prologue_state.signal_grid[wy][wx].clone()
            } else {
                None
            };

            if let Some(Value::Int(len)) = w_sig {
                if len > 0 {
                    // Mode 1: Incubate from Grid
                    let mut source = String::new();
                    source.push_str("strand incubated {\n");

                    for i in 1..=len {
                        if let Some((ey, ex)) = normalize_coords(y as i64, x as i64 + i) {
                            let cell = &vm.grid[ey][ex];
                            match cell {
                                Value::Str(s) => {
                                    // Append raw string (e.g. "add", "push", "dup")
                                    // If empty string, ignore?
                                    if !s.is_empty() {
                                        source.push_str(&format!("{}\n", s));
                                    }
                                }
                                Value::Int(n) => {
                                    // Implicit push
                                    source.push_str(&format!("{}\n", n));
                                }
                                _ => {}
                            }
                        }
                    }
                    source.push_str("}\n");

                    match crate::compiler::compile(&source, None) {
                        Ok(mut new_dna) => {
                            if let Some(strand) = new_dna.helix.strands.pop() {
                                if vm.dna.helix.strands.len() < MAX_STRANDS {
                                    vm.dna.helix.strands.push(strand);
                                    let idx = vm.dna.helix.strands.len() - 1;
                                    vm.output.push(format!(
                                        "PROLOGUE: Incubated Strand {} from Grid",
                                        idx
                                    ));
                                    vm.prologue_state.signal_grid[y][x] =
                                        Some(Value::Int(idx as i64));
                                } else {
                                    vm.output.push(
                                        "PROLOGUE: Incubation failed (MAX_STRANDS)".to_string(),
                                    );
                                }
                            }
                        }
                        Err(e) => {
                            vm.output
                                .push(format!("PROLOGUE: Incubation failed: {}", e));
                        }
                    }
                    return; // Skip Mode 2
                }
            }

            // Mode 2: North (Code) -> Self (Strand Index)
            let code_to_compile = if let Some((ny, nx)) = normalize_coords(y as i64 - 1, x as i64) {
                if let Some(Value::Str(s)) = &vm.prologue_state.signal_grid[ny][nx] {
                    Some(s.clone())
                } else {
                    None
                }
            } else {
                None
            };

            if let Some(code) = code_to_compile {
                match crate::compiler::compile(&code, None) {
                    Ok(dna) => {
                        if let Some(strand) = dna.helix.strands.first() {
                            if vm.dna.helix.strands.len() < MAX_STRANDS {
                                vm.dna.helix.strands.push(strand.clone());
                                let idx = vm.dna.helix.strands.len() - 1;
                                vm.output
                                    .push(format!("PROLOGUE: Genesis created Strand {}", idx));
                                vm.prologue_state.signal_grid[y][x] = Some(Value::Int(idx as i64));
                            } else {
                                vm.output.push(
                                    "PROLOGUE: Genesis failed (MAX_STRANDS limit)".to_string(),
                                );
                            }
                        }
                    }
                    Err(e) => {
                        vm.output.push(format!("PROLOGUE: Genesis failed: {}", e));
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
                let w_sig = vm.prologue_state.signal_grid[wy][wx].clone();
                let e_sig = vm.prologue_state.signal_grid[ey][ex].clone();

                if let (Some(Value::Int(idx_a)), Some(Value::Int(idx_b))) = (w_sig, e_sig) {
                    let len = vm.dna.helix.strands.len();
                    if idx_a >= 0 && (idx_a as usize) < len && idx_b >= 0 && (idx_b as usize) < len
                    {
                        let strand_a = vm.dna.helix.strands[idx_a as usize].clone();
                        let strand_b = vm.dna.helix.strands[idx_b as usize].clone();

                        let split_a = strand_a.genes.len() / 2;
                        let split_b = strand_b.genes.len() / 2;

                        let mut new_genes = Vec::new();
                        new_genes.extend(strand_a.genes.iter().take(split_a).cloned());
                        new_genes.extend(strand_b.genes.iter().skip(split_b).cloned());

                        let new_strand = crate::ast::Strand { genes: new_genes };
                        if vm.dna.helix.strands.len() < MAX_STRANDS {
                            vm.dna.helix.strands.push(new_strand);
                            let new_idx = vm.dna.helix.strands.len() - 1;

                            if let Some((sy, sx)) = normalize_coords(y as i64 + 1, x as i64) {
                                vm.grid[sy][sx] = Value::Int(new_idx as i64);
                                vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1));
                            }
                        } else {
                            vm.output
                                .push("PROLOGUE: Crossover failed (MAX_STRANDS limit)".to_string());
                        }
                    }
                }
            }
        }
        "e" => {
            // Evolve: West (Idx) -> Self (New Idx)
            if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
                if let Some(Value::Int(idx)) = &vm.prologue_state.signal_grid[wy][wx] {
                    let s_idx = *idx as usize;
                    if s_idx < vm.dna.helix.strands.len() {
                        let mut strand = vm.dna.helix.strands[s_idx].clone();
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

                        if vm.dna.helix.strands.len() < MAX_STRANDS {
                            vm.dna.helix.strands.push(strand);
                            let new_idx = vm.dna.helix.strands.len() - 1;
                            vm.prologue_state.signal_grid[y][x] = Some(Value::Int(new_idx as i64));
                        }
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
                let w_sig = vm.prologue_state.signal_grid[wy][wx].clone();
                let e_sig = vm.prologue_state.signal_grid[ey][ex].clone();

                if let (Some(Value::Int(idx_a)), Some(Value::Int(idx_b))) = (w_sig, e_sig) {
                    let len = vm.dna.helix.strands.len();
                    if idx_a >= 0 && (idx_a as usize) < len && idx_b >= 0 && (idx_b as usize) < len
                    {
                        let strand_a = vm.dna.helix.strands[idx_a as usize].clone();
                        let strand_b = vm.dna.helix.strands[idx_b as usize].clone();

                        let split_a = strand_a.genes.len() / 2;
                        let split_b = strand_b.genes.len() / 2;

                        let mut new_genes = Vec::new();
                        new_genes.extend(strand_a.genes.iter().take(split_a).cloned());
                        new_genes.extend(strand_b.genes.iter().skip(split_b).cloned());

                        let new_strand = crate::ast::Strand { genes: new_genes };
                        if vm.dna.helix.strands.len() < MAX_STRANDS {
                            vm.dna.helix.strands.push(new_strand);
                            let new_idx = vm.dna.helix.strands.len() - 1;
                            vm.prologue_state.signal_grid[y][x] = Some(Value::Int(new_idx as i64));
                        }
                    }
                }
            }
        }
        "∞" => {
            // Infinity: Darwinian Selector
            // West: Trigger
            // North: Goal
            // East: Actual
            // South: Subject Area (5x5 Cone)

            if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
                if vm.prologue_state.signal_grid[wy][wx].is_some() {
                    // Triggered
                    let goal = if let Some((ny, nx)) = normalize_coords(y as i64 - 1, x as i64) {
                        if let Some(val) = &vm.prologue_state.signal_grid[ny][nx] {
                            val.clone()
                        } else {
                            Value::Int(0)
                        }
                    } else {
                        Value::Int(0)
                    };

                    let actual = if let Some((ey, ex)) = normalize_coords(y as i64, x as i64 + 1) {
                        if let Some(val) = &vm.prologue_state.signal_grid[ey][ex] {
                            val.clone()
                        } else {
                            Value::Int(0)
                        }
                    } else {
                        Value::Int(0)
                    };

                    if goal == actual {
                        // Success
                        vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1));
                        vm.output.push("DARWIN: Success match".to_string());
                    } else {
                        // Failure -> Mutate South
                        let mut rng = rand::thread_rng();
                        let dy = rng.gen_range(1..=5);
                        let dx = rng.gen_range(-2..=2);

                        if let Some((ty, tx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
                            let val = &mut vm.grid[ty][tx];
                            let old_val = val.clone();
                            match val {
                                Value::Int(n) => *n = rng.gen_range(0..100),
                                Value::Str(_) => {
                                    let runes = [
                                        "~", "*", "+", "-", "%", "&", "|", "^", "!", "?", "A", "S",
                                        "M", "D",
                                    ];
                                    *val = Value::Str(
                                        runes[rng.gen_range(0..runes.len())].to_string(),
                                    );
                                }
                                _ => {}
                            }
                            vm.output.push(format!(
                                "DARWIN: Mutation at {},{} ({} -> {})",
                                tx, ty, old_val, val
                            ));
                        }
                    }
                }
            }
        }
        "Ð" => {
            // Reverse Transcriptase: West (String Code) -> Append to Current Strand
            if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
                if let Some(Value::Str(code)) = &vm.prologue_state.signal_grid[wy][wx] {
                    // Try to parse code as Gene
                    let op = code
                        .parse()
                        .unwrap_or(crate::opcode::OpCode::Unknown(code.clone()));
                    let gene = crate::ast::Gene {
                        op,
                        args: vec![], // For now, only op
                    };

                    let s_idx = vm.ip.0;
                    if s_idx < vm.dna.helix.strands.len() {
                        vm.dna.helix.strands[s_idx].genes.push(gene);
                        vm.output.push(format!(
                            "PROLOGUE: Reverse Transcriptase wrote '{}' to Strand {}",
                            code, s_idx
                        ));
                        vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1));
                    }
                }
            }
        }
        _ => {}
    }
}
