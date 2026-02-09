#![cfg(feature = "nova")]

use super::{ChimeraVM, Value};
use crate::ast::Nucleotide;
use crate::opcode::OpCode;

/// Simulates fluid dynamics (Advection and Diffusion) for the Atmosphere.
///
/// Updates `wind_grid` and `moisture_grid`.
///
/// # Optimization
/// Uses stack-allocated arrays `[[T; GRID_SIZE]; GRID_SIZE]` for intermediate buffers
/// instead of heap-allocated `Vec<Vec<T>>` to avoid 34 allocations per tick.
pub fn process_fluid(vm: &mut ChimeraVM) {
    let size = super::GRID_SIZE;
    let mut new_moisture = [[0i64; super::GRID_SIZE]; super::GRID_SIZE];
    let mut new_wind = [[(0i8, 0i8); super::GRID_SIZE]; super::GRID_SIZE];

    // 1. Advection & Diffusion
    for y in 0..size {
        for x in 0..size {
            let (w_dy, w_dx) = vm.wind_grid[y][x];
            let moisture = vm.moisture_grid[y][x];

            // Decay wind
            let decayed_dy = (w_dy as f32 * 0.95) as i8;
            let decayed_dx = (w_dx as f32 * 0.95) as i8;

            // Add self to new_wind (Inertia)
            new_wind[y][x].0 += decayed_dy;
            new_wind[y][x].1 += decayed_dx;

            // Decay moisture
            let decayed_moisture = (moisture as f32 * 0.98) as i64;

            if decayed_moisture <= 0 && w_dy == 0 && w_dx == 0 {
                continue;
            }

            // Distribute moisture based on wind
            // Target: (y + dy, x + dx)
            // But we need to handle non-integer/fractional distribution for smoothness?
            // For discrete grid with i8 wind:
            // Just move a chunk to the target neighbor.

            let target_y = y as i64 + (w_dy as i64);
            let target_x = x as i64 + (w_dx as i64);

            if let Some((ny, nx)) = vm.normalize_coords(target_y, target_x) {
                // If wind is strong, move more moisture.
                // If wind is 0, diffusion happens (to all neighbors).

                if w_dy == 0 && w_dx == 0 {
                    // Still - Moisture diffuses slowly
                    new_moisture[y][x] += decayed_moisture;
                } else {
                    // Advection: Move moisture to target
                    // Some stays behind (drag)
                    let moved = decayed_moisture / 2;
                    let stayed = decayed_moisture - moved;
                    new_moisture[ny][nx] = new_moisture[ny][nx].saturating_add(moved);
                    new_moisture[y][x] = new_moisture[y][x].saturating_add(stayed);

                    // Wind also advects itself (momentum)
                    // Simplified: Wind at target gets a push from this wind
                    // But we simply decay/update in place for now to avoid explosion.
                }
            } else {
                // Hit wall/boundary - lose moisture/wind (absorb)
                // Or bounce? Let's just absorb for now.
                new_moisture[y][x] += decayed_moisture / 2;
            }
        }
    }

    // Update Grids
    // Clamp values
    for y in 0..size {
        for x in 0..size {
            vm.moisture_grid[y][x] = new_moisture[y][x].min(1000); // Cap moisture
            vm.wind_grid[y][x].0 = new_wind[y][x].0.clamp(-10, 10);
            vm.wind_grid[y][x].1 = new_wind[y][x].1.clamp(-10, 10);
        }
    }
}

