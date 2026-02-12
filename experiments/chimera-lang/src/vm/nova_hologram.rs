#[cfg(feature = "nova")]
use super::{ChimeraVM, Value, GRID_SIZE};
#[cfg(feature = "nova")]
use crate::ast::{Gene, Nucleotide};
#[cfg(feature = "nova")]
use crate::opcode::OpCode;
#[cfg(feature = "nova")]
use std::f64::consts::PI;

#[cfg(feature = "nova")]
pub fn exec_interfere(vm: &mut ChimeraVM, _op: OpCode, args: &[Nucleotide]) -> Option<(usize, usize)> {
    // Input: Strand Index
    // Effect: Adds interference pattern to Hologram Grid
    let mut strand_idx = 0;
    if let Some(Nucleotide::Number(n)) = args.first() {
        strand_idx = *n as usize;
    } else if let Some(val) = vm.stack.pop() {
        if let Value::Int(n) = val {
            strand_idx = n as usize;
        }
    }

    if strand_idx < vm.dna.helix.strands.len() {
        let strand = &vm.dna.helix.strands[strand_idx];
        let center_x = (GRID_SIZE as f64 - 1.0) / 2.0;
        let center_y = (GRID_SIZE as f64 - 1.0) / 2.0;

        for (i, gene) in strand.genes.iter().enumerate() {
            // Encode OpCode into Frequency (k)
            let op_hash = calculate_hash(&gene.op);
            let k = (op_hash % 20) as f64 * 0.5 + 1.0; // 1.0 to 10.5

            // Encode Index into Phase (phi)
            let phi = (i as f64) * (PI / 4.0);

            // Add wave to grid
            for y in 0..GRID_SIZE {
                for x in 0..GRID_SIZE {
                    let dx = x as f64 - center_x;
                    let dy = y as f64 - center_y;
                    let r = (dx * dx + dy * dy).sqrt();

                    // Wave: e^(i(kr + phi))
                    let angle = k * r + phi;
                    let re = angle.cos();
                    let im = angle.sin();

                    // Accumulate
                    vm.hologram_grid[y][x].0 += re;
                    vm.hologram_grid[y][x].1 += im;
                }
            }
        }
        vm.output.push(format!("INTERFERE: Encoded strand {} into hologram", strand_idx));
        vm.energy = vm.energy.saturating_sub(20);
    } else {
        vm.output.push("INTERFERE: Invalid strand index".to_string());
    }
    None
}

#[cfg(feature = "nova")]
pub fn exec_refract(vm: &mut ChimeraVM, _op: OpCode, _args: &[Nucleotide]) -> Option<(usize, usize)> {
    // Input: None (uses Hologram Grid)
    // Effect: Decodes hologram into a new strand

    // We scan for known OpCodes.
    // This is computationally expensive, so we limit the search.
    // We check frequencies corresponding to standard OpCodes.

    let center_x = (GRID_SIZE as f64 - 1.0) / 2.0;
    let center_y = (GRID_SIZE as f64 - 1.0) / 2.0;

    let mut detected_genes: Vec<(usize, OpCode, f64)> = Vec::new();

    // List of OpCodes to scan for (subset for performance)
    let scan_ops = vec![
        OpCode::Push, OpCode::Add, OpCode::Sub, OpCode::Mul, OpCode::Div,
        OpCode::Dup, OpCode::Print, OpCode::Jump, OpCode::Brz,
        OpCode::Photosynthesize, OpCode::Consume, OpCode::GRead, OpCode::GWrite,
        OpCode::Mitosis, OpCode::Apoptosis, OpCode::Hologram, OpCode::Project
    ];

    for op in scan_ops {
        let op_hash = calculate_hash(&op);
        let k = (op_hash % 20) as f64 * 0.5 + 1.0;

        // Scan for this k at different phases (indices)
        // Limit to 16 genes length
        for i in 0..16 {
            let phi = (i as f64) * (PI / 4.0);

            // Calculate overlap (Dot Product)
            let mut sum_re = 0.0;
            // let mut sum_im = 0.0;

            for y in 0..GRID_SIZE {
                for x in 0..GRID_SIZE {
                    let dx = x as f64 - center_x;
                    let dy = y as f64 - center_y;
                    let r = (dx * dx + dy * dy).sqrt();
                    let angle = k * r + phi;

                    // Hologram value H
                    let (h_re, h_im) = vm.hologram_grid[y][x];

                    // Reference wave R* = e^(-i(kr+phi)) = cos - i sin
                    let r_re = angle.cos();
                    let r_im = -angle.sin();

                    // H * R* = (h_re + i h_im)(r_re + i r_im)
                    // Real part = h_re*r_re - h_im*r_im
                    let prod_re = h_re * r_re - h_im * r_im;
                    sum_re += prod_re;
                }
            }

            // Normalize
            let resonance = sum_re / (GRID_SIZE * GRID_SIZE) as f64;

            if resonance > 0.5 { // Threshold
                detected_genes.push((i, op.clone(), resonance));
            }
        }
    }

    // Sort by index and pick best match for each index
    detected_genes.sort_by(|a, b| {
        a.0.cmp(&b.0).then(b.2.partial_cmp(&a.2).unwrap())
    });

    // De-duplicate indices (take strongest)
    let mut genes = Vec::new();
    let mut last_idx = -1;

    for (idx, op, _strength) in detected_genes {
        if idx as i32 > last_idx {
            genes.push(Gene { op, args: vec![] });
            last_idx = idx as i32;
        }
    }

    if !genes.is_empty() {
        if vm.dna.helix.strands.len() >= crate::vm::MAX_STRANDS {
             vm.output.push("REFRACT: Strand limit exceeded".to_string());
             return None;
        }

        vm.dna.helix.strands.push(crate::ast::Strand { genes });
        vm.telomeres.push(50);
        #[cfg(feature = "cortex")]
        {
            vm.activation_levels.push(0);
            vm.synapse_map.push(Vec::new());
        }

        let new_idx = vm.dna.helix.strands.len() - 1;
        vm.cladistics.register_strand(new_idx, Some(vm.ip.0), vm.tick_counter, "Refraction".to_string());

        vm.stack.push(Value::Int(new_idx as i64));
        vm.energy = vm.energy.saturating_sub(30);
        vm.output.push(format!("REFRACT: Reconstructed strand {} from hologram", new_idx));
    } else {
        vm.output.push("REFRACT: No coherent pattern found".to_string());
    }

    None
}

#[cfg(feature = "nova")]
pub fn exec_project(vm: &mut ChimeraVM, _op: OpCode, _args: &[Nucleotide]) -> Option<(usize, usize)> {
    // Project hologram intensity to Grid
    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            let (re, im) = vm.hologram_grid[y][x];
            let magnitude = (re * re + im * im).sqrt();
            let val = (magnitude * 10.0) as i64; // Scale
            if val > 0 {
                vm.grid[y][x] = Value::Int(val);
            }
        }
    }
    vm.output.push("PROJECT: Manifested hologram on grid".to_string());
    vm.energy = vm.energy.saturating_sub(10);
    None
}

#[cfg(feature = "nova")]
fn calculate_hash(op: &OpCode) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    op.hash(&mut hasher);
    hasher.finish()
}
