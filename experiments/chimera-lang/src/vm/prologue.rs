use serde::{Deserialize, Serialize};
use std::collections::{HashSet, VecDeque};
use crate::vm::{Value, GRID_SIZE, ChimeraVM};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrologueAgent {
    pub x: usize,
    pub y: usize,
    pub state: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrologueState {
    pub active: bool,
    pub runes: HashSet<(usize, usize)>,
    pub rules: Vec<String>,
    pub signal_grid: Vec<Vec<Option<Value>>>,
    pub agents: Vec<PrologueAgent>,
}

impl PrologueState {
    pub fn new() -> Self {
        Self {
            active: false,
            runes: HashSet::new(),
            rules: Vec::new(),
            signal_grid: vec![vec![None; GRID_SIZE]; GRID_SIZE],
            agents: Vec::new(),
        }
    }

    pub fn scan_grid_rules(&mut self, grid: &Vec<Vec<Value>>) {
        self.runes.clear();
        self.rules.clear();
        self.agents.clear();

        for y in 0..GRID_SIZE {
            for x in 0..GRID_SIZE {
                if let Value::Str(s) = &grid[y][x] {
                    // Identify Runes
                    if matches!(s.as_str(), "?" | "!" | "~" | "&" | "|" | "@") {
                        self.runes.insert((y, x));

                        if s == "@" {
                            self.agents.push(PrologueAgent {
                                x, y, state: Value::Int(0)
                            });
                        }
                    }
                }
            }
        }
    }
}

pub fn exec_prologue_tick(vm: &mut ChimeraVM) {
    if !vm.prologue_state.active {
        return;
    }

    // 1. Scan Grid for Topology (Runes)
    let grid = vm.grid.clone(); // Clone for read access
    vm.prologue_state.scan_grid_rules(&grid);

    // 2. Clear Signals
    vm.prologue_state.signal_grid = vec![vec![None; GRID_SIZE]; GRID_SIZE];

    // 3. Source Emission (!)
    // '!' reads from North (y-1) and emits to self (and propagates)
    let runes: Vec<(usize, usize)> = vm.prologue_state.runes.iter().cloned().collect();

    for (y, x) in &runes {
        if let Value::Str(s) = &grid[*y][*x] {
            if s == "!" && *y > 0 {
                let val = grid[*y - 1][*x].clone();
                // Only emit truthy values? Or all values?
                // Let's emit non-empty signals.
                if !is_empty_val(&val) {
                    vm.prologue_state.signal_grid[*y][*x] = Some(val);
                }
            }
        }
    }

    // 4. Propagation (Wires ~ and Gates & |)
    // Simple iterative flood fill for wires
    // Gates need specific inputs.
    // Iteration loop to allow signal to travel across grid in one tick
    let max_iterations = GRID_SIZE * 2;
    for _ in 0..max_iterations {
        let mut changes = false;
        let mut next_signals = vm.prologue_state.signal_grid.clone();

        for (y, x) in &runes {
            if let Value::Str(s) = &grid[*y][*x] {
                match s.as_str() {
                    "~" => {
                        // Wires: OR of all neighbors
                        // Propagate signal FROM neighbors TO here
                        let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
                        for (dy, dx) in neighbors {
                            if let Some((ny, nx)) = normalize_coords(*y as i64 + dy, *x as i64 + dx) {
                                if let Some(sig) = &vm.prologue_state.signal_grid[ny][nx] {
                                    if next_signals[*y][*x].is_none() {
                                        next_signals[*y][*x] = Some(sig.clone());
                                        changes = true;
                                    }
                                }
                            }
                        }
                    },
                    "&" => {
                        // AND: West AND East -> Output Self (to be picked up by South wire/sink)
                        // Inputs:
                        if let (Some((wy, wx)), Some((ey, ex))) = (
                            normalize_coords(*y as i64, *x as i64 - 1),
                            normalize_coords(*y as i64, *x as i64 + 1)
                        ) {
                            let w_sig = &vm.prologue_state.signal_grid[wy][wx];
                            let e_sig = &vm.prologue_state.signal_grid[ey][ex];

                            if w_sig.is_some() && e_sig.is_some() {
                                // Output 1 (True)
                                if next_signals[*y][*x].is_none() {
                                    next_signals[*y][*x] = Some(Value::Int(1));
                                    changes = true;
                                }
                            }
                        }
                    },
                    "|" => {
                        // OR: West OR East -> Output Self
                        let mut active = false;
                        if let Some((wy, wx)) = normalize_coords(*y as i64, *x as i64 - 1) {
                            if vm.prologue_state.signal_grid[wy][wx].is_some() { active = true; }
                        }
                        if let Some((ey, ex)) = normalize_coords(*y as i64, *x as i64 + 1) {
                            if vm.prologue_state.signal_grid[ey][ex].is_some() { active = true; }
                        }

                        if active {
                            if next_signals[*y][*x].is_none() {
                                next_signals[*y][*x] = Some(Value::Int(1));
                                changes = true;
                            }
                        }
                    },
                    _ => {}
                }
            }
        }
        vm.prologue_state.signal_grid = next_signals;
        if !changes { break; }
    }

    // 5. Sink Consumption (?)
    // Sinks read from North (signal flow is usually N->S for gates, but wires are omni)
    // Let's say Sink reads from its own cell (if wire propagated to it) OR from North/West/East/South neighbors?
    // Let's stick to "Sinks read from the cell directly North of them" or "Sinks read signal AT their location"
    // Since wires propagate signal TO the wire cell, the sink should probably be connected to a wire.
    // If '?' is at (y,x), and '~' is at (y-1, x) with signal, does '?' get it?
    // In step 4, '~' pulls from neighbors. '?' isn't a wire.
    // So '?' must pull from neighbors.

    for (y, x) in &runes {
        if let Value::Str(s) = &grid[*y][*x] {
            if s == "?" {
                // Check neighbors for signal
                let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
                for (dy, dx) in neighbors {
                    if let Some((ny, nx)) = normalize_coords(*y as i64 + dy, *x as i64 + dx) {
                        if let Some(sig) = &vm.prologue_state.signal_grid[ny][nx] {
                            // Trigger!
                            vm.output.push(format!("PROLOGUE: Sink at {},{} received {:?}", x, y, sig));
                            vm.prologue_state.signal_grid[*y][*x] = Some(sig.clone()); // Light up sink

                            // If signal is a query string, try to solve it?
                            // For now, just log.
                        }
                    }
                }
            }
        }
    }

    // 6. Agents (@)
    // Agents move towards signal
    // We need to update agents list in state, and also update the Grid (move the '@' char)
    // This requires mutable access to grid.

    // We iterate agents from state (snapshot) and update grid.
    let agents = vm.prologue_state.agents.clone();
    let mut new_agents = Vec::new();

    for agent in agents {
        let (y, x) = (agent.y, agent.x);

        // Find neighbor with signal
        let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        let mut target = None;

        for (dy, dx) in neighbors {
            if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
                if vm.prologue_state.signal_grid[ny][nx].is_some() {
                    // Don't move INTO a rune (collision), unless it's a wire/signal?
                    // Let's simple logic: move if empty or wire.
                    let cell = &grid[ny][nx];
                    match cell {
                        Value::Int(0) => { target = Some((ny, nx)); break; },
                        Value::Str(s) if s == "~" => { target = Some((ny, nx)); break; },
                        _ => {}
                    }
                }
            }
        }

        if let Some((ny, nx)) = target {
            // Move agent
            // Clear old pos
            if let Value::Str(s) = &vm.grid[y][x] {
                if s == "@" {
                    vm.grid[y][x] = Value::Int(0);
                }
            }
            // Set new pos
            vm.grid[ny][nx] = Value::Str("@".to_string());
            new_agents.push(PrologueAgent { x: nx, y: ny, state: agent.state });
        } else {
            new_agents.push(agent);
        }
    }
    vm.prologue_state.agents = new_agents;
}

