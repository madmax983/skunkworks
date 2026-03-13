use super::normalize_coords;
use crate::vm::Value;
use rand::Rng;
use std::collections::{HashMap, VecDeque};

#[allow(clippy::too_many_arguments)]
pub fn apply_io_runes(
    rune: &str,
    y: usize,
    x: usize,
    tick: u64,
    current_signals: &[Vec<Option<Value>>],
    next_signals: &mut [Vec<Option<Value>>],
    ether: &mut HashMap<i64, VecDeque<Value>>,
    registers: &mut HashMap<(usize, usize), Value>,
    orca_mode: bool,
) -> bool {
    let mut changes = false;
    let w_sig = if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
        current_signals[wy][wx].clone()
    } else {
        None
    };

    match rune {
        "L" => {
            if let Some(Value::Int(channel)) = w_sig {
                if next_signals[y][x].is_none() {
                    if let Some(queue) = ether.get_mut(&channel) {
                        if let Some(val) = queue.pop_front() {
                            next_signals[y][x] = Some(val);
                            changes = true;
                        }
                    }
                }
            }
        }
        "C" => {
            if let Some(Value::Int(m)) = w_sig {
                if m > 0 {
                    let val = tick % (m as u64);
                    if next_signals[y][x].is_none() {
                        next_signals[y][x] = Some(Value::Int(val as i64));
                        changes = true;
                    }
                }
            }
        }
        "N" => {
            if orca_mode {
                // North: South -> North
                if let Some((sy, sx)) = normalize_coords(y as i64 + 1, x as i64) {
                    if let Some(sig) = &current_signals[sy][sx] {
                        if let Some((ny, nx)) = normalize_coords(y as i64 - 1, x as i64) {
                            if next_signals[ny][nx].is_none() {
                                next_signals[ny][nx] = Some(sig.clone());
                                changes = true;
                            }
                        }
                    }
                }
            } else {
                // West -> North
                if let Some(sig) = w_sig {
                    if let Some((ny, nx)) = normalize_coords(y as i64 - 1, x as i64) {
                        if next_signals[ny][nx].is_none() {
                            next_signals[ny][nx] = Some(sig);
                            changes = true;
                        }
                    }
                }
            }
        }
        "S" => {
            if orca_mode {
                // South: North -> South
                if let Some((ny, nx)) = normalize_coords(y as i64 - 1, x as i64) {
                    if let Some(sig) = &current_signals[ny][nx] {
                        if let Some((sy, sx)) = normalize_coords(y as i64 + 1, x as i64) {
                            if next_signals[sy][sx].is_none() {
                                next_signals[sy][sx] = Some(sig.clone());
                                changes = true;
                            }
                        }
                    }
                }
            } else {
                // West -> South
                if let Some(sig) = w_sig {
                    if let Some((sy, sx)) = normalize_coords(y as i64 + 1, x as i64) {
                        if next_signals[sy][sx].is_none() {
                            next_signals[sy][sx] = Some(sig);
                            changes = true;
                        }
                    }
                }
            }
        }
        "E" => {
            // West -> East
            if let Some(sig) = w_sig {
                if let Some((ey, ex)) = normalize_coords(y as i64, x as i64 + 1) {
                    if next_signals[ey][ex].is_none() {
                        next_signals[ey][ex] = Some(sig);
                        changes = true;
                    }
                }
            }
        }
        "W" => {
            // East -> West
            if let Some((ey, ex)) = normalize_coords(y as i64, x as i64 + 1) {
                if let Some(sig) = &current_signals[ey][ex] {
                    if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
                        if next_signals[wy][wx].is_none() {
                            next_signals[wy][wx] = Some(sig.clone());
                            changes = true;
                        }
                    }
                }
            }
        }
        "K" => {
            let mut rng = rand::thread_rng();
            let val = Value::Int(rng.gen_range(0..100));
            let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
            for (dy, dx) in neighbors {
                if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
                    if next_signals[ny][nx].is_none() {
                        next_signals[ny][nx] = Some(val.clone());
                        changes = true;
                    }
                }
            }
        }
        "R" => {
            // Register
            // Write (West)
            if let Some(sig) = w_sig {
                registers.insert((y, x), sig);
            }
            // Read (North)
            if let Some((ny, nx)) = normalize_coords(y as i64 - 1, x as i64) {
                if current_signals[ny][nx].is_some() {
                    if let Some(val) = registers.get(&(y, x)) {
                        if let Some((sy, sx)) = normalize_coords(y as i64 + 1, x as i64) {
                            if next_signals[sy][sx].is_none() {
                                next_signals[sy][sx] = Some(val.clone());
                                changes = true;
                            }
                        }
                    }
                }
            }
        }
        "Z" => {
            use std::time::{SystemTime, UNIX_EPOCH};
            let start = SystemTime::now();
            let since_the_epoch = start
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards");
            let val = Value::Int((since_the_epoch.as_secs() % 100) as i64);
            let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
            for (dy, dx) in neighbors {
                if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
                    if next_signals[ny][nx].is_none() {
                        next_signals[ny][nx] = Some(val.clone());
                        changes = true;
                    }
                }
            }
        }
        _ => {}
    }
    changes
}
