//! # Prologue: Rune-based Logic System
//!
//! Prologue is a Grid-based Visual Logic Language embedded within Chimera.
//! It allows for the construction of "Digital Circuits" and "Logic Agents" directly on the memory grid.
//!
//! ## Execution Cycle
//!
//! The Prologue system runs in discrete ticks, following this pipeline:
//!
//! 1.  **Scan**: Identify all Runes and Agents on the grid.
//! 2.  **Signal**: Activate `!` Source runes and release pending delayed signals.
//! 3.  **Propagate**: Spread signals through Wires (`~`) and process Logic Gates (`&`, `|`, `+`, etc.).
//!     *   This phase iterates until the grid stabilizes (up to a limit).
//! 4.  **Sink**: Active Sinks (`?`, `$`, `M`, etc.) consume signals and perform actions (Logging, Gene Execution, Grid Writes).
//! 5.  **Agent**: Agents (`@`, `K`, `H`) perceive their surroundings and move.
//!
//! ## Rune Reference
//!
//! | Category | Runes | Description |
//! |---|---|---|
//! | **Source/Sink** | `!` | **Source**: Emits value to North. |
//! | | `?` | **Sink**: Reads from South. Logs or Executes Gene. |
//! | | `$` | **Scribe**: Reads West (Val), Writes South. |
//! | **Wires** | `~` | **Wire**: Conducts signal in all directions. |
//! | | `*` | **Splitter**: North -> Self (One-way). |
//! | | `#` | **Delay**: North -> Self (Next Tick). |
//! | **Logic** | `&` | **AND**: West & East -> South. |
//! | | `\|` | **OR**: West \| East -> South. |
//! | | `+` | **XOR**: West ^ East -> South. |
//! | | `I` | **IF**: West (Cond) -> Passes North (Val). |
//! | **Math** | `A` | **Add**: West + East -> South. |
//! | | `S` | **Sub**: West - East -> South. |
//! | | `M` | **Mutate**: West -> Randomize South. |
//! | | `%` | **Modulo**: West % East -> South. |
//! | **Agents** | `@` | **Seeker**: Moves towards signals. |
//! | | `K` | **Chaos**: Moves randomly. |
//! | | `H` | **Hunter**: Chases other agents. |
//! | | `O` | **Organelle**: Spawns an agent. |
//! | **IO** | `Y` | **Yell**: Pushes to Ether Channel. |
//! | | `L` | **Listen**: Pops from Ether Channel. |
//! | **Flow** | `^` | **Jump**: West -> East (Teleport). |
//! | | `\` | **Mirror**: Reflects 90° (N<->E). |
//!
//! ## Example: Simple Logic Gate
//!
//! This circuit implements `(1 AND 1) -> Log`:
//!
//! ```text
//!   1   1
//!   !   !   (Sources emit 1 North)
//!   ~   ~   (Wires carry signals)
//!   &       (AND Gate receives both)
//!   ?       (Sink receives Result 1)
//! ```

use crate::vm::{ChimeraVM, Value, GRID_SIZE};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};

pub mod alchemy;
pub mod biolum;
pub mod chaos;
pub mod chronos;
pub mod critter;
pub mod construct;
pub mod evolution;
pub mod io;
pub mod list;
pub mod logic;
pub mod math;
pub mod necromancy;
pub mod optics;
pub mod quantum;
pub mod symbiosis;
pub mod teleport;
pub mod topology;
pub mod virology;
pub mod void;

/// An autonomous agent wandering the Prologue grid.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrologueAgent {
    /// X Coordinate (Column)
    pub x: usize,
    /// Y Coordinate (Row)
    pub y: usize,
    /// Internal state memory
    pub state: Value,
}

/// The entire state of the Prologue system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrologueState {
    /// Whether the Prologue system is currently running.
    pub active: bool,
    /// Cache of active Rune locations.
    pub runes: HashSet<(usize, usize)>,
    /// Active rules (unused currently).
    pub rules: Vec<String>,
    /// The overlay grid carrying transient signals for the current tick.
    pub signal_grid: Vec<Vec<Option<Value>>>,
    /// Signals to be released in the next tick (from Delay `#` runes).
    pub delayed_signals: Vec<Vec<Option<Value>>>,
    /// List of active agents.
    pub agents: Vec<PrologueAgent>,
    /// General purpose registers for runes.
    pub registers: HashMap<(usize, usize), Value>,
    /// Teleportation channels.
    pub teleport_channels: HashMap<i64, Value>,
    /// Historical data for time-travel runes.
    pub history: HashMap<(usize, usize), VecDeque<Value>>,
}

impl PrologueState {
    /// Creates a new, empty Prologue state.
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

