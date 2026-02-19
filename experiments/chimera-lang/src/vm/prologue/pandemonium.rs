use super::normalize_coords;
use crate::vm::{ChimeraVM, Value, GRID_SIZE};
use rand::Rng;

pub fn apply_pandemonium_runes(
    rune: &str,
    y: usize,
    x: usize,
    current_signals: &[Vec<Option<Value>>],
    next_signals: &mut Vec<Vec<Option<Value>>>,
) -> bool {
    let mut changes = false;

    match rune {
        "¿" => {
            // Chaos Source: Emits Random Value to North
            if let Some((ny, nx)) = normalize_coords(y as i64 - 1, x as i64) {
                if next_signals[ny][nx].is_none() {
                    let mut rng = rand::thread_rng();
                    let val = rng.gen_range(0..100);
                    next_signals[ny][nx] = Some(Value::Int(val));
                    changes = true;
                }
            }
        }
        "≈" => {
            // Noise Wire: Propagates signal with chance of mutation
            // Checks all 4 neighbors for input (like ~)
            let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
            let mut input = None;

            for (dy, dx) in neighbors {
                if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
                    if let Some(sig) = &current_signals[ny][nx] {
                        input = Some(sig.clone());
                        break; // Take first signal found
                    }
                }
            }

            if let Some(sig) = input {
                // Apply potential noise
                let mut rng = rand::thread_rng();
                let output = if rng.gen_bool(0.1) {
                    // 10% chance of mutation
                    match sig {
                        Value::Int(n) => {
                            let noise = rng.gen_range(-5..=5);
                            Value::Int(n + noise)
                        }
                        Value::Str(s) => {
                            if !s.is_empty() {
                                let mut chars: Vec<char> = s.chars().collect();
                                // Mutate char slightly? or scramble?
                                // Let's just append a random char for chaos
                                chars.push(rng.gen_range(b'a'..=b'z') as char);
                                Value::Str(chars.into_iter().collect())
                            } else {
                                Value::Str("?".to_string())
                            }
                        }
                        _ => sig,
                    }
                } else {
                    sig
                };

                if next_signals[y][x].is_none() {
                    next_signals[y][x] = Some(output);
                    changes = true;
                }
            }
        }
        _ => {}
    }
    changes
}

pub fn apply_pandemonium_sinks(vm: &mut ChimeraVM, rune: &str, y: usize, x: usize) {
    if rune == "¡" {
        // Chaos Sink: Checks neighbors for signal. If active, triggers Glitch.
        let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        let mut triggered = false;

        for (dy, dx) in neighbors {
            if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
                if vm.prologue_state.signal_grid[ny][nx].is_some() {
                    triggered = true;
                    break;
                }
            }
        }

        if triggered {
            // Trigger Glitch
            let mut rng = rand::thread_rng();
            let effect = rng.gen_range(0..3);

            vm.output.push(format!("PANDEMONIUM: Glitch type {} triggered at {},{}", effect, x, y));
            vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1)); // Light up

            match effect {
                0 => {
                    // Scramble Row
                    let target_y = rng.gen_range(0..GRID_SIZE);
                    let mut row = vm.grid[target_y].clone();
                    // Shuffle row
                    use rand::seq::SliceRandom;
                    row.shuffle(&mut rng);
                    vm.grid[target_y] = row;
                    vm.output.push(format!("PANDEMONIUM: Scrambled Row {}", target_y));
                }
                1 => {
                    // Corrupt Cell
                    let target_y = rng.gen_range(0..GRID_SIZE);
                    let target_x = rng.gen_range(0..GRID_SIZE);
                    let runes = ["~", "*", "+", "-", "%", "&", "|", "^", "!", "?", "A", "S", "M", "D"];
                    let new_val = Value::Str(runes[rng.gen_range(0..runes.len())].to_string());
                    vm.grid[target_y][target_x] = new_val;
                    vm.output.push(format!("PANDEMONIUM: Corrupted Cell {},{}", target_x, target_y));
                }
                2 => {
                    // Echo (Spawn random signal nearby)
                    let target_y = rng.gen_range(0..GRID_SIZE);
                    let target_x = rng.gen_range(0..GRID_SIZE);
                    vm.prologue_state.signal_grid[target_y][target_x] = Some(Value::Int(rng.gen_range(0..100)));
                    vm.output.push(format!("PANDEMONIUM: Echo Signal at {},{}", target_x, target_y));
                }
                _ => {}
            }
        }
    }
}
