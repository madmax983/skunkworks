#[cfg(feature = "nova")]
use super::{ChimeraVM, Value, GRID_SIZE};
#[cfg(feature = "nova")]
use crate::ast::{Gene, Nucleotide};
#[cfg(feature = "nova")]
use crate::opcode::OpCode;
#[cfg(feature = "nova")]
use rand::Rng;
#[cfg(feature = "nova")]
use std::f64::consts::PI;
#[cfg(feature = "nova")]
use strum::IntoEnumIterator;

#[cfg(feature = "nova")]
pub fn exec_interfere(
    vm: &mut ChimeraVM,
    _op: OpCode,
    args: &[Nucleotide],
) -> Option<(usize, usize)> {
    // Input: Strand Index
    // Effect: Adds interference pattern to Hologram Grid using DFT Encoding
    // Gene Index i -> Frequency (u, v)
    // OpCode -> Phase
    // Argument -> Amplitude

    let mut strand_idx = 0;
    if let Some(Nucleotide::Number(n)) = args.first() {
        strand_idx = *n as usize;
    } else if let Some(Value::Int(n)) = vm.stack.pop() {
        strand_idx = n as usize;
    }

    if strand_idx < vm.dna.helix.strands.len() {
        let strand = &vm.dna.helix.strands[strand_idx];
        let op_count = OpCode::iter().count() as f64;
        let n_grid = GRID_SIZE as f64;

        for (i, gene) in strand.genes.iter().enumerate() {
            if i >= 16 {
                break;
            } // Limit to 16 genes per hologram layer

            // Map Index to Frequency (u, v)
            // Avoid DC (0,0) and Nyquist boundaries to be safe
            // i=0..15
            let u = ((i % 4) * 2 + 1) as f64; // 1, 3, 5, 7
            let v = ((i / 4) * 2 + 1) as f64; // 1, 3, 5, 7

            // Encode OpCode into Phase
            let op_idx = get_opcode_index(&gene.op) as f64;
            let phi = (op_idx / op_count) * 2.0 * PI;

            // Encode Argument into Amplitude
            let mut amplitude = 1.0;
            if let Some(Nucleotide::Number(n)) = gene.args.first() {
                amplitude = 1.0 + (*n as f64).abs() / 50.0;
            }

            // Inverse DFT (Accumulate)
            // H[y][x] += A * e^(i(2pi(ux+vy)/N + phi))
            for y in 0..GRID_SIZE {
                for x in 0..GRID_SIZE {
                    let angle = 2.0 * PI * (u * (x as f64) + v * (y as f64)) / n_grid + phi;
                    let re = amplitude * angle.cos();
                    let im = amplitude * angle.sin();
                    vm.hologram_grid[y][x].0 += re;
                    vm.hologram_grid[y][x].1 += im;
                }
            }
        }
        vm.output.push(format!(
            "INTERFERE: Encoded strand {} into hologram",
            strand_idx
        ));
        vm.energy = vm.energy.saturating_sub(20);
    } else {
        vm.output
            .push("INTERFERE: Invalid strand index".to_string());
    }
    None
}

#[cfg(feature = "nova")]
pub fn exec_diffract(
    vm: &mut ChimeraVM,
    _op: OpCode,
    args: &[Nucleotide],
) -> Option<(usize, usize)> {
    // Input: Strand Index
    // Effect: Encodes strand with a spatial shift (Phase ramp in freq domain).
    // When refracted normally, the phases will be shifted, scrambling OpCodes.

    let mut strand_idx = 0;
    if let Some(Nucleotide::Number(n)) = args.first() {
        strand_idx = *n as usize;
    } else if let Some(Value::Int(n)) = vm.stack.pop() {
        strand_idx = n as usize;
    }

    if strand_idx < vm.dna.helix.strands.len() {
        let strand = &vm.dna.helix.strands[strand_idx];
        let op_count = OpCode::iter().count() as f64;
        let n_grid = GRID_SIZE as f64;

        // Spatial shift
        let shift_x = 4.0;
        let shift_y = 4.0;

        for (i, gene) in strand.genes.iter().enumerate() {
            if i >= 16 {
                break;
            }

            let u = ((i % 4) * 2 + 1) as f64;
            let v = ((i / 4) * 2 + 1) as f64;

            let op_idx = get_opcode_index(&gene.op) as f64;
            let phi = (op_idx / op_count) * 2.0 * PI;
            let amplitude = 0.5;

            for y in 0..GRID_SIZE {
                for x in 0..GRID_SIZE {
                    // Shifted coords: x' = x - shift_x
                    let xx = x as f64 - shift_x;
                    let yy = y as f64 - shift_y;

                    let angle = 2.0 * PI * (u * xx + v * yy) / n_grid + phi;
                    let re = amplitude * angle.cos();
                    let im = amplitude * angle.sin();

                    vm.hologram_grid[y][x].0 += re;
                    vm.hologram_grid[y][x].1 += im;
                }
            }
        }
        vm.output
            .push(format!("DIFFRACT: Created ghost of strand {}", strand_idx));
        vm.energy = vm.energy.saturating_sub(15);
    }
    None
}

