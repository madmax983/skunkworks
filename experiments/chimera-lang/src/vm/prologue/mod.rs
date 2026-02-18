use crate::vm::{ChimeraVM, Value, GRID_SIZE};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};

pub mod quantum;
pub mod teleport;
pub mod chronos;

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
    pub delayed_signals: Vec<Vec<Option<Value>>>,
    pub agents: Vec<PrologueAgent>,
    pub registers: HashMap<(usize, usize), Value>,
    pub teleport_channels: HashMap<i64, Value>,
    pub history: HashMap<(usize, usize), VecDeque<Value>>,
}

impl PrologueState {
    pub fn new() -> Self {
        Self {
            active: false,
            runes: HashSet::new(),
            rules: Vec::new(),
            signal_grid: vec![vec![None; GRID_SIZE]; GRID_SIZE],
            delayed_signals: vec![vec![None; GRID_SIZE]; GRID_SIZE],
            agents: Vec::new(),
            registers: HashMap::new(),
            teleport_channels: HashMap::new(),
            history: HashMap::new(),
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
                    if matches!(
                        s.as_str(),
                        "?" | "!"
                            | "~"
                            | "&"
                            | "|"
                            | "+"
                            | "*"
                            | "#"
                            | "@"
                            | "$"
                            | "%"
                            | "^"
                            | "M"
                            | "O"
                            | "G"
                            | "E"
                            | "D"
                            | "A"
                            | "S"
                            | "P"
                            | "Q"
                            | "="
                            | ">"
                            | "<"
                            | "I"
                            | "Y"
                            | "L"
                            | "J"
                            | "C"
                            | "("
                            | "N"
                            | "W"
                            | "K"
                            | "R"
                            | "X"
                            | "Z"
                            | "H"
                            | "["
                            | "]"
                            | "U"
                            | "V"
                            | "F"
                            | "T"
                            | "\\"
                            | "/"
                            | "-"
                            | "q"
                            | "m"
                            | "{"
                            | "}"
                            | "s"
                            | "g"
                            | "r"
                    ) {
                        self.runes.insert((y, x));

                        if s == "@" || s == "K" || s == "H" {
                            self.agents.push(PrologueAgent {
                                x,
                                y,
                                state: Value::Int(0),
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
    let grid_snapshot = vm.grid.clone(); // Clone for read access
    vm.prologue_state.scan_grid_rules(&grid_snapshot);

    // 2. Clear Signals & Apply Delays & 3. Source Emission
    prepare_signals(vm, &grid_snapshot);

    // 4. Propagation (Wires ~ and Gates & | + * # % ^)
    process_signal_propagation(vm, &grid_snapshot);

    // 5. Sink Consumption / Actions (?, $, M, O)
    process_sinks(vm, &grid_snapshot);

    // 6. Agents (@)
    process_agents(vm, &grid_snapshot);
}

fn prepare_signals(vm: &mut ChimeraVM, grid: &Vec<Vec<Value>>) {
    // Start with empty signal grid
    let mut current_signals = vec![vec![None; GRID_SIZE]; GRID_SIZE];

    // Apply delayed signals from previous tick
    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            if let Some(val) = &vm.prologue_state.delayed_signals[y][x] {
                current_signals[y][x] = Some(val.clone());
            }
        }
    }
    vm.prologue_state.signal_grid = current_signals;

    // Prepare next tick's delayed signals (cleared initially)
    vm.prologue_state.delayed_signals = vec![vec![None; GRID_SIZE]; GRID_SIZE];

    // Source Emission (!)
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
}

fn process_signal_propagation(vm: &mut ChimeraVM, grid: &Vec<Vec<Value>>) {
    // Simple iterative flood fill for wires
    // Gates need specific inputs.
    // Iteration loop to allow signal to travel across grid in one tick
    let max_iterations = GRID_SIZE * 2;
    let runes: Vec<(usize, usize)> = vm.prologue_state.runes.iter().cloned().collect();
    let tick = vm.tick_counter;

    for _ in 0..max_iterations {
        let mut changes = false;
        let mut next_signals = vm.prologue_state.signal_grid.clone();

        for (y, x) in &runes {
            if let Value::Str(s) = &grid[*y][*x] {
                if apply_propagation_rune(
                    s,
                    *y,
                    *x,
                    tick,
                    &vm.prologue_state.signal_grid,
                    &mut next_signals,
                    &mut vm.prologue_state.delayed_signals,
                    &mut vm.ether,
                    &mut vm.prologue_state.registers,
                    &mut vm.prologue_state.teleport_channels,
                    &mut vm.prologue_state.history,
                ) {
                    changes = true;
                }
            }
        }
        vm.prologue_state.signal_grid = next_signals;
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
    current_signals: &[Vec<Option<Value>>],
    next_signals: &mut Vec<Vec<Option<Value>>>,
    next_delayed: &mut Vec<Vec<Option<Value>>>,
    ether: &mut HashMap<i64, VecDeque<Value>>,
    registers: &mut HashMap<(usize, usize), Value>,
    teleport_channels: &mut HashMap<i64, Value>,
    history: &mut HashMap<(usize, usize), VecDeque<Value>>,
) -> bool {
    if apply_topology_runes(rune, y, x, current_signals, next_signals, next_delayed) {
        return true;
    }
    if apply_math_runes(rune, y, x, current_signals, next_signals) {
        return true;
    }
    if apply_logic_runes(rune, y, x, current_signals, next_signals) {
        return true;
    }
    if apply_io_runes(rune, y, x, tick, current_signals, next_signals, ether, registers) {
        return true;
    }
    if apply_list_runes(rune, y, x, current_signals, next_signals) {
        return true;
    }
    if apply_optics_runes(rune, y, x, current_signals, next_signals) {
        return true;
    }
    if quantum::apply_quantum_runes(rune, y, x, current_signals, next_signals) {
        return true;
    }
    if teleport::apply_teleport_runes(rune, y, x, current_signals, next_signals, teleport_channels) {
        return true;
    }
    if chronos::apply_chronos_runes(rune, y, x, current_signals, next_signals, history) {
        return true;
    }
    false
}

fn apply_optics_runes(
    rune: &str,
    y: usize,
    x: usize,
    current_signals: &[Vec<Option<Value>>],
    next_signals: &mut Vec<Vec<Option<Value>>>,
) -> bool {
    let mut changes = false;
    // Helper to get signal from neighbor
    let get_sig = |dy: i64, dx: i64| -> Option<Value> {
        if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
            current_signals[ny][nx].clone()
        } else {
            None
        }
    };

    // Helper to set signal to neighbor
    let mut set_sig = |dy: i64, dx: i64, val: Value| {
        if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
            if next_signals[ny][nx].is_none() {
                next_signals[ny][nx] = Some(val);
                changes = true;
            }
        }
    };

    match rune {
        "\\" => {
            // Mirror Back
            // N -> E, E -> N, S -> W, W -> S
            if let Some(s) = get_sig(-1, 0) { set_sig(0, 1, s); } // N -> E
            if let Some(s) = get_sig(0, 1) { set_sig(-1, 0, s); } // E -> N
            if let Some(s) = get_sig(1, 0) { set_sig(0, -1, s); } // S -> W
            if let Some(s) = get_sig(0, -1) { set_sig(1, 0, s); } // W -> S
        }
        "/" => {
            // Mirror Forward
            // N -> W, W -> N, S -> E, E -> S
            if let Some(s) = get_sig(-1, 0) { set_sig(0, -1, s); } // N -> W
            if let Some(s) = get_sig(0, -1) { set_sig(-1, 0, s); } // W -> N
            if let Some(s) = get_sig(1, 0) { set_sig(0, 1, s); } // S -> E
            if let Some(s) = get_sig(0, 1) { set_sig(1, 0, s); } // E -> S
        }
        "-" => {
            // Horizontal Beam
            // W <-> E
            if let Some(s) = get_sig(0, -1) { set_sig(0, 1, s); } // W -> E
            if let Some(s) = get_sig(0, 1) { set_sig(0, -1, s); } // E -> W
        }
        _ => {}
    }
    changes
}

fn apply_topology_runes(
    rune: &str,
    y: usize,
    x: usize,
    current_signals: &[Vec<Option<Value>>],
    next_signals: &mut Vec<Vec<Option<Value>>>,
    next_delayed: &mut Vec<Vec<Option<Value>>>,
) -> bool {
    let mut changes = false;
    match rune {
        "~" => {
            // Wires: OR of all neighbors
            let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
            for (dy, dx) in neighbors {
                if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
                    if let Some(sig) = &current_signals[ny][nx] {
                        if next_signals[y][x].is_none() {
                            next_signals[y][x] = Some(sig.clone());
                            changes = true;
                        }
                    }
                }
            }
        }
        "^" | "J" => {
            // Jump: Input West -> Output East (Skipping Self)
            if let (Some((wy, wx)), Some((ey, ex))) = (
                normalize_coords(y as i64, x as i64 - 1),
                normalize_coords(y as i64, x as i64 + 1),
            ) {
                if let Some(sig) = &current_signals[wy][wx] {
                    if next_signals[ey][ex].is_none() {
                        next_signals[ey][ex] = Some(sig.clone());
                        changes = true;
                    }
                }
            }
        }
        "*" => {
            // Splitter: Input North -> Output Self
            if let Some((ny, nx)) = normalize_coords(y as i64 - 1, x as i64) {
                if let Some(sig) = &current_signals[ny][nx] {
                    if next_signals[y][x].is_none() {
                        next_signals[y][x] = Some(sig.clone());
                        changes = true;
                    }
                }
            }
        }
        "#" => {
            // Delay: Input North -> Output to next_delayed
            if let Some((ny, nx)) = normalize_coords(y as i64 - 1, x as i64) {
                if let Some(sig) = &current_signals[ny][nx] {
                    if next_delayed[y][x].is_none() {
                        next_delayed[y][x] = Some(sig.clone());
                    }
                }
            }
        }
        _ => {}
    }
    changes
}

fn apply_math_runes(
    rune: &str,
    y: usize,
    x: usize,
    current_signals: &[Vec<Option<Value>>],
    next_signals: &mut Vec<Vec<Option<Value>>>,
) -> bool {
    let mut changes = false;
    let w_sig = if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
        current_signals[wy][wx].clone()
    } else {
        None
    };
    let e_sig = if let Some((ey, ex)) = normalize_coords(y as i64, x as i64 + 1) {
        current_signals[ey][ex].clone()
    } else {
        None
    };

    match rune {
        "A" => {
            if let (Some(Value::Int(w)), Some(Value::Int(e))) = (&w_sig, &e_sig) {
                if next_signals[y][x].is_none() {
                    next_signals[y][x] = Some(Value::Int(w.wrapping_add(*e)));
                    changes = true;
                }
            }
        }
        "B" => {
            if let (Some(Value::Int(w)), Some(Value::Int(e))) = (&w_sig, &e_sig) {
                if next_signals[y][x].is_none() {
                    next_signals[y][x] = Some(Value::Int(w.wrapping_sub(*e)));
                    changes = true;
                }
            }
        }
        "P" => {
            if let (Some(Value::Int(w)), Some(Value::Int(e))) = (&w_sig, &e_sig) {
                if next_signals[y][x].is_none() {
                    next_signals[y][x] = Some(Value::Int(w.wrapping_mul(*e)));
                    changes = true;
                }
            }
        }
        "Q" => {
            if let (Some(Value::Int(w)), Some(Value::Int(e))) = (&w_sig, &e_sig) {
                if *e != 0 {
                    if next_signals[y][x].is_none() {
                        next_signals[y][x] = Some(Value::Int(w.wrapping_div(*e)));
                        changes = true;
                    }
                }
            }
        }
        "=" => {
            if let (Some(w), Some(e)) = (&w_sig, &e_sig) {
                if next_signals[y][x].is_none() {
                    next_signals[y][x] = Some(Value::Int(if w == e { 1 } else { 0 }));
                    changes = true;
                }
            }
        }
        ">" => {
            if let (Some(Value::Int(w)), Some(Value::Int(e))) = (&w_sig, &e_sig) {
                if next_signals[y][x].is_none() {
                    next_signals[y][x] = Some(Value::Int(if w > e { 1 } else { 0 }));
                    changes = true;
                }
            }
        }
        "<" => {
            if let (Some(Value::Int(w)), Some(Value::Int(e))) = (&w_sig, &e_sig) {
                if next_signals[y][x].is_none() {
                    next_signals[y][x] = Some(Value::Int(if w < e { 1 } else { 0 }));
                    changes = true;
                }
            }
        }
        "%" => {
            if let (Some(Value::Int(w)), Some(Value::Int(e))) = (&w_sig, &e_sig) {
                if *e != 0 {
                    if next_signals[y][x].is_none() {
                        next_signals[y][x] = Some(Value::Int(w % e));
                        changes = true;
                    }
                }
            }
        }
        _ => {}
    }
    changes
}

fn apply_logic_runes(
    rune: &str,
    y: usize,
    x: usize,
    current_signals: &[Vec<Option<Value>>],
    next_signals: &mut Vec<Vec<Option<Value>>>,
) -> bool {
    let mut changes = false;
    let w_sig = if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
        current_signals[wy][wx].clone()
    } else {
        None
    };
    let e_sig = if let Some((ey, ex)) = normalize_coords(y as i64, x as i64 + 1) {
        current_signals[ey][ex].clone()
    } else {
        None
    };

