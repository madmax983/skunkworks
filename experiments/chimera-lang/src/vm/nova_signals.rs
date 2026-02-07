#[cfg(feature = "nova")]
use super::{ChimeraVM, Value, GRID_SIZE};
#[cfg(feature = "nova")]
use crate::opcode::OpCode;

#[cfg(feature = "nova")]
pub fn process_signals(vm: &mut ChimeraVM) {
    let size = GRID_SIZE;
    let mut next_signals = vec![vec![0u8; size]; size];
    let mut executions = Vec::new();

    // 1. Propagation Phase
    for y in 0..size {
        for x in 0..size {
            let signal = vm.signal_grid[y][x];
            if signal == 0 {
                continue;
            }

            let val = &vm.grid[y][x];
            match val {
                Value::Str(s) => {
                    match s.as_str() {
                        "*" => {
                            // Bang: Propagate to all neighbors
                            let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
                            for (dy, dx) in neighbors {
                                if let Some((ny, nx)) =
                                    vm.normalize_coords(y as i64 + dy, x as i64 + dx)
                                {
                                    // Saturating add to prevent overflow
                                    next_signals[ny][nx] =
                                        next_signals[ny][nx].saturating_add(signal);
                                }
                            }
                        }
                        ">" => propagate_directional(vm, y, x, 0, 1, signal, &mut next_signals),
                        "<" => propagate_directional(vm, y, x, 0, -1, signal, &mut next_signals),
                        "^" => propagate_directional(vm, y, x, -1, 0, signal, &mut next_signals),
                        "v" => propagate_directional(vm, y, x, 1, 0, signal, &mut next_signals),
                        "+" => {
                            // Wire: Propagate to all neighbors (Conductor)
                            // Similar to Bang but maybe distinct visually/mechanically?
                            // For now, same as Bang.
                            let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
                            for (dy, dx) in neighbors {
                                if let Some((ny, nx)) =
                                    vm.normalize_coords(y as i64 + dy, x as i64 + dx)
                                {
                                    next_signals[ny][nx] =
                                        next_signals[ny][nx].saturating_add(signal);
                                }
                            }
                        }
                        _ => {
                            // Potentially an OpCode to execute
                            if let Ok(op) = s.parse::<OpCode>() {
                                executions.push((op, signal));
                            }
                        }
                    }
                }
                Value::Int(_) => {
                    // Integers act as insulators or resistors?
                    // For now, they block signals (do nothing).
                }
                _ => {}
            }
        }
    }

    // 2. Update State
    vm.signal_grid = next_signals;

    // 3. Execution Phase
    for (op, signal_strength) in executions {
        // Ephemeral execution: Does not advance IP, just runs the OpCode.
        // We might want to push signal_strength to stack?
        // Or just run it.
        // Let's push signal strength if the op consumes arguments?
        // Simpler: Just run it. If it needs args, it takes from main stack.
        // This makes the grid a "macro keyboard" for the VM.

        // Optional: Push coordinates? No, keep it simple.

        // Safety check: Don't execute dangerous ops?
        // Already handled by VM resilience.

        vm.execute_gene_inner(op.clone(), &[]);
    }
}

#[cfg(feature = "nova")]
fn propagate_directional(
    vm: &ChimeraVM,
    y: usize,
    x: usize,
    dy: i64,
    dx: i64,
    signal: u8,
    next_signals: &mut Vec<Vec<u8>>,
) {
    if let Some((ny, nx)) = vm.normalize_coords(y as i64 + dy, x as i64 + dx) {
        next_signals[ny][nx] = next_signals[ny][nx].saturating_add(signal);
    }
}
