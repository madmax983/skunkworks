use super::normalize_coords;
use crate::vm::{ChimeraVM, Value};
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlitchState {
    pub energy: i64,
    pub mode: usize, // 0: XOR, 1: AND, 2: OR, 3: SHIFT, 4: INVERT
}

impl Default for GlitchState {
    fn default() -> Self {
        Self {
            energy: 100,
            mode: 0,
        }
    }
}

impl GlitchState {
    pub fn to_value(&self) -> Value {
        Value::Str(format!("👾:{}:{}", self.energy, self.mode))
    }

    pub fn from_value(v: &Value) -> Self {
        if let Value::Str(s) = v {
            if s.starts_with("👾") {
                let parts: Vec<&str> = s.split(':').collect();
                if parts.len() >= 3 {
                    let energy = parts[1].parse().unwrap_or(100);
                    let mode = parts[2].parse().unwrap_or(0);
                    return Self { energy, mode };
                }
            }
        }
        Self::default()
    }
}

pub fn process_glitch_agent(
    vm: &mut ChimeraVM,
    agent: &super::PrologueAgent,
    grid_snapshot: &[Vec<Value>],
) -> Option<(super::PrologueAgent, Option<(usize, usize)>)> {
    let (y, x) = (agent.y, agent.x);
    let mut state = GlitchState::from_value(&agent.state);

    if state.energy <= 0 {
        return None;
    }
    state.energy -= 1;

    // Movement: Random
    let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut rng = rand::thread_rng();
    let (dy, dx) = neighbors[rng.gen_range(0..4)];

    let target = normalize_coords(y as i64 + dy, x as i64 + dx);

    let mut moved_target = None;
    let mut updated_agent = agent.clone();

    if let Some((ny, nx)) = target {
        // Check if blocked by another agent or obstacle
        let dest_val = &grid_snapshot[ny][nx];
        let blocked = match dest_val {
            Value::Str(s) => s == "@" || s == "K" || s == "H" || s == "C" || s == "👾",
            _ => false,
        };

        if !blocked {
            moved_target = Some((ny, nx));

            // Apply Glitch Logic to the destination cell
            // We modify the LIVE grid directly
            let target_val = vm.grid[ny][nx].clone();
            if !matches!(target_val, Value::Int(0)) && !matches!(target_val, Value::Str(ref s) if s == ".") {
                 let new_val = apply_glitch_logic(&target_val, state.mode);
                 vm.grid[ny][nx] = new_val;
            } else {
                 // Leave a mark if empty
                 if rng.gen_bool(0.1) {
                     vm.grid[ny][nx] = Value::Str("░".to_string());
                 }
            }

            // Randomly change mode
            if rng.gen_bool(0.1) {
                state.mode = rng.gen_range(0..5);
            }
        }
    }

    updated_agent.state = state.to_value();
    Some((updated_agent, moved_target))
}

fn apply_glitch_logic(val: &Value, mode: usize) -> Value {
    match val {
        Value::Int(n) => {
            let mut rng = rand::thread_rng();
            match mode {
                0 => Value::Int(n ^ rng.gen_range(0..255)), // XOR
                1 => Value::Int(n & rng.gen_range(0..255)), // AND
                2 => Value::Int(n | rng.gen_range(0..255)), // OR
                3 => Value::Int(n << 1), // Shift Left
                4 => Value::Int(!n), // Invert
                _ => Value::Int(*n),
            }
        }
        Value::Str(s) => {
             // String corruption logic: Reverse or Upper/Lower swap
             let mut rng = rand::thread_rng();
             if rng.gen_bool(0.5) {
                 Value::Str(s.chars().rev().collect())
             } else {
                 // Basic swap case for first char
                 let mut chars: Vec<char> = s.chars().collect();
                 if let Some(c) = chars.first_mut() {
                     if c.is_uppercase() {
                         *c = c.to_ascii_lowercase();
                     } else {
                         *c = c.to_ascii_uppercase();
                     }
                 }
                 Value::Str(chars.into_iter().collect())
             }
        }
        _ => val.clone(),
    }
}

pub fn apply_glitch_runes(
    rune: &str,
    y: usize,
    x: usize,
    current_signals: &[Vec<Option<Value>>],
    next_signals: &mut Vec<Vec<Option<Value>>>,
) -> bool {
    let mut changes = false;
    // ≋ (Interference)
    // Reads West (Value). Applies Glitch (Bitwise NOT). Outputs East.
    if rune == "≋" {
        if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
            if let Some(val) = &current_signals[wy][wx] {
                let new_val = apply_glitch_logic(val, 4); // Mode 4 = Invert
                if let Some((ey, ex)) = normalize_coords(y as i64, x as i64 + 1) {
                    if next_signals[ey][ex].is_none() {
                        next_signals[ey][ex] = Some(new_val);
                        changes = true;
                    }
                }
            }
        }
    }
    changes
}
