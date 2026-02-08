#[cfg(feature = "nova")]
use super::{ChimeraVM, Value};

#[cfg(feature = "nova")]
pub fn exec_retroscope(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // stack: ticks, y, x (top)
    if vm.stack.len() >= 3 {
        let x_val = vm.stack.pop().unwrap();
        let y_val = vm.stack.pop().unwrap();
        let ticks_val = vm.stack.pop().unwrap();

        if let (Value::Int(x), Value::Int(y), Value::Int(t)) = (x_val, y_val, ticks_val) {
            if let Some((ny, nx)) = vm.normalize_coords(y, x) {
                if t >= 0 {
                    let ticks = t as usize;
                    if ticks < vm.grid_history.len() {
                        // Index from back: len - 1 is most recent (start of current tick)
                        let idx = vm
                            .grid_history
                            .len()
                            .saturating_sub(1)
                            .saturating_sub(ticks);
                        let val = vm.grid_history[idx][ny][nx].clone();
                        vm.stack.push(val);
                    } else {
                        // Too far back, return 0 (Void)
                        vm.stack.push(Value::Int(0));
                    }
                } else {
                    vm.output
                        .push("Error: Negative ticks for retroscope".to_string());
                }
            } else {
                vm.output
                    .push("Error: Coordinates out of bounds for retroscope".to_string());
            }
        } else {
            vm.output
                .push("Error: Type mismatch for retroscope".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for retroscope".to_string());
    }
    None
}

#[cfg(feature = "nova")]
#[allow(clippy::needless_range_loop)]
pub fn update_relativity(vm: &mut ChimeraVM) {
    // 1. Accrete Gravity from Energy Consumption
    // Each step adds a tiny bit of "mass" to the current location.
    // This creates a gravity well where the code is executing.
    let (cy, cx) = vm.context_loc;
    vm.gravity_grid[cy][cx] = vm.gravity_grid[cy][cx].saturating_add(5);

    // 2. Diffuse Gravity
    // Gravity spreads and lingers.
    let mut buffer = [[0i64; 16]; 16];
    for y in 0..16 {
        for x in 0..16 {
            // High inertia for gravity (it lingers)
            let inertia = 90; // 90% retention from self
            let mut sum = (vm.gravity_grid[y][x] as i128) * (inertia as i128);
            let mut count = inertia;

            let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
            for (dy, dx) in neighbors {
                // Gravity passes through everything, ignoring membranes
                if let Some((ny, nx)) = vm.normalize_coords(y as i64 + dy, x as i64 + dx) {
                    sum += vm.gravity_grid[ny][nx] as i128;
                    count += 1;
                }
            }

            // Decay slightly (Entropy) - 98% retention (very slow decay)
            buffer[y][x] = ((sum / count as i128) * 98 / 100) as i64;
        }
    }
    for y in 0..16 {
        for x in 0..16 {
            vm.gravity_grid[y][x] = buffer[y][x];
        }
    }

    // 3. Update Time Dilation (Time Grid)
    // Mass warps spacetime.
    // g < 50: Normal Time (1)
    // g >= 50: Event Horizon / Stasis (0)
    // This affects Organelles and TimeWarp logic.

    for y in 0..16 {
        for x in 0..16 {
            let g = vm.gravity_grid[y][x];
            let factor = if g > 100 {
                0 // Frozen
            } else {
                1 // Normal
            };
            vm.time_grid[y][x] = factor;
        }
    }
}