    match rune {
        "&" => {
            if w_sig.is_some() && e_sig.is_some() {
                if next_signals[y][x].is_none() {
                    next_signals[y][x] = Some(Value::Int(1));
                    changes = true;
                }
            }
        }
        "|" => {
            if w_sig.is_some() || e_sig.is_some() {
                if next_signals[y][x].is_none() {
                    next_signals[y][x] = Some(Value::Int(1));
                    changes = true;
                }
            }
        }
        "+" => {
            // XOR
            if w_sig.is_some() ^ e_sig.is_some() {
                if next_signals[y][x].is_none() {
                    next_signals[y][x] = Some(Value::Int(1));
                    changes = true;
                }
            }
        }
        "I" => {
            // If: West (Condition) != 0 -> Output North (Value) to Self
            // Need N input too
            let n_sig = if let Some((ny, nx)) = normalize_coords(y as i64 - 1, x as i64) {
                current_signals[ny][nx].clone()
            } else {
                None
            };

            if let Some(Value::Int(cond)) = w_sig {
                if cond != 0 {
                    if let Some(val) = n_sig {
                        if next_signals[y][x].is_none() {
                            next_signals[y][x] = Some(val);
                            changes = true;
                        }
                    }
                }
            }
        }
        _ => {}
    }
    changes
}

