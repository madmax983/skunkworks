use super::normalize_coords;
use crate::vm::Value;
use rand::Rng;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub enum CritterAction {
    Move(usize, usize),
    Attack(usize, usize),
    Build(char, usize, usize),
    Split,
    Mark,
    None,
}

/// Represents the state of a Critter agent.
///
/// Format: "C:Energy:Genes:IP:Direction"
/// Example: "C:100:NSEW:0:0"
#[derive(Debug, Clone)]
pub struct CritterState {
    pub energy: i64,
    pub genes: String,
    pub ip: usize,
    pub direction: usize, // 0=N, 1=E, 2=S, 3=W
}

impl CritterState {
    pub fn new(energy: i64, genes: String, ip: usize, direction: usize) -> Self {
        Self {
            energy,
            genes,
            ip,
            direction,
        }
    }

    pub fn default() -> Self {
        Self {
            energy: 100,
            genes: "R".to_string(), // Random walker
            ip: 0,
            direction: 0,
        }
    }

    pub fn to_value(&self) -> Value {
        Value::Str(self.to_string())
    }
}

impl fmt::Display for CritterState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "C:{}:{}:{}:{}",
            self.energy, self.genes, self.ip, self.direction
        )
    }
}

impl FromStr for CritterState {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(':').collect();
        // Support old format for backward compatibility
        // Old: C:100:R:0 (4 parts)
        // New: C:100:R:0:0 (5 parts)
        if parts.len() >= 4 && parts[0] == "C" {
            let energy = parts[1].parse().map_err(|_| ())?;
            let genes = parts[2].to_string();
            let ip = parts[3].parse().map_err(|_| ())?;
            let direction = if parts.len() >= 5 {
                parts[4].parse().unwrap_or(0)
            } else {
                0
            };
            Ok(Self {
                energy,
                genes,
                ip,
                direction,
            })
        } else {
            Err(())
        }
    }
}

