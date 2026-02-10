use super::{ChimeraVM, Value};
use crate::ast::Nucleotide;
use crate::opcode::OpCode;

const WIND_DECAY: f32 = 0.95;
const MOISTURE_DECAY: f32 = 0.98;
const MAX_MOISTURE: i64 = 1000;
const MAX_WIND: i8 = 10;
const TSUNAMI_MOISTURE_ADD: i64 = 50;
const AEOLUS_COST: i64 = 5;
const TSUNAMI_BASE_COST: i64 = 15;

type WindVector = (i8, i8);

fn calculate_decay(w_dy: i8, w_dx: i8, moisture: i64) -> (i8, i8, i64) {
    let decayed_dy = (w_dy as f32 * WIND_DECAY) as i8;
    let decayed_dx = (w_dx as f32 * WIND_DECAY) as i8;
    let decayed_moisture = (moisture as f32 * MOISTURE_DECAY) as i64;
    (decayed_dy, decayed_dx, decayed_moisture)
}

/// Applies active effects from Hydra components (Pumps, Fans).
pub fn process_hydra_components(vm: &mut ChimeraVM) {
    let size = super::GRID_SIZE;
    for y in 0..size {
        for x in 0..size {
            if let Value::Str(s) = &vm.grid[y][x] {
                match s.as_str() {
                    ">" => vm.wind_grid[y][x] = (0, 5),
                    "<" => vm.wind_grid[y][x] = (0, -5),
                    "^" => vm.wind_grid[y][x] = (-5, 0),
                    "v" => vm.wind_grid[y][x] = (5, 0),
                    "@" => {
                        // Pump: Generate pressure
                        vm.moisture_grid[y][x] =
                            vm.moisture_grid[y][x].saturating_add(50).min(MAX_MOISTURE);
                    }
                    "~" => {
                        // Drain: Remove pressure
                        vm.moisture_grid[y][x] = vm.moisture_grid[y][x].saturating_sub(50);
                    }
                    _ => {}
                }
            }
        }
    }
}

/// Checks sensors and triggers execution if pressure is high.
pub fn process_sensors(vm: &mut ChimeraVM) {
    let size = super::GRID_SIZE;
    let threshold = 100;

    // Collect triggers to execute (to avoid borrowing vm mutably during iteration)
    let mut triggers = Vec::new();

    for y in 0..size {
        for x in 0..size {
            if let Value::Str(s) = &vm.grid[y][x] {
                if s == "!" {
                    let pressure = vm.moisture_grid[y][x];
                    if pressure > threshold {
                        // Check neighbor for code
                        // Default: Right neighbor? Or all neighbors?
                        // Let's check East neighbor for now (0, 1)
                        if let Some((ny, nx)) = vm.normalize_coords(y as i64, x as i64 + 1) {
                            triggers.push((ny, nx));
                        }
                    }
                }
            }
        }
    }

    for (y, x) in triggers {
        let val = vm.grid[y][x].clone();
        match val {
            Value::Str(s) => {
                let old_loc = vm.context_loc;
                vm.context_loc = (y, x);
                if let Ok(op) = s.parse::<OpCode>() {
                    // Sensors cost energy to fire?
                    if vm.energy > 0 {
                        let _ = vm.execute_gene(op, &[]);
                    }
                }
                vm.context_loc = old_loc;
            }
            Value::Int(n) => {
                vm.stack.push(Value::Int(n));
            }
            _ => {}
        }
    }
}

/// Simulates fluid dynamics (Advection and Diffusion) for the Atmosphere.
///
/// Updates `wind_grid` and `moisture_grid`.
pub fn process_fluid(vm: &mut ChimeraVM) {
    let size = super::GRID_SIZE;
    let mut new_moisture = vec![vec![0i64; size]; size];
    let mut new_wind: Vec<Vec<WindVector>> = vec![vec![(0i8, 0i8); size]; size];

    // Helper to check for walls
    let is_solid = |vm: &ChimeraVM, y: usize, x: usize| -> bool {
        if let Value::Str(s) = &vm.grid[y][x] {
            s == "#" || s == "X" // Wall or Valve (Closed by default)
        } else {
            false
        }
    };

    // 1. Advection & Diffusion
    for y in 0..size {
        for x in 0..size {
            if is_solid(vm, y, x) {
                // Solids don't advect, but they might reflect?
                // For now, just skip processing them as sources.
                // But we must preserve their state if any?
                // Walls don't hold moisture/wind usually.
                continue;
            }

            let (w_dy, w_dx) = vm.wind_grid[y][x];
            let moisture = vm.moisture_grid[y][x];

            let (decayed_dy, decayed_dx, decayed_moisture) = calculate_decay(w_dy, w_dx, moisture);

            if decayed_moisture <= 0 && w_dy == 0 && w_dx == 0 {
                continue;
            }

            // Distribute moisture based on wind
            let target_y = y as i64 + (w_dy as i64);
            let target_x = x as i64 + (w_dx as i64);

            if let Some((ny, nx)) = vm.normalize_coords(target_y, target_x) {
                if is_solid(vm, ny, nx) {
                    // Hit a wall: Reflect wind
                    new_wind[y][x].0 -= decayed_dy;
                    new_wind[y][x].1 -= decayed_dx;
                    // Moisture stays here
                    new_moisture[y][x] = new_moisture[y][x].saturating_add(decayed_moisture);
                } else {
                    // Advection: Move wind and moisture to target
                    new_wind[ny][nx].0 += decayed_dy;
                    new_wind[ny][nx].1 += decayed_dx;

                    let moved = decayed_moisture / 2;
                    let stayed = decayed_moisture - moved;
                    new_moisture[ny][nx] = new_moisture[ny][nx].saturating_add(moved);
                    new_moisture[y][x] = new_moisture[y][x].saturating_add(stayed);
                }
            } else {
                // Hit boundary - lose moisture (absorb)
                new_moisture[y][x] += decayed_moisture / 2;
            }
        }
    }

    // Update Grids
    for y in 0..size {
        for x in 0..size {
            // If cell is solid, clear its fluid state (it blocks)
            if is_solid(vm, y, x) {
                vm.moisture_grid[y][x] = 0;
                vm.wind_grid[y][x] = (0, 0);
            } else {
                vm.moisture_grid[y][x] = new_moisture[y][x].min(MAX_MOISTURE);
                vm.wind_grid[y][x].0 = new_wind[y][x].0.clamp(-MAX_WIND, MAX_WIND);
                vm.wind_grid[y][x].1 = new_wind[y][x].1.clamp(-MAX_WIND, MAX_WIND);
            }
        }
    }
}

