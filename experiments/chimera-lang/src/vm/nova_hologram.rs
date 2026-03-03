#[cfg(feature = "nova")]
use super::{ChimeraVM, GRID_SIZE};
#[cfg(feature = "nova")]
use crate::ast::{Gene, Nucleotide};
#[cfg(feature = "nova")]
use crate::opcode::OpCode;
#[cfg(feature = "nova")]
use crate::value::Value;
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
    } else if let Some(val) = vm.stack.pop() {
        if let Value::Int(n) = val {
            strand_idx = n as usize;
        }
    }

    if strand_idx < vm.dna.helix.strands.len() {
        let strand = &vm.dna.helix.strands[strand_idx];
        let op_count = OpCode::iter().count() as f64;
        let n_grid = GRID_SIZE as f64;

        for (i, gene) in strand.genes.iter().enumerate() {
            if i >= 256 {
                break;
            } // Limit to 256 genes per hologram layer

            // Map Index to Frequency (u, v)
            let u = (i % 16) as f64;
            let v = (i / 16) as f64;

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
    } else if let Some(val) = vm.stack.pop() {
        if let Value::Int(n) = val {
            strand_idx = n as usize;
        }
    }

    if strand_idx < vm.dna.helix.strands.len() {
        let strand = &vm.dna.helix.strands[strand_idx];
        let op_count = OpCode::iter().count() as f64;
        let n_grid = GRID_SIZE as f64;

        // Spatial shift
        let shift_x = 4.0;
        let shift_y = 4.0;

        for (i, gene) in strand.genes.iter().enumerate() {
            if i >= 256 {
                break;
            }

            let u = (i % 16) as f64;
            let v = (i / 16) as f64;

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
    let genes = refract_genes(vm);

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
pub fn mutate_hologram(vm: &mut ChimeraVM, intensity: f64) {
    let mut rng = rand::thread_rng();
    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            let (re, im) = vm.hologram_grid[y][x];
            let magnitude: f64 = (re * re + im * im).sqrt();
            if magnitude > 0.001 {
                let phase: f64 = im.atan2(re);
                let noise = rng.gen_range(-intensity..intensity);
                let new_phase = phase + noise;
                vm.hologram_grid[y][x] = (magnitude * new_phase.cos(), magnitude * new_phase.sin());
            }
        }
    }
}

#[cfg(feature = "nova")]
pub fn refract_genes(vm: &ChimeraVM) -> Vec<Gene> {
    let op_codes: Vec<OpCode> = OpCode::iter().collect();
    let op_count = op_codes.len() as f64;
    let n_grid = GRID_SIZE as f64;
    let grid_area = (GRID_SIZE * GRID_SIZE) as f64;

    let mut genes = Vec::new();

    for i in 0..256 {
        let u = (i % 16) as f64;
        let v = (i / 16) as f64;

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

        let magnitude: f64 = (z_re * z_re + z_im * z_im).sqrt();

        if magnitude > 0.1 {
            // Decode Phase
            let phase: f64 = z_im.atan2(z_re);
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
            break;
        }
    }
    genes
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
            let magnitude: f64 = (re * re + im * im).sqrt();
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
pub fn exec_hologram(
    vm: &mut ChimeraVM,
    _op: OpCode,
    _args: &[Nucleotide],
) -> Option<(usize, usize)> {
    vm.hologram_mode = !vm.hologram_mode;
    let status = if vm.hologram_mode { "ON" } else { "OFF" };
    vm.output
        .push(format!("HOLOGRAM: Visualization {}", status));
    None
}

#[cfg(feature = "nova")]
pub fn exec_phase_mutate(
    vm: &mut ChimeraVM,
    _op: OpCode,
    _args: &[Nucleotide],
) -> Option<(usize, usize)> {
    let mut rng = rand::thread_rng();
    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            let (re, im) = vm.hologram_grid[y][x];
            let magnitude: f64 = (re * re + im * im).sqrt();
            if magnitude > 0.001 {
                let phase: f64 = im.atan2(re);
                let noise = rng.gen_range(-0.5..0.5); // Tune this?
                let new_phase = phase + noise;
                vm.hologram_grid[y][x] = (magnitude * new_phase.cos(), magnitude * new_phase.sin());
            }
        }
    }
    vm.output
        .push("PHASE_MUTATE: Scrambled hologram phase".to_string());
    vm.energy = vm.energy.saturating_sub(10);
    None
}

