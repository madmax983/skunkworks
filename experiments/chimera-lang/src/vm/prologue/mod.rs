use crate::vm::{Value, GRID_SIZE};
use rand::Rng;
use std::collections::{HashMap, VecDeque};

pub mod alchemy;
pub mod chronos;
pub mod evolution;
pub mod host;
pub mod io;
pub mod list;
pub mod logic;
pub mod math;
pub mod optics;
pub mod quantum;
pub mod state;
pub mod teleport;
pub mod topology;

pub use host::PrologueHost;
pub use state::*;

pub fn exec_prologue_tick(host: &mut impl PrologueHost, state: &mut PrologueState) {
    if !state.active {
        return;
    }

    // 1. Scan Grid for Topology (Runes)
    // Create snapshot from Host
    let mut grid_snapshot = vec![vec![Value::Int(0); GRID_SIZE]; GRID_SIZE];
    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            grid_snapshot[y][x] = host.grid_read(y, x);
        }
    }

    state.scan_grid_rules(&grid_snapshot);

    // 2. Clear Signals & Apply Delays & 3. Source Emission
    prepare_signals(state, &grid_snapshot);

    // 4. Propagation (Wires ~ and Gates & | + * # % ^)
    process_signal_propagation(host, state, &grid_snapshot);

    // 5. Sink Consumption / Actions (?, $, M, O)
    process_sinks(host, state, &grid_snapshot);

    // 6. Agents (@)
    process_agents(host, state, &grid_snapshot);
}

fn prepare_signals(state: &mut PrologueState, grid: &[Vec<Value>]) {
    // Start with empty signal grid
    let mut current_signals = vec![vec![None; GRID_SIZE]; GRID_SIZE];

    // Apply delayed signals from previous tick
    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            if let Some(val) = &state.delayed_signals[y][x] {
                current_signals[y][x] = Some(val.clone());
            }
        }
    }
    state.signal_grid = current_signals;

    // Prepare next tick's delayed signals (cleared initially)
    state.delayed_signals = vec![vec![None; GRID_SIZE]; GRID_SIZE];

    // Source Emission (!)
    let runes: Vec<(usize, usize)> = state.runes.iter().cloned().collect();

    for (y, x) in &runes {
        if let Value::Str(s) = &grid[*y][*x] {
            if s == "!" && *y > 0 {
                let val = grid[*y - 1][*x].clone();
                if !is_empty_val(&val) {
                    state.signal_grid[*y][*x] = Some(val);
                }
            }
        }
    }
}