#[cfg(feature = "nova")]
pub fn exec_refract(
    vm: &mut ChimeraVM,
    _op: OpCode,
    _args: &[Nucleotide],
) -> Option<(usize, usize)> {
    let op_codes: Vec<OpCode> = OpCode::iter().collect();
    let op_count = op_codes.len() as f64;
    let n_grid = GRID_SIZE as f64;
    let grid_area = (GRID_SIZE * GRID_SIZE) as f64;

    let mut genes = Vec::new();

    for i in 0..16 {
        let u = ((i % 4) * 2 + 1) as f64;
        let v = ((i / 4) * 2 + 1) as f64;

        // DFT: Sum H[y][x] * e^(-i 2pi (ux+vy)/N)
        let mut sum_re = 0.0;
        let mut sum_im = 0.0;

        for y in 0..GRID_SIZE {
            for x in 0..GRID_SIZE {
                let angle = 2.0 * PI * (u * (x as f64) + v * (y as f64)) / n_grid;
                let r_re = angle.cos();
                let r_im = -angle.sin();

                let (h_re, h_im) = vm.hologram_grid[y][x];

                sum_re += h_re * r_re - h_im * r_im;
                sum_im += h_re * r_im + h_im * r_re;
            }
        }

        // Normalize
        // If we added A*e^iphi, sum should be A * N^2 * e^iphi ?
        // Sum of e^... * e^-... = Sum(1) = N^2.
        // So divide by N^2.
        let z_re = sum_re / grid_area;
        let z_im = sum_im / grid_area;

        let magnitude = (z_re * z_re + z_im * z_im).sqrt();

        if magnitude > 0.1 {
            // Decode Phase
            let phase = z_im.atan2(z_re);
            let phase_norm = if phase < 0.0 { phase + 2.0 * PI } else { phase };

            // Round to nearest OpCode slot
            // phase = (idx / count) * 2PI
            // idx = phase * count / 2PI
            let idx_float = (phase_norm * op_count) / (2.0 * PI);
            let idx = idx_float.round() as usize % op_codes.len();

            let detected_op = op_codes[idx].clone();

            // Decode Amplitude
            // A = 1.0 + Arg/50.0
            // Arg = (A - 1.0) * 50.0
            let est_amplitude = magnitude;
            let mut args = Vec::new();

            if est_amplitude > 1.02 {
                // Tolerance for 1.0
                let val = ((est_amplitude - 1.0) * 50.0).round() as i64;
                // Clamp or check validity?
                // The shift might cause amplitude noise too.
                if val != 0 {
                    args.push(Nucleotide::Number(val));
                }
            }

            genes.push(Gene {
                op: detected_op,
                args,
            });
        } else {
            // No signal at this frequency
            // Stop at first gap? Or allow gaps (Nop)?
            // If we stop, we might miss genes if there's a Nop encoded as "No Signal" (if we did that).
            // But we encoded everything with A >= 1.0.
            // So low magnitude means END of strand.
            break;
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
        vm.cladistics.register_strand(
            new_idx,
            Some(vm.ip.0),
            vm.tick_counter,
            "Refraction".to_string(),
        );

        vm.stack.push(Value::Int(new_idx as i64));
        vm.energy = vm.energy.saturating_sub(30);
        vm.output.push(format!(
            "REFRACT: Reconstructed strand {} from hologram",
            new_idx
        ));
    } else {
        vm.output
            .push("REFRACT: No coherent pattern found".to_string());
    }

    None
}

#[cfg(feature = "nova")]
pub fn exec_project(
    vm: &mut ChimeraVM,
    _op: OpCode,
    _args: &[Nucleotide],
) -> Option<(usize, usize)> {
    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            let (re, im) = vm.hologram_grid[y][x];
            let magnitude = (re * re + im * im).sqrt();
            let val = (magnitude * 10.0) as i64;
            if val > 0 {
                vm.grid[y][x] = Value::Int(val);
            }
        }
    }
    vm.output
        .push("PROJECT: Manifested hologram on grid".to_string());
    vm.energy = vm.energy.saturating_sub(10);
    None
}

