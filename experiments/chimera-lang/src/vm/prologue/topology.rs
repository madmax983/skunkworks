use super::normalize_coords;
use crate::vm::Value;

pub fn apply_topology_runes(
    rune: &str,
    y: usize,
    x: usize,
    tick: u64,
    current_signals: &[Vec<Option<Value>>],
    next_signals: &mut Vec<Vec<Option<Value>>>,
    next_delayed: &mut Vec<Vec<Option<Value>>>,
    orca_mode: bool,
) -> bool {
    let mut changes = false;
    match rune {
        "~" => {
            // Wires: OR of all neighbors
            let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
            for (dy, dx) in neighbors {
                if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
                    if let Some(sig) = &current_signals[ny][nx] {
                        if next_signals[y][x].is_none() {
                            next_signals[y][x] = Some(sig.clone());
                            changes = true;
                        }
                    }
                }
            }
        }
        "N" if orca_mode => {
            // Move North: Read South, Write Self
            if let Some((sy, sx)) = normalize_coords(y as i64 + 1, x as i64) {
                if let Some(sig) = &current_signals[sy][sx] {
                    if next_signals[y][x].is_none() {
                        next_signals[y][x] = Some(sig.clone());
                        changes = true;
                    }
                }
            }
        }
        "S" if orca_mode => {
            // Move South: Read North, Write Self
            if let Some((ny, nx)) = normalize_coords(y as i64 - 1, x as i64) {
                if let Some(sig) = &current_signals[ny][nx] {
                    if next_signals[y][x].is_none() {
                        next_signals[y][x] = Some(sig.clone());
                        changes = true;
                    }
                }
            }
        }
        "E" if orca_mode => {
            // Move East: Read West, Write Self
            if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
                if let Some(sig) = &current_signals[wy][wx] {
                    if next_signals[y][x].is_none() {
                        next_signals[y][x] = Some(sig.clone());
                        changes = true;
                    }
                }
            }
        }
        "W" if orca_mode => {
            // Move West: Read East, Write Self
            if let Some((ey, ex)) = normalize_coords(y as i64, x as i64 + 1) {
                if let Some(sig) = &current_signals[ey][ex] {
                    if next_signals[y][x].is_none() {
                        next_signals[y][x] = Some(sig.clone());
                        changes = true;
                    }
                }
            }
        }
        "C" if orca_mode => {
            // Clock: Output (Tick / Rate) % Mod
            // Read Rate (West), Mod (North)
            let w_sig = if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
                current_signals[wy][wx].clone()
            } else {
                None
            };
            let n_sig = if let Some((ny, nx)) = normalize_coords(y as i64 - 1, x as i64) {
                current_signals[ny][nx].clone()
            } else {
                None
            };

            let rate = if let Some(Value::Int(r)) = w_sig {
                if r > 0 {
                    r as u64
                } else {
                    1
                }
            } else {
                1
            };

            let modulo = if let Some(Value::Int(m)) = n_sig {
                if m > 0 {
                    m as u64
                } else {
                    8
                }
            } else {
                8
            };

            let val = (tick / rate) % modulo;

            if next_signals[y][x].is_none() {
                next_signals[y][x] = Some(Value::Int(val as i64));
                changes = true;
            }
        }
        "^" | "J" => {
            // Jump: Input West -> Output East (Skipping Self)
            if let (Some((wy, wx)), Some((ey, ex))) = (
                normalize_coords(y as i64, x as i64 - 1),
                normalize_coords(y as i64, x as i64 + 1),
            ) {
                if let Some(sig) = &current_signals[wy][wx] {
                    if next_signals[ey][ex].is_none() {
                        next_signals[ey][ex] = Some(sig.clone());
                        changes = true;
                    }
                }
            }
        }
        "*" => {
            if orca_mode {
                // Bang: Input Any -> Output All Neighbors
                let mut triggered = false;
                let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];

                // Check if triggered
                for (dy, dx) in neighbors {
                    if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
                        if current_signals[ny][nx].is_some() {
                            triggered = true;
                            break;
                        }
                    }
                }

                if triggered {
                    // Fire to all neighbors
                    for (dy, dx) in neighbors {
                        if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
                            if next_signals[ny][nx].is_none() {
                                next_signals[ny][nx] = Some(Value::Int(1));
                                changes = true;
                            }
                        }
                    }
                    // Also light up self? Usually Bangs flash.
                    if next_signals[y][x].is_none() {
                        next_signals[y][x] = Some(Value::Int(1));
                        changes = true;
                    }
                }
            } else {
                // Splitter: Input North -> Output Self
                if let Some((ny, nx)) = normalize_coords(y as i64 - 1, x as i64) {
                    if let Some(sig) = &current_signals[ny][nx] {
                        if next_signals[y][x].is_none() {
                            next_signals[y][x] = Some(sig.clone());
                            changes = true;
                        }
                    }
                }
            }
        }
        "#" => {
            // Delay: Input North -> Output to next_delayed
            if let Some((ny, nx)) = normalize_coords(y as i64 - 1, x as i64) {
                if let Some(sig) = &current_signals[ny][nx] {
                    if next_delayed[y][x].is_none() {
                        next_delayed[y][x] = Some(sig.clone());
                    }
                }
            }
        }
        _ => {}
    }
    changes
}