#[allow(clippy::too_many_arguments)]
fn apply_io_runes(
    rune: &str,
    y: usize,
    x: usize,
    tick: u64,
    current_signals: &[Vec<Option<Value>>],
    next_signals: &mut Vec<Vec<Option<Value>>>,
    ether: &mut HashMap<i64, VecDeque<Value>>,
    registers: &mut HashMap<(usize, usize), Value>,
) -> bool {
    let mut changes = false;
    let w_sig = if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
        current_signals[wy][wx].clone()
    } else {
        None
    };

    match rune {
        "L" => {
            if let Some(Value::Int(channel)) = w_sig {
                if next_signals[y][x].is_none() {
                    if let Some(queue) = ether.get_mut(&channel) {
                        if let Some(val) = queue.pop_front() {
                            next_signals[y][x] = Some(val);
                            changes = true;
                        }
                    }
                }
            }
        }
        "C" => {
            if let Some(Value::Int(m)) = w_sig {
                if m > 0 {
                    let val = tick % (m as u64);
                    if next_signals[y][x].is_none() {
                        next_signals[y][x] = Some(Value::Int(val as i64));
                        changes = true;
                    }
                }
            }
        }
        "N" => {
            // West -> North
            if let Some(sig) = w_sig {
                if let Some((ny, nx)) = normalize_coords(y as i64 - 1, x as i64) {
                    if next_signals[ny][nx].is_none() {
                        next_signals[ny][nx] = Some(sig);
                        changes = true;
                    }
                }
            }
        }
        "S" => {
            // West -> South
            if let Some(sig) = w_sig {
                if let Some((sy, sx)) = normalize_coords(y as i64 + 1, x as i64) {
                    if next_signals[sy][sx].is_none() {
                        next_signals[sy][sx] = Some(sig);
                        changes = true;
                    }
                }
            }
        }
        "E" => {
            // West -> East
            if let Some(sig) = w_sig {
                if let Some((ey, ex)) = normalize_coords(y as i64, x as i64 + 1) {
                    if next_signals[ey][ex].is_none() {
                        next_signals[ey][ex] = Some(sig);
                        changes = true;
                    }
                }
            }
        }
        "W" => {
            // East -> West
            if let Some((ey, ex)) = normalize_coords(y as i64, x as i64 + 1) {
                if let Some(sig) = &current_signals[ey][ex] {
                    if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
                        if next_signals[wy][wx].is_none() {
                            next_signals[wy][wx] = Some(sig.clone());
                            changes = true;
                        }
                    }
                }
            }
        }
        "K" => {
            let mut rng = rand::thread_rng();
            let val = Value::Int(rng.gen_range(0..100));
            let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
            for (dy, dx) in neighbors {
                if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
                    if next_signals[ny][nx].is_none() {
                        next_signals[ny][nx] = Some(val.clone());
                        changes = true;
                    }
                }
            }
        }
        "R" => {
            // Register
            // Write (West)
            if let Some(sig) = w_sig {
                registers.insert((y, x), sig);
            }
            // Read (North)
            if let Some((ny, nx)) = normalize_coords(y as i64 - 1, x as i64) {
                if current_signals[ny][nx].is_some() {
                    if let Some(val) = registers.get(&(y, x)) {
                        if let Some((sy, sx)) = normalize_coords(y as i64 + 1, x as i64) {
                            if next_signals[sy][sx].is_none() {
                                next_signals[sy][sx] = Some(val.clone());
                                changes = true;
                            }
                        }
                    }
                }
            }
        }
        "Z" => {
            use std::time::{SystemTime, UNIX_EPOCH};
            let start = SystemTime::now();
            let since_the_epoch = start
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards");
            let val = Value::Int((since_the_epoch.as_secs() % 100) as i64);
            let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
            for (dy, dx) in neighbors {
                if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
                    if next_signals[ny][nx].is_none() {
                        next_signals[ny][nx] = Some(val.clone());
                        changes = true;
                    }
                }
            }
        }
        _ => {}
    }
    changes
}

