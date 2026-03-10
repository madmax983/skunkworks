use super::{ChimeraVM, Value, GRID_SIZE};
use crate::ast::Nucleotide;
use crate::opcode::OpCode;

/// Configures the logistics layer (Cartography Grid) for factory automation.
///
/// **OpCode:** `Logistics`
/// **Stack:** `[ ..., type, direction, y, x ]`
/// **Types:** 0=Clear, 1=Belt
/// **Direction:** 0=N, 1=E, 2=S, 3=W
pub fn exec_logistics(
    vm: &mut ChimeraVM,
    _op: OpCode,
    _args: &[Nucleotide],
) -> Option<(usize, usize)> {
    if vm.stack.len() >= 4 {
        let x_val = vm.stack.pop().unwrap();
        let y_val = vm.stack.pop().unwrap();
        let dir_val = vm.stack.pop().unwrap();
        let type_val = vm.stack.pop().unwrap();

        if let (Value::Int(x), Value::Int(y), Value::Int(dir), Value::Int(t)) =
            (x_val, y_val, dir_val, type_val)
        {
            if let Some((ny, nx)) = vm.normalize_coords(y, x) {
                match t {
                    0 => {
                        // Clear
                        vm.cartography_grid[ny][nx] = Value::Int(0);
                        vm.output.push(format!("LOGISTICS: Cleared {},{}", nx, ny));
                    }
                    1 => {
                        // Belt
                        let d_str = match dir.rem_euclid(4) {
                            0 => "N",
                            1 => "E",
                            2 => "S",
                            3 => "W",
                            _ => "N",
                        };
                        vm.cartography_grid[ny][nx] = Value::Str(format!("Belt:{}", d_str));
                        vm.output
                            .push(format!("LOGISTICS: Belt:{} at {},{}", d_str, nx, ny));
                    }
                    _ => {
                        vm.output.push("LOGISTICS: Unknown type".to_string());
                    }
                }
                vm.energy = vm.energy.saturating_sub(5);
            } else {
                vm.output
                    .push("Error: Coordinates out of bounds for logistics".to_string());
            }
        } else {
            vm.output
                .push("Error: Type mismatch for logistics".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for logistics".to_string());
    }
    None
}

/// Processes one tick of the logistics network.
///
/// Iterates over the `cartography_grid` and moves items on the main `grid`
/// if they are on a Belt.
pub fn process_logistics(vm: &mut ChimeraVM) {
    let mut moves: Vec<((usize, usize), (usize, usize))> = Vec::new();

    // 1. Identify moves
    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            // Check if there is an item to move
            if matches!(vm.grid[y][x], Value::Int(0)) {
                continue;
            }

            // Check map for Belt
            if let Value::Str(s) = &vm.cartography_grid[y][x] {
                if s.starts_with("Belt:") {
                    let dir_char = s.chars().nth(5).unwrap_or('N');
                    let (dy, dx) = match dir_char {
                        'N' => (-1, 0),
                        'E' => (0, 1),
                        'S' => (1, 0),
                        'W' => (0, -1),
                        _ => (0, 0),
                    };

                    if let Some((ny, nx)) = vm.normalize_coords(y as i64 + dy, x as i64 + dx) {
                        moves.push(((y, x), (ny, nx)));
                    }
                }
            }
        }
    }

    // 2. Execute moves
    // Simple conflict resolution: First come, first served.
    // Ideally we should double-buffer or sort, but for cellular automata chaos, this is fine.
    // To prevent "teleporting" through a chain in one tick, we need to be careful.
    // If we process (0,0) -> (0,1), then next loop (0,1) -> (0,2), item moves 2 steps.
    // To simulate simultaneous movement, we should use a buffer or move in reverse order?
    // Reverse order depends on direction.
    // Safest is to only move if target is empty in *current* grid, OR if target is also moving?
    // "Factory" logic usually allows full throughput.
    // Let's re-evaluate.
    // A Belt moves item from A to B.
    // Success condition: B is empty OR B is being moved to C.
    // This requires dependency resolution.
    // Simplification: Only move if B is empty (Value::Int(0)).
    // This causes "jamming" if the line is full, which is realistic.
    // But it prevents full throughput (item A moves to B, B moves to C in same tick).
    // To allow full throughput, we need 2-step process.

    // Step A: Determine intent.
    // We used to build intents here, but we simplified logic.
    // We just iterate moves directly.

    // We'll use a `processed` set to avoid moving the same item twice in one tick (no teleporting).
    let mut processed = vec![vec![false; GRID_SIZE]; GRID_SIZE];

    for ((sy, sx), (ty, tx)) in moves {
        if processed[sy][sx] {
            continue;
        } // Already moved (unlikely with this logic)

        let target_empty = matches!(vm.grid[ty][tx], Value::Int(0));

        if target_empty {
            vm.grid[ty][tx] = vm.grid[sy][sx].clone();
            vm.grid[sy][sx] = Value::Int(0);
            processed[ty][tx] = true; // Mark target as processed so it doesn't move again this tick
        }
    }
}