pub fn exec_aeolus(
    vm: &mut ChimeraVM,
    _op: OpCode,
    _args: &[Nucleotide],
) -> Option<(usize, usize)> {
    // Stack: [ ..., angle, strength ]
    if vm.stack.len() >= 2 {
        let str_val = vm.stack.pop().unwrap();
        let ang_val = vm.stack.pop().unwrap();

        if let (Value::Int(ang), Value::Int(str)) = (ang_val, str_val) {
            let strength = str.clamp(0, 10) as i8;
            let (dy, dx) = match ang.rem_euclid(8) {
                0 => (-1, 0),  // N
                1 => (-1, 1),  // NE
                2 => (0, 1),   // E
                3 => (1, 1),   // SE
                4 => (1, 0),   // S
                5 => (1, -1),  // SW
                6 => (0, -1),  // W
                7 => (-1, -1), // NW
                _ => (0, 0),
            };

            let vec = (dy * strength, dx * strength);
            let (cy, cx) = vm.context_loc;
            vm.wind_grid[cy][cx] = vec;

            vm.energy = vm.energy.saturating_sub(5);
            vm.output
                .push(format!("AEOLUS: Wind set to {:?} at {},{}", vec, cx, cy));
        } else {
            vm.output
                .push("Error: Type mismatch for Aeolus".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for Aeolus".to_string());
    }
    None
}

pub fn exec_storm(vm: &mut ChimeraVM, _op: OpCode, _args: &[Nucleotide]) -> Option<(usize, usize)> {
    // Stack: [ ..., intensity, radius ]
    if vm.stack.len() >= 2 {
        let rad_val = vm.stack.pop().unwrap();
        let int_val = vm.stack.pop().unwrap();

        if let (Value::Int(int), Value::Int(rad)) = (int_val, rad_val) {
            if rad > 0 && int > 0 {
                let (cy, cx) = vm.context_loc;
                let coords = vm.get_circular_coords(cx as i64, cy as i64, rad);
                for (tx, ty) in coords {
                    vm.moisture_grid[ty][tx] = vm.moisture_grid[ty][tx].saturating_add(int);
                }
                vm.energy = vm.energy.saturating_sub(int / 2 + rad);
                vm.output
                    .push(format!("STORM: Rain intensity {} at {},{}", int, cx, cy));
            }
        } else {
            vm.output.push("Error: Type mismatch for Storm".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for Storm".to_string());
    }
    None
}

pub fn exec_tsunami(
    vm: &mut ChimeraVM,
    _op: OpCode,
    _args: &[Nucleotide],
) -> Option<(usize, usize)> {
    // Stack: [ ..., direction, power ]
    // Applies strong wind in a cone/line from current position
    if vm.stack.len() >= 2 {
        let pow_val = vm.stack.pop().unwrap();
        let dir_val = vm.stack.pop().unwrap();

        if let (Value::Int(dir), Value::Int(pow)) = (dir_val, pow_val) {
            let power = pow.clamp(1, 20) as i8;
            let (dy, dx) = match dir.rem_euclid(4) {
                0 => (0, 1),  // E
                1 => (1, 0),  // S
                2 => (0, -1), // W
                3 => (-1, 0), // N
                _ => (0, 0),
            };

            let (cy, cx) = vm.context_loc;
            let mut affected = 0;

            // Linear wave for 8 cells
            for i in 0..8 {
                if let Some((ny, nx)) = vm.normalize_coords(cy as i64 + dy * i, cx as i64 + dx * i)
                {
                    vm.wind_grid[ny][nx] = (dy as i8 * power, dx as i8 * power);
                    // Also push moisture
                    vm.moisture_grid[ny][nx] = vm.moisture_grid[ny][nx].saturating_add(50);
                    affected += 1;
                }
            }

            vm.energy = vm.energy.saturating_sub(15 + power as i64);
            vm.output
                .push(format!("TSUNAMI: Wave affected {} cells", affected));
        } else {
            vm.output
                .push("Error: Type mismatch for Tsunami".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for Tsunami".to_string());
    }
    None
}

pub fn exec_dry(vm: &mut ChimeraVM, _op: OpCode, _args: &[Nucleotide]) -> Option<(usize, usize)> {
    // Stack: [ ..., radius ]
    if let Some(Value::Int(r)) = vm.stack.pop() {
        if r > 0 {
            let (cy, cx) = vm.context_loc;
            let coords = vm.get_circular_coords(cx as i64, cy as i64, r);
            let mut removed = 0;
            for (tx, ty) in coords {
                removed += vm.moisture_grid[ty][tx];
                vm.moisture_grid[ty][tx] = 0;
            }
            vm.energy = vm.energy.saturating_sub(r * 2);
            vm.output.push(format!("DRY: Removed {} moisture", removed));
        }
    } else {
        vm.output.push("Error: Invalid arg for Dry".to_string());
    }
    None
}
