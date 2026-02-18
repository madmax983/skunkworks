use super::normalize_coords;
use crate::vm::Value;
use std::collections::{HashMap, VecDeque};

pub fn apply_chronos_runes(
    rune: &str,
    y: usize,
    x: usize,
    current_signals: &[Vec<Option<Value>>],
    next_signals: &mut Vec<Vec<Option<Value>>>,
    history: &mut HashMap<(usize, usize), VecDeque<Value>>,
) -> bool {
    let mut changes = false;
    let w_sig = if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
        current_signals[wy][wx].clone()
    } else {
        None
    };

    // Important: Only execute state-modifying actions if we haven't already outputted to Self in this tick.
    // This prevents multiple executions during the propagation loop iterations.
    if next_signals[y][x].is_some() {
        return false;
    }

    match rune {
        "s" => {
            // Sporulate (Save): Input West -> Self History. Output West -> Self.
            if let Some(val) = w_sig {
                let queue = history.entry((y, x)).or_insert_with(VecDeque::new);
                queue.push_back(val.clone());
                // Limit history size to prevent memory explosion
                if queue.len() > 50 {
                    queue.pop_front();
                }

                next_signals[y][x] = Some(val);
                changes = true;
            }
        }
        "g" => {
            // Germinate (Get/Pop Front): Read West Neighbor's History.
            if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
                if let Some(queue) = history.get_mut(&(wy, wx)) {
                    if let Some(val) = queue.pop_front() {
                        next_signals[y][x] = Some(val);
                        changes = true;
                    }
                }
            }
        }
        "r" => {
            // Retrograde (Reverse/Pop Back): Read West Neighbor's History.
            if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
                if let Some(queue) = history.get_mut(&(wy, wx)) {
                    if let Some(val) = queue.pop_back() {
                        next_signals[y][x] = Some(val);
                        changes = true;
                    }
                }
            }
        }
        _ => {}
    }
    changes
}
