use super::normalize_coords;
use crate::vm::{ChimeraVM, Value, GRID_SIZE};
use rand::Rng;
use std::fmt;
use std::str::FromStr;

/// Represents the state of an Alchemist Agent (`⚗`).
///
/// The Alchemist wanders the grid, gathering ingredients into its Crucible.
/// When the Crucible contains a valid recipe, it transmutes the ingredients into a result.
///
/// Format: "⚗:Mode:Direction:CrucibleLen:Item1,Item2,..."
/// Mode: 0=Gather, 1=Eject
#[derive(Debug, Clone)]
pub struct AlchemistState {
    pub mode: u8,
    pub direction: usize, // 0=N, 1=E, 2=S, 3=W
    pub crucible: Vec<Value>,
}

impl AlchemistState {
    pub fn new() -> Self {
        Self {
            mode: 0,
            direction: 1, // Default East
            crucible: Vec::new(),
        }
    }

    pub fn to_value(&self) -> Value {
        Value::Str(self.to_string())
    }
}

impl Default for AlchemistState {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for AlchemistState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // We need to serialize values. Simple values only for now.
        // Complex values (Lists) might be tricky, so we'll flatten or ignore for now.
        let items: Vec<String> = self.crucible.iter().map(|v| match v {
            Value::Int(n) => format!("I{}", n),
            Value::Str(s) => format!("S{}", s.replace(',', "\\,")), // Escape commas? Simple for now.
            _ => "U".to_string(), // Unknown/Unsupported
        }).collect();

        write!(f, "⚗:{}:{}:{}", self.mode, self.direction, items.join(","))
    }
}

impl FromStr for AlchemistState {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() >= 4 && parts[0] == "⚗" {
            let mode = parts[1].parse().unwrap_or(0);
            let direction = parts[2].parse().unwrap_or(1);

            let mut crucible = Vec::new();
            if parts.len() > 3 && !parts[3].is_empty() {
                for item_str in parts[3].split(',') {
                    if let Some(rest) = item_str.strip_prefix('I') {
                        if let Ok(n) = rest.parse::<i64>() {
                            crucible.push(Value::Int(n));
                        }
                    } else if let Some(rest) = item_str.strip_prefix('S') {
                        crucible.push(Value::Str(rest.replace("\\,", ",")));
                    }
                }
            }

            Ok(Self {
                mode,
                direction,
                crucible,
            })
        } else {
            Ok(Self::default())
        }
    }
}

/// Processes the logic for an Alchemist agent.
pub fn process_alchemist_logic(
    vm: &mut ChimeraVM,
    agent: &super::PrologueAgent,
    grid_snapshot: &[Vec<Value>],
) -> Option<(super::PrologueAgent, Option<(usize, usize)>)> {
    let (y, x) = (agent.y, agent.x);
    let mut state = if let Value::Str(s) = &agent.state {
        s.parse::<AlchemistState>().unwrap_or_default()
    } else {
        AlchemistState::default()
    };

    // 1. Check for Recipes (Transmutation)
    if let Some(result) = check_recipes(&state.crucible) {
        // Recipe found!
        // Eject Result behind (opposite to direction)
        let (dy, dx) = match state.direction {
            0 => (1, 0),  // S (Behind N)
            1 => (0, -1), // W (Behind E)
            2 => (-1, 0), // N (Behind S)
            3 => (0, 1),  // E (Behind W)
            _ => (0, 0),
        };

        if let Some((ty, tx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
            if matches!(vm.grid[ty][tx], Value::Int(0)) {
                vm.grid[ty][tx] = result;
                state.crucible.clear();
                vm.output.push(format!("ALCHEMIST: Transmuted recipe at {},{}", x, y));
            }
        }
    }

    // 2. Scan for Ingredients
    // Look ahead
    let (dy, dx) = match state.direction {
        0 => (-1, 0),
        1 => (0, 1),
        2 => (1, 0),
        3 => (0, -1),
        _ => (0, 0),
    };

    let next_pos = normalize_coords(y as i64 + dy, x as i64 + dx);

    let mut moved = false;
    let mut new_pos = None;

    if let Some((ny, nx)) = next_pos {
        let target_val = &grid_snapshot[ny][nx];
        match target_val {
            Value::Int(0) => {
                // Empty space, move there
                moved = true;
                new_pos = Some((ny, nx));
            }
            Value::Int(_) | Value::Str(_) => {
                // Ingredient! Gather it.
                // Only simple types for now.
                // Only gather if not a Rune (Runes are Strings usually of len 1, but Values can be too)
                // Let's assume anything that is a Value::Int or Value::Str is fair game unless it's a known Agent char?
                // Actually, let's just pick up Ints and Strings that are NOT single char Runes?
                // Or maybe specifically Values.

                // Let's just try to pick it up.
                // We verify it's not a Rune by checking if it's in the Runes set?
                // The grid snapshot contains Values.
                // Let's assume we pick up anything.

                if state.crucible.len() < 5 {
                     state.crucible.push(target_val.clone());
                     // Consume it (it becomes empty space for us to move into next tick)
                     // But we can't modify grid_snapshot.
                     // We modify vm.grid.
                     vm.grid[ny][nx] = Value::Int(0); // Consumed

                     // Move into the now empty spot?
                     // Yes.
                     moved = true;
                     new_pos = Some((ny, nx));
                     vm.output.push(format!("ALCHEMIST: Gathered {:?} at {},{}", target_val, nx, ny));
                } else {
                    // Crucible full. Turn or Eject?
                    // Turn randomly.
                    let mut rng = rand::thread_rng();
                    state.direction = rng.gen_range(0..4);
                }
            }
            _ => {
                // Obstacle (Wall, Agent, Complex Rune)
                // Turn randomly
                let mut rng = rand::thread_rng();
                state.direction = rng.gen_range(0..4);
            }
        }
    } else {
        // Wall (Bounds)
        // Bounce / Turn
        let mut rng = rand::thread_rng();
        state.direction = rng.gen_range(0..4);
    }

    let mut updated_agent = agent.clone();
    updated_agent.state = state.to_value();

    Some((updated_agent, new_pos))
}

fn check_recipes(crucible: &[Value]) -> Option<Value> {
    // 1. Sum: [Int(A), Int(B)] -> Int(A+B)
    if crucible.len() == 2 {
        if let (Value::Int(a), Value::Int(b)) = (&crucible[0], &crucible[1]) {
            return Some(Value::Int(a + b));
        }
        // Concat: [Str(A), Str(B)]
        if let (Value::Str(a), Value::Str(b)) = (&crucible[0], &crucible[1]) {
            return Some(Value::Str(format!("{}{}", a, b)));
        }
    }

    // 2. Repeat: [Str(S), Int(N)]
    if crucible.len() == 2 {
        if let (Value::Str(s), Value::Int(n)) = (&crucible[0], &crucible[1]) {
            return Some(Value::Str(s.repeat((*n).max(0) as usize)));
        }
         if let (Value::Int(n), Value::Str(s)) = (&crucible[0], &crucible[1]) {
            return Some(Value::Str(s.repeat((*n).max(0) as usize)));
        }
    }

    None
}
