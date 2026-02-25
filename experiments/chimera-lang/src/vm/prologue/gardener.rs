use super::normalize_coords;
use crate::vm::Value;
use rand::Rng;
use std::collections::VecDeque;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub enum GardenerAction {
    Move(usize, usize),
    Plant(usize, usize, Value),
    Nurture(usize, usize),
    Harvest(usize, usize),
    Die,
    None,
}

/// Represents the state of a Gardener agent (♣).
///
/// Format: "♣:Energy:Mode:Inventory"
/// Example: "♣:100:0:1,2,3"
/// Mode: 0 = Wander/Plant, 1 = Tend/Harvest
#[derive(Debug, Clone)]
pub struct GardenerState {
    pub energy: i64,
    pub mode: usize,
    pub inventory: VecDeque<Value>,
}

impl GardenerState {
    pub fn new(energy: i64, mode: usize, inventory: VecDeque<Value>) -> Self {
        Self {
            energy,
            mode,
            inventory,
        }
    }

    pub fn default() -> Self {
        let mut inv = VecDeque::new();
        inv.push_back(Value::Int(1)); // Start with a seed
        Self {
            energy: 100,
            mode: 0,
            inventory: inv,
        }
    }

    pub fn to_value(&self) -> Value {
        Value::Str(self.to_string())
    }
}

impl fmt::Display for GardenerState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let inv_str: Vec<String> = self.inventory.iter().map(|v| v.to_string()).collect();
        write!(f, "♣:{}:{}:{}", self.energy, self.mode, inv_str.join(","))
    }
}

impl FromStr for GardenerState {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() >= 4 && parts[0] == "♣" {
            let energy = parts[1].parse().map_err(|_| ())?;
            let mode = parts[2].parse().map_err(|_| ())?;

            let mut inventory = VecDeque::new();
            if !parts[3].is_empty() {
                for item_str in parts[3].split(',') {
                    if let Ok(i) = item_str.parse::<i64>() {
                        inventory.push_back(Value::Int(i));
                    } else {
                        inventory.push_back(Value::Str(item_str.to_string()));
                    }
                }
            }

            Ok(Self {
                energy,
                mode,
                inventory,
            })
        } else {
            Err(())
        }
    }
}

pub fn process_gardener_logic(
    _vm: &mut crate::vm::ChimeraVM,
    agent: &super::PrologueAgent,
    grid_snapshot: &[Vec<Value>],
) -> Option<(super::PrologueAgent, Option<(usize, usize)>)> {
    let (y, x) = (agent.y, agent.x);

    // Parse State
    let mut state = if let Value::Str(s) = &agent.state {
        s.parse::<GardenerState>()
            .unwrap_or(GardenerState::default())
    } else {
        GardenerState::default()
    };

    // 1. Check Vitality
    if state.energy <= 0 {
        return None; // Die
    }
    state.energy -= 1; // Metabolic cost

    // 2. Check Environment
    // What are we standing on? (In snapshot, it's "♣", but we need to check underfoot if we were tracking it)
    // Actually, the grid_snapshot has "♣" at (y,x).
    // The Prologue system doesn't explicitly track underfoot for all agents like the Shuttle.
    // However, we can check neighbors to act.

    let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut rng = rand::thread_rng();

    // Decision Logic
    // If Mode 0 (Planting): Look for empty spot to plant.
    // If Mode 1 (Tending): Look for plants to water or harvest.

    // Randomly switch modes sometimes
    if rng.gen_bool(0.05) {
        state.mode = 1 - state.mode;
    }

    let mut action = GardenerAction::None;

    if state.mode == 0 && !state.inventory.is_empty() {
        // Planting Mode
        for (dy, dx) in neighbors {
            if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
                let cell = &grid_snapshot[ny][nx];
                if matches!(cell, Value::Int(0)) || matches!(cell, Value::Str(s) if s == ".") {
                    // Found Soil
                    if let Some(seed) = state.inventory.pop_front() {
                        action = GardenerAction::Plant(ny, nx, seed);
                        break;
                    }
                }
            }
        }
    } else {
        // Tending Mode (or empty inventory)
        for (dy, dx) in neighbors {
            if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
                let cell = &grid_snapshot[ny][nx];
                if let Value::Int(n) = cell {
                    if *n > 0 {
                        if *n >= 10 {
                            action = GardenerAction::Harvest(ny, nx);
                        } else {
                            action = GardenerAction::Nurture(ny, nx);
                        }
                        break;
                    }
                }
            }
        }
    }

    // If no action found, Move
    if action == GardenerAction::None {
        let mut valid_moves = Vec::new();
        for (dy, dx) in neighbors {
            if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
                // Avoid walking into other agents or walls
                match &grid_snapshot[ny][nx] {
                    Value::Int(0) => valid_moves.push((ny, nx)),
                    Value::Str(s) if s == "." => valid_moves.push((ny, nx)),
                    _ => {}
                }
            }
        }

        if !valid_moves.is_empty() {
            let idx = rng.gen_range(0..valid_moves.len());
            let (ny, nx) = valid_moves[idx];
            action = GardenerAction::Move(ny, nx);
        }
    }

    // Execute Action
    let mut updated_agent = agent.clone();
    let mut moved_target = None;

    match action {
        GardenerAction::Move(ny, nx) => {
            moved_target = Some((ny, nx));
        }
        GardenerAction::Plant(ny, nx, seed) => {
            // Write to VM grid (needs mutable access, handled by return value usually? No, caller handles move)
            // Wait, process_agents iterates and WE need to update the grid if it's a non-move action?
            // The `process_agents` function in `mod.rs` mainly handles movement.
            // But we can mutate the VM grid directly here since we have `&mut ChimeraVM`.
            _vm.grid[ny][nx] = seed;
            state.energy -= 5;
        }
        GardenerAction::Nurture(ny, nx) => {
            if let Value::Int(n) = _vm.grid[ny][nx] {
                _vm.grid[ny][nx] = Value::Int(n + 1);
                state.energy -= 2;
            }
        }
        GardenerAction::Harvest(ny, nx) => {
            if let Value::Int(n) = _vm.grid[ny][nx] {
                // Gain energy
                state.energy += n * 2;
                // Add seed back
                state.inventory.push_back(Value::Int(1)); // Simple seed
                                                          // Clear grid
                _vm.grid[ny][nx] = Value::Int(0);
            }
        }
        _ => {}
    }

    updated_agent.state = state.to_value();
    Some((updated_agent, moved_target))
}
