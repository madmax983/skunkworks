#![cfg(feature = "nova")]

use super::{ChimeraVM, Value};

pub fn exec_resonate(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // Stack: [ ..., amplitude, frequency ]
    if vm.stack.len() >= 2 {
        let amp_val = vm.stack.pop().unwrap();
        let freq_val = vm.stack.pop().unwrap();

        if let (Value::Int(freq_int), Value::Int(amp_int)) = (freq_val, amp_val) {
            let freq = freq_int as f32;
            let amp = amp_int as f32;
            let (cy, cx) = vm.context_loc;

            // Update local cell
            vm.resonance_grid[cy][cx] = (freq, amp);

            // Diffusion/Radiation to immediate neighbors (radius 1)
            let neighbors = vm.get_circular_coords(cx as i64, cy as i64, 1);
            for (nx, ny) in neighbors {
                if (nx, ny) != (cx, cy) {
                    // Simple interference model: Overwrite if incoming is significantly stronger
                    let (_curr_freq, curr_amp) = vm.resonance_grid[ny][nx];
                    // Decayed amplitude at neighbor
                    let spread_amp = amp * 0.5;

                    if curr_amp < spread_amp {
                        vm.resonance_grid[ny][nx] = (freq, spread_amp);
                    }
                }
            }

            // Energy Cost
            vm.energy = vm.energy.saturating_sub(5 + (amp_int / 10));
            vm.output.push(format!(
                "RESONATE: {}Hz @ {:.1} at {},{}",
                freq, amp, cx, cy
            ));
        } else {
            vm.output
                .push("Error: Type mismatch for resonate".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for resonate".to_string());
    }
    None
}

pub fn exec_sonic_claim(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // Stack: [ ..., target_frequency ]
    if let Some(val) = vm.stack.pop() {
        if let Value::Int(target_freq_int) = val {
            let target_freq = target_freq_int as f32;
            let (cy, cx) = vm.context_loc;
            let (local_freq, local_amp) = vm.resonance_grid[cy][cx];

            // Claim Logic:
            // 1. Frequency must match within tolerance (±5 Hz)
            // 2. Amplitude must be sufficient (> 10.0)
            if (local_freq - target_freq).abs() < 5.0 && local_amp > 10.0 {
                let owner = vm.ip.0;

                // Only claim if not already owned by self (saves energy/logs)
                if vm.sovereignty_grid[cy][cx] != Some(owner) {
                    vm.sovereignty_grid[cy][cx] = Some(owner);
                    vm.energy = vm.energy.saturating_sub(10);
                    vm.output.push(format!(
                        "SONIC_CLAIM: Strand {} claimed {},{}",
                        owner, cx, cy
                    ));
                }
            } else {
                // Feedback on why it failed can be useful for debugging strands
                if local_amp <= 10.0 {
                    vm.output
                        .push(format!("SONIC_CLAIM: Signal too weak ({:.1})", local_amp));
                } else {
                    vm.output.push(format!(
                        "SONIC_CLAIM: Frequency mismatch ({} vs {})",
                        local_freq, target_freq
                    ));
                }
            }
        } else {
            vm.output
                .push("Error: Type mismatch for sonic_claim".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for sonic_claim".to_string());
    }
    None
}

pub fn exec_dampen(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // Stack: [ ..., amount, radius ]
    if vm.stack.len() >= 2 {
        let amount_val = vm.stack.pop().unwrap();
        let radius_val = vm.stack.pop().unwrap();

        if let (Value::Int(radius), Value::Int(amount)) = (radius_val, amount_val) {
            if radius > 0 {
                let (cy, cx) = vm.context_loc;
                let coords = vm.get_circular_coords(cx as i64, cy as i64, radius as i64);
                let amp_reduction = amount as f32;

                let mut count = 0;
                for (nx, ny) in coords {
                    let (freq, amp) = vm.resonance_grid[ny][nx];
                    if amp > 0.0 {
                        let new_amp = (amp - amp_reduction).max(0.0);
                        vm.resonance_grid[ny][nx] = (freq, new_amp);
                        count += 1;
                    }
                }

                vm.energy = vm.energy.saturating_sub(radius * 2);
                vm.output.push(format!(
                    "DAMPEN: Reduced amplitude by {} in {} cells",
                    amount, count
                ));
            }
        } else {
            vm.output
                .push("Error: Type mismatch for dampen".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for dampen".to_string());
    }
    None
}

/// Simulates continuous wave propagation (diffusion) and decay.
/// Should be called once per VM tick if Resonance is enabled.
pub fn process_resonance(vm: &mut ChimeraVM) {
    let size = super::GRID_SIZE;
    let mut new_grid = vm.resonance_grid.clone();

    for y in 0..size {
        for x in 0..size {
            let (self_freq, self_amp) = vm.resonance_grid[y][x];

            if self_amp < 0.1 {
                new_grid[y][x] = (0.0, 0.0);
                continue;
            }

            // Neighbors (Von Neumann for efficiency or Moore?)
            // Let's use simple Von Neumann (4-neighbors)
            let mut neighbors_amp_sum = 0.0;
            let mut neighbors_weighted_freq = 0.0;
            let mut count = 0;

            for (dy, dx) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let ny = y as i64 + dy;
                let nx = x as i64 + dx;

                // Use normalize_coords for topology support
                if let Some((ny, nx)) = vm.normalize_coords(ny, nx) {
                    let (n_freq, n_amp) = vm.resonance_grid[ny][nx];
                    if n_amp > 0.1 {
                        neighbors_amp_sum += n_amp;
                        neighbors_weighted_freq += n_freq * n_amp;
                        count += 1;
                    }
                }
            }

            // Diffusion:
            // New Amp = Self * Retention + Neighbors * Intake
            // Simple averaging
            let retention = 0.6;
            let diffusion = 0.4;

            let avg_neighbor_amp = if count > 0 {
                neighbors_amp_sum / count as f32
            } else {
                0.0
            };

            let mut new_amp = self_amp * retention + avg_neighbor_amp * diffusion;

            // Frequency mixing: Weighted average based on amplitude
            let mut new_freq = self_freq;
            if count > 0 && (self_amp + neighbors_amp_sum) > 0.0 {
                let total_weighted_freq = (self_freq * self_amp) + neighbors_weighted_freq;
                let total_amp = self_amp + neighbors_amp_sum;
                new_freq = total_weighted_freq / total_amp;
            }

            // Global Decay
            new_amp *= 0.95;

            new_grid[y][x] = (new_freq, new_amp);
        }
    }

    vm.resonance_grid = new_grid;
}
