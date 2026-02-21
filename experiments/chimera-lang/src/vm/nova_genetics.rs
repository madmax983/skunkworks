#![cfg(feature = "nova")]

use super::{ChimeraVM, Value, MAX_STRANDS};
use crate::ast::Nucleotide;
use crate::opcode::OpCode;
use crate::{ChimeraParser, Rule};
use pest::Parser;
use rand::Rng;

pub const MAX_EPIGENOME_SIZE: usize = 1024;
pub const MAX_INCUBATE_LENGTH: usize = 1024;

/// Helper to convert a Value to a Nucleotide (static AST node)
pub fn value_to_nucleotide(v: &Value, depth: usize) -> Option<Nucleotide> {
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
        Value::Superposition(_) => None, // Cannot compile superposition to static AST
        Value::Symbol(_) => None,        // Symbols are runtime values, cannot persist to static AST
        Value::Color(_, _, _) => None,
    }
}

/// Reshuffles the entire DNA based on the current Grid state.
///
/// This is the ultimate self-modification enzyme. The organism treats the
/// world (Grid) as its genetic code, rebooting itself from the environment.
///
/// **OpCode:** `Metamorphosis`
/// **Effect:** Clears DNA, reads Grid as DNA, resets Energy to 50, IP to (0,0), Stack to [].
///
/// # The Algorithm
/// 1. Scan Grid row by row.
/// 2. Convert each cell (`Int`, `Str`) into a Gene.
/// 3. Compile new Strands.
/// 4. Wipe all state (Stack, Telomeres, Epigenome).
/// 5. Rebirth.
pub fn exec_metamorphosis(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    let rows = vm.grid.len();
    let cols = if rows > 0 { vm.grid[0].len() } else { 0 };

    let mut new_strands = Vec::new();

    for row in &vm.grid {
        let mut genes = Vec::new();
        let mut x = 0;
        while x < cols {
            let val = &row[x];
            match val {
                Value::Int(n) => {
                    genes.push(crate::ast::Gene {
                        op: OpCode::Push,
                        args: vec![crate::ast::Nucleotide::Number(*n)],
                    });
                    x += 1;
                }
                Value::Junction(t, vals) => {
                    let mut nuc_vals = Vec::new();
                    for v in vals {
                        if let Some(n) = value_to_nucleotide(v, 0) {
                            nuc_vals.push(n);
                        } else {
                            vm.output.push(
                                "METAMORPHOSIS: Warning: Recursion limit exceeded".to_string(),
                            );
                        }
                    }
                    genes.push(crate::ast::Gene {
                        op: OpCode::Push,
                        args: vec![crate::ast::Nucleotide::Junction(*t, nuc_vals)],
                    });
                    x += 1;
                }
                Value::Superposition(_) => {
                    x += 1; // Skip
                }
                Value::Symbol(_) => {
                    x += 1; // Skip symbols
                }
                Value::Color(_, _, _) => {
                    x += 1; // Skip colors
                }
                Value::Str(s) => {
                    let op = s.parse().unwrap_or(OpCode::Unknown(s.clone()));
                    let mut args = Vec::new();
                    match op {
                        OpCode::Push | OpCode::Jump | OpCode::Brz | OpCode::Call => {
                            if x + 1 < cols {
                                if let Some(arg) = value_to_nucleotide(&row[x + 1], 0) {
                                    args.push(arg);
                                } else {
                                    vm.output.push(
                                        "METAMORPHOSIS: Warning: Recursion limit exceeded"
                                            .to_string(),
                                    );
                                }
                                x += 1;
                            }
                        }
                        #[cfg(feature = "cortex")]
                        OpCode::Gate => {
                            if x + 1 < cols {
                                if let Some(arg) = value_to_nucleotide(&row[x + 1], 0) {
                                    args.push(arg);
                                } else {
                                    vm.output.push(
                                        "METAMORPHOSIS: Warning: Recursion limit exceeded"
                                            .to_string(),
                                    );
                                }
                                x += 1;
                            }
                        }
                        _ => {}
                    }
                    genes.push(crate::ast::Gene { op, args });
                    x += 1;
                }
            }
        }
        if !genes.is_empty() {
            new_strands.push(crate::ast::Strand { genes });
        }
    }

    if !new_strands.is_empty() {
        if new_strands.len() > MAX_STRANDS {
            vm.output
                .push("METAMORPHOSIS: Failed (Too many strands)".to_string());
            return Some((0, 0));
        }
        vm.dna.helix.strands = new_strands;

        // Reset State
        vm.cladistics = crate::vm::cladistics::Cladistics::new();
        // Register new strands as roots
        for i in 0..vm.dna.helix.strands.len() {
            vm.cladistics
                .register_strand(i, None, vm.tick_counter, "Metamorphosis".to_string());
        }

        vm.energy = 50;
        vm.ip = (0, 0);
        vm.stack.clear();
        vm.telomeres = vec![50; vm.dna.helix.strands.len()];
        vm.epigenome.clear();
        vm.receptors.clear();
        #[cfg(feature = "cortex")]
        {
            vm.activation_levels = vec![0; vm.dna.helix.strands.len()];
            vm.synapse_map = vec![Vec::new(); vm.dna.helix.strands.len()];
        }
        vm.market.clear();

        vm.output
            .push("METAMORPHOSIS: The chrysalis breaks...".to_string());
        Some((0, 0))
    } else {
        vm.output
            .push("METAMORPHOSIS: Failed (Grid empty/invalid)".to_string());
        Some((0, 0))
    }
}