pub fn exec_aeolus(
    vm: &mut ChimeraVM,
    _op: OpCode,
    _args: &[Nucleotide],
) -> Option<(usize, usize)> {
    // Stack: [ ..., angle, strength ]
    if vm.stack.len() < 2 {
        vm.output
            .push("Error: Stack underflow for Aeolus".to_string());
        return None;
    }

    let str_val = vm.stack.pop().unwrap();
    let ang_val = vm.stack.pop().unwrap();

    let (Value::Int(ang), Value::Int(str)) = (ang_val, str_val) else {
        vm.output
            .push("Error: Type mismatch for Aeolus".to_string());
        return None;
    };

    let strength = str.clamp(0, MAX_WIND as i64) as i8;
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

    vm.energy = vm.energy.saturating_sub(AEOLUS_COST);
    vm.output
        .push(format!("AEOLUS: Wind set to {:?} at {},{}", vec, cx, cy));
    None
}

pub fn exec_storm(vm: &mut ChimeraVM, _op: OpCode, _args: &[Nucleotide]) -> Option<(usize, usize)> {
    // Stack: [ ..., intensity, radius ]
    if vm.stack.len() < 2 {
        vm.output
            .push("Error: Stack underflow for Storm".to_string());
        return None;
    }

    let rad_val = vm.stack.pop().unwrap();
    let int_val = vm.stack.pop().unwrap();

    let (Value::Int(int), Value::Int(rad)) = (int_val, rad_val) else {
        vm.output.push("Error: Type mismatch for Storm".to_string());
        return None;
    };

    if rad <= 0 || int <= 0 {
        return None;
    }

    let (cy, cx) = vm.context_loc;
    let coords = vm.get_circular_coords(cx as i64, cy as i64, rad);
    for (tx, ty) in coords {
        vm.moisture_grid[ty][tx] = vm.moisture_grid[ty][tx].saturating_add(int);
    }
    vm.energy = vm.energy.saturating_sub(int / 2 + rad);
    vm.output
        .push(format!("STORM: Rain intensity {} at {},{}", int, cx, cy));
    None
}

pub fn exec_tsunami(
    vm: &mut ChimeraVM,
    _op: OpCode,
    _args: &[Nucleotide],
) -> Option<(usize, usize)> {
    // Stack: [ ..., direction, power ]
    // Applies strong wind in a cone/line from current position
    if vm.stack.len() < 2 {
        vm.output
            .push("Error: Stack underflow for Tsunami".to_string());
        return None;
    }

    let pow_val = vm.stack.pop().unwrap();
    let dir_val = vm.stack.pop().unwrap();

    let (Value::Int(dir), Value::Int(pow)) = (dir_val, pow_val) else {
        vm.output
            .push("Error: Type mismatch for Tsunami".to_string());
        return None;
    };

    let power = pow.clamp(1, MAX_WIND as i64 * 2) as i8;
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
        if let Some((ny, nx)) = vm.normalize_coords(cy as i64 + dy * i, cx as i64 + dx * i) {
            vm.wind_grid[ny][nx] = (dy as i8 * power, dx as i8 * power);
            // Also push moisture
            vm.moisture_grid[ny][nx] =
                vm.moisture_grid[ny][nx].saturating_add(TSUNAMI_MOISTURE_ADD);
            affected += 1;
        }
    }

    vm.energy = vm.energy.saturating_sub(TSUNAMI_BASE_COST + power as i64);
    vm.output
        .push(format!("TSUNAMI: Wave affected {} cells", affected));
    None
}

pub fn exec_dry(vm: &mut ChimeraVM, _op: OpCode, _args: &[Nucleotide]) -> Option<(usize, usize)> {
    // Stack: [ ..., radius ]
    let Some(Value::Int(r)) = vm.stack.pop() else {
        vm.output.push("Error: Invalid arg for Dry".to_string());
        return None;
    };

    if r <= 0 {
        return None;
    }

    let (cy, cx) = vm.context_loc;
    let coords = vm.get_circular_coords(cx as i64, cy as i64, r);
    let mut removed = 0;
    for (tx, ty) in coords {
        removed += vm.moisture_grid[ty][tx];
        vm.moisture_grid[ty][tx] = 0;
    }
    vm.energy = vm.energy.saturating_sub(r * 2);
    vm.output.push(format!("DRY: Removed {} moisture", removed));
    None
}
