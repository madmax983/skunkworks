use super::normalize_coords;
use crate::vm::Value;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HyperState {
    /// Maps 2D Grid Coords (y, x) -> (z, w)
    pub extra_dims: HashMap<(usize, usize), (i64, i64)>,
    /// Current Projection Planes (0=XY, 1=XZ, 2=XW, 3=YZ, 4=YW, 5=ZW)
    pub projection_mode: u8,
    /// Rotation angle (0-360)
    pub rotation: f64,
}

impl Default for HyperState {
    fn default() -> Self {
        Self::new()
    }
}

impl HyperState {
    pub fn new() -> Self {
        Self {
            extra_dims: HashMap::new(),
            projection_mode: 0,
            rotation: 0.0,
        }
    }
}

pub fn apply_hyper_runes(
    rune: &str,
    y: usize,
    x: usize,
    current_signals: &[Vec<Option<Value>>],
    next_signals: &mut [Vec<Option<Value>>],
    hyper_state: &mut HyperState,
) -> bool {
    let mut changes = false;
    let w_sig = if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
        current_signals[wy][wx].clone()
    } else {
        None
    };

    match rune {
        "⇪" => {
            // Hyper-Step (Ascend): West (Int) -> Change W of South Neighbor
            // Only execute if not already active in this tick
            if next_signals[y][x].is_none() {
                if let Some(Value::Int(delta)) = w_sig {
                    if let Some((sy, sx)) = normalize_coords(y as i64 + 1, x as i64) {
                        let entry = hyper_state.extra_dims.entry((sy, sx)).or_insert((0, 0));
                        entry.1 += delta; // Modify W

                        next_signals[y][x] = Some(Value::Int(1));
                        changes = true;
                    }
                }
            }
        }
        "↻" => {
            // Rotate: West (Angle) -> Update Rotation
            if next_signals[y][x].is_none() {
                if let Some(Value::Int(angle)) = w_sig {
                    hyper_state.rotation = (hyper_state.rotation + angle as f64) % 360.0;
                    next_signals[y][x] = Some(Value::Int(1));
                    changes = true;
                }
            }
        }
        "⌖" => {
            // Project: West (Mode) -> Update Projection
            if next_signals[y][x].is_none() {
                if let Some(Value::Int(mode)) = w_sig {
                    hyper_state.projection_mode = (mode as u8) % 6;
                    next_signals[y][x] = Some(Value::Int(1));
                    changes = true;
                }
            }
        }
        "▣" => {
            // Tesseract: Source of 4D. Emits current W coord to Self.
            // If no W coord, emits 0.
            let w = if let Some((_z, w)) = hyper_state.extra_dims.get(&(y, x)) {
                *w
            } else {
                0
            };

            if next_signals[y][x] != Some(Value::Int(w)) {
                next_signals[y][x] = Some(Value::Int(w));
                changes = true;
            }
        }
        _ => {}
    }
    changes
}