#[cfg(feature = "nova")]
pub fn exec_phase_mutate(
    vm: &mut ChimeraVM,
    _op: OpCode,
    _args: &[Nucleotide],
) -> Option<(usize, usize)> {
    let mut strand_idx = 0;
    let mut severity = 0.0;

    if let Some(Value::Int(n)) = vm.stack.pop() {
        strand_idx = n as usize;
    }

    if let Some(Value::Int(n)) = vm.stack.pop() {
        severity = n as f64 / 100.0;
    }

    if strand_idx < vm.dna.helix.strands.len() {
        let strand = &vm.dna.helix.strands[strand_idx];
        let op_codes: Vec<OpCode> = OpCode::iter().collect();
        let op_count = op_codes.len() as f64;
        let n_grid = GRID_SIZE as f64;

        // Local Buffer
        let mut buffer = [[(0.0, 0.0); GRID_SIZE]; GRID_SIZE];

        // Encode
        for (i, gene) in strand.genes.iter().enumerate() {
            if i >= 16 {
                break;
            }
            let u = ((i % 4) * 2 + 1) as f64;
            let v = ((i / 4) * 2 + 1) as f64;

            let op_idx = get_opcode_index(&gene.op) as f64;
            let phi = (op_idx / op_count) * 2.0 * PI;

            let mut amplitude = 1.0;
            if let Some(Nucleotide::Number(n)) = gene.args.first() {
                amplitude = 1.0 + (*n as f64).abs() / 50.0;
            }

            for (y, row) in buffer.iter_mut().enumerate() {
                for (x, cell) in row.iter_mut().enumerate() {
                    let angle = 2.0 * PI * (u * (x as f64) + v * (y as f64)) / n_grid + phi;
                    let re = amplitude * angle.cos();
                    let im = amplitude * angle.sin();
                    cell.0 += re;
                    cell.1 += im;
                }
            }
        }

        // Apply Phase Noise
        let mut rng = rand::thread_rng();
        for row in buffer.iter_mut() {
            for (re, im) in row.iter_mut() {
                let magnitude = (*re * *re + *im * *im).sqrt();
                if magnitude > 0.001 {
                    let mut phase = im.atan2(*re);
                    let noise = (rng.gen::<f64>() - 0.5) * 2.0 * PI * severity;
                    phase += noise;

                    *re = magnitude * phase.cos();
                    *im = magnitude * phase.sin();
                }
            }
        }

        // Decode
        let mut new_genes = Vec::new();
        let grid_area = (GRID_SIZE * GRID_SIZE) as f64;

        for i in 0..16 {
            let u = ((i % 4) * 2 + 1) as f64;
            let v = ((i / 4) * 2 + 1) as f64;

            let mut sum_re = 0.0;
            let mut sum_im = 0.0;

            // Here we need (y, x) for angle, so iteration needs index.
            for (y, row) in buffer.iter().enumerate() {
                for (x, (h_re, h_im)) in row.iter().enumerate() {
                    let angle = 2.0 * PI * (u * (x as f64) + v * (y as f64)) / n_grid;
                    let r_re = angle.cos();
                    let r_im = -angle.sin();

                    sum_re += h_re * r_re - h_im * r_im;
                    sum_im += h_re * r_im + h_im * r_re;
                }
            }

            let z_re = sum_re / grid_area;
            let z_im = sum_im / grid_area;
            let magnitude = (z_re * z_re + z_im * z_im).sqrt();

            if magnitude > 0.1 {
                let phase = z_im.atan2(z_re);
                let phase_norm = if phase < 0.0 { phase + 2.0 * PI } else { phase };
                let idx_float = (phase_norm * op_count) / (2.0 * PI);
                let idx = idx_float.round() as usize % op_codes.len();
                let detected_op = op_codes[idx].clone();

                let mut args = Vec::new();
                if magnitude > 1.02 {
                    let val = ((magnitude - 1.0) * 50.0).round() as i64;
                    if val != 0 {
                        args.push(Nucleotide::Number(val));
                    }
                }

                new_genes.push(Gene {
                    op: detected_op,
                    args,
                });
            } else {
                break;
            }
        }

        if !new_genes.is_empty() {
            if vm.dna.helix.strands.len() >= crate::vm::MAX_STRANDS {
                vm.output
                    .push("PHASE_MUTATE: Strand limit exceeded".to_string());
                return None;
            }

            vm.dna.helix.strands.push(crate::ast::Strand { genes: new_genes });
            vm.telomeres.push(50);
            #[cfg(feature = "cortex")]
            {
                vm.activation_levels.push(0);
                vm.synapse_map.push(Vec::new());
            }

            let new_idx = vm.dna.helix.strands.len() - 1;
            vm.cladistics.register_strand(
                new_idx,
                Some(strand_idx),
                vm.tick_counter,
                "HoloMutate".to_string(),
            );

            vm.stack.push(Value::Int(new_idx as i64));
            vm.energy = vm.energy.saturating_sub(40);
            vm.output
                .push(format!("PHASE_MUTATE: Mutated {} -> {}", strand_idx, new_idx));
        } else {
            vm.output
                .push("PHASE_MUTATE: Signal lost in noise".to_string());
        }
    } else {
        vm.output
            .push("PHASE_MUTATE: Invalid strand index".to_string());
    }

    None
}

#[cfg(feature = "nova")]
fn get_opcode_index(op: &OpCode) -> usize {
    OpCode::iter().position(|x| x == *op).unwrap_or(0)
}