    /// Scans the entire grid to identify Runes and Agents.
    ///
    /// This populates the `runes` cache and `agents` list for the upcoming tick.
    /// It effectively compiles the grid into an active circuit description.
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
                        // IO
                        "?" | "!"
                        // Topology
                            | "~"
                        // Logic
                            | "&"
                            | "|"
                            | "+"
                        // Circuit
                            | "*"
                            | "#"
                        // Agents
                            | "@"
                            | "$"
                        // Math
                            | "%"
                            | "^"
                            | "M"
                        // Biology
                            | "O"
                            | "G"
                        // Control
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
                        // Lists
                            | "["
                            | "]"
                            | "U"
                            | "V"
                            | "F"
                            | "T"
                        // Optics
                            | "\\"
                            | "/"
                            | "-"
                        // Quantum
                            | "q"
                            | "m"
                        // Teleport
                            | "{"
                            | "}"
                        // Chronos
                            | "s"
                            | "g"
                            | "r"
                        // Alchemy
                            | "t"
                            | "f"
                            | "d"
                        // Evolution
                            | "e"
                            | "b"
                            | "l"
                            | "n"
                            | "∞"
                        // Void
                            | "µ"
                            | "Ø"
                            | "§"
                        // Construct
                            | "B"
                            | "Π"
                        // Virology
                            | "v"
                            | "i"
                            | "a"
                        // Biolum
                            | "Φ"
                            | "Λ"
                            | "Ω"
                        // Chaos
                            | "k"
                            | "z"
                            | "h"
                            | "†"
                            | "‡"
                            | "Ψ"
                        // Symbiosis
                            | "u"
                            | "y"
                            | "w"
                            | "j"
                    ) {
                        self.runes.insert((y, x));

                        if s == "@" || s == "K" || s == "H" || s == "C" {
                            // Try to retrieve persistent state
                            let state = self.registers.get(&(y, x)).cloned().unwrap_or_else(|| {
                                if s == "C" {
                                    critter::CritterState::default().to_value()
                                } else {
                                    Value::Int(0)
                                }
                            });
                            self.agents.push(PrologueAgent { x, y, state });
                        }
                    }
                }
            }
        }
    }
}

/// Executes a single tick of the Prologue system.
///
/// This function coordinates the 5-phase execution cycle:
/// 1.  **Scan**: Maps the grid topology.
/// 2.  **Signal**: Initializes signals from Sources and Delays.
/// 3.  **Propagate**: Spreads signals through the network.
/// 4.  **Sink**: Triggers effects at Sink terminals.
/// 5.  **Agent**: Updates agent positions.
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

/// Prepares the signal grid for the current tick.
///
/// *   Clears the previous tick's transient signals.
/// *   Applies any signals delayed from the previous tick (via `#`).
/// *   Activates Source runes (`!`) to emit their values.
fn prepare_signals(vm: &mut ChimeraVM, grid: &[Vec<Value>]) {
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
            if s == "!" && *x > 0 {
                let val = grid[*y][*x - 1].clone();
                // Only emit truthy values? Or all values?
                // Let's emit non-empty signals.
                if !is_empty_val(&val) {
                    vm.prologue_state.signal_grid[*y][*x] = Some(val);
                }
            }
        }
    }
}

/// Iteratively propagates signals across the grid.
///
/// Simulates instantaneous travel through wires and logic gates.
/// Iteration continues until the grid state stabilizes (no changes) or `max_iterations` is reached.
/// This allows signals to travel the entire width of the grid in a single tick.
fn process_signal_propagation(vm: &mut ChimeraVM, grid: &[Vec<Value>]) {
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
                    &vm.dna,
                    &vm.prologue_state.signal_grid,
                    &mut next_signals,
                    &mut vm.prologue_state.delayed_signals,
                    &mut vm.ether,
                    &mut vm.prologue_state.registers,
                    &mut vm.prologue_state.teleport_channels,
                    &mut vm.prologue_state.history,
                    grid,
                    &vm.light_grid,
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

/// Applies the logic for a single "Active" Rune (Wire, Gate, Math, etc.).
///
/// Returns `true` if the signal state changed, prompting another propagation iteration.
#[allow(clippy::too_many_arguments)]
fn apply_propagation_rune(
    rune: &str,
    y: usize,
    x: usize,
    tick: u64,
    dna: &crate::ast::Dna,
    current_signals: &[Vec<Option<Value>>],
    next_signals: &mut Vec<Vec<Option<Value>>>,
    next_delayed: &mut Vec<Vec<Option<Value>>>,
    ether: &mut HashMap<i64, VecDeque<Value>>,
    registers: &mut HashMap<(usize, usize), Value>,
    teleport_channels: &mut HashMap<i64, Value>,
    history: &mut HashMap<(usize, usize), VecDeque<Value>>,
    grid: &[Vec<Value>],
    light_grid: &[Vec<i64>],
) -> bool {
    if construct::apply_construct_runes(rune, y, x, current_signals, next_delayed, grid) {
        return true;
    }
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
    if void::apply_void_runes(rune, y, x, current_signals, next_signals) {
        return true;
    }
    if virology::apply_virology_runes(rune, y, x, current_signals, next_signals) {
        return true;
    }
    if biolum::apply_biolum_runes(rune, y, x, current_signals, next_signals, light_grid) {
        return true;
    }
    if chaos::apply_chaos_runes(rune, y, x, current_signals, next_signals) {
        return true;
    }
    if necromancy::apply_necromancy_runes(rune, y, x, current_signals, next_signals) {
        return true;
    }
    false
}

/// Scans the grid for Sink Runes and triggers their side effects.
///
/// Sinks are runes that consume signals to interact with the world (Logging, VM, Grid).
/// They do not propagate signals further in the same tick (usually).
fn process_sinks(vm: &mut ChimeraVM, grid: &[Vec<Value>]) {
    let runes: Vec<(usize, usize)> = vm.prologue_state.runes.iter().cloned().collect();

    for (y, x) in &runes {
        if let Value::Str(s) = &grid[*y][*x] {
            apply_sink_rune(vm, s, *y, *x);
        }
    }
}

/// Executes the logic for a Sink Rune.
///
/// Handles `?` (Sink), `$` (Scribe), `M` (Mutate), `O` (Organelle), etc.
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
                    vm.ether.entry(*channel).or_default().push_back(val.clone());
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
        _ => {
            construct::apply_construct_sinks(vm, rune, y, x);
            evolution::apply_evolution_sinks(vm, rune, y, x);
            virology::apply_virology_sinks(vm, rune, y, x);
            biolum::apply_biolum_sinks(vm, rune, y, x);
            chaos::apply_chaos_sinks(vm, rune, y, x);
            necromancy::apply_necromancy_sinks(vm, rune, y, x);
            symbiosis::apply_symbiosis_sinks(vm, rune, y, x);
        }
    }
}

