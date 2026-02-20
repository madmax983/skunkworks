use super::normalize_coords;
use crate::vm::Value;

pub fn apply_biolum_runes(
    rune: &str,
    y: usize,
    x: usize,
    _current_signals: &[Vec<Option<Value>>],
    next_signals: &mut Vec<Vec<Option<Value>>>,
    light_grid: &[Vec<i64>],
) -> bool {
    let mut changes = false;
    // Λ (Lambda) - Light Sensor
    // Reads light intensity at (y,x) from vm.light_grid
    // Outputs value to Self
    if rune == "Λ" {
        let intensity = light_grid[y][x];
        if next_signals[y][x].is_none() {
            next_signals[y][x] = Some(Value::Int(intensity));
            changes = true;
        }
    }
    changes
}

pub fn apply_biolum_sinks(vm: &mut crate::vm::ChimeraVM, rune: &str, y: usize, x: usize) {
    match rune {
        "Φ" => {
            // Φ (Phi) - Photon Emitter
            // West: Intensity
            // North: Red
            // East: Green
            // South: Blue

            let w_sig = if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
                vm.prologue_state.signal_grid[wy][wx].clone()
            } else {
                None
            };

            if let Some(Value::Int(intensity)) = w_sig {
                if intensity > 0 {
                    let n_sig = if let Some((ny, nx)) = normalize_coords(y as i64 - 1, x as i64) {
                        vm.prologue_state.signal_grid[ny][nx].clone()
                    } else {
                        None
                    };

                    let e_sig = if let Some((ey, ex)) = normalize_coords(y as i64, x as i64 + 1) {
                        vm.prologue_state.signal_grid[ey][ex].clone()
                    } else {
                        None
                    };

                    let s_sig = if let Some((sy, sx)) = normalize_coords(y as i64 + 1, x as i64) {
                        vm.prologue_state.signal_grid[sy][sx].clone()
                    } else {
                        None
                    };

                    let r = if let Some(Value::Int(v)) = n_sig {
                        v.clamp(0, 255) as u8
                    } else {
                        255
                    };
                    let g = if let Some(Value::Int(v)) = e_sig {
                        v.clamp(0, 255) as u8
                    } else {
                        255
                    };
                    let b = if let Some(Value::Int(v)) = s_sig {
                        v.clamp(0, 255) as u8
                    } else {
                        255
                    };

                    let i = intensity.clamp(0, 255);

                    vm.light_grid[y][x] = vm.light_grid[y][x].saturating_add(i);
                    // Use overwrite for color source, simple additive model is hard with just u8 tuple
                    // Let's just set source color
                    vm.light_color_grid[y][x] = (r, g, b);

                    // Consume energy?
                    vm.energy = vm.energy.saturating_sub(1);

                    // Light up self signal
                    vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1));
                }
            }
        }
        "Ω" => {
            // Ω (Omega) - Light Absorber
            // West: Intensity to absorb
            let w_sig = if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
                vm.prologue_state.signal_grid[wy][wx].clone()
            } else {
                None
            };

            if let Some(Value::Int(intensity)) = w_sig {
                if intensity > 0 {
                    let i = intensity.clamp(0, 255);
                    vm.light_grid[y][x] = vm.light_grid[y][x].saturating_sub(i);
                    // Light up self signal
                    vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1));
                }
            }
        }
        _ => {}
    }
}