fn apply_list_runes(
    rune: &str,
    y: usize,
    x: usize,
    current_signals: &[Vec<Option<Value>>],
    next_signals: &mut Vec<Vec<Option<Value>>>,
) -> bool {
    let mut changes = false;
    let w_sig = if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
        current_signals[wy][wx].clone()
    } else {
        None
    };

    match rune {
        "[" => {
            // Collect: N, E, S, W -> Junction
            let mut items = Vec::new();
            // Clockwise from North
            let neighbors = [(-1, 0), (0, 1), (1, 0), (0, -1)]; // N E S W
            for (dy, dx) in neighbors {
                if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
                    if let Some(sig) = &current_signals[ny][nx] {
                        items.push(sig.clone());
                    }
                }
            }
            if !items.is_empty() {
                if next_signals[y][x].is_none() {
                    next_signals[y][x] = Some(Value::Junction(crate::ast::JunctionType::Any, items));
                    changes = true;
                }
            }
        }
        "]" => {
            // Scatter: West (Junction) -> N, E, S
            if let Some(Value::Junction(_, items)) = w_sig {
                // Distribute items
                // Item 0 -> N
                if items.len() > 0 {
                    if let Some((ny, nx)) = normalize_coords(y as i64 - 1, x as i64) {
                        if next_signals[ny][nx].is_none() {
                            next_signals[ny][nx] = Some(items[0].clone());
                            changes = true;
                        }
                    }
                }
                // Item 1 -> E
                if items.len() > 1 {
                    if let Some((ey, ex)) = normalize_coords(y as i64, x as i64 + 1) {
                        if next_signals[ey][ex].is_none() {
                            next_signals[ey][ex] = Some(items[1].clone());
                            changes = true;
                        }
                    }
                }
                // Item 2 -> S
                if items.len() > 2 {
                    if let Some((sy, sx)) = normalize_coords(y as i64 + 1, x as i64) {
                        if next_signals[sy][sx].is_none() {
                            next_signals[sy][sx] = Some(items[2].clone());
                            changes = true;
                        }
                    }
                }
            }
        }
        "U" => {
            // Unwrap (Head): West -> Self
            if let Some(Value::Junction(_, items)) = w_sig {
                if let Some(head) = items.first() {
                    if next_signals[y][x].is_none() {
                        next_signals[y][x] = Some(head.clone());
                        changes = true;
                    }
                }
            }
        }
        "V" => {
            // Vector (Tail): West -> Self
            if let Some(Value::Junction(t, items)) = w_sig {
                if items.len() > 1 {
                    let tail = items[1..].to_vec();
                    if next_signals[y][x].is_none() {
                        next_signals[y][x] = Some(Value::Junction(t, tail));
                        changes = true;
                    }
                }
            }
        }
        "F" => {
            // Filter: West (List), North (Mask) -> Self
            let n_sig = if let Some((ny, nx)) = normalize_coords(y as i64 - 1, x as i64) {
                current_signals[ny][nx].clone()
            } else {
                None
            };

            if let (Some(Value::Junction(t, items)), Some(mask)) = (w_sig, n_sig) {
                // How does mask work?
                // If mask is Int(1), pass all?
                // If mask is a Junction of booleans?
                // Or maybe mask is just "Truthiness"?
                // Let's implement: If Mask is truthy, pass list? No that's trivial.
                // Maybe F applies a filter logic... but we don't have lambdas here easily.
                // Let's make it simple: Filter by Type? Or Filter non-empty?

                // Let's make it: Filter out items equal to Mask.
                // Or: Keep items equal to Mask?
                // Let's say: Remove items equal to Mask.

                let filtered: Vec<Value> = items.into_iter().filter(|v| *v != mask).collect();
                if next_signals[y][x].is_none() {
                    next_signals[y][x] = Some(Value::Junction(t, filtered));
                    changes = true;
                }
            }
        }
        "T" => {
            // Take: West (List), North (Count) -> Self
            let n_sig = if let Some((ny, nx)) = normalize_coords(y as i64 - 1, x as i64) {
                current_signals[ny][nx].clone()
            } else {
                None
            };

            if let (Some(Value::Junction(t, items)), Some(Value::Int(n))) = (w_sig, n_sig) {
                let count = n.max(0) as usize;
                let taken: Vec<Value> = items.into_iter().take(count).collect();
                if next_signals[y][x].is_none() {
                    next_signals[y][x] = Some(Value::Junction(t, taken));
                    changes = true;
                }
            }
        }
        _ => {}
    }
    changes
}

