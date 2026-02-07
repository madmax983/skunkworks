#[cfg(feature = "nova")]
use crate::vm::{ChimeraVM, Value};
#[cfg(feature = "nova")]
use crate::opcode::OpCode;
#[cfg(feature = "nova")]
use crate::ast::Nucleotide;
#[cfg(feature = "nova")]
use rand::Rng;

#[cfg(feature = "nova")]
const MAX_EPIGENOME_SIZE: usize = 1024;

#[cfg(feature = "nova")]
fn value_to_nucleotide(v: &Value, depth: usize) -> Option<Nucleotide> {
    if depth > crate::vm::MAX_RECURSION_DEPTH {
        return None;
    }
    match v {
        Value::Int(n) => Some(Nucleotide::Number(*n)),
        Value::Str(s) => Some(Nucleotide::String(s.clone())),
        Value::Junction(t, vals) => {
            let mut nuc_vals = Vec::new();
            for val in vals {
                if let Some(n) = value_to_nucleotide(val, depth + 1) {
                    nuc_vals.push(n);
                } else {
                    return None;
                }
            }
            Some(Nucleotide::Junction(*t, nuc_vals))
        }
        Value::Superposition(_) => None,
    }
}

#[cfg(feature = "nova")]
pub fn exec_genetics_op(vm: &mut ChimeraVM, op: OpCode, _args: &[Nucleotide]) -> Option<(usize, usize)> {
    match op {
        OpCode::Splice => exec_splice(vm),
        OpCode::Recombine => exec_recombine(vm),
        OpCode::CrisprScan => exec_crispr_scan(vm),
        OpCode::Cas9Cut => exec_cas9_cut(vm),
        OpCode::Ligase => exec_ligase(vm),
        OpCode::Mitosis => exec_mitosis(vm),
        OpCode::Apoptosis => exec_apoptosis(vm),
        OpCode::Integrase => exec_integrase(vm),
        OpCode::Excision => exec_excision(vm),
        OpCode::Methylate => exec_methylate(vm),
        OpCode::Demethylate => exec_demethylate(vm),
        OpCode::Telomerase => exec_telomerase(vm),
        OpCode::TLen => exec_tlen(vm),
        OpCode::SIndex => exec_sindex(vm),
        _ => None,
    }
}

