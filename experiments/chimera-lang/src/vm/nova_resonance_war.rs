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
            vm.output.push(format!("RESONATE: {}Hz @ {:.1} at {},{}", freq, amp, cx, cy));
        } else {
            vm.output.push("Error: Type mismatch for resonate".to_string());
        }
    } else {
        vm.output.push("Error: Stack underflow for resonate".to_string());
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
                    vm.output.push(format!("SONIC_CLAIM: Strand {} claimed {},{}", owner, cx, cy));
                }
            } else {
                // Feedback on why it failed can be useful for debugging strands
                if local_amp <= 10.0 {
                     vm.output.push(format!("SONIC_CLAIM: Signal too weak ({:.1})", local_amp));
                } else {
                     vm.output.push(format!("SONIC_CLAIM: Frequency mismatch ({} vs {})", local_freq, target_freq));
                }
            }
        } else {
            vm.output.push("Error: Type mismatch for sonic_claim".to_string());
        }
    } else {
        vm.output.push("Error: Stack underflow for sonic_claim".to_string());
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
                vm.output.push(format!("DAMPEN: Reduced amplitude by {} in {} cells", amount, count));
            }
        } else {
            vm.output.push("Error: Type mismatch for dampen".to_string());
        }
    } else {
        vm.output.push("Error: Stack underflow for dampen".to_string());
    }
    None
}