#[cfg(feature = "nova")]
pub fn exec_quantum_scribe(
    vm: &mut ChimeraVM,
    _op: OpCode,
    _args: &[Nucleotide],
) -> Option<(usize, usize)> {
    // stack: threshold
    let threshold = if let Some(Value::Int(t)) = vm.stack.pop() {
        t as f64 / 10.0 // Scale down
    } else {
        1.0
    };

    let (cy, cx) = vm.context_loc;
    let (re, im) = vm.hologram_grid[cy][cx];
    let magnitude: f64 = (re * re + im * im).sqrt();

    if magnitude > threshold {
        // Collapse Phase to Char
        let phase: f64 = im.atan2(re); // -PI to PI
        let normalized = (phase + PI) / (2.0 * PI); // 0.0 to 1.0
        let idx = (normalized * 94.0).round().clamp(0.0, 93.0) as u8;
        let char_code = idx + 33; // ASCII '!' (33) to '~' (126)
        let c = char_code as char;

        vm.grid[cy][cx] = Value::Str(c.to_string());
        vm.output.push(format!(
            "QUANTUM_SCRIBE: Collapsed to '{}' at {},{}",
            c, cx, cy
        ));
    } else {
        vm.output.push(format!(
            "QUANTUM_SCRIBE: Magnitude {:.2} too low at {},{}",
            magnitude, cx, cy
        ));
    }
    vm.energy = vm.energy.saturating_sub(5);
    None
}

#[cfg(feature = "nova")]
pub fn exec_quantum_scan(
    vm: &mut ChimeraVM,
    _op: OpCode,
    _args: &[Nucleotide],
) -> Option<(usize, usize)> {
    // stack: weight
    let weight = if let Some(Value::Int(w)) = vm.stack.pop() {
        w as f64 / 10.0
    } else {
        1.0
    };

    let (cy, cx) = vm.context_loc;
    let val = &vm.grid[cy][cx];

    if let Value::Str(s) = val {
        if let Some(c) = s.chars().next() {
            if c.is_ascii_graphic() {
                let code = c as u8;
                if (33..=126).contains(&code) {
                    let idx = code - 33;
                    let normalized = idx as f64 / 94.0;
                    let phase = normalized * 2.0 * PI - PI; // -PI to PI

                    let re = weight * phase.cos();
                    let im = weight * phase.sin();

                    vm.hologram_grid[cy][cx].0 += re;
                    vm.hologram_grid[cy][cx].1 += im;

                    vm.output
                        .push(format!("QUANTUM_SCAN: Encoded '{}' into hologram", c));
                }
            }
        }
    }
    vm.energy = vm.energy.saturating_sub(5);
    None
}

#[cfg(feature = "nova")]
pub fn exec_holo_invoke(
    vm: &mut ChimeraVM,
    _op: OpCode,
    _args: &[Nucleotide],
) -> Option<(usize, usize)> {
    if vm.recursion_depth > super::MAX_RECURSION_DEPTH {
        vm.output.push("HOLO_INVOKE: Recursion limit".to_string());
        return None;
    }
    vm.recursion_depth += 1;

    if let Some(Value::Str(input_string)) = vm.stack.pop() {
        let genes = refract_genes(vm);
        if genes.is_empty() {
            vm.output
                .push("HOLO_INVOKE: No grammar in hologram".to_string());
            vm.stack.push(Value::Int(0)); // Fail
        } else {
            let start_stack_depth = vm.stack.len();
            // Execute genes to build grammar
            for gene in genes {
                let _ = vm.execute_gene_inner(gene.op, &gene.args);
            }

            if vm.stack.len() > start_stack_depth {
                let grammar = vm.stack.pop().unwrap();
                match crate::vm::babel::run_parser(&grammar, &input_string) {
                    Ok((ast, consumed)) => {
                        vm.output
                            .push(format!("HOLO_INVOKE: Parsed {} chars", consumed));
                        vm.stack.push(ast);
                    }
                    Err(_) => {
                        vm.output.push("HOLO_INVOKE: Parse failed".to_string());
                        vm.stack.push(Value::Int(0));
                    }
                }
            } else {
                vm.output
                    .push("HOLO_INVOKE: Hologram produced no grammar".to_string());
                vm.stack.push(Value::Int(0));
            }
        }
    } else {
        vm.output
            .push("HOLO_INVOKE: Stack underflow (expected string)".to_string());
    }

    vm.recursion_depth -= 1;
    vm.energy = vm.energy.saturating_sub(10);
    None
}

