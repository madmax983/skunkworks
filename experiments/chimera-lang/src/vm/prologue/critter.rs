use crate::vm::{Value, GRID_SIZE};
use rand::Rng;
use super::normalize_coords;

/// Represents the state of a Critter agent.
///
/// Format: "C:Energy:Genes:IP:Dir"
/// Example: "C:100:FRL:0:0"
#[derive(Debug, Clone)]
pub struct CritterState {
    pub energy: i64,
    pub genes: String,
    pub ip: usize,
    pub dir: u8, // 0=N, 1=E, 2=S, 3=W
}

#[derive(Debug, Clone, PartialEq)]
pub enum CritterAction {
    Move(usize, usize),
    Eat(usize, usize),
    Attack(usize, usize),
    Split(usize, usize), // Target Y, Target X for child
    Mark(usize, usize),
    None,
}

impl CritterState {
    pub fn new(energy: i64, genes: String, ip: usize, dir: u8) -> Self {
        Self { energy, genes, ip, dir }
    }

    pub fn default() -> Self {
        Self {
            energy: 100,
            genes: "F?F".to_string(), // Forward, Random, Forward
            ip: 0,
            dir: 0,
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() >= 4 && parts[0] == "C" {
            let energy = parts[1].parse().ok()?;
            let genes = parts[2].to_string();
            let ip = parts[3].parse().ok()?;
            let dir = if parts.len() >= 5 {
                parts[4].parse().ok().unwrap_or(0)
            } else {
                0
            };
            Some(Self { energy, genes, ip, dir })
        } else {
            None
        }
    }

    pub fn to_string(&self) -> String {
        format!("C:{}:{}:{}:{}", self.energy, self.genes, self.ip, self.dir)
    }

    pub fn to_value(&self) -> Value {
        Value::Str(self.to_string())
    }
}

pub fn process_critter_tick(
    critter: &mut CritterState,
    y: usize,
    x: usize,
    grid_snapshot: &[Vec<Value>],
) -> CritterAction {
    // 1. Check Energy
    if critter.energy <= 0 {
        return CritterAction::None; // Dead
    }

    // 2. Execute Gene
    let gene_char = if !critter.genes.is_empty() {
        critter.genes.chars().nth(critter.ip % critter.genes.len()).unwrap_or('F')
    } else {
        'F'
    };

    // Update IP for next tick
    critter.ip = (critter.ip + 1) % critter.genes.len().max(1);
    critter.energy -= 1; // Metabolic cost

    // 3. Execute Action
    match gene_char {
        'F' => {
            // Forward
            let (dy, dx) = dir_to_delta(critter.dir);
            if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
                CritterAction::Move(ny, nx)
            } else {
                CritterAction::None // Wall
            }
        }
        'B' => {
            // Backward
            let (dy, dx) = dir_to_delta((critter.dir + 2) % 4);
            if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
                CritterAction::Move(ny, nx)
            } else {
                CritterAction::None
            }
        }
        'L' => {
            // Turn Left
            critter.dir = (critter.dir + 3) % 4;
            CritterAction::None
        }
        'R' => {
            // Turn Right
            critter.dir = (critter.dir + 1) % 4;
            CritterAction::None
        }
        'E' => {
            // Eat (Forward)
            let (dy, dx) = dir_to_delta(critter.dir);
            if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
                 CritterAction::Eat(ny, nx)
            } else {
                CritterAction::None
            }
        }
        'A' => {
            // Attack (Forward)
            let (dy, dx) = dir_to_delta(critter.dir);
            if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
                 CritterAction::Attack(ny, nx)
            } else {
                CritterAction::None
            }
        }
        'S' => {
            // Split (Backward - lay egg behind)
            if critter.energy > 50 {
                let (dy, dx) = dir_to_delta((critter.dir + 2) % 4);
                 if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
                    CritterAction::Split(ny, nx)
                } else {
                    CritterAction::None
                }
            } else {
                CritterAction::None
            }
        }
        'M' => {
            // Mark (Current Pos)
            CritterAction::Mark(y, x)
        }
        '?' => {
            // Random Move
            let mut rng = rand::thread_rng();
            let dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];
            let (dy, dx) = dirs[rng.gen_range(0..4)];
            if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
                CritterAction::Move(ny, nx)
            } else {
                CritterAction::None
            }
        }
        _ => CritterAction::None,
    }
}

fn dir_to_delta(dir: u8) -> (i64, i64) {
    match dir {
        0 => (-1, 0), // N
        1 => (0, 1),  // E
        2 => (1, 0),  // S
        3 => (0, -1), // W
        _ => (0, 0),
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
        child_genes = "F".to_string();
    }

    // Mutation
    if rng.gen_bool(0.1) {
        let idx = rng.gen_range(0..child_genes.len());
        let mutations = ['F', 'B', 'L', 'R', 'E', 'A', 'S', 'M', '?'];
        let new_char = mutations[rng.gen_range(0..mutations.len())];
        child_genes.replace_range(idx..idx+1, &new_char.to_string());
    }

    // Insert/Delete mutation
     if rng.gen_bool(0.05) {
        if rng.gen_bool(0.5) && child_genes.len() < 16 {
             // Insert
             let idx = rng.gen_range(0..child_genes.len()+1);
             let mutations = ['F', 'B', 'L', 'R', 'E', 'A', 'S', 'M', '?'];
             let new_char = mutations[rng.gen_range(0..mutations.len())];
             child_genes.insert(idx, new_char);
        } else if child_genes.len() > 1 {
            // Delete
            let idx = rng.gen_range(0..child_genes.len());
            child_genes.remove(idx);
        }
     }

    CritterState {
        energy: 50, // Child starts with 50
        genes: child_genes,
        ip: 0,
        dir: rng.gen_range(0..4),
    }
}