pub fn process_critter_move(
    critter: &mut CritterState,
    y: usize,
    x: usize,
    grid_snapshot: &[Vec<Value>],
) -> CritterAction {
    // 1. Check Energy
    if critter.energy <= 0 {
        return CritterAction::None; // Should be dead, handled by caller
    }

    // 2. Execute Gene
    let gene_char = if !critter.genes.is_empty() {
        critter
            .genes
            .chars()
            .nth(critter.ip % critter.genes.len())
            .unwrap_or('R')
    } else {
        'R'
    };

    // Update IP for next tick
    critter.ip = (critter.ip + 1) % critter.genes.len().max(1);
    critter.energy -= 1; // Metabolic cost

    // 3. Determine action
    match gene_char {
        'F' => {
            // Forward
            let (dy, dx) = match critter.direction {
                0 => (-1, 0), // N
                1 => (0, 1),  // E
                2 => (1, 0),  // S
                3 => (0, -1), // W
                _ => (0, 0),
            };
            if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
                CritterAction::Move(ny, nx)
            } else {
                CritterAction::None
            }
        }
        'B' => {
            // Backward
            let (dy, dx) = match critter.direction {
                0 => (1, 0),  // S
                1 => (0, -1), // W
                2 => (-1, 0), // N
                3 => (0, 1),  // E
                _ => (0, 0),
            };
            if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
                CritterAction::Move(ny, nx)
            } else {
                CritterAction::None
            }
        }
        'L' => {
            // Turn Left
            critter.direction = (critter.direction + 3) % 4;
            CritterAction::None
        }
        'R' => {
            // Turn Right
            critter.direction = (critter.direction + 1) % 4;
            CritterAction::None
        }
        'A' => {
            // Attack Forward
            let (dy, dx) = match critter.direction {
                0 => (-1, 0),
                1 => (0, 1),
                2 => (1, 0),
                3 => (0, -1),
                _ => (0, 0),
            };
            if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
                CritterAction::Attack(ny, nx)
            } else {
                CritterAction::None
            }
        }
        'S' => CritterAction::Split,
        'M' => CritterAction::Mark,
        '+' | '*' | 'x' | '^' | 'v' => {
            // Build Action
            let char_to_build = match gene_char {
                '+' => '~',
                '*' => '!',
                'x' => '?',
                '^' => '&',
                'v' => '|',
                _ => '~',
            };

            // Build in front
            let (dy, dx) = match critter.direction {
                0 => (-1, 0),
                1 => (0, 1),
                2 => (1, 0),
                3 => (0, -1),
                _ => (0, 0),
            };
            if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
                CritterAction::Build(char_to_build, ny, nx)
            } else {
                CritterAction::None
            }
        }
        'i' => {
            // Sense (Input): Check if path ahead is clear (empty/0).
            // If clear, skip next gene (increment IP).
            // If blocked, proceed to next gene.
            let (dy, dx) = match critter.direction {
                0 => (-1, 0),
                1 => (0, 1),
                2 => (1, 0),
                3 => (0, -1),
                _ => (0, 0),
            };

            let is_clear = if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
                matches!(grid_snapshot[ny][nx], Value::Int(0))
            } else {
                false // Out of bounds is blocked
            };

            if is_clear {
                critter.ip = (critter.ip + 1) % critter.genes.len().max(1);
            }
            CritterAction::None
        }
        '?' => {
            // Random Action
            let mut rng = rand::thread_rng();
            if rng.gen_bool(0.5) {
                critter.direction = rng.gen_range(0..4);
                CritterAction::None
            } else {
                let (dy, dx) = match critter.direction {
                    0 => (-1, 0),
                    1 => (0, 1),
                    2 => (1, 0),
                    3 => (0, -1),
                    _ => (0, 0),
                };
                if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
                    CritterAction::Move(ny, nx)
                } else {
                    CritterAction::None
                }
            }
        }
        // Legacy Support
        'N' => {
            critter.direction = 0;
            if let Some((ny, nx)) = normalize_coords(y as i64 - 1, x as i64) {
                CritterAction::Move(ny, nx)
            } else {
                CritterAction::None
            }
        }
        'E' => {
            critter.direction = 1;
            if let Some((ny, nx)) = normalize_coords(y as i64, x as i64 + 1) {
                CritterAction::Move(ny, nx)
            } else {
                CritterAction::None
            }
        }
        'W' => {
            critter.direction = 3;
            if let Some((ny, nx)) = normalize_coords(y as i64, x as i64 - 1) {
                CritterAction::Move(ny, nx)
            } else {
                CritterAction::None
            }
        }
        // 'S' is now Split. South legacy support removed or mapped to 'v'?
        // Assuming 'S' gene meant South in old saves, this breaks them.
        // But such is evolution.
        _ => CritterAction::None,
    }
}

pub fn breed(parent1: &CritterState, parent2: &CritterState) -> CritterState {
    let mut rng = rand::thread_rng();

    // Crossover
    let split = rng.gen_range(0..parent1.genes.len().min(parent2.genes.len()).max(1));
    let mut child_genes = String::new();
    if !parent1.genes.is_empty() {
        child_genes.push_str(&parent1.genes[0..split]);
    }
    if split < parent2.genes.len() {
        child_genes.push_str(&parent2.genes[split..]);
    }

    if child_genes.is_empty() {
        child_genes = "R".to_string();
    }

    // Mutation
    if rng.gen_bool(0.1) {
        let idx = rng.gen_range(0..child_genes.len());
        let mutations = [
            'F', 'B', 'L', 'R', 'A', 'S', 'M', '?', '+', '*', 'x', '^', 'v', 'i',
        ];
        let new_char = mutations[rng.gen_range(0..mutations.len())];
        child_genes.replace_range(idx..idx + 1, &new_char.to_string());
    }

    CritterState {
        energy: 50, // Child starts with 50
        genes: child_genes,
        ip: 0,
        direction: rng.gen_range(0..4),
    }
}