fn process_signal_propagation(host: &mut impl PrologueHost, state: &mut PrologueState, grid: &[Vec<Value>]) {
    let max_iterations = GRID_SIZE * 2;
    let runes: Vec<(usize, usize)> = state.runes.iter().cloned().collect();
    let tick = host.tick_counter();
    let dna = host.dna().clone(); // Dna is cheap to clone? No, but we need it. Host returns &Dna.
    // apply_propagation_rune takes &Dna.

    for _ in 0..max_iterations {
        let mut changes = false;
        let mut next_signals = state.signal_grid.clone();

        for (y, x) in &runes {
            if let Value::Str(s) = &grid[*y][*x] {
                if apply_propagation_rune(
                    s,
                    *y,
                    *x,
                    tick,
                    host,
                    &dna,
                    &state.signal_grid,
                    &mut next_signals,
                    &mut state.delayed_signals,
                    &mut state.registers,
                    &mut state.teleport_channels,
                    &mut state.history,
                ) {
                    changes = true;
                }
            }
        }
        state.signal_grid = next_signals;
        if !changes {
            break;
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn apply_propagation_rune(
    rune: &str,
    y: usize,
    x: usize,
    tick: u64,
    host: &mut impl PrologueHost, // Added Host
    dna: &crate::ast::Dna,
    current_signals: &[Vec<Option<Value>>],
    next_signals: &mut Vec<Vec<Option<Value>>>,
    next_delayed: &mut Vec<Vec<Option<Value>>>,
    registers: &mut HashMap<(usize, usize), Value>,
    teleport_channels: &mut HashMap<i64, Value>,
    history: &mut HashMap<(usize, usize), VecDeque<Value>>,
) -> bool {
    // Pass host.ether_get_map() to submodules
    let ether = host.ether_get_map();

    if topology::apply_topology_runes(rune, y, x, current_signals, next_signals, next_delayed) {
        return true;
    }
    if math::apply_math_runes(rune, y, x, current_signals, next_signals) {
        return true;
    }
    if logic::apply_logic_runes(rune, y, x, current_signals, next_signals) {
        return true;
    }
    if io::apply_io_runes(
        rune,
        y,
        x,
        tick,
        current_signals,
        next_signals,
        ether,
        registers,
    ) {
        return true;
    }
    if list::apply_list_runes(rune, y, x, current_signals, next_signals) {
        return true;
    }
    if optics::apply_optics_runes(rune, y, x, current_signals, next_signals) {
        return true;
    }
    if quantum::apply_quantum_runes(rune, y, x, current_signals, next_signals) {
        return true;
    }
    if teleport::apply_teleport_runes(rune, y, x, current_signals, next_signals, teleport_channels)
    {
        return true;
    }
    if chronos::apply_chronos_runes(rune, y, x, current_signals, next_signals, history) {
        return true;
    }
    if alchemy::apply_alchemy_runes(rune, y, x, current_signals, next_signals) {
        return true;
    }
    if evolution::apply_evolution_runes(rune, y, x, dna, current_signals, next_signals) {
        return true;
    }
    false
}

fn process_sinks(host: &mut impl PrologueHost, state: &mut PrologueState, grid: &[Vec<Value>]) {
    let runes: Vec<(usize, usize)> = state.runes.iter().cloned().collect();

    for (y, x) in &runes {
        if let Value::Str(s) = &grid[*y][*x] {
            apply_sink_rune(host, state, s, *y, *x);
        }
    }
}

fn apply_sink_rune(host: &mut impl PrologueHost, state: &mut PrologueState, rune: &str, y: usize, x: usize) {
    match rune {
        "?" => {
            // Sink
            let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
            for (dy, dx) in neighbors {
                if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
                    let sig_opt = state.signal_grid[ny][nx].clone();

                    if let Some(sig) = sig_opt {
                        host.output_push(format!("PROLOGUE: Sink at {},{} received {:?}", x, y, sig));
                        state.signal_grid[y][x] = Some(sig.clone());

                        if let Value::Str(name) = sig {
                            if let Some(idx) = host.dictionary_get(&name) {
                                host.interrupt(idx);
                            }
                        }
                    }
                }
            }
        }
        "$" => {
            // Scribe: Write West -> South
            if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
                if let Some(sig) = &state.signal_grid[wy][wx] {
                    if let Some((sy, sx)) = normalize_coords(y as i64 + 1, x as i64) {
                        host.grid_write(sy, sx, sig.clone());
                        state.signal_grid[y][x] = Some(sig.clone());
                    }
                }
            }
        }
        "M" => {
            // Mutate
            if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
                if state.signal_grid[wy][wx].is_some() {
                    if let Some((sy, sx)) = normalize_coords(y as i64 + 1, x as i64) {
                        let mut rng = rand::thread_rng();
                        let val = rng.gen_range(0..100);
                        host.grid_write(sy, sx, Value::Int(val));
                        state.signal_grid[y][x] = Some(Value::Int(1));
                    }
                }
            }
        }
        "O" => {
            // Organelle
            if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
                if let Some(sig) = &state.signal_grid[wy][wx] {
                    if let Some((sy, sx)) = normalize_coords(y as i64 + 1, x as i64) {
                        let agent_type = match sig {
                            Value::Int(2) => "K",
                            Value::Int(3) => "H",
                            _ => "@",
                        };
                        host.grid_write(sy, sx, Value::Str(agent_type.to_string()));
                        state.signal_grid[y][x] = Some(Value::Int(1));
                    }
                }
            }
        }
        "E" => {
            // Eval
            let code_to_eval = if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
                if let Some(Value::Str(s)) = &state.signal_grid[wy][wx] {
                    Some(s.clone())
                } else {
                    None
                }
            } else {
                None
            };

            if let Some(code) = code_to_eval {
                match crate::compiler::compile(&code, None) {
                    Ok(dna) => {
                        if let Some(strand) = dna.helix.strands.first() {
                            for gene in &strand.genes {
                                host.execute_gene(gene.op.clone(), &gene.args);
                            }
                            host.output_push("PROLOGUE: Eval executed.".to_string());
                            state.signal_grid[y][x] = Some(Value::Int(1));
                        }
                    }
                    Err(e) => {
                        host.output_push(format!("PROLOGUE: Eval failed: {}", e));
                    }
                }
            }
        }
        "D" => {
            // Data
            let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
            let mut data = Vec::new();
            for (dy, dx) in neighbors {
                if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
                    let val = host.grid_read(ny, nx);
                    if !is_empty_val(&val) {
                        data.push(val);
                    }
                }
            }
            if !data.is_empty() {
                state.signal_grid[y][x] =
                    Some(Value::Junction(crate::ast::JunctionType::Any, data));
            }
        }
        "Y" => {
            // Yell
            if let (Some((wy, wx)), Some((ey, ex))) = (
                normalize_coords(y as i64, x as i64 - 1),
                normalize_coords(y as i64, x as i64 + 1),
            ) {
                let w_sig = &state.signal_grid[wy][wx];
                let e_sig = &state.signal_grid[ey][ex];

                if let (Some(val), Some(Value::Int(channel))) = (w_sig, e_sig) {
                    let map = host.ether_get_map();
                    map.entry(*channel).or_default().push_back(val.clone());
                    state.signal_grid[y][x] = Some(Value::Int(1));
                }
            }
        }
        "(" => {
            // Warp
            if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
                if state.signal_grid[wy][wx].is_some() {
                    if let (Some((ny, nx)), Some((sy, sx))) = (
                        normalize_coords(y as i64 - 1, x as i64),
                        normalize_coords(y as i64 + 1, x as i64),
                    ) {
                        let n_val = host.grid_read(ny, nx);
                        let s_val = host.grid_read(sy, sx);
                        host.grid_write(ny, nx, s_val);
                        host.grid_write(sy, sx, n_val);
                        state.signal_grid[y][x] = Some(Value::Int(1));
                    }
                }
            }
        }
        _ => {
            evolution::apply_evolution_sinks(host, state, rune, y, x);
        }
    }
}

