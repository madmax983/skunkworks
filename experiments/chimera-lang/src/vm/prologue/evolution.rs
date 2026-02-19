use super::normalize_coords;
use crate::ast::{Dna, Gene, JunctionType, Nucleotide, Strand};
use crate::opcode::OpCode;
use crate::vm::{ChimeraVM, Value, GRID_SIZE, MAX_STRANDS};
use rand::Rng;
use std::str::FromStr;

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

pub fn apply_evolution_sinks(vm: &mut ChimeraVM, rune: &str, y: usize, x: usize) {
    match rune {
        "G" => {
            // Genesis: North (Code), West (Config) -> Self (Strand Index)
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
        "p" => {
            // Polymerase: West (List of Genes) -> Self (Strand Idx)
            // Example Input: Junction(Dish, [Str("push"), Int(10), Str("add")])
            // Wait, flat list is ambiguous.
            // Better Input: Junction(Dish, [Junction(Dish, [Str("push"), Int(10)]), Str("add")])
            if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
                if let Some(Value::Junction(_, items)) = &vm.prologue_state.signal_grid[wy][wx] {
                    let mut genes = Vec::new();
                    for item in items {
                        if let Some(gene) = value_to_gene(item) {
                            genes.push(gene);
                        }
                    }

                    if !genes.is_empty() {
                        if vm.dna.helix.strands.len() < MAX_STRANDS {
                            vm.dna.helix.strands.push(Strand { genes });
                            let new_idx = vm.dna.helix.strands.len() - 1;
                            vm.prologue_state.signal_grid[y][x] = Some(Value::Int(new_idx as i64));
                            vm.output.push(format!(
                                "PROLOGUE: Polymerase synthesized Strand {}",
                                new_idx
                            ));
                        } else {
                            vm.output.push(
                                "PROLOGUE: Polymerase failed (MAX_STRANDS limit)".to_string(),
                            );
                        }
                    }
                }
            }
        }
        "o" => {
            // Operon: West (Strand Idx) -> Writes Genes to Grid (South..East)
            if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
                if let Some(Value::Int(idx)) = &vm.prologue_state.signal_grid[wy][wx] {
                    let s_idx = *idx as usize;
                    if s_idx < vm.dna.helix.strands.len() {
                        let strand = &vm.dna.helix.strands[s_idx];
                        let mut cursor_y = y as i64 + 1;
                        let mut cursor_x = x as i64;

                        for gene in &strand.genes {
                            let val = gene_to_value(gene);
                            // Normalize (wrap) coordinates
                            // We move East. If X wraps, we move to next row (like a typewriter)
                            // Or just wrap X.
                            // Let's wrap X.
                            // But normalize_coords returns None if out of bounds.
                            // We want modulo wrapping for printing long strands?
                            // Or just clamp?
                            // Let's use simple modulo wrapping for X and Y.
                            let ny = (cursor_y % GRID_SIZE as i64 + GRID_SIZE as i64)
                                % GRID_SIZE as i64;
                            let nx = (cursor_x % GRID_SIZE as i64 + GRID_SIZE as i64)
                                % GRID_SIZE as i64;

                            vm.grid[ny as usize][nx as usize] = val;

                            cursor_x += 1;
                            if cursor_x >= GRID_SIZE as i64 {
                                cursor_x = 0;
                                cursor_y += 1;
                            }
                        }
                        // Light up self
                        vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1));
                    }
                }
            }
        }
        _ => {}
    }
}

fn value_to_gene(v: &Value) -> Option<Gene> {
    match v {
        Value::Str(s) => {
            // OpCode without args
            if let Ok(op) = OpCode::from_str(s) {
                Some(Gene { op, args: vec![] })
            } else {
                None
            }
        }
        Value::Junction(_, items) => {
            // [OpCode, Arg1, Arg2...]
            if let Some(Value::Str(op_str)) = items.first() {
                if let Ok(op) = OpCode::from_str(op_str) {
                    let mut args = Vec::new();
                    for arg_val in items.iter().skip(1) {
                        if let Some(nuc) = value_to_nucleotide(arg_val) {
                            args.push(nuc);
                        }
                    }
                    Some(Gene { op, args })
                } else {
                    None
                }
            } else {
                None
            }
        }
        _ => None,
    }
}

fn value_to_nucleotide(v: &Value) -> Option<Nucleotide> {
    match v {
        Value::Int(n) => Some(Nucleotide::Number(*n)),
        Value::Str(s) => Some(Nucleotide::String(s.clone())),
        Value::Junction(t, items) => {
            let mut nucs = Vec::new();
            for item in items {
                if let Some(n) = value_to_nucleotide(item) {
                    nucs.push(n);
                }
            }
            Some(Nucleotide::Junction(*t, nucs))
        }
        _ => None,
    }
}

fn gene_to_value(gene: &Gene) -> Value {
    if gene.args.is_empty() {
        Value::Str(gene.op.to_string())
    } else {
        let mut items = vec![Value::Str(gene.op.to_string())];
        for arg in &gene.args {
            items.push(nucleotide_to_value(arg));
        }
        Value::Junction(JunctionType::Dish, items)
    }
}

fn nucleotide_to_value(n: &Nucleotide) -> Value {
    match n {
        Nucleotide::Number(i) => Value::Int(*i),
        Nucleotide::String(s) => Value::Str(s.clone()),
        Nucleotide::Identifier(s) => Value::Str(s.clone()),
        Nucleotide::Junction(t, list) => {
            let items = list.iter().map(nucleotide_to_value).collect();
            Value::Junction(*t, items)
        }
    }
}
