#[cfg(feature = "nova")]
use super::{ChimeraVM, Value, GRID_SIZE};
#[cfg(feature = "nova")]
use crate::opcode::OpCode;
#[cfg(feature = "nova")]
use rand::Rng;

#[cfg(feature = "nova")]
fn char_to_val(c: char) -> Option<i64> {
    match c {
        '0'..='9' => Some(c as i64 - '0' as i64),
        'a'..='z' => Some(c as i64 - 'a' as i64 + 10),
        'A'..='Z' => Some(c as i64 - 'A' as i64 + 10),
        _ => None,
    }
}

#[cfg(feature = "nova")]
fn val_to_char(v: i64) -> char {
    let v = v.rem_euclid(36);
    if v < 10 {
        ((v as u8) + b'0') as char
    } else {
        ((v as u8 - 10) + b'a') as char
    }
}

#[cfg(feature = "nova")]
fn peek(vm: &ChimeraVM, y: usize, x: usize, dy: i64, dx: i64) -> Option<i64> {
    if let Some((ny, nx)) = vm.normalize_coords(y as i64 + dy, x as i64 + dx) {
        match &vm.grid[ny][nx] {
            Value::Int(n) => Some(*n),
            Value::Str(s) => {
                if s.len() == 1 {
                    char_to_val(s.chars().next().unwrap())
                } else {
                    None
                }
            }
            _ => None,
        }
    } else {
        None
    }
}

#[cfg(feature = "nova")]
struct GridWrite {
    y: usize,
    x: usize,
    val: Value,
}

