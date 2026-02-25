use super::{normalize_coords, PrologueAgent};
use crate::vm::{ChimeraVM, Value};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WizardState {
    pub mana: i64,
    pub sanity: i64,
    pub experiments_performed: i64,
}

impl Default for WizardState {
    fn default() -> Self {
        Self {
            mana: 100,
            sanity: 100,
            experiments_performed: 0,
        }
    }
}

impl WizardState {
    pub fn to_value(&self) -> Value {
        Value::Str(format!(
            "🧙:{}:{}:{}",
            self.mana, self.sanity, self.experiments_performed
        ))
    }
}

impl FromStr for WizardState {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Expected format: "🧙:MANA:SANITY:EXPERIMENTS"
        // Check if it starts with the Wizard emoji or just parse generically if needed
        let s = s.trim();
        if s.starts_with("🧙") {
            let parts: Vec<&str> = s.split(':').collect();
            // Allow for varying lengths in case we add more fields later
            if parts.len() >= 4 {
                let mana = parts[1].parse().unwrap_or(100);
                let sanity = parts[2].parse().unwrap_or(100);
                let experiments = parts[3].parse().unwrap_or(0);
                return Ok(WizardState {
                    mana,
                    sanity,
                    experiments_performed: experiments,
                });
            }
        }
        Ok(WizardState::default())
    }
}

#[derive(Debug)]
pub enum WizardAction {
    Move(usize, usize),
    CastPolymorph(usize, usize),    // Target Y, X
    CastAlchemy(usize, usize),      // Target Y, X
    CastShortCircuit(usize, usize), // Target Y, X
    CastBabel(usize, usize),        // Target Y, X
    Rest,
}

pub fn process_wizard_logic(
    vm: &mut ChimeraVM,
    agent: &PrologueAgent,
    grid_snapshot: &[Vec<Value>],
) -> Option<(PrologueAgent, Option<(usize, usize)>)> {
    let mut updated_agent = agent.clone();
    let (y, x) = (agent.y, agent.x);

    // Parse state
    let mut wizard_state = if let Value::Str(s) = &agent.state {
        s.parse::<WizardState>().unwrap_or_default()
    } else {
        WizardState::default()
    };

    // Regenerate Mana
    if wizard_state.mana < 100 {
        wizard_state.mana += 2;
    }

    // Decide Action
    let action = decide_action(vm, &wizard_state, y, x, grid_snapshot);

    match action {
        WizardAction::Rest => {
            wizard_state.mana += 10;
            if wizard_state.mana > 100 {
                wizard_state.mana = 100;
            }
            wizard_state.sanity += 5;
            updated_agent.state = wizard_state.to_value();
            Some((updated_agent, None))
        }
        WizardAction::Move(ny, nx) => {
            // Check if blocked
            if !is_blocked(&grid_snapshot[ny][nx]) {
                updated_agent.state = wizard_state.to_value();
                Some((updated_agent, Some((ny, nx))))
            } else {
                // Blocked, stay put
                updated_agent.state = wizard_state.to_value();
                Some((updated_agent, None))
            }
        }
        WizardAction::CastPolymorph(ny, nx) => {
            if wizard_state.mana >= 20 {
                wizard_state.mana -= 20;
                wizard_state.experiments_performed += 1;
                cast_polymorph(vm, ny, nx);
            }
            updated_agent.state = wizard_state.to_value();
            Some((updated_agent, None))
        }
        WizardAction::CastAlchemy(ny, nx) => {
            if wizard_state.mana >= 15 {
                wizard_state.mana -= 15;
                wizard_state.experiments_performed += 1;
                cast_alchemy(vm, ny, nx);
            }
            updated_agent.state = wizard_state.to_value();
            Some((updated_agent, None))
        }
        WizardAction::CastShortCircuit(ny, nx) => {
            if wizard_state.mana >= 10 {
                wizard_state.mana -= 10;
                wizard_state.experiments_performed += 1;
                cast_short_circuit(vm, ny, nx);
            }
            updated_agent.state = wizard_state.to_value();
            Some((updated_agent, None))
        }
        WizardAction::CastBabel(ny, nx) => {
            if wizard_state.mana >= 25 {
                wizard_state.mana -= 25;
                wizard_state.experiments_performed += 1;
                cast_babel(vm, ny, nx);
            }
            updated_agent.state = wizard_state.to_value();
            Some((updated_agent, None))
        }
    }
}

