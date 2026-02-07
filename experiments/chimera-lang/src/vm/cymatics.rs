#![cfg(feature = "cymatics")]
use super::{ChimeraVM, Value};
use crate::ast::Nucleotide;
use crate::opcode::OpCode;

pub fn exec_cymatics_op(vm: &mut ChimeraVM, op: OpCode, _args: &[Nucleotide]) {
    match op {
        OpCode::Cymatics => {
            vm.cymatics_mode = !vm.cymatics_mode;
            let status = if vm.cymatics_mode { "ON" } else { "OFF" };
            vm.output.push(format!("CYMATICS: Simulation {}", status));
        }
        OpCode::Strike => {
            if let Some(val) = vm.stack.pop() {
                if let Value::Int(s) = val {
                    let strength = (s as f32) / 10.0;
                    if let Some(grid) = &mut vm.cymatics_grid {
                        let (cy, cx) = vm.context_loc;
                        grid.pluck(cx, cy, strength);
                        vm.output.push(format!("STRIKE: {},{} str={:.1}", cx, cy, strength));
                    }
                    vm.energy = vm.energy.saturating_sub(2);
                } else {
                    vm.output.push("Error: Type mismatch for strike".to_string());
                }
            } else {
                vm.output.push("Error: Stack underflow for strike".to_string());
            }
        }
        OpCode::Tone => {
            // Stack: freq, strength
            if vm.stack.len() >= 2 {
                let s_val = vm.stack.pop().unwrap();
                let f_val = vm.stack.pop().unwrap();
                if let (Value::Int(s), Value::Int(f)) = (s_val, f_val) {
                    // For now, Tone acts as a driven oscillator for one tick
                    // Ideally this should set persistent state, but we lack fields.
                    // We simulate it by forcing the value at (x,y) to sin(t * freq)
                    // We don't have 't' easily accessible in VM step for phase.
                    // So we'll just treat it as a shaped Pluck for now.
                    let strength = (s as f32) / 10.0;
                    let _freq = f as f32;

                    if let Some(grid) = &mut vm.cymatics_grid {
                        let (cy, cx) = vm.context_loc;
                        // Just add energy based on freq?
                        // Higher freq = sharper pluck?
                        grid.pluck(cx, cy, strength);
                        vm.output.push(format!("TONE: {},{} (Single Cycle)", cx, cy));
                    }
                    vm.energy = vm.energy.saturating_sub(1);
                }
            } else {
                vm.output.push("Error: Stack underflow for tone".to_string());
            }
        }
        OpCode::Sift => {
            if let Some(val) = vm.stack.pop() {
                if let Value::Int(threshold) = val {
                    let th = (threshold as f32) / 100.0;
                    // Sift happens automatically in step() if enabled, but this OpCode might configure it?
                    // Or maybe this OpCode runs a sift step immediately.
                    // The plan said "Moves matter...".
                    // Let's make it run one step of sifting.
                    sift_matter_step(vm, th);
                    vm.output.push("SIFT: Matter rearranged".to_string());
                    vm.energy = vm.energy.saturating_sub(5);
                }
            }
        }
        OpCode::Reshape => {
            if let Some(val) = vm.stack.pop() {
                if let Value::Int(threshold) = val {
                    let th = (threshold as f32) / 10.0;
                    reshape_walls(vm, th);
                    vm.output.push("RESHAPE: Walls modified".to_string());
                    vm.energy = vm.energy.saturating_sub(10);
                }
            }
        }
        _ => {}
    }
}

pub fn sift_matter(vm: &mut ChimeraVM) {
    // Default threshold for auto-sift
    sift_matter_step(vm, 0.1);
}

fn sift_matter_step(vm: &mut ChimeraVM, _threshold: f32) {
    // We need to move items towards nodes (lower amplitude).
    // To avoid conflicts (swapping items), we can use a temporary buffer or move sequentially.
    // Sequential is risky.
    // Let's find all moves first.

    let rows = vm.grid.len();
    if rows == 0 { return; }
    let cols = vm.grid[0].len();

    let mut moves: Vec<((usize, usize), (usize, usize))> = Vec::new();

    if let Some(grid) = &vm.cymatics_grid {
        for y in 0..rows {
            for x in 0..cols {
                if matches!(vm.grid[y][x], Value::Int(0)) {
                    continue;
                }

                let curr_amp = grid.get(x, y).abs();
                // Find neighbor with lowest amplitude
                let mut best_amp = curr_amp;
                let mut best_pos = (y, x);

                let neighbors = [
                    (y.wrapping_sub(1), x),
                    (y + 1, x),
                    (y, x.wrapping_sub(1)),
                    (y, x + 1)
                ];

                for (ny, nx) in neighbors {
                    if ny < rows && nx < cols {
                        let amp = grid.get(nx, ny).abs();
                        if amp < best_amp {
                            best_amp = amp;
                            best_pos = (ny, nx);
                        }
                    }
                }

                if best_pos != (y, x) {
                    moves.push(((y, x), best_pos));
                }
            }
        }
    }

    // Apply moves if target is empty
    // Sort moves by amplitude difference? (Greedy)
    // For now simple.

    for (from, to) in moves {
        if matches!(vm.grid[to.0][to.1], Value::Int(0)) {
            let val = vm.grid[from.0][from.1].clone();
            vm.grid[to.0][to.1] = val;
            vm.grid[from.0][from.1] = Value::Int(0);
        }
    }
}

fn reshape_walls(vm: &mut ChimeraVM, threshold: f32) {
    #[cfg(feature = "nova")]
    if let Some(grid) = &vm.cymatics_grid {
        let rows = vm.membranes.len();
        let cols = if rows > 0 { vm.membranes[0].len() } else { 0 };

        for y in 0..rows {
            for x in 0..cols {
                let amp = grid.get(x, y).abs();
                if amp > threshold {
                    // Break walls
                    vm.membranes[y][x] = 0;
                }
            }
        }
    }
}
