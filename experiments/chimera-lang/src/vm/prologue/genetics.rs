use crate::vm::{ChimeraVM, Value};
use super::normalize_coords;
use rand::Rng;

pub fn apply_genetic_sinks(vm: &mut ChimeraVM, rune: &str, y: usize, x: usize) {
    match rune {
        "G" => {
            // Genesis: North (Code) -> Self (Strand Index)
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
                            vm.dna.helix.strands.push(strand.clone());
                            let idx = vm.dna.helix.strands.len() - 1;
                            vm.output
                                .push(format!("PROLOGUE: Genesis created Strand {}", idx));
                            vm.prologue_state.signal_grid[y][x] = Some(Value::Int(idx as i64));
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
                    if idx_a >= 0
                        && (idx_a as usize) < len
                        && idx_b >= 0
                        && (idx_b as usize) < len
                    {
                        let strand_a = vm.dna.helix.strands[idx_a as usize].clone();
                        let strand_b = vm.dna.helix.strands[idx_b as usize].clone();

                        let split_a = strand_a.genes.len() / 2;
                        let split_b = strand_b.genes.len() / 2;

                        let mut new_genes = Vec::new();
                        new_genes.extend(strand_a.genes.iter().take(split_a).cloned());
                        new_genes.extend(strand_b.genes.iter().skip(split_b).cloned());

                        let new_strand = crate::ast::Strand { genes: new_genes };
                        vm.dna.helix.strands.push(new_strand);
                        let new_idx = vm.dna.helix.strands.len() - 1;

                        if let Some((sy, sx)) = normalize_coords(y as i64 + 1, x as i64) {
                            vm.grid[sy][sx] = Value::Int(new_idx as i64);
                            vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1));
                        }
                    }
                }
            }
        }
        "b" => {
            // Breed (Uniform Crossover): West (Idx A), East (Idx B), North (Prob 0-100) -> South (New Idx)
            let w_sig = if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
                vm.prologue_state.signal_grid[wy][wx].clone()
            } else { None };

            let e_sig = if let Some((ey, ex)) = normalize_coords(y as i64, x as i64 + 1) {
                vm.prologue_state.signal_grid[ey][ex].clone()
            } else { None };

            let n_sig = if let Some((ny, nx)) = normalize_coords(y as i64 - 1, x as i64) {
                vm.prologue_state.signal_grid[ny][nx].clone()
            } else { None };

            if let (Some(Value::Int(idx_a)), Some(Value::Int(idx_b))) = (w_sig, e_sig) {
                let prob = match n_sig {
                    Some(Value::Int(p)) => p.clamp(0, 100),
                    _ => 50,
                };

                let len = vm.dna.helix.strands.len();
                if idx_a >= 0 && (idx_a as usize) < len && idx_b >= 0 && (idx_b as usize) < len {
                    let strand_a = &vm.dna.helix.strands[idx_a as usize];
                    let strand_b = &vm.dna.helix.strands[idx_b as usize];

                    let max_genes = strand_a.genes.len().max(strand_b.genes.len());
                    let mut new_genes = Vec::new();
                    let mut rng = rand::thread_rng();

                    for i in 0..max_genes {
                        let gene_a = strand_a.genes.get(i);
                        let gene_b = strand_b.genes.get(i);

                        let pick_a = rng.gen_range(0..100) < prob;

                        if pick_a {
                            if let Some(g) = gene_a { new_genes.push(g.clone()); }
                            else if let Some(g) = gene_b { new_genes.push(g.clone()); }
                        } else {
                            if let Some(g) = gene_b { new_genes.push(g.clone()); }
                            else if let Some(g) = gene_a { new_genes.push(g.clone()); }
                        }
                    }

                    let new_strand = crate::ast::Strand { genes: new_genes };
                    vm.dna.helix.strands.push(new_strand);
                    let new_idx = vm.dna.helix.strands.len() - 1;

                    if let Some((sy, sx)) = normalize_coords(y as i64 + 1, x as i64) {
                        vm.grid[sy][sx] = Value::Int(new_idx as i64);
                        vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1));
                    }
                }
            }
        }
        "h" => {
            // Hybridize (Masked): West (Idx A), East (Idx B), North (Mask Idx) -> South (New Idx)
            let w_sig = if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
                vm.prologue_state.signal_grid[wy][wx].clone()
            } else { None };

            let e_sig = if let Some((ey, ex)) = normalize_coords(y as i64, x as i64 + 1) {
                vm.prologue_state.signal_grid[ey][ex].clone()
            } else { None };

            let n_sig = if let Some((ny, nx)) = normalize_coords(y as i64 - 1, x as i64) {
                vm.prologue_state.signal_grid[ny][nx].clone()
            } else { None };

            if let (Some(Value::Int(idx_a)), Some(Value::Int(idx_b)), Some(Value::Int(idx_m))) = (w_sig, e_sig, n_sig) {
                let len = vm.dna.helix.strands.len();
                if idx_a >= 0 && (idx_a as usize) < len
                   && idx_b >= 0 && (idx_b as usize) < len
                   && idx_m >= 0 && (idx_m as usize) < len
                {
                    let strand_a = &vm.dna.helix.strands[idx_a as usize];
                    let strand_b = &vm.dna.helix.strands[idx_b as usize];
                    let strand_mask = &vm.dna.helix.strands[idx_m as usize];

                    let max_genes = strand_a.genes.len().max(strand_b.genes.len());
                    let mut new_genes = Vec::new();

                    for i in 0..max_genes {
                        let gene_a = strand_a.genes.get(i);
                        let gene_b = strand_b.genes.get(i);
                        // Mask Logic: If mask gene exists and is NOT Nop, pick B, else A?
                        // Or if mask gene op is '1' (Wait, OpCode is enum).
                        // Let's use: If mask gene exists, pick B. If mask gene is missing (shorter), pick A.
                        // Or better: If mask gene has args, pick B.

                        let use_b = if let Some(mg) = strand_mask.genes.get(i) {
                            !mg.args.is_empty() // Arbitrary logic: Non-empty args = Switch
                        } else {
                            false
                        };

                        if use_b {
                            if let Some(g) = gene_b { new_genes.push(g.clone()); }
                            else if let Some(g) = gene_a { new_genes.push(g.clone()); }
                        } else {
                            if let Some(g) = gene_a { new_genes.push(g.clone()); }
                            else if let Some(g) = gene_b { new_genes.push(g.clone()); }
                        }
                    }

                    let new_strand = crate::ast::Strand { genes: new_genes };
                    vm.dna.helix.strands.push(new_strand);
                    let new_idx = vm.dna.helix.strands.len() - 1;

                    if let Some((sy, sx)) = normalize_coords(y as i64 + 1, x as i64) {
                        vm.grid[sy][sx] = Value::Int(new_idx as i64);
                        vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1));
                    }
                }
            }
        }
        "c" => {
            // Clone: West (Idx) -> South (New Idx)
            let w_sig = if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
                vm.prologue_state.signal_grid[wy][wx].clone()
            } else { None };

            if let Some(Value::Int(idx)) = w_sig {
                let len = vm.dna.helix.strands.len();
                if idx >= 0 && (idx as usize) < len {
                    let strand = vm.dna.helix.strands[idx as usize].clone();
                    vm.dna.helix.strands.push(strand);
                    let new_idx = vm.dna.helix.strands.len() - 1;

                    if let Some((sy, sx)) = normalize_coords(y as i64 + 1, x as i64) {
                        vm.grid[sy][sx] = Value::Int(new_idx as i64);
                        vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1));
                    }
                }
            }
        }
        "i" => {
            // Inject: West (Target Idx), North (Payload Idx), East (Insert At) -> South (New Idx)
            let w_sig = if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
                vm.prologue_state.signal_grid[wy][wx].clone()
            } else { None };

            let n_sig = if let Some((ny, nx)) = normalize_coords(y as i64 - 1, x as i64) {
                vm.prologue_state.signal_grid[ny][nx].clone()
            } else { None };

            let e_sig = if let Some((ey, ex)) = normalize_coords(y as i64, x as i64 + 1) {
                vm.prologue_state.signal_grid[ey][ex].clone()
            } else { None };

            if let (Some(Value::Int(target_idx)), Some(Value::Int(payload_idx)), Some(Value::Int(insert_at))) = (w_sig, n_sig, e_sig) {
                let len = vm.dna.helix.strands.len();
                if target_idx >= 0 && (target_idx as usize) < len
                   && payload_idx >= 0 && (payload_idx as usize) < len
                {
                    let mut new_genes = vm.dna.helix.strands[target_idx as usize].genes.clone();
                    let payload_genes = &vm.dna.helix.strands[payload_idx as usize].genes;

                    let pos = (insert_at.max(0) as usize).min(new_genes.len());

                    // Insert payload
                    for (k, gene) in payload_genes.iter().enumerate() {
                        new_genes.insert(pos + k, gene.clone());
                    }

                    let new_strand = crate::ast::Strand { genes: new_genes };
                    vm.dna.helix.strands.push(new_strand);
                    let new_idx = vm.dna.helix.strands.len() - 1;

                    if let Some((sy, sx)) = normalize_coords(y as i64 + 1, x as i64) {
                        vm.grid[sy][sx] = Value::Int(new_idx as i64);
                        vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1));
                    }
                }
            }
        }
        _ => {}
    }
}