fn is_empty_val(v: &Value) -> bool {
    match v {
        Value::Int(0) => true,
        Value::Str(s) => s.is_empty(),
        _ => false,
    }
}

fn normalize_coords(y: i64, x: i64) -> Option<(usize, usize)> {
    if y >= 0 && y < GRID_SIZE as i64 && x >= 0 && x < GRID_SIZE as i64 {
        Some((y as usize, x as usize))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vm::ChimeraVM;
    use crate::ast::Dna;
    use crate::ast::Helix;

    #[test]
    fn test_prologue_circuit() {
        let dna = Dna { helix: Helix { strands: vec![] } };
        let mut vm = ChimeraVM::new(dna);
        vm.prologue_state.active = true;

        // Setup Circuit: 42 -> ! -> ~ -> ?
        vm.grid[4][5] = Value::Int(42);
        vm.grid[5][5] = Value::Str("!".to_string());
        vm.grid[6][5] = Value::Str("~".to_string());
        vm.grid[7][5] = Value::Str("?".to_string());

        exec_prologue_tick(&mut vm);

        // Check if signal propagated to wire
        assert!(vm.prologue_state.signal_grid[6][5].is_some());
        if let Some(Value::Int(v)) = &vm.prologue_state.signal_grid[6][5] {
            assert_eq!(*v, 42);
        } else {
            panic!("Wire did not carry signal 42");
        }

        // Check output
        let output = vm.output.join("\n");
        assert!(output.contains("PROLOGUE: Sink at 5,7 received Int(42)"));
    }
}
