use crate::vm::{ChimeraVM, Value};
use super::normalize_coords;
use crate::ast::{Dna, Strand, Nucleotide};
use crate::opcode::OpCode;
use rand::Rng;

pub fn apply_evolution_runes(
    rune: &str,
    y: usize,
    x: usize,
    current_signals: &[Vec<Option<Value>>],
    next_signals: &mut Vec<Vec<Option<Value>>>,
    dna: &Dna,
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
            // Length: West (Strand ID) -> Self (Length)
            if let Some(Value::Int(idx)) = w_sig {
                if idx >= 0 && (idx as usize) < dna.helix.strands.len() {
                    let len = dna.helix.strands[idx as usize].genes.len();
                    if next_signals[y][x].is_none() {
                        next_signals[y][x] = Some(Value::Int(len as i64));
                        changes = true;
                    }
                }
            }
        }
        "n" => {
            // Nucleotide: West (Strand ID), North (Index) -> Self (OpCode Name)
            if let (Some(Value::Int(strand_idx)), Some(Value::Int(gene_idx))) = (w_sig, n_sig) {
                if strand_idx >= 0 && (strand_idx as usize) < dna.helix.strands.len() {
                    let strand = &dna.helix.strands[strand_idx as usize];
                    if gene_idx >= 0 && (gene_idx as usize) < strand.genes.len() {
                        let op = &strand.genes[gene_idx as usize].op;
                        let op_name = format!("{:?}", op);
                        if next_signals[y][x].is_none() {
                            next_signals[y][x] = Some(Value::Str(op_name));
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
    vm: &mut ChimeraVM,
    rune: &str,
    y: usize,
    x: usize,
) {
    match rune {
        "e" => {
            // Evolve: West (Strand ID) -> Mutate -> New Strand ID (Self)
            if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
                if let Some(Value::Int(idx)) = &vm.prologue_state.signal_grid[wy][wx] {
                    if *idx >= 0 && (*idx as usize) < vm.dna.helix.strands.len() {
                        let mut new_strand = vm.dna.helix.strands[*idx as usize].clone();
                        // Mutate
                        let mut rng = rand::thread_rng();
                        if !new_strand.genes.is_empty() {
                            let g_idx = rng.gen_range(0..new_strand.genes.len());
                            // Flip an arg if exists
                            if !new_strand.genes[g_idx].args.is_empty() {
                                new_strand.genes[g_idx].args[0] = Nucleotide::Number(rng.gen_range(0..100));
                            }
                        }

                        vm.dna.helix.strands.push(new_strand);
                        let new_idx = vm.dna.helix.strands.len() - 1;

                        // Output to Delayed Signals for Next Tick
                        vm.prologue_state.delayed_signals[y][x] = Some(Value::Int(new_idx as i64));
                        // Light up for current tick
                        vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1));
                    }
                }
            }
        }
        "b" | "X" => {
            // Breed/Crossover: West (A), East (B) -> New Strand ID (Self)
            if let (Some((wy, wx)), Some((ey, ex))) = (
                normalize_coords(y as i64, x as i64 - 1),
                normalize_coords(y as i64, x as i64 + 1),
            ) {
                let w_sig = &vm.prologue_state.signal_grid[wy][wx];
                let e_sig = &vm.prologue_state.signal_grid[ey][ex];

                if let (Some(Value::Int(idx_a)), Some(Value::Int(idx_b))) = (w_sig, e_sig) {
                    let len = vm.dna.helix.strands.len();
                    if *idx_a >= 0 && (*idx_a as usize) < len && *idx_b >= 0 && (*idx_b as usize) < len {
                        let strand_a = &vm.dna.helix.strands[*idx_a as usize];
                        let strand_b = &vm.dna.helix.strands[*idx_b as usize];

                        let split_a = strand_a.genes.len() / 2;
                        let split_b = strand_b.genes.len() / 2;

                        let mut new_genes = Vec::new();
                        new_genes.extend(strand_a.genes.iter().take(split_a).cloned());
                        new_genes.extend(strand_b.genes.iter().skip(split_b).cloned());

                        let new_strand = Strand { genes: new_genes };
                        vm.dna.helix.strands.push(new_strand);
                        let new_idx = vm.dna.helix.strands.len() - 1;

                        vm.prologue_state.delayed_signals[y][x] = Some(Value::Int(new_idx as i64));
                        vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1));
                    }
                }
            }
        }
        "G" => {
             // Genesis: North (Code) -> New Strand ID (Self)
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

                            vm.prologue_state.delayed_signals[y][x] = Some(Value::Int(idx as i64));
                            vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1));
                        }
                    }
                    Err(_) => {
                         // Ignored
                    }
                }
            }
        }
        _ => {}
    }
}