fn process_sinks(vm: &mut ChimeraVM, grid: &Vec<Vec<Value>>) {
    let runes: Vec<(usize, usize)> = vm.prologue_state.runes.iter().cloned().collect();

    for (y, x) in &runes {
        if let Value::Str(s) = &grid[*y][*x] {
            apply_sink_rune(vm, s, *y, *x);
        }
    }
}

fn apply_sink_rune(vm: &mut ChimeraVM, rune: &str, y: usize, x: usize) {
    match rune {
        "?" => {
            // Sink
            // Check neighbors for signal
            let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
            for (dy, dx) in neighbors {
                if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
                    let sig_opt = vm.prologue_state.signal_grid[ny][nx].clone();

                    if let Some(sig) = sig_opt {
                        vm.output
                            .push(format!("PROLOGUE: Sink at {},{} received {:?}", x, y, sig));
                        vm.prologue_state.signal_grid[y][x] = Some(sig.clone()); // Light up

                        if let Value::Str(name) = sig {
                            if let Some(&idx) = vm.dictionary.get(&name) {
                                vm.interrupt(idx);
                            }
                        }
                    }
                }
            }
        }
        "$" => {
            // Scribe: Write West -> South
            if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
                if let Some(sig) = &vm.prologue_state.signal_grid[wy][wx] {
                    // Write to South
                    if let Some((sy, sx)) = normalize_coords(y as i64 + 1, x as i64) {
                        vm.grid[sy][sx] = sig.clone();
                        vm.prologue_state.signal_grid[y][x] = Some(sig.clone());
                        // Light up
                    }
                }
            }
        }
        "M" => {
            // Mutate: Signal West -> Randomize South
            if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
                if vm.prologue_state.signal_grid[wy][wx].is_some() {
                    if let Some((sy, sx)) = normalize_coords(y as i64 + 1, x as i64) {
                        let mut rng = rand::thread_rng();
                        let val = rng.gen_range(0..100);
                        vm.grid[sy][sx] = Value::Int(val);
                        vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1));
                        // Light up
                    }
                }
            }
        }
        "O" => {
            // Organelle: Signal West -> Spawn Agent South
            if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
                if let Some(sig) = &vm.prologue_state.signal_grid[wy][wx] {
                    if let Some((sy, sx)) = normalize_coords(y as i64 + 1, x as i64) {
                        let agent_type = match sig {
                            Value::Int(2) => "K", // Chaos
                            Value::Int(3) => "H", // Hunter
                            _ => "@",             // Seeker
                        };
                        vm.grid[sy][sx] = Value::Str(agent_type.to_string());
                        vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1));
                    }
                }
            }
        }
        "G" => {
            // Genesis: North (Code), West (Config) -> Self (Strand Index)
            let code_to_compile = if let Some((ny, nx)) = normalize_coords(y as i64 - 1, x as i64) {
                if let Some(Value::Str(s)) = &vm.prologue_state.signal_grid[ny][nx] {
                    Some(s.clone())
                } else {
                    None
                }
            } else {
                None
            };

            if let Some(code) = code_to_compile {
                match crate::compiler::compile(&code, None) {
                    Ok(dna) => {
                        if let Some(strand) = dna.helix.strands.first() {
                            vm.dna.helix.strands.push(strand.clone());
                            let idx = vm.dna.helix.strands.len() - 1;
                            vm.output
                                .push(format!("PROLOGUE: Genesis created Strand {}", idx));
                            vm.prologue_state.signal_grid[y][x] = Some(Value::Int(idx as i64));
                        }
                    }
                    Err(e) => {
                        vm.output.push(format!("PROLOGUE: Genesis failed: {}", e));
                    }
                }
            }
        }
        "E" => {
            // Eval: West (Code) -> Self (Result)
            let code_to_eval = if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
                if let Some(Value::Str(s)) = &vm.prologue_state.signal_grid[wy][wx] {
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
                                vm.execute_gene_inner(gene.op.clone(), &gene.args);
                            }
                            vm.output.push("PROLOGUE: Eval executed.".to_string());
                            vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1));
                        }
                    }
                    Err(e) => {
                        vm.output.push(format!("PROLOGUE: Eval failed: {}", e));
                    }
                }
            }
        }
        "D" => {
            // Data: Neighbors -> Self (List)
            let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)]; // N S W E
            let mut data = Vec::new();
            for (dy, dx) in neighbors {
                if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
                    let val = vm.grid[ny][nx].clone();
                    if !is_empty_val(&val) {
                        data.push(val);
                    }
                }
            }
            if !data.is_empty() {
                vm.prologue_state.signal_grid[y][x] =
                    Some(Value::Junction(crate::ast::JunctionType::Any, data));
            }
        }
        "Y" => {
            // Yell: West (Value), East (Channel) -> Push to Ether
            if let (Some((wy, wx)), Some((ey, ex))) = (
                normalize_coords(y as i64, x as i64 - 1),
                normalize_coords(y as i64, x as i64 + 1),
            ) {
                let w_sig = &vm.prologue_state.signal_grid[wy][wx];
                let e_sig = &vm.prologue_state.signal_grid[ey][ex];

                if let (Some(val), Some(Value::Int(channel))) = (w_sig, e_sig) {
                    vm.ether
                        .entry(*channel)
                        .or_insert_with(std::collections::VecDeque::new)
                        .push_back(val.clone());
                    vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1));
                    // Light up
                }
            }
        }
        "(" => {
            // Warp: Swap North and South values (Grid Modification)
            // Triggered by West signal
            if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
                if vm.prologue_state.signal_grid[wy][wx].is_some() {
                    if let (Some((ny, nx)), Some((sy, sx))) = (
                        normalize_coords(y as i64 - 1, x as i64),
                        normalize_coords(y as i64 + 1, x as i64),
                    ) {
                        // We need to swap values in the grid
                        // To avoid borrowing issues, we can't swap directly if we hold references?
                        // But we have mutable access to VM.
                        let n_val = vm.grid[ny][nx].clone();
                        let s_val = vm.grid[sy][sx].clone();
                        vm.grid[ny][nx] = s_val;
                        vm.grid[sy][sx] = n_val;
                        vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1));
                        // Light up
                    }
                }
            }
        }
        "X" => {
            // Crossover: West (Idx A), East (Idx B) -> South (New Idx)
            if let (Some((wy, wx)), Some((ey, ex))) = (
                normalize_coords(y as i64, x as i64 - 1),
                normalize_coords(y as i64, x as i64 + 1),
            ) {
                let w_sig = vm.prologue_state.signal_grid[wy][wx].clone();
                let e_sig = vm.prologue_state.signal_grid[ey][ex].clone();

                if let (Some(Value::Int(idx_a)), Some(Value::Int(idx_b))) = (w_sig, e_sig) {
                    let len = vm.dna.helix.strands.len();
                    if idx_a >= 0
                        && (idx_a as usize) < len
                        && idx_b >= 0
                        && (idx_b as usize) < len
                    {
                        let strand_a = vm.dna.helix.strands[idx_a as usize].clone();
                        let strand_b = vm.dna.helix.strands[idx_b as usize].clone();

                        let split_a = strand_a.genes.len() / 2;
                        let split_b = strand_b.genes.len() / 2;

                        let mut new_genes = Vec::new();
                        new_genes.extend(strand_a.genes.iter().take(split_a).cloned());
                        new_genes.extend(strand_b.genes.iter().skip(split_b).cloned());

                        let new_strand = crate::ast::Strand { genes: new_genes };
                        vm.dna.helix.strands.push(new_strand);
                        let new_idx = vm.dna.helix.strands.len() - 1;

                        if let Some((sy, sx)) = normalize_coords(y as i64 + 1, x as i64) {
                            vm.grid[sy][sx] = Value::Int(new_idx as i64);
                            vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1));
                        }
                    }
                }
            }
        }
        _ => {}
    }
}

fn process_agents(vm: &mut ChimeraVM, grid_snapshot: &Vec<Vec<Value>>) {
    // Agents move towards signal
    // We need to update agents list in state, and also update the Grid (move the '@' char)
    // This requires mutable access to grid.

    // We iterate agents from state (snapshot) and update grid.
    let agents = vm.prologue_state.agents.clone();
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
            // Chaos: Move Randomly
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
            // Hunter: Seek Prey (@ or K)
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
            // If no prey, seek signal like @
            if target.is_none() {
                for (dy, dx) in neighbors {
                    if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
                        if vm.prologue_state.signal_grid[ny][nx].is_some() {
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
            // Seeker (@): Seek Signal
            for (dy, dx) in neighbors {
                if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
                    if vm.prologue_state.signal_grid[ny][nx].is_some() {
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
            // Clear old pos
            if let Value::Str(s) = &vm.grid[y][x] {
                // Only clear if it matches our agent type (avoid clearing overwrites?)
                if s == &current_type {
                    vm.grid[y][x] = Value::Int(0);
                }
            }
            // Set new pos
            vm.grid[ny][nx] = Value::Str(current_type.clone());
            new_agents.push(PrologueAgent {
                x: nx,
                y: ny,
                state: agent.state,
            });
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
