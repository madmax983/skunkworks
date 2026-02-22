use crate::vm::{ChimeraVM, Value, GRID_SIZE};
use crate::ast::Nucleotide;
use crate::opcode::OpCode;
use serde::{Deserialize, Serialize};
use rand::Rng;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Constraint {
    Adjacent(String, String),
    Distance(String, String, i32),
    Row(String, usize),
    Col(String, usize),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChromatinState {
    pub active: bool,
    pub constraints: Vec<Constraint>,
    pub temperature: f64,
}

impl Default for ChromatinState {
    fn default() -> Self {
        Self {
            active: false,
            constraints: Vec::new(),
            temperature: 100.0,
        }
    }
}

impl ChromatinState {
    pub fn new() -> Self {
        Self::default()
    }
}

pub struct ChromatinSolver;

impl ChromatinSolver {
    pub fn solve(state: &mut ChromatinState, grid: &mut Vec<Vec<Value>>) {
        if state.constraints.is_empty() {
            return;
        }

        let mut rng = rand::thread_rng();
        let max_steps = 100; // Limit iterations per tick

        for _ in 0..max_steps {
            // Pick two random cells
            let y1 = rng.gen_range(0..GRID_SIZE);
            let x1 = rng.gen_range(0..GRID_SIZE);
            let y2 = rng.gen_range(0..GRID_SIZE);
            let x2 = rng.gen_range(0..GRID_SIZE);

            if (y1, x1) == (y2, x2) {
                continue;
            }

            // Calculate Energy (Violations) before swap
            let e1 = Self::calculate_energy(&state.constraints, grid);

            // Swap
            let temp = grid[y1][x1].clone();
            grid[y1][x1] = grid[y2][x2].clone();
            grid[y2][x2] = temp;

            // Calculate Energy after swap
            let e2 = Self::calculate_energy(&state.constraints, grid);

            // Accept or Reject (Metropolis)
            // If e2 < e1 (improvement), accept.
            // If e2 >= e1, accept with probability exp(-(e2-e1)/T)
            let delta = (e2 as f64) - (e1 as f64);
            if delta < 0.0 {
                // Keep swap
            } else {
                let prob = (-delta / state.temperature).exp();
                if rng.gen::<f64>() < prob {
                    // Keep swap (Simulated Annealing)
                } else {
                    // Revert swap
                    let temp = grid[y1][x1].clone();
                    grid[y1][x1] = grid[y2][x2].clone();
                    grid[y2][x2] = temp;
                }
            }
        }

        // Cool down
        state.temperature *= 0.99;
        if state.temperature < 0.1 {
            state.temperature = 0.1;
        }
    }

    fn calculate_energy(constraints: &[Constraint], grid: &[Vec<Value>]) -> usize {
        let mut violations = 0;

        let mut positions: std::collections::HashMap<String, Vec<(usize, usize)>> = std::collections::HashMap::new();
        for y in 0..GRID_SIZE {
            for x in 0..GRID_SIZE {
                if let Value::Str(s) = &grid[y][x] {
                    positions.entry(s.clone()).or_default().push((y, x));
                }
            }
        }

        for c in constraints {
            match c {
                Constraint::Adjacent(a, b) => {
                    if let (Some(pos_a), Some(pos_b)) = (positions.get(a), positions.get(b)) {
                        let mut min_dist = usize::MAX;
                        for (ay, ax) in pos_a {
                            for (by, bx) in pos_b {
                                let dist = ((*ay as i64 - *by as i64).abs() + (*ax as i64 - *bx as i64).abs()) as usize;
                                if dist < min_dist {
                                    min_dist = dist;
                                }
                            }
                        }
                        if min_dist > 1 {
                            violations += min_dist;
                        }
                    } else {
                        violations += 100;
                    }
                }
                Constraint::Distance(a, b, d) => {
                     if let (Some(pos_a), Some(pos_b)) = (positions.get(a), positions.get(b)) {
                        let mut min_dist = usize::MAX;
                        for (ay, ax) in pos_a {
                            for (by, bx) in pos_b {
                                let dist = ((*ay as i64 - *by as i64).abs() + (*ax as i64 - *bx as i64).abs()) as usize;
                                if dist < min_dist {
                                    min_dist = dist;
                                }
                            }
                        }
                        let diff = (min_dist as i64 - *d as i64).abs() as usize;
                        violations += diff;
                    } else {
                        violations += 100;
                    }
                }
                Constraint::Row(a, row) => {
                    if let Some(pos_a) = positions.get(a) {
                        let mut min_diff = usize::MAX;
                        for (ay, _) in pos_a {
                            let diff = (*ay as i64 - *row as i64).abs() as usize;
                            if diff < min_diff {
                                min_diff = diff;
                            }
                        }
                        violations += min_diff;
                    } else {
                        violations += 100;
                    }
                }
                Constraint::Col(a, col) => {
                    if let Some(pos_a) = positions.get(a) {
                        let mut min_diff = usize::MAX;
                        for (_, ax) in pos_a {
                            let diff = (*ax as i64 - *col as i64).abs() as usize;
                            if diff < min_diff {
                                min_diff = diff;
                            }
                        }
                        violations += min_diff;
                    } else {
                        violations += 100;
                    }
                }
            }
        }
        violations
    }
}

pub fn apply_chromatin_op(vm: &mut ChimeraVM, _op: OpCode, _args: &[Nucleotide]) -> Option<(usize, usize)> {
    // Pop constraint string
    if let Some(val) = vm.stack.pop() {
        if let Value::Str(s) = val {
            parse_constraints(&s, &mut vm.prologue_state.chromatin_state);
            vm.prologue_state.chromatin_state.active = true;
            vm.prologue_state.chromatin_state.temperature = 100.0; // Reset temp
            vm.output.push("CHROMATIN: Constraints updated.".to_string());
        } else {
            vm.output.push("CHROMATIN Error: Expected String".to_string());
        }
    } else {
        // Toggle if no arg
        vm.prologue_state.chromatin_state.active = !vm.prologue_state.chromatin_state.active;
        let status = if vm.prologue_state.chromatin_state.active { "ON" } else { "OFF" };
        vm.output.push(format!("CHROMATIN: Solver {}", status));
    }
    None
}

fn parse_constraints(input: &str, state: &mut ChromatinState) {
    state.constraints.clear();
    let tokens: Vec<&str> = input.split_whitespace().collect();
    for token in tokens {
        if let Some(c) = parse_constraint(token) {
            state.constraints.push(c);
        }
    }
}

fn parse_constraint(token: &str) -> Option<Constraint> {
    if token.starts_with("adj(") && token.ends_with(')') {
        let content = &token[4..token.len()-1];
        let parts: Vec<&str> = content.split(',').collect();
        if parts.len() == 2 {
            return Some(Constraint::Adjacent(parts[0].to_string(), parts[1].to_string()));
        }
    }
    if token.starts_with("dist(") && token.ends_with(')') {
        let content = &token[5..token.len()-1];
        let parts: Vec<&str> = content.split(',').collect();
        if parts.len() == 3 {
            if let Ok(d) = parts[2].parse::<i32>() {
                return Some(Constraint::Distance(parts[0].to_string(), parts[1].to_string(), d));
            }
        }
    }
    if token.starts_with("row(") && token.ends_with(')') {
        let content = &token[4..token.len()-1];
        let parts: Vec<&str> = content.split(',').collect();
        if parts.len() == 2 {
            if let Ok(r) = parts[1].parse::<usize>() {
                return Some(Constraint::Row(parts[0].to_string(), r));
            }
        }
    }
    if token.starts_with("col(") && token.ends_with(')') {
        let content = &token[4..token.len()-1];
        let parts: Vec<&str> = content.split(',').collect();
        if parts.len() == 2 {
            if let Ok(c) = parts[1].parse::<usize>() {
                return Some(Constraint::Col(parts[0].to_string(), c));
            }
        }
    }
    None
}
