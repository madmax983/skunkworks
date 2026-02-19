use crate::vm::Value;
use rand::Rng;
use super::normalize_coords;

/// Represents the state of a Critter agent.
///
/// Format: "C:Energy:Genes:IP"
/// Example: "C:100:NSEW:0"
#[derive(Debug, Clone)]
pub struct CritterState {
    pub energy: i64,
    pub genes: String,
    pub ip: usize,
}

impl CritterState {
    pub fn new(energy: i64, genes: String, ip: usize) -> Self {
        Self { energy, genes, ip }
    }

    pub fn default() -> Self {
        Self {
            energy: 100,
            genes: "R".to_string(), // Random walker
            ip: 0,
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() >= 4 && parts[0] == "C" {
            let energy = parts[1].parse().ok()?;
            let genes = parts[2].to_string();
            let ip = parts[3].parse().ok()?;
            Some(Self { energy, genes, ip })
        } else {
            None
        }
    }

    pub fn to_string(&self) -> String {
        format!("C:{}:{}:{}", self.energy, self.genes, self.ip)
    }

    pub fn to_value(&self) -> Value {
        Value::Str(self.to_string())
    }
}

pub fn process_critter_move(
    critter: &mut CritterState,
    y: usize,
    x: usize,
    _grid_snapshot: &[Vec<Value>],
) -> (usize, usize) {
    // 1. Check Energy
    if critter.energy <= 0 {
        return (y, x); // Should be dead, handled by caller
    }

    // 2. Execute Gene
    let gene_char = if !critter.genes.is_empty() {
        critter.genes.chars().nth(critter.ip % critter.genes.len()).unwrap_or('R')
    } else {
        'R'
    };

    // Update IP for next tick
    critter.ip = (critter.ip + 1) % critter.genes.len().max(1);
    critter.energy -= 1; // Metabolic cost

    // 3. Determine direction
    let (dy, dx) = match gene_char {
        'N' => (-1, 0),
        'S' => (1, 0),
        'W' => (0, -1),
        'E' => (0, 1),
        'R' => {
            let mut rng = rand::thread_rng();
            let dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];
            dirs[rng.gen_range(0..4)]
        }
        _ => (0, 0),
    };

    // 4. Calculate target
    let target = normalize_coords(y as i64 + dy, x as i64 + dx);

    if let Some((ny, nx)) = target {
        (ny, nx)
    } else {
        (y, x) // Hit wall
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
        let mutations = ['N', 'S', 'E', 'W', 'R'];
        let new_char = mutations[rng.gen_range(0..5)];
        child_genes.replace_range(idx..idx+1, &new_char.to_string());
    }

    CritterState {
        energy: 50, // Child starts with 50
        genes: child_genes,
        ip: 0,
    }
}