fn decide_action(
    _vm: &mut ChimeraVM,
    state: &WizardState,
    y: usize,
    x: usize,
    grid: &[Vec<Value>],
) -> WizardAction {
    let mut rng = rand::thread_rng();

    // Low Mana -> Rest
    if state.mana < 10 {
        return WizardAction::Rest;
    }

    // Look for targets adjacent
    let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut potential_targets = Vec::new();

    for (dy, dx) in neighbors {
        if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
            let cell = &grid[ny][nx];
            if matches!(cell, Value::Int(0)) {
                continue;
            }
            potential_targets.push((ny, nx, cell.clone()));
        }
    }

    // 30% chance to experiment if targets exist
    if !potential_targets.is_empty() && rng.gen_bool(0.3) {
        let (ty, tx, val) = potential_targets[rng.gen_range(0..potential_targets.len())].clone();

        match val {
            Value::Str(s) => {
                if s == "@" || s == "K" || s == "H" || s == "C" {
                    return WizardAction::CastPolymorph(ty, tx);
                } else if s == "!" || s == "~" {
                    return WizardAction::CastShortCircuit(ty, tx);
                } else {
                    // Other strings -> Babel or Alchemy
                    if rng.gen_bool(0.5) {
                        return WizardAction::CastBabel(ty, tx);
                    } else {
                        return WizardAction::CastAlchemy(ty, tx);
                    }
                }
            }
            Value::Int(_) => {
                 return WizardAction::CastAlchemy(ty, tx);
            }
            _ => {}
        }
    }

    // Otherwise Move
    let mut valid_moves = Vec::new();
    for (dy, dx) in neighbors {
        if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
            if !is_blocked(&grid[ny][nx]) {
                valid_moves.push((ny, nx));
            }
        }
    }

    if !valid_moves.is_empty() {
        let (my, mx) = valid_moves[rng.gen_range(0..valid_moves.len())];
        WizardAction::Move(my, mx)
    } else {
        WizardAction::Rest
    }
}

fn is_blocked(val: &Value) -> bool {
    !matches!(val, Value::Int(0))
}

fn cast_polymorph(vm: &mut ChimeraVM, y: usize, x: usize) {
    let mut rng = rand::thread_rng();
    let types = ["@", "K", "H", "C", "♻", "Φ"];
    let new_type = types[rng.gen_range(0..types.len())];
    vm.grid[y][x] = Value::Str(new_type.to_string());
    vm.output.push(format!("🧙 POLYMORPH: Agent at {},{} turned into {}", x, y, new_type));
    // Note: This doesn't update the PrologueState registers immediately, but the next scan cycle will pick it up.
    // However, if we want to be clean, we should probably clear the old register.
    vm.prologue_state.registers.remove(&(y, x));
}

fn cast_alchemy(vm: &mut ChimeraVM, y: usize, x: usize) {
     let val = vm.grid[y][x].clone();
     match val {
         Value::Int(n) => {
             // Transmute Int -> String
             vm.grid[y][x] = Value::Str(n.to_string());
         }
         Value::Str(s) => {
             // Transmute String -> Int (Length)
             vm.grid[y][x] = Value::Int(s.len() as i64);
         }
         _ => {}
     }
     vm.output.push(format!("🧙 ALCHEMY: Transmuted cell at {},{}", x, y));
}

fn cast_short_circuit(vm: &mut ChimeraVM, y: usize, x: usize) {
    let mut rng = rand::thread_rng();
    if rng.gen_bool(0.5) {
        vm.grid[y][x] = Value::Str("~".to_string());
    } else {
        vm.grid[y][x] = Value::Str("!".to_string());
    }
     vm.output.push(format!("🧙 SHORT CIRCUIT: Fried cell at {},{}", x, y));
}

fn cast_babel(vm: &mut ChimeraVM, y: usize, x: usize) {
    if let Value::Str(mut s) = vm.grid[y][x].clone() {
         let mut rng = rand::thread_rng();
         let noise = ["bla", "glitch", "?", "#", "error"];
         s.push_str(noise[rng.gen_range(0..noise.len())]);
         vm.grid[y][x] = Value::Str(s);
         vm.output.push(format!("🧙 BABEL: Corrupted text at {},{}", x, y));
    }
}