#[cfg(feature = "nova")]
pub fn exec_holo_speak(
    vm: &mut ChimeraVM,
    _op: OpCode,
    _args: &[Nucleotide],
) -> Option<(usize, usize)> {
    if vm.recursion_depth > super::MAX_RECURSION_DEPTH {
        vm.output.push("HOLO_SPEAK: Recursion limit".to_string());
        return None;
    }
    vm.recursion_depth += 1;

    let genes = refract_genes(vm);
    if genes.is_empty() {
        vm.output
            .push("HOLO_SPEAK: No grammar in hologram".to_string());
        vm.stack.push(Value::Str("".to_string()));
    } else {
        let start_stack_depth = vm.stack.len();
        for gene in genes {
            let _ = vm.execute_gene_inner(gene.op, &gene.args);
        }

        if vm.stack.len() > start_stack_depth {
            let grammar = vm.stack.pop().unwrap();
            let s = crate::vm::babel::generate_string(&grammar);
            vm.output.push(format!("HOLO_SPEAK: '{}'", s));
            vm.stack.push(Value::Str(s));
        } else {
            vm.output
                .push("HOLO_SPEAK: Hologram produced no grammar".to_string());
            vm.stack.push(Value::Str("".to_string()));
        }
    }

    vm.recursion_depth -= 1;
    vm.energy = vm.energy.saturating_sub(10);
    None
}

#[cfg(feature = "nova")]
fn get_opcode_index(op: &OpCode) -> usize {
    OpCode::iter().position(|x| x == *op).unwrap_or(0)
}

#[cfg(feature = "nova")]
pub fn exec_holo_sonify(
    vm: &mut ChimeraVM,
    _op: OpCode,
    _args: &[Nucleotide],
) -> Option<(usize, usize)> {
    // Stack: [ ..., threshold ]
    let threshold = if let Some(Value::Int(t)) = vm.stack.pop() {
        t as f64 / 10.0
    } else {
        0.5
    };

    let mut notes_generated = 0;

    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            let (re, im) = vm.hologram_grid[y][x];
            let magnitude: f64 = (re * re + im * im).sqrt();

            if magnitude > threshold {
                // Pitch: Map spatial position to scale
                // Simple pentatonic mapping? Or chromatic?
                // Let's do a simple chromatic map for now based on index
                let base_pitch = 36; // C2
                let pitch = (base_pitch + (x + y * GRID_SIZE) % 64) as u8;

                // Velocity: Based on Magnitude
                let velocity = (magnitude * 20.0).clamp(1.0, 127.0) as u8;

                // Duration: Based on Phase (-PI to PI) -> (1 to 16 ticks)
                let phase: f64 = im.atan2(re);
                let duration = (((phase + PI) / (2.0 * PI)) * 16.0).clamp(1.0, 16.0) as u8;

                vm.midi_messages.push(super::MidiEvent::NoteOn {
                    channel: 0,
                    note: pitch,
                    velocity,
                    duration,
                });
                notes_generated += 1;
            }
        }
    }

    vm.output.push(format!(
        "HOLO_SONIFY: Generated {} notes (Thresh {:.2})",
        notes_generated, threshold
    ));
    vm.energy = vm.energy.saturating_sub(notes_generated.min(50) as i64);
    None
}

#[cfg(feature = "nova")]
pub fn exec_cymatic_scan(
    vm: &mut ChimeraVM,
    _op: OpCode,
    _args: &[Nucleotide],
) -> Option<(usize, usize)> {
    // Stack: [ ..., scale ]
    let scale = if let Some(Value::Int(s)) = vm.stack.pop() {
        s as f64 / 10.0
    } else {
        1.0
    };

    // Need to access audio_snapshot (requires resonance feature)
    #[cfg(feature = "resonance")]
    {
        let pressure_len = vm.audio_snapshot.pressure.len();
        let grid_area = GRID_SIZE * GRID_SIZE;

        if pressure_len == grid_area {
            let mut total_energy = 0.0;
            for y in 0..GRID_SIZE {
                for x in 0..GRID_SIZE {
                    let idx = y * GRID_SIZE + x;
                    let p = vm.audio_snapshot.pressure[idx] as f64;

                    // Map pressure to Real part of Hologram
                    // Maybe use previous Real as Imaginary to rotate phase?
                    // Or just add to Real.
                    vm.hologram_grid[y][x].0 += p * scale;
                    total_energy += p.abs();
                }
            }
            vm.output.push(format!(
                "CYMATIC_SCAN: Encoded audio energy {:.2} into hologram",
                total_energy
            ));
        } else {
            vm.output
                .push("CYMATIC_SCAN: Audio snapshot size mismatch".to_string());
        }
    }
    #[cfg(not(feature = "resonance"))]
    {
        vm.output
            .push("CYMATIC_SCAN: Resonance feature disabled".to_string());
    }

    vm.energy = vm.energy.saturating_sub(10);
    None
}
