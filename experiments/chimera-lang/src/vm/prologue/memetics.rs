use super::normalize_coords;
use crate::vm::{ChimeraVM, Value, MAX_STRING_LEN};
use rand::Rng;

/// Applies Memetic Runes during the signal propagation phase.
///
/// Runes:
/// *   `ι` (Iota) - **Meme Source**: Reads West (Grid Value). Emits to Self (Signal).
/// *   `ε` (Epsilon) - **Evolve**: Reads West (Signal). Mutates 1 char. Emits to Self (Signal).
/// *   `φ` (Phi) - **Censor**: Reads West (Signal) and North (Filter). Output West to Self if NOT match.
pub fn apply_memetic_runes(
    rune: &str,
    y: usize,
    x: usize,
    current_signals: &[Vec<Option<Value>>],
    next_signals: &mut [Vec<Option<Value>>],
    grid: &[Vec<Value>],
) -> bool {
    let mut changes = false;

    // Helper to get neighbor signals
    let w_sig = if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
        current_signals[wy][wx].as_ref()
    } else {
        None
    };

    let n_sig = if let Some((ny, nx)) = normalize_coords(y as i64 - 1, x as i64) {
        current_signals[ny][nx].as_ref()
    } else {
        None
    };

    match rune {
        "ι" => {
            // Iota: Meme Source (Reads West GRID Value)
            if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
                let val = &grid[wy][wx];
                // Emit only if non-empty
                let is_empty = match val {
                    Value::Int(0) => true,
                    Value::Str(s) => s.is_empty(),
                    _ => false,
                };

                if !is_empty && next_signals[y][x] != Some(val.clone()) {
                    next_signals[y][x] = Some(val.clone());
                    changes = true;
                }
            }
        }
        "ε" => {
            // Epsilon: Evolve (Mutate)
            // Reads West (Signal). Mutates. Emits to Self (Signal).
            if let Some(val) = w_sig {
                let s = match val {
                    Value::Str(s) => s.clone(),
                    _ => format!("{}", val),
                };
                let mutated = mutate_string(&s);
                let new_val = Value::Str(mutated);

                if next_signals[y][x] != Some(new_val.clone()) {
                    next_signals[y][x] = Some(new_val);
                    changes = true;
                }
            }
        }
        "φ" => {
            // Phi: Censor
            // Reads West (Signal). If West contains North (Signal), block.
            if let Some(w_val) = w_sig {
                let allow = if let Some(n_val) = n_sig {
                    let w_str = match w_val {
                        Value::Str(s) => s.clone(),
                        _ => format!("{}", w_val),
                    };
                    let n_str = match n_val {
                        Value::Str(s) => s.clone(),
                        _ => format!("{}", n_val),
                    };
                    !w_str.contains(&n_str)
                } else {
                    true
                };

                if allow && next_signals[y][x] != Some(w_val.clone()) {
                    next_signals[y][x] = Some(w_val.clone());
                    changes = true;
                }
            }
        }
        _ => {}
    }

    changes
}

/// Applies Memetic Sinks (Grid Interactions).
///
/// Runes:
/// *   `σ` (Sigma) - **Spread**: Reads West (Signal). Writes to Grid neighbors (Radius 1).
/// *   `κ` (Kappa) - **Imitate**: Reads 8 neighbors (Grid). Copies longest string to Self (Grid).
pub fn apply_memetic_sinks(vm: &mut ChimeraVM, rune: &str, y: usize, x: usize) {
    match rune {
        "σ" => {
            // Spread: West (Signal) -> Grid Neighbors
            // Check signal at West
            if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
                if let Some(sig) = &vm.prologue_state.signal_grid[wy][wx] {
                    // Spread to 8 neighbors
                    for dy in -1..=1 {
                        for dx in -1..=1 {
                            if dy == 0 && dx == 0 {
                                continue;
                            }
                            if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
                                // Overwrite? Or only empty? Let's overwrite.
                                vm.grid[ny][nx] = sig.clone();
                            }
                        }
                    }
                    vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1)); // Light up
                }
            }
        }
        "κ" => {
            // Imitate: Neighbors (Grid) -> Self (Grid)
            let mut best_meme: Option<String> = None;

            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dy == 0 && dx == 0 {
                        continue;
                    }
                    if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
                        if let Value::Str(s) = &vm.grid[ny][nx] {
                            if best_meme
                                .as_ref()
                                .is_none_or(|current| s.len() > current.len())
                            {
                                best_meme = Some(s.clone());
                            }
                        }
                    }
                }
            }

            if let Some(meme) = best_meme {
                vm.grid[y][x] = Value::Str(meme);
                vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1));
            }
        }
        "µ" => {
            // Mu: Meme Spread (West Signal -> Neighbors)
            if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
                if let Some(sig) = &vm.prologue_state.signal_grid[wy][wx] {
                    let mut spread_count = 0;
                    for dy in -1..=1 {
                        for dx in -1..=1 {
                            if dy == 0 && dx == 0 {
                                continue;
                            }
                            if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
                                match &mut vm.grid[ny][nx] {
                                    Value::Str(_) => {
                                        // Infect String
                                        vm.grid[ny][nx] = sig.clone();
                                        spread_count += 1;
                                    }
                                    Value::Int(n) => {
                                        // Mutate Integer (Small deviation)
                                        let mut rng = rand::thread_rng();
                                        *n = n.wrapping_add(rng.gen_range(-5..=5));
                                        spread_count += 1;
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                    if spread_count > 0 {
                        vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1));
                    }
                }
            }
        }
        _ => {}
    }
}

fn mutate_string(s: &str) -> String {
    if s.len() >= MAX_STRING_LEN {
        return s.to_string();
    }
    let mut chars: Vec<char> = s.chars().collect();
    if chars.is_empty() {
        return "a".to_string();
    }
    let mut rng = rand::thread_rng();
    let mutation_type = rng.gen_range(0..3); // 0=Sub, 1=Ins, 2=Del

    match mutation_type {
        0 => {
            // Substitution
            let idx = rng.gen_range(0..chars.len());
            let new_char = rng.gen_range(32u8..127u8) as char;
            chars[idx] = new_char;
        }
        1 => {
            // Insertion
            let idx = rng.gen_range(0..=chars.len());
            let new_char = rng.gen_range(32u8..127u8) as char;
            chars.insert(idx, new_char);
        }
        2 => {
            // Deletion
            if !chars.is_empty() {
                let idx = rng.gen_range(0..chars.len());
                chars.remove(idx);
            }
        }
        _ => {}
    }
    chars.into_iter().collect()
}