fn process_agents(host: &mut impl PrologueHost, state: &mut PrologueState, grid_snapshot: &[Vec<Value>]) {
    let agents = state.agents.clone();
    let mut new_agents = Vec::new();

    for agent in agents {
        let (y, x) = (agent.y, agent.x);

        let mut current_type = "@".to_string();
        if let Value::Str(s) = &grid_snapshot[y][x] {
            current_type = s.clone();
        }

        let mut target = None;
        let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        if current_type == "K" {
            // Chaos
            let mut rng = rand::thread_rng();
            let mut possible_moves = Vec::new();
            for (dy, dx) in neighbors {
                if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
                    if let Value::Int(0) = &grid_snapshot[ny][nx] {
                        possible_moves.push((ny, nx));
                    }
                }
            }
            if !possible_moves.is_empty() {
                let idx = rng.gen_range(0..possible_moves.len());
                target = Some(possible_moves[idx]);
            }
        } else if current_type == "H" {
            // Hunter
            for (dy, dx) in neighbors {
                if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
                    if let Value::Str(s) = &grid_snapshot[ny][nx] {
                        if s == "@" || s == "K" {
                            target = Some((ny, nx));
                            break;
                        }
                    }
                }
            }
            if target.is_none() {
                for (dy, dx) in neighbors {
                    if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
                        if state.signal_grid[ny][nx].is_some() {
                            let cell = &grid_snapshot[ny][nx];
                            if matches!(cell, Value::Int(0) | Value::Str(_)) {
                                target = Some((ny, nx));
                                break;
                            }
                        }
                    }
                }
            }
        } else {
            // Seeker
            for (dy, dx) in neighbors {
                if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
                    if state.signal_grid[ny][nx].is_some() {
                        let cell = &grid_snapshot[ny][nx];
                        match cell {
                            Value::Int(0) => {
                                target = Some((ny, nx));
                                break;
                            }
                            Value::Str(s) if s == "~" => {
                                target = Some((ny, nx));
                                break;
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        if let Some((ny, nx)) = target {
            // Move agent
            if let Value::Str(s) = host.grid_read(y, x) {
                if s == current_type {
                    host.grid_write(y, x, Value::Int(0));
                }
            }
            host.grid_write(ny, nx, Value::Str(current_type.clone()));
            new_agents.push(PrologueAgent {
                x: nx,
                y: ny,
                state: agent.state,
            });
        } else {
            new_agents.push(agent);
        }
    }
    state.agents = new_agents;
}

fn is_empty_val(v: &Value) -> bool {
    match v {
        Value::Int(0) => true,
        Value::Str(s) => s.is_empty(),
        _ => false,
    }
}

pub fn normalize_coords(y: i64, x: i64) -> Option<(usize, usize)> {
    if y >= 0 && y < GRID_SIZE as i64 && x >= 0 && x < GRID_SIZE as i64 {
        Some((y as usize, x as usize))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Dna;
    use crate::ast::Helix;
    use crate::vm::ChimeraVM;

    #[test]
    fn test_prologue_circuit() {
        let dna = Dna {
            helix: Helix { strands: vec![] },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.prologue_state.active = true;

        // Setup Circuit: 42 -> ! -> ~ -> ?
        vm.grid[4][5] = Value::Int(42);
        vm.grid[5][5] = Value::Str("!".to_string());
        vm.grid[6][5] = Value::Str("~".to_string());
        vm.grid[7][5] = Value::Str("?".to_string());

        let mut state = std::mem::take(&mut vm.prologue_state);
        exec_prologue_tick(&mut vm, &mut state);
        vm.prologue_state = state;

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