/// Updates the position of Prologue Agents (`@`, `K`, `H`, `C`).
///
/// Agents observe the grid (Snapshot) and move towards interesting features (Signals, Prey).
/// This function updates both the `agents` list in the state and the `grid` itself (moving the char).
fn process_agents(vm: &mut ChimeraVM, grid_snapshot: &[Vec<Value>]) {
    // Agents move towards signal
    // We need to update agents list in state, and also update the Grid (move the '@' char)
    // This requires mutable access to grid.

    // We iterate agents from state (snapshot) and update grid.
    let agents = vm.prologue_state.agents.clone();
    let mut new_agents = Vec::new();

    for mut agent in agents {
        let (y, x) = (agent.y, agent.x);

        let mut current_type = "@".to_string();
        if let Value::Str(s) = &grid_snapshot[y][x] {
            current_type = s.clone();
        }

        let mut target = None;
        let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        if current_type == "C" {
            // Critter: Genetic Movement
            if let Value::Str(state_str) = &agent.state {
                if let Some(mut critter) = critter::CritterState::parse(state_str) {
                    let (ny, nx) = critter::process_critter_move(&mut critter, y, x, grid_snapshot);
                    agent.state = critter.to_value();

                    // Check collision
                    let dest_val = &vm.grid[ny][nx]; // Check LIVE grid

                    let mut blocked = false;
                    if let Value::Str(s) = dest_val {
                        if s == "C" && (ny != y || nx != x) {
                            // Collided with another Critter (or self if didn't move)
                            // Breed?
                            // For simplicity, just block and breed nearby if possible
                            let mut rng = rand::thread_rng();
                            let spawn_dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];
                            let (dy, dx) = spawn_dirs[rng.gen_range(0..4)];
                            if let Some((sy, sx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
                                if matches!(vm.grid[sy][sx], Value::Int(0)) {
                                    // Get other critter state
                                    let other_state = vm.prologue_state.registers.get(&(ny, nx))
                                        .and_then(|v| if let Value::Str(s) = v { critter::CritterState::parse(s) } else { None })
                                        .unwrap_or(critter::CritterState::default());

                                    let child = critter::breed(&critter, &other_state);
                                    vm.grid[sy][sx] = Value::Str("C".to_string());
                                    vm.prologue_state.registers.insert((sy, sx), child.to_value());
                                }
                            }
                            blocked = true;
                        } else if s == "!" {
                            // Eat Food
                            critter.energy += 20;
                            agent.state = critter.to_value();
                        } else if !is_empty_val(dest_val) {
                             blocked = true;
                        }
                    } else if !is_empty_val(dest_val) {
                         blocked = true;
                    }

                    if !blocked {
                        target = Some((ny, nx));
                    }

                    // Update state in registers regardless of move (energy usage)
                    // If moving, we handle it below. If blocked, update here?
                    // The logic below updates registers for target.
                    // If blocked, target is None, so we should update (y,x) register.
                    if target.is_none() {
                         vm.prologue_state.registers.insert((y, x), agent.state.clone());
                    }

                    // Check death
                    if critter.energy <= 0 {
                         vm.grid[y][x] = Value::Int(0);
                         vm.prologue_state.registers.remove(&(y, x));
                         continue; // Remove from agents list
                    }
                }
            }
        } else if current_type == "K" {
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

            // Move Registers
            if current_type == "C" {
                vm.prologue_state.registers.remove(&(y, x));
                vm.prologue_state.registers.insert((ny, nx), agent.state.clone());
            }

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

/// Helper to safely convert (i64, i64) coordinates to (usize, usize).
///
/// Returns `None` if the coordinates are out of bounds (0..GRID_SIZE).
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