#[cfg(feature = "nova")]
fn exec_splice(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // stack: method, strand_b, strand_a (bottom)
    if vm.stack.len() >= 3 {
        let method_val = vm.stack.pop().unwrap();
        let strand_b_val = vm.stack.pop().unwrap();
        let strand_a_val = vm.stack.pop().unwrap();

        if let (Value::Int(s_a), Value::Int(s_b), Value::Int(method)) =
            (strand_a_val, strand_b_val, method_val)
        {
            let idx_a = s_a as usize;
            let idx_b = s_b as usize;
            let helix_len = vm.dna.helix.strands.len();

            if s_a >= 0 && s_b >= 0 && idx_a < helix_len && idx_b < helix_len {
                let genes_a = &vm.dna.helix.strands[idx_a].genes;
                let genes_b = &vm.dna.helix.strands[idx_b].genes;
                let len_a = genes_a.len();
                let len_b = genes_b.len();
                let max_len = len_a.max(len_b);

                let mut new_genes = Vec::new();
                let mut rng = rand::thread_rng();

                match method {
                    0 => {
                        // Interleave
                        for i in 0..max_len {
                            if i < len_a {
                                new_genes.push(genes_a[i].clone());
                            }
                            if i < len_b {
                                new_genes.push(genes_b[i].clone());
                            }
                        }
                        vm.output
                            .push(format!("SPLICE: Interleaved strands {} and {}", s_a, s_b));
                    }
                    1 => {
                        // Uniform Crossover
                        for i in 0..max_len {
                            if i < len_a && i < len_b {
                                if rng.gen_bool(0.5) {
                                    new_genes.push(genes_a[i].clone());
                                } else {
                                    new_genes.push(genes_b[i].clone());
                                }
                            } else if i < len_a {
                                new_genes.push(genes_a[i].clone());
                            } else if i < len_b {
                                new_genes.push(genes_b[i].clone());
                            }
                        }
                        vm.output
                            .push(format!("SPLICE: Crossover strands {} and {}", s_a, s_b));
                    }
                    2 => {
                        // Midpoint Split (Head A + Tail B)
                        let mid_a = len_a / 2;
                        let mid_b = len_b / 2;
                        for gene in genes_a.iter().take(mid_a) {
                            new_genes.push(gene.clone());
                        }
                        for gene in genes_b.iter().skip(mid_b) {
                            new_genes.push(gene.clone());
                        }
                        vm.output
                            .push(format!("SPLICE: Hybridized strands {} and {}", s_a, s_b));
                    }
                    _ => {
                        vm.output.push("Error: Invalid splice method".to_string());
                    }
                }

                if !new_genes.is_empty() {
                    vm.dna
                        .helix
                        .strands
                        .push(crate::ast::Strand { genes: new_genes });
                    vm.telomeres.push(50);
                    #[cfg(feature = "cortex")]
                    {
                        vm.activation_levels.push(0);
                        vm.synapse_map.push(Vec::new());
                    }
                    let new_idx = vm.dna.helix.strands.len() - 1;

                    vm.cladistics.register_strand(
                        new_idx,
                        Some(idx_a),
                        vm.tick_counter,
                        format!("Splice({}, {})", idx_a, idx_b)
                    );

                    vm.stack.push(Value::Int(new_idx as i64));
                    vm.energy = vm.energy.saturating_sub(30);
                } else if method <= 2 {
                    // If result empty but method valid (e.g. empty parents)
                    vm.stack.push(Value::Int(-1));
                }
            } else {
                vm.output
                    .push("Error: Strand index out of bounds for splice".to_string());
            }
        } else {
            vm.output
                .push("Error: Type mismatch for splice".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for splice".to_string());
    }
    None
}

#[cfg(feature = "nova")]
fn exec_recombine(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // stack: split_point, strand_b, strand_a (bottom)
    if vm.stack.len() >= 3 {
        let split_val = vm.stack.pop().unwrap();
        let strand_b_val = vm.stack.pop().unwrap();
        let strand_a_val = vm.stack.pop().unwrap();

        match (strand_a_val, strand_b_val, split_val) {
            (Value::Int(sa), Value::Int(sb), Value::Int(split)) => {
                let helix_len = vm.dna.helix.strands.len();
                let sa_idx = sa as usize;
                let sb_idx = sb as usize;
                let split_idx = split as usize;

                if sa >= 0 && sb >= 0 && split >= 0 && sa_idx < helix_len && sb_idx < helix_len {
                    // We need to check split bounds for both strands
                    let len_a = vm.dna.helix.strands[sa_idx].genes.len();
                    let len_b = vm.dna.helix.strands[sb_idx].genes.len();

                    if split_idx <= len_a && split_idx <= len_b {
                        if sa_idx == sb_idx {
                            // Recombining same strand with itself at same point is a no-op.
                            vm.output
                                .push("Warning: Recombining strand with itself".to_string());
                        } else {
                            // Ensure ordered access to avoid panic
                            let (lower, upper) = if sa_idx < sb_idx {
                                (sa_idx, sb_idx)
                            } else {
                                (sb_idx, sa_idx)
                            };

                            let (first_slice, second_slice) =
                                vm.dna.helix.strands.split_at_mut(upper);
                            let strand_low = &mut first_slice[lower];
                            let strand_high = &mut second_slice[0]; // relative index 0 is absolute 'upper'

                            // Identify which is A and B
                            let (strand_a, strand_b) = if sa_idx < sb_idx {
                                (strand_low, strand_high)
                            } else {
                                (strand_high, strand_low)
                            };

                            let mut tail_a = strand_a.genes.split_off(split_idx);
                            let mut tail_b = strand_b.genes.split_off(split_idx);

                            strand_a.genes.append(&mut tail_b);
                            strand_b.genes.append(&mut tail_a);

                            vm.output.push(format!(
                                "RECOMBINATION: Swapped tails of strand {} and {} at {}",
                                sa, sb, split
                            ));
                        }
                    } else {
                        vm.output
                            .push("Error: Split point out of bounds".to_string());
                    }
                } else {
                    vm.output
                        .push("Error: Strand index out of bounds".to_string());
                }
            }
            _ => vm
                .output
                .push("Error: Type mismatch for recombine".to_string()),
        }
    } else {
        vm.output
            .push("Error: Stack underflow for recombine".to_string());
    }
    None
}

#[cfg(feature = "nova")]
fn exec_crispr_scan(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // stack: guide_idx, target_idx (bottom)
    if vm.stack.len() >= 2 {
        let guide_val = vm.stack.pop().unwrap();
        let target_val = vm.stack.pop().unwrap();

        if let (Value::Int(g_idx), Value::Int(t_idx)) = (guide_val, target_val) {
            let g_idx = g_idx as usize;
            let t_idx = t_idx as usize;
            let helix_len = vm.dna.helix.strands.len();

            if g_idx < helix_len && t_idx < helix_len {
                let guide_strand = &vm.dna.helix.strands[g_idx];
                let target_strand = &vm.dna.helix.strands[t_idx];

                // We need to match sequence of gene names
                // Use op.to_string() for comparison
                let guide_names: Vec<String> = guide_strand
                    .genes
                    .iter()
                    .map(|g| g.op.to_string())
                    .collect();
                let target_names: Vec<String> = target_strand
                    .genes
                    .iter()
                    .map(|g| g.op.to_string())
                    .collect();

                let mut found_idx: i64 = -1;

                if !guide_names.is_empty() && guide_names.len() <= target_names.len() {
                    for i in 0..=(target_names.len() - guide_names.len()) {
                        if target_names[i..i + guide_names.len()] == guide_names[..] {
                            found_idx = i as i64;
                            break;
                        }
                    }
                } else if guide_names.is_empty() {
                    // Explicitly handle empty guide to ensure -1 (though init value handles it)
                    found_idx = -1;
                }

                vm.stack.push(Value::Int(found_idx));
                vm.energy = vm.energy.saturating_sub(5);
                vm.output.push(format!(
                    "CRISPR_SCAN: Scanned strand {} for pattern from {} -> {}",
                    t_idx, g_idx, found_idx
                ));
            } else {
                vm.output
                    .push("Error: Strand index out of bounds for crispr_scan".to_string());
            }
        } else {
            vm.output
                .push("Error: Type mismatch for crispr_scan".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for crispr_scan".to_string());
    }
    None
}

#[cfg(feature = "nova")]
fn exec_cas9_cut(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // stack: cut_index, strand_idx (bottom)
    if vm.stack.len() >= 2 {
        let cut_val = vm.stack.pop().unwrap();
        let strand_val = vm.stack.pop().unwrap();

        if let (Value::Int(cut), Value::Int(s_idx)) = (cut_val, strand_val) {
            let s_idx = s_idx as usize;
            let cut_idx = cut as usize;
            let helix_len = vm.dna.helix.strands.len();

            if s_idx < helix_len {
                let strand_len = vm.dna.helix.strands[s_idx].genes.len();
                if cut >= 0 && cut_idx <= strand_len {
                    // Perform split
                    // We need to mutate the strand.
                    let strand = &mut vm.dna.helix.strands[s_idx];
                    let tail_genes = strand.genes.split_off(cut_idx);

                    // Create new strand
                    vm.dna
                        .helix
                        .strands
                        .push(crate::ast::Strand { genes: tail_genes });
                    vm.telomeres.push(50);

                    #[cfg(feature = "cortex")]
                    {
                        vm.activation_levels.push(0);
                        vm.synapse_map.push(Vec::new());
                    }

                    let new_strand_idx = vm.dna.helix.strands.len() - 1;

                    vm.cladistics.register_strand(
                        new_strand_idx,
                        Some(s_idx),
                        vm.tick_counter,
                        "Cas9Cut".to_string()
                    );

                    vm.stack.push(Value::Int(new_strand_idx as i64));
                    vm.energy = vm.energy.saturating_sub(10);

                    vm.output.push(format!(
                        "CAS9_CUT: Cut strand {} at {}, created strand {}",
                        s_idx, cut_idx, new_strand_idx
                    ));

                    // Handling Self-Modification Safety:
                    // If we cut the strand we are currently executing,
                    // and the cut point is BEFORE or AT our current IP, execution context moves.
                    if s_idx == vm.ip.0 && vm.ip.1 >= cut_idx {
                        let new_gene_idx = vm.ip.1 - cut_idx;
                        // We jump to the next instruction in the NEW strand
                        return Some((new_strand_idx, new_gene_idx + 1));
                    }
                } else {
                    vm.output.push("Error: Cut index out of bounds".to_string());
                }
            } else {
                vm.output
                    .push("Error: Strand index out of bounds for cas9_cut".to_string());
            }
        } else {
            vm.output
                .push("Error: Type mismatch for cas9_cut".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for cas9_cut".to_string());
    }
    None
}

#[cfg(feature = "nova")]
fn exec_ligase(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // stack: donor_idx, recipient_idx (bottom)
    if vm.stack.len() >= 2 {
        let donor_val = vm.stack.pop().unwrap();
        let recipient_val = vm.stack.pop().unwrap();

        if let (Value::Int(d_idx), Value::Int(r_idx)) = (donor_val, recipient_val) {
            let d_idx = d_idx as usize;
            let r_idx = r_idx as usize;
            let helix_len = vm.dna.helix.strands.len();

            if d_idx < helix_len && r_idx < helix_len {
                if d_idx == r_idx {
                    vm.output
                        .push("Warning: Ligase on same strand is no-op".to_string());
                } else {
                    // We need to move genes from donor to recipient.
                    let (lower, upper) = if d_idx < r_idx {
                        (d_idx, r_idx)
                    } else {
                        (r_idx, d_idx)
                    };

                    let (first_slice, second_slice) =
                        vm.dna.helix.strands.split_at_mut(upper);
                    let strand_low = &mut first_slice[lower];
                    let strand_high = &mut second_slice[0];

                    let (strand_d, strand_r) = if d_idx < r_idx {
                        (strand_low, strand_high)
                    } else {
                        (strand_high, strand_low)
                    };

                    strand_r.genes.append(&mut strand_d.genes);
                    // donor genes are now empty.

                    vm.energy = vm.energy.saturating_sub(10);
                    vm.output
                        .push(format!("LIGASE: Appended strand {} to {}", d_idx, r_idx));
                }
            } else {
                vm.output
                    .push("Error: Strand index out of bounds for ligase".to_string());
            }
        } else {
            vm.output
                .push("Error: Type mismatch for ligase".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for ligase".to_string());
    }
    None
}

#[cfg(feature = "nova")]
fn exec_mitosis(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // stack: strand_idx (target to clone)
    if let Some(val) = vm.stack.pop() {
        match val {
            Value::Int(idx) => {
                let s_idx = idx as usize;
                if s_idx < vm.dna.helix.strands.len() {
                    // Clone the strand
                    let new_strand = vm.dna.helix.strands[s_idx].clone();
                    vm.dna.helix.strands.push(new_strand);
                    vm.telomeres.push(50); // Default life

                    #[cfg(feature = "cortex")]
                    {
                        vm.activation_levels.push(0);
                        vm.synapse_map.push(Vec::new());
                    }

                    // Inherit epigenetics
                    // We need to find all keys (s_idx, g_idx) and insert (new_idx, g_idx)
                    let new_s_idx = vm.dna.helix.strands.len() - 1;

                    vm.cladistics.register_strand(
                        new_s_idx,
                        Some(s_idx),
                        vm.tick_counter,
                        "Mitosis".to_string()
                    );

                    let genes_to_methylate: Vec<usize> = vm
                        .epigenome
                        .iter()
                        .filter(|(s, _)| *s == s_idx)
                        .map(|(_, g)| *g)
                        .collect();

                    for g_idx in genes_to_methylate {
                        vm.epigenome.insert((new_s_idx, g_idx));
                    }

                    vm.energy = vm.energy.saturating_sub(30); // Cost
                    vm.output
                        .push(format!("MITOSIS: Cloned strand {} to {}", s_idx, new_s_idx));
                } else {
                    vm.output
                        .push("Error: Strand index out of bounds for mitosis".to_string());
                }
            }
            _ => vm
                .output
                .push("Error: Type mismatch for mitosis".to_string()),
        }
    } else {
        vm.output
            .push("Error: Stack underflow for mitosis".to_string());
    }
    None
}

#[cfg(feature = "nova")]
fn exec_apoptosis(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // stack: strand_idx
    if let Some(val) = vm.stack.pop() {
        match val {
            Value::Int(idx) => {
                let s_idx = idx as usize;
                if s_idx < vm.dna.helix.strands.len() {
                    // Backup to graveyard
                    let strand = vm.dna.helix.strands[s_idx].clone();
                    vm.graveyard.push(strand);

                    vm.dna.helix.strands[s_idx].genes.clear();

                    // Remove associated epigenetics
                    vm.epigenome.retain(|(s, _)| *s != s_idx);

                    vm.cladistics.kill_strand(s_idx, vm.tick_counter);

                    vm.energy = vm.energy.saturating_sub(10);
                    vm.output
                        .push(format!("APOPTOSIS: Cleared strand {}", s_idx));
                } else {
                    vm.output.push(
                        "Error: Strand index out of bounds for apoptosis".to_string(),
                    );
                }
            }
            _ => vm
                .output
                .push("Error: Type mismatch for apoptosis".to_string()),
        }
    } else {
        vm.output
            .push("Error: Stack underflow for apoptosis".to_string());
    }
    None
}

#[cfg(feature = "nova")]
fn exec_integrase(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // stack: arg, name, gene_idx, strand_idx (bottom)
    if vm.stack.len() >= 4 {
        let arg_val = vm.stack.pop().unwrap();
        let name_val = vm.stack.pop().unwrap();
        let gene_idx_val = vm.stack.pop().unwrap();
        let strand_idx_val = vm.stack.pop().unwrap();

        match (strand_idx_val, gene_idx_val, name_val, arg_val) {
            (Value::Int(s), Value::Int(g), Value::Str(name), arg) => {
                let s_idx = s as usize;
                let g_idx = g as usize;
                let helix_len = vm.dna.helix.strands.len();

                if s >= 0 && s_idx < helix_len {
                    let strand_len = vm.dna.helix.strands[s_idx].genes.len();
                    if g >= 0 && g_idx <= strand_len {
                        // Create Gene
                        let new_gene = crate::ast::Gene {
                            op: name.parse().unwrap_or(OpCode::Unknown(name.clone())),
                            args: if let Some(n) = value_to_nucleotide(&arg, 0) {
                                vec![n]
                            } else {
                                vm.output.push(
                                    "INTEGRASE: Warning: Recursion limit exceeded"
                                        .to_string(),
                                );
                                vec![]
                            },
                        };

                        // Insert
                        vm.dna.helix.strands[s_idx].genes.insert(g_idx, new_gene);

                        // Update Epigenome: Shift all markers at (s_idx, k >= g_idx) to k+1
                        let mut new_markers = Vec::new();
                        let mut to_remove = Vec::new();
                        for &(ms, mg) in vm.epigenome.iter() {
                            if ms == s_idx && mg >= g_idx {
                                to_remove.push((ms, mg));
                                new_markers.push((ms, mg + 1));
                            }
                        }
                        for marker in to_remove {
                            vm.epigenome.remove(&marker);
                        }
                        for marker in new_markers {
                            vm.epigenome.insert(marker);
                        }

                        vm.energy = vm.energy.saturating_sub(20);
                        vm.output
                            .push(format!("INTEGRASE: Inserted {} at {}:{}", name, s, g));

                        // Update IP if we inserted before or at current execution
                        if vm.ip.0 == s_idx && vm.ip.1 >= g_idx {
                            vm.ip.1 += 1;
                        }
                    } else {
                        vm.output.push(
                            "Error: Gene index out of bounds for integrase".to_string(),
                        );
                    }
                } else {
                    vm.output.push(
                        "Error: Strand index out of bounds for integrase".to_string(),
                    );
                }
            }
            _ => vm
                .output
                .push("Error: Type mismatch for integrase".to_string()),
        }
    } else {
        vm.output
            .push("Error: Stack underflow for integrase".to_string());
    }
    None
}

#[cfg(feature = "nova")]
fn exec_excision(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // stack: gene_idx, strand_idx (bottom)
    if vm.stack.len() >= 2 {
        let gene_idx_val = vm.stack.pop().unwrap();
        let strand_idx_val = vm.stack.pop().unwrap();

        match (strand_idx_val, gene_idx_val) {
            (Value::Int(s), Value::Int(g)) => {
                let s_idx = s as usize;
                let g_idx = g as usize;
                let helix_len = vm.dna.helix.strands.len();

                if s >= 0 && s_idx < helix_len {
                    let strand_len = vm.dna.helix.strands[s_idx].genes.len();
                    if g >= 0 && g_idx < strand_len {
                        // Remove
                        vm.dna.helix.strands[s_idx].genes.remove(g_idx);

                        // Update Epigenome: Remove marker at g_idx, shift k > g_idx to k-1
                        vm.epigenome.remove(&(s_idx, g_idx));
                        let mut new_markers = Vec::new();
                        let mut to_remove = Vec::new();
                        for &(ms, mg) in vm.epigenome.iter() {
                            if ms == s_idx && mg > g_idx {
                                to_remove.push((ms, mg));
                                new_markers.push((ms, mg - 1));
                            }
                        }
                        for marker in to_remove {
                            vm.epigenome.remove(&marker);
                        }
                        for marker in new_markers {
                            vm.epigenome.insert(marker);
                        }

                        vm.energy = vm.energy.saturating_sub(15);
                        vm.output.push(format!("EXCISION: Removed {}:{}", s, g));

                        // Update IP
                        if vm.ip.0 == s_idx {
                            if g_idx < vm.ip.1 {
                                // Removed before current. Shift IP left.
                                vm.ip.1 -= 1;
                                return None; // step() increments +1. Net 0 change (but content moved left, so we execute next).
                            } else if g_idx == vm.ip.1 {
                                // Removed current.
                                // Next gene slid into current slot.
                                // We want to execute it.
                                // So return IP as is, prevent step() increment.
                                return Some(vm.ip);
                            }
                        }
                    } else {
                        vm.output.push(
                            "Error: Gene index out of bounds for excision".to_string(),
                        );
                    }
                } else {
                    vm.output
                        .push("Error: Strand index out of bounds for excision".to_string());
                }
            }
            _ => vm
                .output
                .push("Error: Type mismatch for excision".to_string()),
        }
    } else {
        vm.output
            .push("Error: Stack underflow for excision".to_string());
    }
    None
}

#[cfg(feature = "nova")]
fn exec_methylate(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if vm.stack.len() >= 2 {
        let gene_val = vm.stack.pop().unwrap();
        let strand_val = vm.stack.pop().unwrap();
        if let (Value::Int(g_idx), Value::Int(s_idx)) = (gene_val, strand_val) {
            if vm.epigenome.len() < MAX_EPIGENOME_SIZE {
                vm.epigenome.insert((s_idx as usize, g_idx as usize));
                vm.output.push(format!("METHYLATED: {}:{}", s_idx, g_idx));
            } else {
                vm.output
                    .push("Error: Epigenome size limit exceeded".to_string());
            }
        } else {
            vm.output
                .push("Error: Invalid args for methylate".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for methylate".to_string());
    }
    None
}

#[cfg(feature = "nova")]
fn exec_demethylate(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if vm.stack.len() >= 2 {
        let gene_val = vm.stack.pop().unwrap();
        let strand_val = vm.stack.pop().unwrap();
        if let (Value::Int(g_idx), Value::Int(s_idx)) = (gene_val, strand_val) {
            vm.epigenome.remove(&(s_idx as usize, g_idx as usize));
            vm.output.push(format!("DEMETHYLATED: {}:{}", s_idx, g_idx));
        } else {
            vm.output
                .push("Error: Invalid args for demethylate".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for demethylate".to_string());
    }
    None
}

#[cfg(feature = "nova")]
fn exec_telomerase(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if let Some(val) = vm.stack.pop() {
        match val {
            Value::Int(amount) => {
                if amount > 0 {
                    let idx = vm.ip.0;
                    if idx < vm.telomeres.len() {
                        vm.telomeres[idx] = vm.telomeres[idx].saturating_add(amount);
                        vm.energy = vm.energy.saturating_sub(25); // High cost
                        vm.output.push(format!(
                            "TELOMERASE: Extended strand {} by {}",
                            idx, amount
                        ));
                    }
                }
            }
            _ => vm
                .output
                .push("Error: Invalid arg for telomerase".to_string()),
        }
    } else {
        vm.output
            .push("Error: Stack underflow for telomerase".to_string());
    }
    None
}

#[cfg(feature = "nova")]
fn exec_tlen(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    let idx = vm.ip.0;
    if idx < vm.telomeres.len() {
        vm.stack.push(Value::Int(vm.telomeres[idx]));
    } else {
        vm.stack.push(Value::Int(0));
    }
    None
}

#[cfg(feature = "nova")]
fn exec_sindex(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    vm.stack.push(Value::Int(vm.ip.0 as i64));
    None
}