#[cfg(feature = "nova")]
pub fn process_signals(vm: &mut ChimeraVM) {
    let size = GRID_SIZE;
    let mut next_signals = vec![vec![0u8; size]; size];
    let mut grid_writes: Vec<GridWrite> = Vec::new();
    let mut executions = Vec::new();

    // 1. Scan Phase
    for y in 0..size {
        for x in 0..size {
            let signal = vm.signal_grid[y][x];

            let val = &vm.grid[y][x];
            let c = match val {
                Value::Str(s) if s.len() == 1 => s.chars().next().unwrap(),
                Value::Str(s) => {
                     match s.as_str() {
                        "*" | ">" | "<" | "^" | "v" | "+" => s.chars().next().unwrap(),
                        _ => '\0'
                    }
                },
                _ => '\0',
            };

            let is_uppercase = c.is_ascii_uppercase();
            let is_bang = c == '*';
            let active = signal > 0 || is_uppercase || is_bang;

            if !active {
                continue;
            }

            match c {
                '*' => {
                    let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
                    for (dy, dx) in neighbors {
                        if let Some((ny, nx)) =
                            vm.normalize_coords(y as i64 + dy, x as i64 + dx)
                        {
                            next_signals[ny][nx] = next_signals[ny][nx].saturating_add(1);
                        }
                    }
                }
                '>' => propagate_directional(vm, y, x, 0, 1, 1, &mut next_signals),
                '<' => propagate_directional(vm, y, x, 0, -1, 1, &mut next_signals),
                '^' => propagate_directional(vm, y, x, -1, 0, 1, &mut next_signals),
                'v' => propagate_directional(vm, y, x, 1, 0, 1, &mut next_signals),
                '+' => {
                    let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
                    for (dy, dx) in neighbors {
                        if let Some((ny, nx)) =
                            vm.normalize_coords(y as i64 + dy, x as i64 + dx)
                        {
                            next_signals[ny][nx] = next_signals[ny][nx].saturating_add(signal);
                        }
                    }
                }
                'N' => try_move(vm, y, x, -1, 0, &mut grid_writes, "N"),
                'S' => try_move(vm, y, x, 1, 0, &mut grid_writes, "S"),
                'E' => try_move(vm, y, x, 0, 1, &mut grid_writes, "E"),
                'W' => try_move(vm, y, x, 0, -1, &mut grid_writes, "W"),
                'A' | 'a' => binary_op(vm, y, x, &mut grid_writes, |a, b| a.wrapping_add(b)),
                'B' | 'b' => binary_op(vm, y, x, &mut grid_writes, |a, b| a.wrapping_sub(b)),
                'M' | 'm' => binary_op(vm, y, x, &mut grid_writes, |a, b| a.wrapping_mul(b)),
                'D' | 'd' => binary_op(vm, y, x, &mut grid_writes, |a, b| if b != 0 { a.wrapping_div(b) } else { 0 }),
                'I' | 'i' => {
                    if let Some(n) = peek(vm, y, x, -1, 0) {
                        let max = peek(vm, y, x, 0, 1).unwrap_or(35);
                        let res = if n >= max { 0 } else { n + 1 };
                         if let Some((sy, sx)) = vm.normalize_coords(y as i64 + 1, x as i64) {
                            grid_writes.push(GridWrite { y: sy, x: sx, val: Value::Str(val_to_char(res).to_string()) });
                        }
                    }
                }
                'R' | 'r' => {
                    let min = peek(vm, y, x, -1, 0).unwrap_or(0);
                    let max = peek(vm, y, x, 0, 1).unwrap_or(35);
                    let mut rng = rand::thread_rng();
                    let range_min = min.min(max);
                    let range_max = min.max(max);
                    let res = rng.gen_range(range_min..=range_max);
                    if let Some((sy, sx)) = vm.normalize_coords(y as i64 + 1, x as i64) {
                        grid_writes.push(GridWrite { y: sy, x: sx, val: Value::Str(val_to_char(res).to_string()) });
                    }
                }
                'C' | 'c' => {
                    let rate = peek(vm, y, x, 0, 1).unwrap_or(1).max(1);
                    let mod_val = peek(vm, y, x, -1, 0).unwrap_or(8).max(1);
                    let res = (vm.tick_counter as i64 / rate) % mod_val;
                    if let Some((sy, sx)) = vm.normalize_coords(y as i64 + 1, x as i64) {
                        grid_writes.push(GridWrite { y: sy, x: sx, val: Value::Str(val_to_char(res).to_string()) });
                    }
                }
                'X' | 'x' => {
                    // Write: North (val) -> (West (x), East (y))
                    if let (Some(val), Some(x_off), Some(y_off)) = (peek(vm, y, x, -1, 0), peek(vm, y, x, 0, -1), peek(vm, y, x, 0, 1)) {
                        if let Some((ty, tx)) = vm.normalize_coords(y as i64 + y_off, x as i64 + x_off) {
                             grid_writes.push(GridWrite { y: ty, x: tx, val: Value::Str(val_to_char(val).to_string()) });
                        }
                    }
                }
                'O' | 'o' => {
                    // Offset: Read (West (x), East (y)) -> South
                    if let (Some(x_off), Some(y_off)) = (peek(vm, y, x, 0, -1), peek(vm, y, x, 0, 1)) {
                        if let Some(val) = peek(vm, y, x, y_off, x_off) {
                             if let Some((sy, sx)) = vm.normalize_coords(y as i64 + 1, x as i64) {
                                grid_writes.push(GridWrite { y: sy, x: sx, val: Value::Str(val_to_char(val).to_string()) });
                            }
                        }
                    }
                }
                _ => {
                    if let Value::Str(s) = val {
                         if let Ok(op) = s.parse::<OpCode>() {
                             if signal > 0 {
                                executions.push((op, signal));
                             }
                        }
                    }
                }
            }
        }
    }

    // 2. Apply Writes
    for w in grid_writes {
        vm.grid[w.y][w.x] = w.val;
    }

    // 3. Update Signal State
    vm.signal_grid = next_signals;

    // 4. Execution Phase
    for (op, _signal_strength) in executions {
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

#[cfg(feature = "nova")]
fn binary_op<F>(vm: &ChimeraVM, y: usize, x: usize, grid_writes: &mut Vec<GridWrite>, op: F)
where F: Fn(i64, i64) -> i64 {
    if let (Some(n), Some(e)) = (peek(vm, y, x, -1, 0), peek(vm, y, x, 0, 1)) {
        let res = op(n, e);
        if let Some((sy, sx)) = vm.normalize_coords(y as i64 + 1, x as i64) {
            grid_writes.push(GridWrite { y: sy, x: sx, val: Value::Str(val_to_char(res).to_string()) });
        }
    }
}

#[cfg(feature = "nova")]
fn try_move(vm: &ChimeraVM, y: usize, x: usize, dy: i64, dx: i64, grid_writes: &mut Vec<GridWrite>, c: &str) {
    if let Some((ny, nx)) = vm.normalize_coords(y as i64 + dy, x as i64 + dx) {
        let target_val = &vm.grid[ny][nx];
        // Only move if target is empty (Int(0) or Str("."))
        let is_empty = match target_val {
            Value::Int(0) => true,
            Value::Str(s) => s == ".",
            _ => false,
        };

        if is_empty {
            grid_writes.push(GridWrite { y, x, val: Value::Int(0) });
            grid_writes.push(GridWrite { y: ny, x: nx, val: Value::Str(c.to_string()) });
        }
    }
}
