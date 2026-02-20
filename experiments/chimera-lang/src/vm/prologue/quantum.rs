use super::normalize_coords;
use crate::vm::{ChimeraVM, Value};
use rand::Rng;

fn val_to_idx(v: &Value) -> Option<usize> {
    match v {
        Value::Int(n) => Some(*n as usize),
        Value::Str(s) => s.parse::<usize>().ok(),
        _ => None,
    }
}

pub fn apply_quantum_runes(
    rune: &str,
    y: usize,
    x: usize,
    current_signals: &[Vec<Option<Value>>],
    next_signals: &mut Vec<Vec<Option<Value>>>,
) -> bool {
    let mut changes = false;
    let w_sig = if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
        current_signals[wy][wx].clone()
    } else {
        None
    };

    match rune {
        "q" => {
            // Quantum Source: West -> Self (Superposition)
            if let Some(val) = w_sig {
                if next_signals[y][x].is_none() {
                    let new_val = match val {
                        Value::Int(n) => {
                            // Create superposition of n and n+1
                            Value::Superposition(vec![
                                (Value::Int(n), 0.5),
                                (Value::Int(n + 1), 0.5),
                            ])
                        }
                        _ => val, // Pass through others
                    };
                    next_signals[y][x] = Some(new_val);
                    changes = true;
                }
            }
        }
        "m" => {
            // Measure: West -> Self (Collapsed)
            if let Some(val) = w_sig {
                if next_signals[y][x].is_none() {
                    let collapsed = match val {
                        Value::Superposition(states) => {
                            let mut rng = rand::thread_rng();
                            let r: f64 = rng.gen();
                            let mut cumulative = 0.0;
                            let mut selected = states
                                .last()
                                .map(|(v, _)| v.clone())
                                .unwrap_or(Value::Int(0));

                            for (v, p) in states {
                                cumulative += p;
                                if r <= cumulative {
                                    selected = v;
                                    break;
                                }
                            }
                            selected
                        }
                        _ => val, // Already measured
                    };
                    next_signals[y][x] = Some(collapsed);
                    changes = true;
                }
            }
        }
        _ => {}
    }
    changes
}

pub fn apply_quantum_sinks(vm: &mut ChimeraVM, rune: &str, y: usize, x: usize) {
    match rune {
        "8" => {
            // Entangle: West (Strand A) + East (Strand B) -> Entangle
            if let (Some((wy, wx)), Some((ey, ex))) = (
                normalize_coords(y as i64, x as i64 - 1),
                normalize_coords(y as i64, x as i64 + 1),
            ) {
                let w_sig = &vm.prologue_state.signal_grid[wy][wx];
                let e_sig = &vm.prologue_state.signal_grid[ey][ex];

                if let (Some(a), Some(b)) = (w_sig, e_sig) {
                    let idx_a_opt = val_to_idx(a);
                    let idx_b_opt = val_to_idx(b);

                    if let (Some(idx_a), Some(idx_b)) = (idx_a_opt, idx_b_opt) {
                        let len = vm.dna.helix.strands.len();
                        if idx_a < len && idx_b < len {
                            vm.entangled_pairs.insert(idx_a, idx_b);
                            vm.entangled_pairs.insert(idx_b, idx_a);
                            vm.output
                                .push(format!("PROLOGUE: Entangled {} <-> {}", idx_a, idx_b));
                            // Light up
                            vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1));
                        }
                    }
                }
            }
        }
        _ => {}
    }
}