pub fn exec_chronos_splice(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // stack: spore_id, strand_idx (top)
    if vm.stack.len() >= 2 {
        let s_val = vm.stack.pop().unwrap(); // strand_idx
        let id_val = vm.stack.pop().unwrap(); // spore_id

        if let (Value::Int(spore_id), Value::Int(s_idx)) = (id_val, s_val) {
            let id = spore_id as usize;
            let idx = s_idx as usize;

            if id < vm.spores.len() {
                let spore = &vm.spores[id];
                if idx < spore.dna.helix.strands.len() {
                    if vm.dna.helix.strands.len() >= MAX_STRANDS {
                        vm.output
                            .push("Error: Strand limit exceeded for chronos_splice".to_string());
                        return None;
                    }
                    // Clone strand from spore
                    let strand = spore.dna.helix.strands[idx].clone();

                    // Add to current genome
                    vm.dna.helix.strands.push(strand);
                    vm.telomeres.push(50);
                    #[cfg(feature = "cortex")]
                    {
                        vm.activation_levels.push(0);
                        vm.synapse_map.push(Vec::new());
                    }

                    let new_idx = vm.dna.helix.strands.len() - 1;

                    vm.cladistics.register_strand(
                        new_idx,
                        Some(idx),
                        vm.tick_counter,
                        format!("ChronosSplice({}, {})", id, idx),
                    );

                    vm.stack.push(Value::Int(new_idx as i64));
                    vm.energy = vm.energy.saturating_sub(75); // Expensive time travel
                    vm.output.push(format!(
                        "CHRONOS_SPLICE: Retrieved strand {} from Spore {}",
                        idx, id
                    ));
                } else {
                    vm.output
                        .push("Error: Strand index out of bounds in spore".to_string());
                }
            } else {
                vm.output.push("Error: Spore ID out of bounds".to_string());
            }
        } else {
            vm.output
                .push("Error: Type mismatch for chronos_splice".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for chronos_splice".to_string());
    }
    None
}

/// Stitches two strands together with high-voltage seams.
///
/// **OpCode:** `Frankenstein`
/// **Stack:** `[ ..., strand_a, strand_b, stitches ] -> [ ..., new_strand_idx ]`
pub fn exec_frankenstein(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // stack: stitches, strand_b, strand_a (bottom)
    if vm.stack.len() >= 3 {
        let stitches_val = vm.stack.pop().unwrap();
        let strand_b_val = vm.stack.pop().unwrap();
        let strand_a_val = vm.stack.pop().unwrap();

        if let (Value::Int(s_a), Value::Int(s_b), Value::Int(stitches)) =
            (strand_a_val, strand_b_val, stitches_val)
        {
            let idx_a = s_a as usize;
            let idx_b = s_b as usize;
            let helix_len = vm.dna.helix.strands.len();

            if s_a >= 0 && s_b >= 0 && idx_a < helix_len && idx_b < helix_len {
                let genes_a = &vm.dna.helix.strands[idx_a].genes;
                let genes_b = &vm.dna.helix.strands[idx_b].genes;
                let len_a = genes_a.len();
                let len_b = genes_b.len();

                let n_stitches = stitches.max(1) as usize;
                let chunk_size_a = (len_a / (n_stitches + 1)).max(1);
                let chunk_size_b = (len_b / (n_stitches + 1)).max(1);

                let mut new_genes = Vec::new();
                let mut ptr_a = 0;
                let mut ptr_b = 0;

                for i in 0..=n_stitches {
                    // Alternate chunks
                    if i % 2 == 0 {
                        // Take from A
                        let end = (ptr_a + chunk_size_a).min(len_a);
                        if ptr_a < len_a {
                            new_genes.extend_from_slice(&genes_a[ptr_a..end]);
                            ptr_a = end;
                        }
                    } else {
                        // Take from B
                        let end = (ptr_b + chunk_size_b).min(len_b);
                        if ptr_b < len_b {
                            new_genes.extend_from_slice(&genes_b[ptr_b..end]);
                            ptr_b = end;
                        }
                    }

                    // Insert Spark at seam (if not last chunk)
                    if i < n_stitches {
                        #[cfg(feature = "elektra")]
                        let spark = crate::ast::Gene {
                            op: OpCode::Lightning,
                            args: vec![], // Lightning args handled by VM or usually grid based, but here acts as a "Spark"
                        };
                        #[cfg(not(feature = "elektra"))]
                        let spark = crate::ast::Gene {
                            op: OpCode::Glitch,
                            args: vec![Nucleotide::Number(1)], // Minor glitch
                        };
                        new_genes.push(spark);
                    }
                }

                // Append remainders if any (Frankenstein is messy)
                if ptr_a < len_a {
                    new_genes.extend_from_slice(&genes_a[ptr_a..]);
                }
                if ptr_b < len_b {
                    new_genes.extend_from_slice(&genes_b[ptr_b..]);
                }

                if vm.dna.helix.strands.len() >= MAX_STRANDS {
                    vm.output
                        .push("Error: Strand limit exceeded for Frankenstein".to_string());
                    vm.stack.push(Value::Int(-1));
                    return None;
                }

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
                    format!("Frankenstein({}, {})", idx_a, idx_b),
                );

                vm.stack.push(Value::Int(new_idx as i64));
                vm.energy = vm.energy.saturating_sub(100); // Very expensive
                vm.output.push(format!(
                    "FRANKENSTEIN: It's Alive! Created strand {}",
                    new_idx
                ));
            } else {
                vm.output
                    .push("Error: Strand index out of bounds".to_string());
            }
        } else {
            vm.output
                .push("Error: Type mismatch for Frankenstein".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for Frankenstein".to_string());
    }
    None
}

/// Performs a single-point crossover at a random index.
///
/// **OpCode:** `Crossover`
/// **Stack:** `[ ..., strand_a, strand_b ] -> [ ..., new_strand_1, new_strand_2 ]`
pub fn exec_crossover(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if vm.stack.len() >= 2 {
        let b_val = vm.stack.pop().unwrap();
        let a_val = vm.stack.pop().unwrap();

        if let (Value::Int(idx_a), Value::Int(idx_b)) = (a_val, b_val) {
            let a = idx_a as usize;
            let b = idx_b as usize;
            let helix_len = vm.dna.helix.strands.len();

            if a < helix_len && b < helix_len {
                if helix_len + 1 >= MAX_STRANDS {
                    vm.output
                        .push("CROSSOVER ERROR: Strand limit exceeded".to_string());
                    return None;
                }

                let genes_a = &vm.dna.helix.strands[a].genes;
                let genes_b = &vm.dna.helix.strands[b].genes;
                let len_a = genes_a.len();
                let len_b = genes_b.len();
                let min_len = len_a.min(len_b);

                if min_len > 0 {
                    let mut rng = rand::thread_rng();
                    let cut = rng.gen_range(0..min_len);

                    let mut new_genes_1 = genes_a[0..cut].to_vec();
                    new_genes_1.extend_from_slice(&genes_b[cut..]);

                    let mut new_genes_2 = genes_b[0..cut].to_vec();
                    new_genes_2.extend_from_slice(&genes_a[cut..]);

                    // Add first child
                    vm.dna
                        .helix
                        .strands
                        .push(crate::ast::Strand { genes: new_genes_1 });
                    vm.telomeres.push(50);
                    #[cfg(feature = "cortex")]
                    {
                        vm.activation_levels.push(0);
                        vm.synapse_map.push(Vec::new());
                    }
                    let child_1 = vm.dna.helix.strands.len() - 1;
                    vm.cladistics.register_strand(
                        child_1,
                        Some(a),
                        vm.tick_counter,
                        "Crossover".to_string(),
                    );

                    // Add second child
                    vm.dna
                        .helix
                        .strands
                        .push(crate::ast::Strand { genes: new_genes_2 });
                    vm.telomeres.push(50);
                    #[cfg(feature = "cortex")]
                    {
                        vm.activation_levels.push(0);
                        vm.synapse_map.push(Vec::new());
                    }
                    let child_2 = vm.dna.helix.strands.len() - 1;
                    vm.cladistics.register_strand(
                        child_2,
                        Some(b),
                        vm.tick_counter,
                        "Crossover".to_string(),
                    );

                    vm.stack.push(Value::Int(child_1 as i64));
                    vm.stack.push(Value::Int(child_2 as i64));
                    vm.energy = vm.energy.saturating_sub(20);
                    vm.output.push(format!(
                        "CROSSOVER: {}+{} -> {}, {}",
                        a, b, child_1, child_2
                    ));
                } else {
                    vm.output.push("CROSSOVER ERROR: Empty strand".to_string());
                }
            } else {
                vm.output
                    .push("CROSSOVER ERROR: Invalid strand index".to_string());
            }
        } else {
            vm.output.push("CROSSOVER ERROR: Type mismatch".to_string());
        }
    } else {
        vm.output
            .push("CROSSOVER ERROR: Stack underflow".to_string());
    }
    None
}

/// Combines two strands using a genetic splicing method.
///
/// **OpCode:** `Splice`
/// **Stack:** `[ ..., method, strand_b, strand_a ] -> [ ..., new_strand_idx ]`
/// **Methods:** 0=Interleave, 1=Uniform Crossover, 2=Midpoint Split.
pub fn exec_splice(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
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
                    if vm.dna.helix.strands.len() >= MAX_STRANDS {
                        vm.output
                            .push("Error: Strand limit exceeded for splice".to_string());
                        vm.stack.push(Value::Int(-1));
                        return None;
                    }
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
                        format!("Splice({}, {})", idx_a, idx_b),
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

/// Swaps the tails of two strands at a specific index.
///
/// **OpCode:** `Recombine`
/// **Stack:** `[ ..., split_point, strand_b, strand_a ] -> [ ... ]`
pub fn exec_recombine(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
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
                        // Perform recombination
                        // We need to borrow strands mutably.
                        // Since they are in the same Vec, we need split_at_mut or similar trickery,
                        // or just use indices if we can modify the Vec safely.
                        // We can't get two mutable references to the same Vec at different indices directly.
                        // So we'll use `split_at_mut` if they are different indices, or just do nothing if same.

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

/// Scans a target strand for a pattern matching a guide strand.
///
/// **OpCode:** `CrisprScan`
/// **Stack:** `[ ..., guide_idx, target_idx ] -> [ ..., match_index ]`
pub fn exec_crispr_scan(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
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

/// Cuts a strand at a specific index, creating a new strand from the tail.
///
/// **OpCode:** `Cas9Cut`
/// **Stack:** `[ ..., cut_index, strand_idx ] -> [ ..., new_strand_idx ]`
pub fn exec_cas9_cut(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
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
                    if vm.dna.helix.strands.len() >= MAX_STRANDS {
                        vm.output
                            .push("Error: Strand limit exceeded for cas9_cut".to_string());
                        return None;
                    }
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
                        "Cas9Cut".to_string(),
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

pub fn exec_incubate(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // stack: len, y, x (top)
    if vm.stack.len() >= 3 {
        let x_val = vm.stack.pop().unwrap();
        let y_val = vm.stack.pop().unwrap();
        let len_val = vm.stack.pop().unwrap();

        if let (Value::Int(x), Value::Int(y), Value::Int(len)) = (x_val, y_val, len_val) {
            if len > 0 {
                if vm.dna.helix.strands.len() >= MAX_STRANDS {
                    vm.output
                        .push("Error: Strand limit exceeded for incubate".to_string());
                    return None;
                }
                let mut genes = Vec::new();
                let mut valid = true;
                let max_len = (len as usize).min(MAX_INCUBATE_LENGTH);

                // Read sequence from grid
                let mut sequence = Vec::new();
                for k in 0..max_len {
                    if let Some((ny, nx)) = vm.normalize_coords(y, x + k as i64) {
                        sequence.push(vm.grid[ny][nx].clone());
                    } else {
                        valid = false;
                        vm.output
                            .push("Error: Incubate range out of bounds".to_string());
                        break;
                    }
                }

                if valid {
                    let mut k = 0;
                    while k < sequence.len() {
                        match &sequence[k] {
                            Value::Int(n) => {
                                // Treated as push(n)
                                genes.push(crate::ast::Gene {
                                    op: OpCode::Push,
                                    args: vec![crate::ast::Nucleotide::Number(*n)],
                                });
                                k += 1;
                            }
                            Value::Junction(t, vals) => {
                                // Treated as push(junction)
                                let mut nuc_vals = Vec::new();
                                for v in vals {
                                    if let Some(n) = value_to_nucleotide(v, 0) {
                                        nuc_vals.push(n);
                                    } else {
                                        vm.output.push(
                                            "INCUBATE: Warning: Recursion limit exceeded"
                                                .to_string(),
                                        );
                                    }
                                }
                                genes.push(crate::ast::Gene {
                                    op: OpCode::Push,
                                    args: vec![crate::ast::Nucleotide::Junction(*t, nuc_vals)],
                                });
                                k += 1;
                            }
                            Value::Superposition(_) => {
                                k += 1; // Skip
                            }
                            Value::Symbol(_) => {
                                k += 1; // Skip
                            }
                            Value::Color(_, _, _) => {
                                k += 1; // Skip
                            }
                            Value::Str(s) => {
                                let op = s.parse().unwrap_or(OpCode::Unknown(s.clone()));
                                let mut args = Vec::new();
                                match op {
                                    OpCode::Push
                                    | OpCode::Jump
                                    | OpCode::Brz
                                    | OpCode::Call
                                    | OpCode::Bind
                                    | OpCode::Unbind
                                    | OpCode::Decohere
                                    | OpCode::Telomerase
                                    | OpCode::Mitosis
                                    | OpCode::Apoptosis
                                    | OpCode::Cas9Cut
                                    | OpCode::Ligase
                                    | OpCode::Entangle => {
                                        #[cfg(feature = "cortex")]
                                        {
                                            // Gate is handled by cortex feature, but we can't easily conditionally match inside match arm list
                                            // So we'll just check if it's one of these.
                                        }
                                        // 1 Arg
                                        if k + 1 < sequence.len() {
                                            if let Some(arg) =
                                                value_to_nucleotide(&sequence[k + 1], 0)
                                            {
                                                args.push(arg);
                                            } else {
                                                vm.output.push(
                                                    "INCUBATE: Warning: Recursion limit exceeded"
                                                        .to_string(),
                                                );
                                            }
                                            k += 1; // Consume arg
                                        }
                                    }
                                    #[cfg(feature = "cortex")]
                                    OpCode::Gate | OpCode::Link | OpCode::Sever | OpCode::Spark => {
                                        if k + 1 < sequence.len() {
                                            if let Some(arg) =
                                                value_to_nucleotide(&sequence[k + 1], 0)
                                            {
                                                args.push(arg);
                                            } else {
                                                vm.output.push(
                                                    "INCUBATE: Warning: Recursion limit exceeded"
                                                        .to_string(),
                                                );
                                            }
                                            k += 1; // Consume arg
                                        }
                                    }
                                    _ => {}
                                }

                                genes.push(crate::ast::Gene { op, args });
                                k += 1;
                            }
                        }
                    }

                    vm.dna.helix.strands.push(crate::ast::Strand { genes });
                    vm.telomeres.push(50);
                    #[cfg(feature = "cortex")]
                    {
                        vm.activation_levels.push(0);
                        vm.synapse_map.push(Vec::new());
                    }

                    let new_idx = vm.dna.helix.strands.len() - 1;
                    vm.cladistics.register_strand(
                        new_idx,
                        Some(vm.ip.0),
                        vm.tick_counter,
                        "Incubate".to_string(),
                    );

                    vm.energy = vm.energy.saturating_sub(20); // Cost
                    vm.output.push(format!(
                        "INCUBATE: Created new strand {} from grid",
                        new_idx
                    ));
                }
            } else {
                vm.output
                    .push("Error: Invalid length for incubate".to_string());
            }
        } else {
            vm.output
                .push("Error: Type mismatch for incubate".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for incubate".to_string());
    }
    None
}

pub fn exec_methylate(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
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

pub fn exec_demethylate(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
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

pub fn exec_telomerase(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if let Some(val) = vm.stack.pop() {
        match val {
            Value::Int(amount) => {
                if amount > 0 {
                    let idx = vm.ip.0;
                    if idx < vm.telomeres.len() {
                        vm.telomeres[idx] = vm.telomeres[idx].saturating_add(amount);
                        vm.energy = vm.energy.saturating_sub(25); // High cost
                        vm.output
                            .push(format!("TELOMERASE: Extended strand {} by {}", idx, amount));
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

pub fn exec_tlen(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    let idx = vm.ip.0;
    if idx < vm.telomeres.len() {
        vm.stack.push(Value::Int(vm.telomeres[idx]));
    } else {
        vm.stack.push(Value::Int(0));
    }
    None
}

pub fn exec_ligase(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
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

                    let (first_slice, second_slice) = vm.dna.helix.strands.split_at_mut(upper);
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

pub fn exec_mitosis(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // stack: strand_idx (target to clone)
    if let Some(val) = vm.stack.pop() {
        match val {
            Value::Int(idx) => {
                let s_idx = idx as usize;
                if s_idx < vm.dna.helix.strands.len() {
                    if vm.dna.helix.strands.len() >= MAX_STRANDS {
                        vm.output
                            .push("Error: Strand limit exceeded for mitosis".to_string());
                        return None;
                    }
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
                        "Mitosis".to_string(),
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

pub fn exec_apoptosis(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
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
                    vm.output
                        .push("Error: Strand index out of bounds for apoptosis".to_string());
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

pub fn exec_integrase(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
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
                                    "INTEGRASE: Warning: Recursion limit exceeded".to_string(),
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
                        // Default None means step() will increment IP +1.
                        // If we shifted IP +1 here, total is +2.
                        // This skips the inserted gene (if at g_idx) and the current gene (now at g_idx+1).
                        // Correct.
                    } else {
                        vm.output
                            .push("Error: Gene index out of bounds for integrase".to_string());
                    }
                } else {
                    vm.output
                        .push("Error: Strand index out of bounds for integrase".to_string());
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

pub fn exec_excision(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
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
                        vm.output
                            .push("Error: Gene index out of bounds for excision".to_string());
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

pub fn exec_meme(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if let Some(gene) = &vm.last_gene {
        let gene_clone = gene.clone();
        let helix_len = vm.dna.helix.strands.len();
        if helix_len > 0 {
            let mut rng = rand::thread_rng();
            let target_strand = rng.gen_range(0..helix_len);
            let strand_len = vm.dna.helix.strands[target_strand].genes.len();
            let insert_pos = rng.gen_range(0..=strand_len); // Can insert at end

            vm.dna.helix.strands[target_strand]
                .genes
                .insert(insert_pos, gene_clone);

            vm.energy = vm.energy.saturating_sub(20);
            vm.output.push(format!(
                "MEME: Infected strand {} at {} with {}",
                target_strand, insert_pos, gene.op
            ));
        }
    } else {
        vm.output
            .push("MEME: No previous gene to spread".to_string());
    }
    None
}

pub fn exec_drift(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if let Some(val) = vm.stack.pop() {
        if let Value::Int(prob) = val {
            let p = prob.clamp(0, 100);
            let mut rng = rand::thread_rng();
            let mut changes = 0;

            let enzymes = [
                OpCode::Push,
                OpCode::Add,
                OpCode::Sub,
                OpCode::Mul,
                OpCode::Div,
                OpCode::Dup,
                OpCode::Print,
                OpCode::Swap,
                OpCode::Drop,
                OpCode::Jump,
                OpCode::Brz,
                OpCode::Photosynthesize,
                OpCode::Consume,
                OpCode::GRead,
                OpCode::GWrite,
                OpCode::Genome,
                OpCode::Meme,
                OpCode::Poly, // Self-reference!
            ];

            for strand in &mut vm.dna.helix.strands {
                for gene in &mut strand.genes {
                    if rng.gen_range(0..100) < p {
                        let new_op = enzymes[rng.gen_range(0..enzymes.len())].clone();
                        gene.op = new_op;
                        changes += 1;
                    }
                }
            }

            vm.energy = vm.energy.saturating_sub(50 + changes);
            vm.output.push(format!("DRIFT: Mutated {} genes", changes));
        } else {
            vm.output.push("Error: Type mismatch for drift".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for drift".to_string());
    }
    None
}

pub fn exec_poly(vm: &mut ChimeraVM, args: &[Nucleotide]) -> Option<(usize, usize)> {
    // stack: [ val ] (peek)
    if let Some(val) = vm.stack.last() {
        // args: [ op_int, op_str ]
        if args.len() >= 2 {
            let op_to_run = match val {
                Value::Int(_) => Some(&args[0]),
                Value::Str(_) => Some(&args[1]),
                _ => None,
            };

            if let Some(nucleotide) = op_to_run {
                let op_str = match nucleotide {
                    Nucleotide::String(s) => Some(s.clone()),
                    Nucleotide::Identifier(s) => Some(s.clone()),
                    _ => None,
                };

                if let Some(s) = op_str {
                    if let Ok(op) = s.parse::<OpCode>() {
                        return vm.execute_gene_inner(op, &[]);
                    } else {
                        vm.output.push(format!("POLY: Invalid OpCode {}", s));
                    }
                } else {
                    vm.output
                        .push("POLY: Arg must be String or Identifier".to_string());
                }
            }
        } else {
            vm.output.push("POLY: Requires 2 arguments".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for poly".to_string());
    }
    None
}

pub fn exec_compile(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if let Some(val) = vm.stack.pop() {
        if let Value::Str(s) = val {
            match ChimeraParser::parse(Rule::strand, &s) {
                Ok(mut pairs) => {
                    let pair = pairs.next().unwrap();
                    match crate::ast::Strand::try_from_pair(pair) {
                        Ok(strand) => {
                            if vm.dna.helix.strands.len() >= MAX_STRANDS {
                                vm.output
                                    .push("COMPILE ERROR: Strand limit exceeded".to_string());
                            } else {
                                vm.dna.helix.strands.push(strand);
                                vm.telomeres.push(50);
                                #[cfg(feature = "cortex")]
                                {
                                    vm.activation_levels.push(0);
                                    vm.synapse_map.push(Vec::new());
                                }
                                let new_idx = vm.dna.helix.strands.len() - 1;

                                vm.cladistics.register_strand(
                                    new_idx,
                                    Some(vm.ip.0),
                                    vm.tick_counter,
                                    "Compile".to_string(),
                                );

                                vm.stack.push(Value::Int(new_idx as i64));
                                vm.energy = vm.energy.saturating_sub(50);
                                vm.output.push("COMPILE: Success".to_string());
                            }
                        }
                        Err(e) => {
                            vm.output.push(format!("COMPILE ERROR: {}", e));
                        }
                    }
                }
                Err(e) => {
                    vm.output.push(format!("COMPILE ERROR: {}", e));
                }
            }
        } else {
            vm.output
                .push("Error: Type mismatch for compile".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for compile".to_string());
    }
    None
}

pub fn strand_to_string(strand: &crate::ast::Strand) -> String {
    fn format_nucleotide(n: &crate::ast::Nucleotide, depth: usize) -> String {
        if depth > crate::vm::MAX_RECURSION_DEPTH {
            return "...".to_string();
        }
        match n {
            crate::ast::Nucleotide::Number(i) => i.to_string(),
            crate::ast::Nucleotide::String(s) => format!("\"{}\"", s),
            crate::ast::Nucleotide::Identifier(s) => s.clone(),
            crate::ast::Nucleotide::Junction(t, args) => {
                let t_str = match t {
                    crate::ast::JunctionType::Any => "any",
                    crate::ast::JunctionType::All => "all",
                    crate::ast::JunctionType::Dish => "dish",
                };
                let args_str: Vec<String> = args
                    .iter()
                    .map(|arg| format_nucleotide(arg, depth + 1))
                    .collect();
                format!("{}({})", t_str, args_str.join(" "))
            }
        }
    }

    let mut s = String::from("[ ");
    for gene in &strand.genes {
        s.push_str(gene.op.as_ref());
        s.push('(');
        for (i, arg) in gene.args.iter().enumerate() {
            if i > 0 {
                s.push(' ');
            }
            s.push_str(&format_nucleotide(arg, 0));
        }
        s.push_str(") ");
    }
    s.push(']');
    s
}

pub fn exec_decompile(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if let Some(val) = vm.stack.pop() {
        if let Value::Int(idx) = val {
            let s_idx = idx as usize;
            if s_idx < vm.dna.helix.strands.len() {
                let strand = &vm.dna.helix.strands[s_idx];
                let s = strand_to_string(strand);
                vm.stack.push(Value::Str(s));
                vm.energy = vm.energy.saturating_sub(10);
            } else {
                vm.output
                    .push("Error: Strand index out of bounds for decompile".to_string());
            }
        } else {
            vm.output
                .push("Error: Type mismatch for decompile".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for decompile".to_string());
    }
    None
}

/// Triggers a metamorphic reboot based on a CA rule.
///
/// **OpCode:** `Genesis`
/// **Stack:** `[ ..., rule_id ] -> [ ... ]`
/// **Effect:** Replaces entire DNA with genes derived from the Grid state using CA rules.
pub fn exec_genesis(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if let Some(val) = vm.stack.pop() {
        if let Value::Int(rule_id) = val {
            // Retrieve Rule
            let rule = if let Some(r) = vm.garden.rules.get(&rule_id) {
                r.clone()
            } else {
                vm.output
                    .push(format!("GENESIS: Rule {} not found", rule_id));
                return None;
            };

            // Calculate "Born" cells
            let rows = vm.grid.len();
            let cols = if rows > 0 { vm.grid[0].len() } else { 0 };
            let mut new_genes = Vec::new();

            for y in 0..rows {
                for x in 0..cols {
                    // Count neighbors
                    let mut neighbors_count = 0;
                    for dy in -1..=1 {
                        for dx in -1..=1 {
                            if dy == 0 && dx == 0 {
                                continue;
                            }
                            if let Some((ny, nx)) =
                                vm.normalize_coords(y as i64 + dy, x as i64 + dx)
                            {
                                if let Value::Int(n) = &vm.grid[ny][nx] {
                                    if *n > 0 {
                                        neighbors_count += 1;
                                    }
                                }
                            }
                        }
                    }

                    let val = &vm.grid[y][x];
                    let is_alive = match val {
                        Value::Int(n) if *n > 0 => true,
                        _ => false,
                    };

                    let mut keep = false;
                    if is_alive {
                        if rule.survival.contains(&neighbors_count) {
                            keep = true;
                        }
                    } else if rule.birth.contains(&neighbors_count) {
                        keep = true;
                    }

                    if keep {
                        // Create gene from cell value
                        match val {
                            Value::Int(n) => {
                                new_genes.push(crate::ast::Gene {
                                    op: OpCode::Push,
                                    args: vec![crate::ast::Nucleotide::Number(*n)],
                                });
                            }
                            Value::Str(s) => {
                                // Try to parse as OpCode
                                let op = s.parse().unwrap_or(OpCode::Unknown(s.clone()));
                                new_genes.push(crate::ast::Gene {
                                    op,
                                    args: vec![], // No args for simple genesis
                                });
                            }
                            _ => {
                                // Default gene
                                new_genes.push(crate::ast::Gene {
                                    op: OpCode::Nop,
                                    args: vec![],
                                });
                            }
                        }
                    }
                }
            }

            if new_genes.is_empty() {
                vm.output
                    .push("GENESIS: The void remains... (No genes created)".to_string());
                return None;
            }

            // Wipe State
            vm.dna.helix.strands.clear();
            vm.organelles.clear();
            vm.epigenome.clear();
            vm.telomeres.clear();
            #[cfg(feature = "cortex")]
            {
                vm.activation_levels.clear();
                vm.synapse_map.clear();
            }
            vm.symbiotes.clear();
            vm.reflexes.clear();

            // Create New Strand
            vm.dna
                .helix
                .strands
                .push(crate::ast::Strand { genes: new_genes });
            vm.telomeres.push(100);
            #[cfg(feature = "cortex")]
            {
                vm.activation_levels.push(0);
                vm.synapse_map.push(Vec::new());
            }

            // Reset Execution
            vm.energy = crate::vm::INITIAL_ENERGY;
            vm.ip = (0, 0);
            vm.stack.clear();

            vm.output
                .push("GENESIS: A new world is born from the ashes.".to_string());

            // Register root
            vm.cladistics = crate::vm::cladistics::Cladistics::new();
            vm.cladistics
                .register_strand(0, None, vm.tick_counter, "Genesis".to_string());

            return Some((0, 0));
        } else {
            vm.output
                .push("Error: Type mismatch for genesis".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for genesis".to_string());
    }
    None
}
