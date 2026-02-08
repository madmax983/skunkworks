#![cfg(feature = "nova")]

use super::{ChimeraVM, Value, GRID_SIZE};
use crate::ast::Nucleotide;
use crate::opcode::OpCode;
use rand::Rng;

pub fn exec_geology_op(
    vm: &mut ChimeraVM,
    op: OpCode,
    _args: &[Nucleotide],
) -> Option<(usize, usize)> {
    match op {
        OpCode::Quake => exec_quake(vm),
        OpCode::Erode => exec_erode(vm),
        OpCode::Sediment => exec_sediment(vm),
        OpCode::Tectonics => exec_tectonics(vm),
        OpCode::Volcano => exec_volcano(vm),
        _ => None,
    }
}

fn exec_quake(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    let mut rng = rand::thread_rng();
    let intensity = if let Some(Value::Int(i)) = vm.stack.pop() {
        i.clamp(1, 10)
    } else {
        1
    };

    // Shift rows
    for _ in 0..intensity {
        // 50% chance to shift a row, 50% to shift a col
        if rng.gen_bool(0.5) {
            let row_idx = rng.gen_range(0..GRID_SIZE);
            let shift = rng.gen_range(-1..=1);
            if shift > 0 {
                vm.grid[row_idx].rotate_right(1);
            } else if shift < 0 {
                vm.grid[row_idx].rotate_left(1);
            }
        } else {
            let col_idx = rng.gen_range(0..GRID_SIZE);
            let shift = rng.gen_range(-1..=1);
            if shift != 0 {
                // Extract col, rotate, write back
                let mut col: Vec<Value> = (0..GRID_SIZE)
                    .map(|y| vm.grid[y][col_idx].clone())
                    .collect();
                if shift > 0 {
                    col.rotate_right(1);
                } else {
                    col.rotate_left(1);
                }
                for (y, val) in col.into_iter().enumerate() {
                    vm.grid[y][col_idx] = val;
                }
            }
        }
    }

    vm.energy = vm.energy.saturating_sub(intensity * 5);
    vm.output.push(format!("QUAKE: Magnitude {}", intensity));
    None
}

fn exec_erode(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if let Some(Value::Int(r)) = vm.stack.pop() {
        let (cy, cx) = vm.context_loc;
        let coords = vm.get_circular_coords(cx as i64, cy as i64, r);
        let mut count = 0;
        for (x, y) in coords {
            if let Value::Int(n) = &mut vm.grid[y][x] {
                if *n > 0 {
                    *n -= 1;
                    count += 1;
                }
            }
        }
        vm.energy = vm.energy.saturating_sub(count / 2 + 5);
        vm.output.push(format!("ERODE: Weathered {} cells", count));
    } else {
        vm.output.push("Error: Type mismatch for erode".to_string());
    }
    None
}

fn exec_sediment(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if let Some(Value::Int(r)) = vm.stack.pop() {
        let (cy, cx) = vm.context_loc;
        let coords = vm.get_circular_coords(cx as i64, cy as i64, r);
        let mut count = 0;
        for (x, y) in coords {
            if let Value::Int(n) = &mut vm.grid[y][x] {
                if *n < 100 {
                    *n += 1;
                    count += 1;
                }
            }
        }
        vm.energy = vm.energy.saturating_sub(count / 2 + 5);
        vm.output
            .push(format!("SEDIMENT: Deposited on {} cells", count));
    } else {
        vm.output
            .push("Error: Type mismatch for sediment".to_string());
    }
    None
}

fn exec_tectonics(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // stack: dy, dx, h, w (top)
    if vm.stack.len() >= 4 {
        let w_val = vm.stack.pop().unwrap();
        let h_val = vm.stack.pop().unwrap();
        let dx_val = vm.stack.pop().unwrap();
        let dy_val = vm.stack.pop().unwrap();

        if let (Value::Int(dy), Value::Int(dx), Value::Int(h), Value::Int(w)) =
            (dy_val, dx_val, h_val, w_val)
        {
            let (cy, cx) = vm.context_loc;

            // Define plate boundaries (centered on context)
            let start_y = (cy as i64).saturating_sub(h / 2);
            let end_y = start_y + h;
            let start_x = (cx as i64).saturating_sub(w / 2);
            let end_x = start_x + w;

            // Collect cells to move
            let mut moving_cells = Vec::new();
            for y in start_y..end_y {
                for x in start_x..end_x {
                    if let Some((ny, nx)) = vm.normalize_coords(y, x) {
                        moving_cells.push(((ny, nx), vm.grid[ny][nx].clone()));
                        // Clear old pos (will be overwritten if overlap, but we clear first to simulate "lift")
                        vm.grid[ny][nx] = Value::Int(0);
                    }
                }
            }

            // Place cells in new location
            for ((oy, ox), val) in moving_cells {
                // Apply delta
                // Note: we calculate new pos based on ORIGINAL coordinates, not normalized, to maintain relative structure
                // But since we stored (ny, nx), we must approximate.
                // Better: recalculate new pos from oy, ox with delta.
                if let Some((ty, tx)) = vm.normalize_coords(oy as i64 + dy, ox as i64 + dx) {
                    vm.grid[ty][tx] = val;
                }
            }

            vm.energy = vm.energy.saturating_sub(20);
            vm.output
                .push(format!("TECTONICS: Shifted plate by {},{}", dx, dy));
        } else {
            vm.output
                .push("Error: Type mismatch for tectonics".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for tectonics".to_string());
    }
    None
}

fn exec_volcano(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if let Some(Value::Int(power)) = vm.stack.pop() {
        let (cy, cx) = vm.context_loc;
        let p = power.clamp(1, 10);

        // Center erupts
        vm.grid[cy][cx] = Value::Int(99);

        // Spew lava
        let mut rng = rand::thread_rng();
        for _ in 0..(p * 2) {
            let dy = rng.gen_range(-2..=2);
            let dx = rng.gen_range(-2..=2);
            if let Some((ny, nx)) = vm.normalize_coords(cy as i64 + dy, cx as i64 + dx) {
                vm.grid[ny][nx] = Value::Int(50 + rng.gen_range(0..20));
            }
        }

        vm.energy = vm.energy.saturating_sub(p * 5);
        vm.output.push(format!("VOLCANO: Erupted at {},{}", cx, cy));
    } else {
        vm.output
            .push("Error: Type mismatch for volcano".to_string());
    }
    None
}
