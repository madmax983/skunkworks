#[cfg(feature = "nova")]
use crate::ast::Nucleotide;
#[cfg(feature = "nova")]
use crate::opcode::OpCode;
#[cfg(feature = "nova")]
use crate::vm::{ChimeraVM, Value, GRID_SIZE};
#[cfg(feature = "nova")]
use rand::Rng;
#[cfg(feature = "nova")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "nova")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scent {
    pub signature: String,
    pub x: f64,
    pub y: f64,
    pub intensity: f64,
    pub age: usize,
}

#[cfg(feature = "nova")]
pub fn exec_scent_op(
    vm: &mut ChimeraVM,
    op: OpCode,
    _args: &[Nucleotide],
) -> Option<(usize, usize)> {
    match op {
        OpCode::Emit => exec_emit(vm),
        OpCode::Smell => exec_smell(vm),
        OpCode::Track => exec_track(vm),
        _ => None,
    }
}

#[cfg(feature = "nova")]
fn exec_emit(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // Stack: [ ..., intensity, signature_string ]
    if vm.stack.len() >= 2 {
        let sig_val = vm.stack.pop().unwrap();
        let int_val = vm.stack.pop().unwrap();

        if let (Value::Str(sig), Value::Int(int)) = (sig_val, int_val) {
            if int > 0 {
                let (cy, cx) = vm.context_loc;
                // Add fuzz to position for natural spread
                let mut rng = rand::thread_rng();
                let fuzz_x = rng.gen_range(-0.2..0.2);
                let fuzz_y = rng.gen_range(-0.2..0.2);

                let scent = Scent {
                    signature: sig.clone(),
                    x: cx as f64 + 0.5 + fuzz_x,
                    y: cy as f64 + 0.5 + fuzz_y,
                    intensity: int as f64,
                    age: 0,
                };

                // Limit total scents to prevent DoS
                if vm.pheromones.len() < 1024 {
                    vm.pheromones.push(scent);
                    vm.energy = vm.energy.saturating_sub(int / 10 + 1);
                    vm.output.push(format!("EMIT: '{}' at {},{}", sig, cx, cy));
                } else {
                    vm.output
                        .push("EMIT: Pheromone cloud saturated".to_string());
                }
            }
        } else {
            vm.output.push("Error: Type mismatch for emit".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for emit".to_string());
    }
    None
}

#[cfg(feature = "nova")]
fn exec_smell(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // Stack: [ ... ] -> [ ..., dy, dx, intensity, signature ]
    // Finds nearest/strongest scent
    let (cy, cx) = vm.context_loc;
    let pos_x = cx as f64 + 0.5;
    let pos_y = cy as f64 + 0.5;

    let mut best_scent: Option<&Scent> = None;
    let mut max_score = -1.0;

    for scent in &vm.pheromones {
        let dx = scent.x - pos_x;
        let dy = scent.y - pos_y;
        let dist_sq = dx * dx + dy * dy;

        // Inverse square law for smell intensity
        let perceived_intensity = scent.intensity / (1.0 + dist_sq);

        if perceived_intensity > 1.0 && perceived_intensity > max_score {
            max_score = perceived_intensity;
            best_scent = Some(scent);
        }
    }

    if let Some(scent) = best_scent {
        let dx = (scent.x - pos_x).round() as i64;
        let dy = (scent.y - pos_y).round() as i64;

        vm.stack.push(Value::Str(scent.signature.clone()));
        vm.stack.push(Value::Int(max_score as i64));
        vm.stack.push(Value::Int(dx));
        vm.stack.push(Value::Int(dy));
    } else {
        // Nothing found
        vm.stack.push(Value::Str("None".to_string()));
        vm.stack.push(Value::Int(0));
        vm.stack.push(Value::Int(0));
        vm.stack.push(Value::Int(0));
    }

    vm.energy = vm.energy.saturating_sub(1);
    None
}

#[cfg(feature = "nova")]
fn exec_track(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // Stack: [ ..., signature ] -> [ ..., dy, dx ]
    if let Some(val) = vm.stack.pop() {
        if let Value::Str(target_sig) = val {
            let (cy, cx) = vm.context_loc;
            let pos_x = cx as f64 + 0.5;
            let pos_y = cy as f64 + 0.5;

            // Gradient descent: Sum vectors weighted by intensity
            let mut sum_dx: f64 = 0.0;
            let mut sum_dy: f64 = 0.0;
            let mut total_weight: f64 = 0.0;

            for scent in &vm.pheromones {
                if scent.signature == target_sig {
                    let dx = scent.x - pos_x;
                    let dy = scent.y - pos_y;
                    let dist_sq = dx * dx + dy * dy;
                    let weight = scent.intensity / (0.1 + dist_sq);

                    sum_dx += dx * weight;
                    sum_dy += dy * weight;
                    total_weight += weight;
                }
            }

            if total_weight > 0.0 {
                // Normalize roughly
                let final_dx = (sum_dx / total_weight * 2.0).clamp(-1.0, 1.0).round() as i64;
                let final_dy = (sum_dy / total_weight * 2.0).clamp(-1.0, 1.0).round() as i64;

                vm.stack.push(Value::Int(final_dx));
                vm.stack.push(Value::Int(final_dy));
            } else {
                vm.stack.push(Value::Int(0));
                vm.stack.push(Value::Int(0));
            }
            vm.energy = vm.energy.saturating_sub(2);
        } else {
            vm.output.push("Error: Type mismatch for track".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for track".to_string());
    }
    None
}

#[cfg(feature = "nova")]
pub fn process_scents(vm: &mut ChimeraVM) {
    let decay_rate = 0.95; // 5% decay per tick
    let min_intensity = 1.0;

    // Move scents (Wind + Diffusion)
    // We need to iterate mutably.
    // Also, wind affects particles.

    let size = GRID_SIZE as f64;

    // Temporary buffer to avoid borrow checker issues if we needed random access,
    // but here we just iterate.

    vm.pheromones.retain_mut(|scent| {
        // Apply decay
        scent.intensity *= decay_rate;
        scent.age += 1;

        // Apply Brownian motion
        let mut rng = rand::thread_rng();
        scent.x += rng.gen_range(-0.1..0.1);
        scent.y += rng.gen_range(-0.1..0.1);

        // Apply Wind
        let gx = scent.x.clamp(0.0, size - 1.0) as usize;
        let gy = scent.y.clamp(0.0, size - 1.0) as usize;
        let (w_dy, w_dx) = vm.wind_grid[gy][gx];

        scent.x += (w_dx as f64) * 0.1;
        scent.y += (w_dy as f64) * 0.1;

        // Wrap coordinates (Torus topology assumption for simplicity, or clamp)
        // vm.topology handles normalization, but here we have floats.
        // Let's wrap manually for now or clamp.

        scent.x = scent.x.rem_euclid(size);
        scent.y = scent.y.rem_euclid(size);

        scent.intensity >= min_intensity
    });
}
