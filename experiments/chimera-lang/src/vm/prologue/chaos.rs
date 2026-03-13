use super::normalize_coords;
use crate::vm::{ChimeraVM, Value};
use rand::Rng;

pub fn apply_chaos_runes(
    rune: &str,
    y: usize,
    x: usize,
    _current_signals: &[Vec<Option<Value>>],
    next_signals: &mut [Vec<Option<Value>>],
) -> bool {
    let mut changes = false;

    if rune == "k" {
        // Chaos Source: Emits random value to all neighbors
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
    changes
}

pub fn apply_chaos_sinks(vm: &mut ChimeraVM, rune: &str, y: usize, x: usize) {
    // Check signal from West
    let w_sig = if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
        vm.prologue_state.signal_grid[wy][wx].clone()
    } else {
        None
    };

    if w_sig.is_none() {
        return;
    }

    match rune {
        "z" => {
            // Glitch: Corrupts a random neighbor
            let mut rng = rand::thread_rng();
            let neighbors = [(-1, 0), (1, 0), (0, 1)]; // N, S, E
            let (dy, dx) = neighbors[rng.gen_range(0..neighbors.len())];

            if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
                // Randomly choose between Int and Char
                if rng.gen_bool(0.5) {
                    vm.grid[ny][nx] = Value::Int(rng.gen_range(0..100));
                } else {
                    let chars = ['!', '?', '@', '#', '$', '%', '&', '*', 'X', 'O', '0', '1'];
                    let c = chars[rng.gen_range(0..chars.len())];
                    vm.grid[ny][nx] = Value::Str(c.to_string());
                }
                // Light up self
                vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1));
            }
        }
        "h" => {
            // Havoc: Spawns a Chaos Agent (K) at a random empty location
            let mut rng = rand::thread_rng();
            // Try 10 times to find an empty spot
            for _ in 0..10 {
                let rx = rng.gen_range(0..crate::vm::GRID_SIZE);
                let ry = rng.gen_range(0..crate::vm::GRID_SIZE);

                if matches!(vm.grid[ry][rx], Value::Int(0) | Value::Str(_)) {
                    // Check if string is empty or "."
                    let is_empty = match &vm.grid[ry][rx] {
                        Value::Int(0) => true,
                        Value::Str(s) => s == "." || s == " " || s.is_empty(),
                        _ => false,
                    };

                    if is_empty {
                        vm.grid[ry][rx] = Value::Str("K".to_string());
                        vm.prologue_state.agents.push(super::PrologueAgent {
                            x: rx,
                            y: ry,
                            state: Value::Int(0),
                            stack: Vec::new(),
                        });
                        // Light up self
                        vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1));
                        break;
                    }
                }
            }
        }
        _ => {}
    }
}
