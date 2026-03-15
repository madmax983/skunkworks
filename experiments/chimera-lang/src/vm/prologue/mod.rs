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
//! 2.  **Reality**: Update reality bubbles from World Runes (🌐).
//! 3.  **Signal**: Activate `!` Source runes and release pending delayed signals.
//! 4.  **Propagate**: Spread signals through Wires (`~`) and process Logic Gates (`&`, `|`, `+`, etc.).
//!     *   This phase iterates until the grid stabilizes (up to a limit).
//! 5.  **Physics**: Execute local reality physics (Silicon CA, Life).
//! 6.  **Sink**: Active Sinks (`?`, `$`, `M`, etc.) consume signals and perform actions (Logging, Gene Execution, Grid Writes).
//! 7.  **Agent**: Agents (`@`, `K`, `H`) perceive their surroundings and move.
//!
//! ## Rune Reference
//!
//! | Category | Runes | Description |
//! |---|---|---|
//! | **Source/Sink** | `!` | **Source**: Reads West, Emits to Self. |
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
//! | **Alchemy** | `t` | **Transmute**: West (Val), North (Mode) -> Self. |
//! | | `f` | **Fuse**: West + East -> Self (Concat/Add). |
//! | | `d` | **Distill**: West -> North (Head), South (Tail). |
//! | **Agents** | `@` | **Seeker**: Moves towards signals. |
//! | | `K` | **Chaos**: Moves randomly. |
//! | | `H` | **Hunter**: Chases other agents. |
//! | | `O` | **Organelle**: Spawns an agent. |
//! | **IO** | `Y` | **Yell**: Pushes to Ether Channel. |
//! | | `L` | **Listen**: Pops from Ether Channel. |
//! | **Flow** | `^` | **Jump**: West -> East (Teleport). |
//! | | `\` | **Mirror**: Reflects 90° (N<->E). |
//! | **Reality** | `🌐` | **World**: Defines local physics (West=Radius, North=Mode). |

use crate::vm::{ChimeraVM, Value, GRID_SIZE};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};

pub mod alchemist;
pub mod alchemy;
pub mod altar;
pub mod architect;
pub mod astral;
pub mod automaton;
pub mod biolum;
pub mod catalyst;
pub mod chaos;
pub mod chroma;
pub mod chromatin;
pub mod chronos;
pub mod construct;
pub mod critter;
pub mod echo;
pub mod elektra;
pub mod elemental;
pub mod epigenetics;
pub mod evolution;
pub mod fission;
pub mod forth;
pub mod gardener;
pub mod genetics;
pub mod golem;
pub mod hyper;
pub mod io;
pub mod lexicon;
pub mod library;
pub mod ligase;
pub mod linguistics;
pub mod list;
pub mod logic;
pub mod logic_agent;
pub mod logos;
pub mod math;
pub mod memetics;
pub mod mesmerist;
pub mod mycelium;
pub mod narrative;
pub mod necromancy;
#[cfg(feature = "biophysics")]
pub mod neural;
pub mod oneiric;
pub mod optics;
pub mod oracle;
pub mod pandemonium;
pub mod phage;
pub mod philosopher;
pub mod phonetics;
pub mod pilot;
pub mod plasmid;
pub mod prism;
pub mod psionics;
pub mod quantum;
pub mod resonance;
pub mod rhythm;
pub mod ribozyme;
pub mod runecraft;
pub mod scavenger;
pub mod scholar;
pub mod sequencer;
pub mod siren;
pub mod splicer;
pub mod symbiosis;
pub mod teleport;
pub mod topology;
pub mod virology;
pub mod void;
pub mod weave;
pub mod weave_reality;
pub mod weaver;
pub mod wizard;
pub mod zeta;

// Golem Materials (Public for shared use)
pub const MAT_HEART: &str = "🗿";
pub const MAT_EARTH: &str = "◊";
pub const MAT_BRICK: &str = "🧱";
pub const MAT_WALL: &str = "#";
pub const MAT_SHIELD: &str = "🛡️";

/// An autonomous agent wandering the Prologue grid.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrologueAgent {
    /// X Coordinate (Column)
    pub x: usize,
    /// Y Coordinate (Row)
    pub y: usize,
    /// Internal state memory
    pub state: Value,
    /// Stack memory for Forth Interpreter
    #[serde(default)]
    pub stack: Vec<Value>,
}

/// A dynamic transmutation rule for Hermetic Alchemy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlchemyRule {
    pub ingredients: Vec<Value>,
    pub result: Value,
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
    /// Echo buffers for recording/playback.
    pub echoes: HashMap<(usize, usize), echo::EchoBuffer>,
    /// Historical data for time-travel runes.
    pub history: HashMap<(usize, usize), VecDeque<Value>>,
    /// Epigenetic layer (Methylation/Phosphorylation).
    #[serde(default = "default_epigenetic_grid")]
    pub epigenetic_grid: Vec<Vec<epigenetics::EpigeneticMark>>,
    /// Oneiric Grid: Parallel Dream Simulation.
    #[serde(default)]
    pub oneiric_grid: oneiric::OneiricGrid,
    /// Narrative Library (Book Rune Storage).
    #[serde(default)]
    pub library: HashMap<String, Value>,
    /// Logos Engine (Grammar System).
    #[serde(default = "default_logos_engine")]
    pub logos_engine: logos::LogosEngine,
    /// Orca Mode: Enables omni-directional signal flow and alternative rune behavior.
    #[serde(default)]
    pub orca_mode: bool,
    /// Void Buffer: Global LIFO storage for Void runes.
    #[serde(default)]
    pub void_buffer: VecDeque<Value>,
    /// Hyper State (4D Coordinates)
    #[serde(default)]
    pub hyper_state: hyper::HyperState,
    /// Rhythm State (Sequencer/Clock)
    #[serde(default)]
    pub rhythm_state: rhythm::RhythmState,
    /// Mycelium Network (Active Spores)
    #[serde(default)]
    pub mycelium_network: HashSet<(usize, usize)>,
    /// Mycelium Buffer (Shared Network Storage)
    #[serde(default)]
    pub mycelium_buffer: VecDeque<Value>,
    /// Scratch buffer for signal propagation (Double Buffering).
    #[serde(skip, default)]
    pub scratch_signal_grid: Vec<Vec<Option<Value>>>,
    /// Custom Runes (User Defined).
    #[serde(default)]
    pub custom_runes: HashMap<String, usize>,
    /// Reality State (Physics Modes).
    #[serde(default)]
    pub reality_state: weave_reality::RealityState,
    /// Hermetic Alchemy Book (User Defined Transmutations).
    #[serde(default)]
    pub alchemy_book: Vec<AlchemyRule>,
}

fn default_logos_engine() -> logos::LogosEngine {
    logos::LogosEngine::new()
}

fn default_epigenetic_grid() -> Vec<Vec<epigenetics::EpigeneticMark>> {
    vec![vec![epigenetics::EpigeneticMark::None; GRID_SIZE]; GRID_SIZE]
}

impl Default for PrologueState {
    fn default() -> Self {
        Self::new()
    }
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
            echoes: HashMap::new(),
            history: HashMap::new(),
            epigenetic_grid: vec![vec![epigenetics::EpigeneticMark::None; GRID_SIZE]; GRID_SIZE],
            oneiric_grid: oneiric::OneiricGrid::new(),
            library: HashMap::new(),
            logos_engine: logos::LogosEngine::new(),
            orca_mode: false,
            void_buffer: VecDeque::new(),
            hyper_state: hyper::HyperState::default(),
            rhythm_state: rhythm::RhythmState::default(),
            mycelium_network: HashSet::new(),
            mycelium_buffer: VecDeque::new(),
            scratch_signal_grid: vec![vec![None; GRID_SIZE]; GRID_SIZE],
            custom_runes: HashMap::new(),
            reality_state: weave_reality::RealityState::default(),
            alchemy_book: Vec::new(),
        }
    }

    /// Scans the entire grid to identify Runes and Agents.
    pub fn scan_grid_rules(&mut self, grid: &[Vec<Value>]) {
        self.runes.clear();
        self.rules.clear();
        self.agents.clear();

        for (y, row) in grid.iter().enumerate().take(GRID_SIZE) {
            for (x, cell) in row.iter().enumerate().take(GRID_SIZE) {
                if let Value::Str(s) = cell {
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
                            | "♦"
                            | "•"
                            | "°"
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
                            | ")"
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
                            | "8"
                            | "{"
                            | "}"
                            | "s"
                            | "g"
                            | "r"
                            | "c"
                            | "t"
                            | "f"
                            | "d"
                            | "e"
                            | "b"
                            | "l"
                            | "n"
                            | "∞"
                            | "Ð"
                            | "µ"
                            | "Ø"
                            | "§"
                            | "ꝏ"
                            | "B"
                            | "Π"
                            | "🤖"
                            | "⟳"
                            | "↔"
                            | "↕"
                            | "❏"
                            | "▓"
                            | "░"
                            | "v"
                            | "i"
                            | "a"
                            | "Φ"
                            | "Λ"
                            | "Ω"
                            | "🎓"
                            | "k"
                            | "z"
                            | "h"
                            | "†"
                            | "‡"
                            | "Ψ"
                            | "u"
                            | "y"
                            | "w"
                            | "j"
                            | "x"
                            | "p"
                            | "o"
                            | "¿"
                            | "¡"
                            | "≈"
                            | "."
                            | ":"
                            | ","
                            | "☣"
                            | "♻"
                            | "χ"
                            | "Δ"
                            | "∇"
                            | "◊"
                            | "○"
                            | "☆"
                            | "☿"
                            | "☢"
                            | "✇"
                            | "⌘"
                            | "✦"
                            | "☾"
                            | "☀"
                            | "⚡"
                            | "≡"
                            | "∿"
                            | "🔌"
                            | "💡"
                            | "🔋"
                            | "♒"
                            | "⇝"
                            | "⏧"
                            | "¶"
                            | "λ"
                            | "¥"
                            | "∃"
                            | "Θ"
                            | "Ξ"
                            | "Σ"
                            | "♪"
                            | "♫"
                            | "🥁"
                            | "▲"
                            | "▼"
                            | "🧬"
                            | "⚛"
                            | "⚒"
                            | "🧶"
                            | "💉"
                            | "\""
                            | "®"
                            | ";"
                            | "©"
                            | "↑"
                            | "↓"
                            | "≅"
                            | "ι"
                            | "κ"
                            | "ε"
                            | "σ"
                            | "φ"
                            | "Æ"
                            | "α"
                            | "ω"
                            | "✍"
                            | "📖"
                            | "📚"
                            | "🔖"
                            | "🎨"
                            | "🖌"
                            | "👁"
                            | "🔴"
                            | "🟢"
                            | "🔵"
                            | "♬"
                            | "Γ"
                            | "«"
                            | "»"
                            | "η"
                            | "γ"
                            | "₣"
                            | "⚓"
                            | "ζ"
                            | "⇪"
                            | "↻"
                            | "⌖"
                            | "▣"
                            | "⏱️"
                            | "🎹"
                            | "🎚️"
                            | "🔍"
                            | "✏"
                            | "🗑"
                            | "➕"
                            | "🍄"
                            | "📥"
                            | "📤"
                            | "🦋"
                            | "🌱"
                            | "🕷"
                            | "£"
                            | "✂"
                            | "🔗"
                            | "🦠"
                            | "🛠"
                            | "⨁"
                            | "🌀"
                            | "⚗"
                            | "ð"
                            | "║"
                            | "♣"
                            | "🧙"
                            | "⛩"
                            | "💤"
                            | "👹"
                            | "★"
                            | "🌐"
                            | "🗿"
                            | "♨"
                            | "Ϡ"
                    ) {
                        self.runes.insert((y, x));
                        self.register_agent(s, y, x);
                    } else if self.custom_runes.contains_key(s) {
                        self.runes.insert((y, x));
                    }
                }
            }
        }
    }

    fn register_agent(&mut self, s: &str, y: usize, x: usize) {
        if s == "@"
            || s == "K"
            || s == "H"
            || (s == "C" && !self.orca_mode)
            || s == "♻"
            || s == "♬"
            || s == "🌀"
            || s == "⚗"
            || s == "₣"
            || s == "ζ"
            || s == "Φ"
            || s == "P"
            || s == "⚓"
            || s == "∃"
            || s == "χ"
            || s == "🕷"
            || s == "✂"
            || s == "🔗"
            || s == "🦠"
            || s == "🛠"
            || s == "🎓"
            || s == "ð"
            || s == "♣"
            || s == "🧙"
            || s == "💤"
            || s == "👹"
            || s == "★"
            || s == "🤖"
            || s == "🗿"
        {
            let raw_state = self
                .registers
                .get(&(y, x))
                .cloned()
                .unwrap_or(Value::Int(0));
            let (state, stack) = unpack_agent_data(raw_state);
            let final_state = if matches!(state, Value::Int(0)) {
                // Initialize default state if needed
                Value::Int(0)
            } else {
                state
            };
            self.agents.push(PrologueAgent {
                x,
                y,
                state: final_state,
                stack,
            });
        }
    }
}

pub fn exec_prologue_tick(vm: &mut ChimeraVM) {
    if !vm.prologue_state.active {
        return;
    }

    lexicon::process_lexicon(vm);

    let grid_snapshot = vm.grid.clone();
    vm.prologue_state.scan_grid_rules(&grid_snapshot);

    weave_reality::scan_reality_bubbles(vm);

    #[cfg(feature = "biophysics")]
    neural::scan_neural_grid(vm);

    prepare_signals(vm, &grid_snapshot);

    #[cfg(feature = "biophysics")]
    neural::fire_neurons(vm);

    process_signal_propagation(vm, &grid_snapshot);
    process_reality_physics(vm);

    process_sinks(vm, &grid_snapshot);

    #[cfg(feature = "biophysics")]
    neural::integrate_neurons(vm);

    process_agents(vm, &grid_snapshot);
    oneiric::process_oneiric_tick(vm);

    #[cfg(feature = "oracle")]
    {
        if !vm.omens.is_empty() {
            crate::vm::oracle::process_omens(vm);
        }
    }
}

fn process_reality_physics(vm: &mut ChimeraVM) {
    // Collect updates first to avoid in-place mutation artifacts
    let mut updates = Vec::new();

    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            let mode = vm.prologue_state.reality_state.get_mode(y, x);
            match mode {
                weave_reality::RealityMode::Silicon => {
                    #[cfg(feature = "silicon")]
                    if let Some(new_val) = crate::vm::silicon::step_cell_wireworld(vm, y, x) {
                        updates.push((y, x, new_val));
                    }
                }
                weave_reality::RealityMode::Life => {
                    // Implement Life CA step here or call helper
                    // Count neighbors
                    let mut neighbors = 0;
                    for dy in -1..=1 {
                        for dx in -1..=1 {
                            if dy == 0 && dx == 0 {
                                continue;
                            }
                            if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
                                if let Value::Int(n) = vm.grid[ny][nx] {
                                    if n > 0 {
                                        neighbors += 1;
                                    }
                                }
                            }
                        }
                    }
                    let current = if let Value::Int(n) = vm.grid[y][x] {
                        n > 0
                    } else {
                        false
                    };
                    let next = if current {
                        neighbors == 2 || neighbors == 3
                    } else {
                        neighbors == 3
                    };
                    if next != current {
                        updates.push((y, x, Value::Int(if next { 1 } else { 0 })));
                    }
                }
                _ => {}
            }
        }
    }

    // Apply updates
    for (y, x, val) in updates {
        vm.grid[y][x] = val;
    }
}

fn prepare_signals(vm: &mut ChimeraVM, grid: &[Vec<Value>]) {
    /// Clears and reuses grid memory via double buffering per memory guidelines to avoid costly allocations
    for row in vm.prologue_state.signal_grid.iter_mut() {
        row.fill(None);
    }
    for (y, row) in vm
        .prologue_state
        .signal_grid
        .iter_mut()
        .enumerate()
        .take(GRID_SIZE)
    {
        for (x, cell) in row.iter_mut().enumerate().take(GRID_SIZE) {
            if let Some(val) = &vm.prologue_state.delayed_signals[y][x] {
                *cell = Some(val.clone());
            }
        }
    }
    for row in vm.prologue_state.delayed_signals.iter_mut() {
        row.fill(None);
    }

    let runes: Vec<(usize, usize)> = vm.prologue_state.runes.iter().cloned().collect();

    for (y, x) in &runes {
        if vm.prologue_state.epigenetic_grid[*y][*x] == epigenetics::EpigeneticMark::Methylated {
            continue;
        }

        if let Value::Str(s) = &grid[*y][*x] {
            if s == "!" && *x > 0 {
                let val = grid[*y][*x - 1].clone();
                if !is_empty_val(&val) {
                    vm.prologue_state.signal_grid[*y][*x] = Some(val);
                }
            }
            fission::prepare_fission_sources(s, *y, *x, &mut vm.prologue_state.signal_grid);
            if s == "🌀" {
                mesmerist::emit_gaze(vm, *y, *x);
            }
        }
    }
}

fn process_signal_propagation(vm: &mut ChimeraVM, grid: &[Vec<Value>]) {
    let max_iterations = GRID_SIZE * 2;
    let runes: Vec<(usize, usize)> = vm.prologue_state.runes.iter().cloned().collect();
    let tick = vm.tick_counter;

    if vm.prologue_state.scratch_signal_grid.len() != GRID_SIZE {
        vm.prologue_state.scratch_signal_grid = vec![vec![None; GRID_SIZE]; GRID_SIZE];
    }

    for _ in 0..max_iterations {
        let mut changes = false;
        let mut next_signals = std::mem::take(&mut vm.prologue_state.scratch_signal_grid);
        if next_signals.len() != GRID_SIZE {
            next_signals = vec![vec![None; GRID_SIZE]; GRID_SIZE];
        }
        next_signals.clone_from(&vm.prologue_state.signal_grid);

        for (y, x) in &runes {
            if vm.prologue_state.epigenetic_grid[*y][*x] == epigenetics::EpigeneticMark::Methylated
            {
                continue;
            }

            let mode = vm.prologue_state.reality_state.get_mode(*y, *x);
            let global_orca = vm.prologue_state.orca_mode;
            let effective_orca = global_orca || mode == weave_reality::RealityMode::Orca;

            if let Value::Str(s) = &grid[*y][*x] {
                #[cfg(feature = "elektra")]
                let (v_grid, r_grid, c_grid) = (
                    &mut vm.voltage_grid,
                    &mut vm.resistance_grid,
                    &mut vm.capacitance_grid,
                );
                #[cfg(not(feature = "elektra"))]
                let (v_grid, r_grid, c_grid) = (&mut vec![], &mut vec![], &mut vec![]);

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
                    &mut vm.prologue_state.echoes,
                    &mut vm.prologue_state.history,
                    &mut vm.prologue_state.void_buffer,
                    &mut vm.prologue_state.mycelium_buffer,
                    &mut vm.void_rifts,
                    grid,
                    &vm.light_grid,
                    &mut vm.prologue_state.oneiric_grid,
                    v_grid,
                    r_grid,
                    c_grid,
                    &mut vm.energy,
                    &vm.chroma_grid,
                    &mut vm.prologue_state.logos_engine,
                    effective_orca,
                    &mut vm.prologue_state.hyper_state,
                    &mut vm.prologue_state.rhythm_state,
                    &vm.prologue_state.alchemy_book,
                    #[cfg(feature = "resonance")]
                    &vm.audio_tx,
                    #[cfg(not(feature = "resonance"))]
                    &None,
                    &mut vm.output,
                ) {
                    changes = true;
                }
            }
        }

        std::mem::swap(&mut vm.prologue_state.signal_grid, &mut next_signals);
        vm.prologue_state.scratch_signal_grid = next_signals;

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
    dna: &crate::ast::Dna,
    current_signals: &[Vec<Option<Value>>],
    next_signals: &mut [Vec<Option<Value>>],
    next_delayed: &mut Vec<Vec<Option<Value>>>,
    ether: &mut HashMap<i64, VecDeque<Value>>,
    registers: &mut HashMap<(usize, usize), Value>,
    teleport_channels: &mut HashMap<i64, Value>,
    echoes: &mut HashMap<(usize, usize), echo::EchoBuffer>,
    history: &mut HashMap<(usize, usize), VecDeque<Value>>,
    void_buffer: &mut VecDeque<Value>,
    mycelium_buffer: &mut VecDeque<Value>,
    void_rifts: &mut [crate::vm::nova_void::VoidRift],
    grid: &[Vec<Value>],
    light_grid: &[Vec<i64>],
    oneiric_grid: &mut oneiric::OneiricGrid,
    voltage_grid: &mut Vec<Vec<f32>>,
    resistance_grid: &mut Vec<Vec<f32>>,
    capacitance_grid: &mut Vec<Vec<f32>>,
    energy: &mut i64,
    chroma_grid: &[Vec<crate::vm::ChromaCell>],
    logos_engine: &mut logos::LogosEngine,
    orca_mode: bool,
    hyper_state: &mut hyper::HyperState,
    rhythm_state: &mut rhythm::RhythmState,
    alchemy_book: &[AlchemyRule],
    #[cfg(feature = "resonance")] audio_tx: &Option<
        crossbeam_channel::Sender<resonance_audio::audio::AudioCommand>,
    >,
    #[cfg(not(feature = "resonance"))] audio_tx: &Option<()>,
    output: &mut Vec<String>,
) -> bool {
    if chroma::apply_chroma_runes(rune, y, x, current_signals, next_signals, chroma_grid) {
        return true;
    }

    #[cfg(feature = "elektra")]
    if elektra::apply_elektra_runes(
        rune,
        y,
        x,
        tick,
        current_signals,
        next_signals,
        voltage_grid,
        resistance_grid,
        capacitance_grid,
        energy,
        registers,
    ) {
        return true;
    }

    #[cfg(feature = "biophysics")]
    if neural::apply_neural_runes(rune, y, x, current_signals, next_signals, registers) {
        return true;
    }

    if oneiric::apply_oneiric_runes(rune, y, x, current_signals, next_signals, oneiric_grid) {
        return true;
    }
    if construct::apply_construct_runes(rune, y, x, current_signals, next_delayed, grid) {
        return true;
    }
    if architect::apply_architect_runes(rune, y, x, current_signals, next_signals) {
        return true;
    }
    if topology::apply_topology_runes(topology::TopologyArgs {
        rune,
        y,
        x,
        tick,
        current_signals,
        next_signals,
        next_delayed,
        orca_mode,
    }) {
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
        orca_mode,
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
    if echo::apply_echo_runes(rune, y, x, current_signals, next_signals, echoes, tick) {
        return true;
    }
    if alchemy::apply_alchemy_runes(rune, y, x, current_signals, next_signals) {
        return true;
    }
    if fission::apply_fission_runes(rune, y, x, current_signals, next_signals) {
        return true;
    }
    if rune == "G" && genetics::apply_genesis_rune(y, x, current_signals, next_signals) {
        return true;
    }
    if evolution::apply_evolution_runes(rune, y, x, dna, current_signals, next_signals) {
        return true;
    }
    if void::apply_void_runes(
        rune,
        y,
        x,
        current_signals,
        next_signals,
        void_buffer,
        void_rifts,
    ) {
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
    if pandemonium::apply_pandemonium_runes(rune, y, x, current_signals, next_signals) {
        return true;
    }
    if elemental::apply_elemental_runes(rune, y, x, current_signals, next_signals, alchemy_book) {
        return true;
    }
    if psionics::apply_psionics_runes(rune, y, x, current_signals, next_signals, grid) {
        return true;
    }
    if prism::apply_prism_runes(rune, y, x, current_signals, next_signals, dna) {
        return true;
    }
    if memetics::apply_memetic_runes(rune, y, x, current_signals, next_signals, grid) {
        return true;
    }
    if linguistics::apply_linguistics_runes(rune, y, x, current_signals, next_signals) {
        return true;
    }
    if narrative::apply_narrative_runes(rune, y, x, current_signals, next_signals) {
        return true;
    }
    if logos::apply_logos_runes(rune, y, x, current_signals, next_signals, logos_engine, dna) {
        return true;
    }
    if hyper::apply_hyper_runes(rune, y, x, current_signals, next_signals, hyper_state) {
        return true;
    }
    if rhythm::apply_rhythm_runes(
        rune,
        y,
        x,
        tick,
        current_signals,
        next_signals,
        rhythm_state,
        audio_tx,
        output,
    ) {
        return true;
    }
    if sequencer::apply_sequencer_runes(rune, y, x, dna, current_signals, next_signals) {
        return true;
    }
    if mycelium::apply_mycelium_runes(rune, y, x, current_signals, next_signals, mycelium_buffer) {
        return true;
    }
    if catalyst::apply_catalyst_runes(rune, y, x, current_signals, next_signals) {
        return true;
    }
    false
}

fn process_sinks(vm: &mut ChimeraVM, grid: &[Vec<Value>]) {
    let runes: Vec<(usize, usize)> = vm.prologue_state.runes.iter().cloned().collect();

    for (y, x) in &runes {
        if let Value::Str(s) = &grid[*y][*x] {
            let mark = vm.prologue_state.epigenetic_grid[*y][*x];
            if mark == epigenetics::EpigeneticMark::Methylated {
                continue;
            }

            let iterations = if mark == epigenetics::EpigeneticMark::Phosphorylated {
                2
            } else {
                1
            };

            for _ in 0..iterations {
                apply_sink_rune(vm, s, *y, *x);
            }
        }
    }
}

fn apply_sink_rune(vm: &mut ChimeraVM, rune: &str, y: usize, x: usize) {
    if let Some(&strand_idx) = vm.prologue_state.custom_runes.get(rune) {
        let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        let mut triggered = false;

        for (dy, dx) in neighbors {
            if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
                if vm.prologue_state.signal_grid[ny][nx].is_some() {
                    triggered = true;
                    break;
                }
            }
        }

        if triggered && vm.ip.0 != strand_idx {
            vm.context_loc = (y, x);
            vm.interrupt(strand_idx);
            vm.output
                .push(format!("RUNECRAFT: Executed Custom Rune '{}'", rune));
            vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1));
        }
        return;
    }

    match rune {
        "🎓" => {
            if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
                if let Value::Str(s) = &vm.grid[wy][wx] {
                    if s == "@" {
                        if let Some((ny, nx)) = normalize_coords(y as i64 - 1, x as i64) {
                            if let Some(Value::Str(goal)) = &vm.prologue_state.signal_grid[ny][nx] {
                                if let Some((sy, sx)) = normalize_coords(y as i64 + 1, x as i64) {
                                    vm.grid[sy][sx] = Value::Str("Φ".to_string());
                                    vm.prologue_state
                                        .registers
                                        .insert((sy, sx), Value::Str(goal.clone()));
                                    vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1));
                                }
                            }
                        }
                    }
                }
            }
        }
        "?" => {
            let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1), (0, 0)];
            let mut found_signals = Vec::new();

            for (dy, dx) in neighbors {
                if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
                    if let Some(sig) = &vm.prologue_state.signal_grid[ny][nx] {
                        found_signals.push(sig.clone());
                    }
                }
            }

            for sig in found_signals {
                vm.output
                    .push(format!("PROLOGUE: Sink at {},{} received {:?}", x, y, sig));
                if let Value::Str(name) = sig {
                    if let Some(&idx) = vm.dictionary.get(&name) {
                        vm.interrupt(idx);
                    }
                }
            }
        }
        "$" => {
            if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
                if let Some(sig) = &vm.prologue_state.signal_grid[wy][wx] {
                    if let Some((sy, sx)) = normalize_coords(y as i64 + 1, x as i64) {
                        vm.grid[sy][sx] = sig.clone();
                        vm.prologue_state.signal_grid[y][x] = Some(sig.clone());
                    }
                }
            }
        }
        "M" => {
            if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
                if vm.prologue_state.signal_grid[wy][wx].is_some() {
                    if let Some((sy, sx)) = normalize_coords(y as i64 + 1, x as i64) {
                        let mut rng = rand::thread_rng();
                        let val = rng.gen_range(0..100);
                        vm.grid[sy][sx] = Value::Int(val);
                        vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1));
                    }
                }
            }
        }
        "O" => {
            if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
                if let Some(sig) = &vm.prologue_state.signal_grid[wy][wx] {
                    if let Some((sy, sx)) = normalize_coords(y as i64 + 1, x as i64) {
                        let agent_type = match sig {
                            Value::Int(2) => "K",
                            Value::Int(3) => "H",
                            _ => "@",
                        };
                        vm.grid[sy][sx] = Value::Str(agent_type.to_string());
                        vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1));
                    }
                }
            }
        }
        "E" => {
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
            let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
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
            if let (Some((wy, wx)), Some((ey, ex))) = (
                normalize_coords(y as i64, x as i64 - 1),
                normalize_coords(y as i64, x as i64 + 1),
            ) {
                let w_sig = &vm.prologue_state.signal_grid[wy][wx];
                let e_sig = &vm.prologue_state.signal_grid[ey][ex];

                if let (Some(val), Some(Value::Int(channel))) = (w_sig, e_sig) {
                    vm.ether.entry(*channel).or_default().push_back(val.clone());
                    vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1));
                }
            }
        }
        "(" => {
            if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
                if vm.prologue_state.signal_grid[wy][wx].is_some() {
                    if let (Some((ny, nx)), Some((sy, sx))) = (
                        normalize_coords(y as i64 - 1, x as i64),
                        normalize_coords(y as i64 + 1, x as i64),
                    ) {
                        let n_val = vm.grid[ny][nx].clone();
                        let s_val = vm.grid[sy][sx].clone();
                        vm.grid[ny][nx] = s_val;
                        vm.grid[sy][sx] = n_val;
                        vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1));
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
            pandemonium::apply_pandemonium_sinks(vm, rune, y, x);
            epigenetics::apply_epigenetic_runes(vm, rune, y, x);
            scavenger::apply_scavenger_sinks(vm, rune, y, x);
            chronos::apply_chronos_sinks(vm, rune, y, x);
            quantum::apply_quantum_sinks(vm, rune, y, x);
            #[cfg(feature = "oracle")]
            oracle::apply_oracle_sinks(vm, rune, y, x);
            psionics::apply_psionics_sinks(vm, rune, y, x);
            resonance::apply_resonance_sinks(vm, rune, y, x);
            prism::apply_prism_sinks(vm, rune, y, x);
            memetics::apply_memetic_sinks(vm, rune, y, x);
            narrative::apply_narrative_sinks(vm, rune, y, x);
            chroma::apply_chroma_sinks(vm, rune, y, x);
            sequencer::apply_sequencer_sinks(vm, rune, y, x);
            mycelium::apply_mycelium_sinks(vm, rune, y, x);
            logos::apply_logos_sinks(vm, rune, y, x);
            #[cfg(feature = "biophysics")]
            neural::apply_neural_sinks(vm, rune, y, x);

            library::apply_library_sinks(vm, rune, y, x);
            runecraft::apply_runecraft_sinks(vm, rune, y, x);
            phonetics::apply_phonetic_sinks(vm, rune, y, x);
            weave::apply_weave_sinks(vm, rune, y, x);
            catalyst::apply_catalyst_sinks(vm, rune, y, x);
            if rune == "Z" {
                genetics::apply_ligation_rune(vm, y, x);
            }
            altar::apply_altar_runes(vm, rune, y, x);
        }
    }
}

fn process_critter_logic(
    vm: &mut ChimeraVM,
    agent: &PrologueAgent,
    grid_snapshot: &[Vec<Value>],
) -> Option<(PrologueAgent, Option<(usize, usize)>)> {
    let (y, x) = (agent.y, agent.x);
    if let Value::Str(state_str) = &agent.state {
        if let Ok(mut critter) = state_str.parse::<critter::CritterState>() {
            let action = critter::process_critter_move(&mut critter, y, x, grid_snapshot);
            let mut updated_agent = agent.clone();
            updated_agent.state = critter.to_value();

            if critter.energy <= 0 {
                vm.grid[y][x] = Value::Int(0);
                vm.prologue_state.registers.remove(&(y, x));
                return None;
            }

            match action {
                critter::CritterAction::Move(ny, nx) => {
                    let dest_val = &vm.grid[ny][nx];
                    let mut blocked = false;
                    let mut moved_target = None;

                    if let Value::Str(s) = dest_val {
                        if s == "C" && (ny != y || nx != x) {
                            let mut rng = rand::thread_rng();
                            let mut spawn_dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];
                            use rand::seq::SliceRandom;
                            spawn_dirs.shuffle(&mut rng);

                            for (dy, dx) in spawn_dirs {
                                if let Some((sy, sx)) =
                                    normalize_coords(y as i64 + dy, x as i64 + dx)
                                {
                                    if matches!(vm.grid[sy][sx], Value::Int(0)) {
                                        let other_state = vm
                                            .prologue_state
                                            .registers
                                            .get(&(ny, nx))
                                            .and_then(|v| {
                                                if let Value::Str(s) = v {
                                                    s.parse().ok()
                                                } else {
                                                    None
                                                }
                                            })
                                            .unwrap_or(critter::CritterState::default());

                                        let child = critter::breed(&critter, &other_state);
                                        vm.grid[sy][sx] = Value::Str("C".to_string());
                                        vm.prologue_state
                                            .registers
                                            .insert((sy, sx), child.to_value());
                                        break;
                                    }
                                }
                            }
                            blocked = true;
                        } else if s == "!" {
                            critter.energy += 20;
                            updated_agent.state = critter.to_value();
                            moved_target = Some((ny, nx));
                        } else if !is_empty_val(dest_val) {
                            blocked = true;
                        }
                    } else if !is_empty_val(dest_val) {
                        blocked = true;
                    }

                    if !blocked && moved_target.is_none() {
                        moved_target = Some((ny, nx));
                    }

                    if moved_target.is_none() {
                        vm.prologue_state
                            .registers
                            .insert((y, x), updated_agent.state.clone());
                    }

                    return Some((updated_agent, moved_target));
                }
                critter::CritterAction::Attack(ny, nx) => {
                    let dest_val = vm.grid[ny][nx].clone();
                    if let Value::Str(s) = dest_val {
                        if s == "@" || s == "K" || s == "H" || s == "C" {
                            vm.grid[ny][nx] = Value::Int(0);
                            if s == "C" {
                                vm.prologue_state.registers.remove(&(ny, nx));
                            }
                            critter.energy += 30;
                            updated_agent.state = critter.to_value();
                        }
                    }
                    vm.prologue_state
                        .registers
                        .insert((y, x), updated_agent.state.clone());
                    return Some((updated_agent, None));
                }
                critter::CritterAction::Split => {
                    if critter.energy > 50 {
                        let mut rng = rand::thread_rng();
                        let spawn_dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];
                        let (dy, dx) = spawn_dirs[rng.gen_range(0..4)];
                        if let Some((sy, sx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
                            if matches!(vm.grid[sy][sx], Value::Int(0)) {
                                let child = critter::breed(&critter, &critter);
                                vm.grid[sy][sx] = Value::Str("C".to_string());
                                vm.prologue_state
                                    .registers
                                    .insert((sy, sx), child.to_value());
                                critter.energy -= 30;
                                updated_agent.state = critter.to_value();
                            }
                        }
                    }
                    vm.prologue_state
                        .registers
                        .insert((y, x), updated_agent.state.clone());
                    return Some((updated_agent, None));
                }
                critter::CritterAction::Build(c, ny, nx) => {
                    let safe_to_build = match &vm.grid[ny][nx] {
                        Value::Int(0) => true,
                        Value::Str(s) => s == ".",
                        _ => false,
                    };

                    if safe_to_build {
                        vm.grid[ny][nx] = Value::Str(c.to_string());
                    }

                    vm.prologue_state
                        .registers
                        .insert((y, x), updated_agent.state.clone());
                    return Some((updated_agent, None));
                }
                critter::CritterAction::Mark => {
                    let (dy, dx) = match critter.direction {
                        0 => (-1, 0),
                        1 => (0, 1),
                        2 => (1, 0),
                        3 => (0, -1),
                        _ => (0, 0),
                    };
                    if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
                        if matches!(vm.grid[ny][nx], Value::Int(0)) {
                            vm.grid[ny][nx] = Value::Str(".".to_string());
                        }
                    }
                    vm.prologue_state
                        .registers
                        .insert((y, x), updated_agent.state.clone());
                    return Some((updated_agent, None));
                }
                critter::CritterAction::None => {
                    vm.prologue_state
                        .registers
                        .insert((y, x), updated_agent.state.clone());
                    return Some((updated_agent, None));
                }
            }
        }
    }
    Some((agent.clone(), None))
}

fn process_chaos_logic(
    _vm: &mut ChimeraVM,
    agent: &PrologueAgent,
    grid_snapshot: &[Vec<Value>],
) -> Option<(usize, usize)> {
    let (y, x) = (agent.y, agent.x);
    let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
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
        Some(possible_moves[idx])
    } else {
        None
    }
}

fn process_hunter_logic(
    vm: &mut ChimeraVM,
    agent: &PrologueAgent,
    grid_snapshot: &[Vec<Value>],
) -> Option<(usize, usize)> {
    let (y, x) = (agent.y, agent.x);
    let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    for (dy, dx) in neighbors {
        if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
            if let Value::Str(s) = &grid_snapshot[ny][nx] {
                if s == "@" || s == "K" {
                    return Some((ny, nx));
                }
            }
        }
    }

    for (dy, dx) in neighbors {
        if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
            if vm.prologue_state.signal_grid[ny][nx].is_some() {
                let cell = &grid_snapshot[ny][nx];
                if matches!(cell, Value::Int(0) | Value::Str(_)) {
                    return Some((ny, nx));
                }
            }
        }
    }
    None
}

fn process_seeker_logic(
    vm: &mut ChimeraVM,
    agent: &PrologueAgent,
    grid_snapshot: &[Vec<Value>],
) -> Option<(usize, usize)> {
    let (y, x) = (agent.y, agent.x);
    let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    for (dy, dx) in neighbors {
        if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
            if vm.prologue_state.signal_grid[ny][nx].is_some() {
                let cell = &grid_snapshot[ny][nx];
                match cell {
                    Value::Int(0) => return Some((ny, nx)),
                    Value::Str(s) if s == "~" => return Some((ny, nx)),
                    _ => {}
                }
            }
        }
    }
    None
}

fn process_agents(vm: &mut ChimeraVM, grid_snapshot: &[Vec<Value>]) {
    let agents = vm.prologue_state.agents.clone();
    let mut new_agents = Vec::new();
    let mut written_cells = HashSet::new();

    for mut agent in agents {
        let (y, x) = (agent.y, agent.x);

        let code_to_run = if let Some(Value::Str(code)) = &vm.prologue_state.signal_grid[y][x] {
            if code.len() > 1 && !code.starts_with('~') && !code.starts_with('!') {
                Some(code.clone())
            } else {
                None
            }
        } else {
            None
        };

        if let Some(code) = code_to_run {
            let wrapped_code = format!("strand mesmer {{ {} }}", code);
            match crate::compiler::compile(&wrapped_code, None) {
                Ok(dna) => {
                    if let Some(strand) = dna.helix.strands.first() {
                        let mut agent_stack = std::mem::take(&mut agent.stack);
                        std::mem::swap(&mut vm.stack, &mut agent_stack);

                        for gene in &strand.genes {
                            let _ = vm.execute_gene_inner(gene.op.clone(), &gene.args);
                        }

                        std::mem::swap(&mut vm.stack, &mut agent_stack);
                        agent.stack = agent_stack;
                        vm.output.push(format!(
                            "MESMERISM: Agent at {},{} executed '{}'",
                            x, y, code
                        ));
                    }
                }
                Err(e) => {
                    vm.output.push(format!(
                        "MESMERISM ERROR: Compile failed for '{}': {}",
                        code, e
                    ));
                }
            }
        }

        let mut current_type = "@".to_string();
        if let Value::Str(s) = &grid_snapshot[y][x] {
            current_type = s.clone();
        }

        let target = if current_type == "C" {
            match process_critter_logic(vm, &agent, grid_snapshot) {
                Some((updated_agent, t)) => {
                    agent = updated_agent;
                    t
                }
                None => continue,
            }
        } else if current_type == "K" {
            process_chaos_logic(vm, &agent, grid_snapshot)
        } else if current_type == "H" {
            process_hunter_logic(vm, &agent, grid_snapshot)
        } else if current_type == "♻" {
            scavenger::process_scavenger_logic(vm, &agent, grid_snapshot)
        } else if current_type == "♬" {
            match siren::process_siren_logic(vm, &agent, grid_snapshot) {
                Some((updated_agent, t)) => {
                    agent = updated_agent;
                    t
                }
                None => continue,
            }
        } else if current_type == "₣" {
            match forth::process_forth_agent(vm, &agent, grid_snapshot) {
                Some((updated_agent, t)) => {
                    agent = updated_agent;
                    t
                }
                None => continue,
            }
        } else if current_type == "Φ" {
            match philosopher::process_philosopher_logic(vm, &agent, grid_snapshot) {
                Some((updated_agent, t)) => {
                    agent = updated_agent;
                    t
                }
                None => continue,
            }
        } else if current_type == "ζ" {
            match zeta::process_zeta_agent(vm, &agent, grid_snapshot) {
                Some((updated_agent, t)) => {
                    agent = updated_agent;
                    t
                }
                None => continue,
            }
        } else if current_type == "P" {
            match plasmid::process_plasmid_logic(vm, &agent, grid_snapshot) {
                Some((updated_agent, t)) => {
                    agent = updated_agent;
                    t
                }
                None => continue,
            }
        } else if current_type == "⚓" {
            match pilot::process_pilot_logic(vm, &agent, grid_snapshot) {
                Some((updated_agent, t)) => {
                    agent = updated_agent;
                    t
                }
                None => continue,
            }
        } else if current_type == "∃" {
            match logic_agent::process_logic_agent(vm, &agent, grid_snapshot) {
                Some((updated_agent, t)) => {
                    agent = updated_agent;
                    t
                }
                None => continue,
            }
        } else if current_type == "χ" {
            match chromatin::process_chromatin_agent(vm, &agent, grid_snapshot) {
                Some((updated_agent, t)) => {
                    agent = updated_agent;
                    t
                }
                None => continue,
            }
        } else if current_type == "🕷" {
            match weaver::process_weaver_agent(vm, &agent, grid_snapshot) {
                Some((updated_agent, t)) => {
                    agent = updated_agent;
                    t
                }
                None => continue,
            }
        } else if current_type == "✂" {
            match splicer::process_splicer_agent(vm, &agent, grid_snapshot) {
                Some((updated_agent, t)) => {
                    agent = updated_agent;
                    t
                }
                None => continue,
            }
        } else if current_type == "🔗" {
            match ligase::process_ligase_agent(vm, &agent, grid_snapshot) {
                Some((updated_agent, t)) => {
                    agent = updated_agent;
                    t
                }
                None => continue,
            }
        } else if current_type == "🦠" {
            match phage::process_phage_agent(vm, &agent, grid_snapshot) {
                Some((updated_agent, t)) => {
                    agent = updated_agent;
                    t
                }
                None => continue,
            }
        } else if current_type == "🛠" {
            match ribozyme::process_ribozyme_agent(vm, &agent, grid_snapshot) {
                Some((updated_agent, t)) => {
                    agent = updated_agent;
                    t
                }
                None => continue,
            }
        } else if current_type == "🎓" {
            match scholar::process_scholar_logic(vm, &agent, grid_snapshot) {
                Some((updated_agent, t)) => {
                    agent = updated_agent;
                    t
                }
                None => continue,
            }
        } else if current_type == "🌀" {
            mesmerist::process_mesmerist_logic(vm, &agent, grid_snapshot)
        } else if current_type == "⚗" {
            match alchemist::process_alchemist_logic(vm, &agent, grid_snapshot) {
                Some((updated_agent, t)) => {
                    agent = updated_agent;
                    t
                }
                None => continue,
            }
        } else if current_type == "ð" {
            match weave::process_shuttle_agent(vm, &agent, grid_snapshot) {
                Some((updated_agent, t)) => {
                    agent = updated_agent;
                    t
                }
                None => continue,
            }
        } else if current_type == "♣" {
            match gardener::process_gardener_logic(vm, &agent, grid_snapshot) {
                Some((updated_agent, t)) => {
                    agent = updated_agent;
                    t
                }
                None => continue,
            }
        } else if current_type == "🧙" {
            match wizard::process_wizard_logic(vm, &agent, grid_snapshot) {
                Some((updated_agent, t)) => {
                    agent = updated_agent;
                    t
                }
                None => continue,
            }
        } else if current_type == "💤" {
            match oneiric::process_dream_weaver_logic(vm, &agent, grid_snapshot) {
                Some((updated_agent, t)) => {
                    agent = updated_agent;
                    t
                }
                None => continue,
            }
        } else if current_type == "👹" {
            match oneiric::process_nightmare_logic(vm, &agent, grid_snapshot) {
                Some((updated_agent, t)) => {
                    agent = updated_agent;
                    t
                }
                None => continue,
            }
        } else if current_type == "★" {
            match astral::process_astral_agent(vm, &agent, grid_snapshot) {
                Some((updated_agent, t)) => {
                    agent = updated_agent;
                    t
                }
                None => continue,
            }
        } else if current_type == "🤖" {
            match automaton::process_automaton_agent(vm, &agent, grid_snapshot) {
                Some((updated_agent, t)) => {
                    agent = updated_agent;
                    t
                }
                None => continue,
            }
        } else if current_type == "🗿" {
            match golem::process_golem_logic(vm, &agent, grid_snapshot) {
                Some((updated_agent, t)) => {
                    agent = updated_agent;
                    t
                }
                None => continue,
            }
        } else {
            process_seeker_logic(vm, &agent, grid_snapshot)
        };

        if let Some((ny, nx)) = target {
            vm.grid[ny][nx] = Value::Str(current_type.clone());
            written_cells.insert((ny, nx));

            if current_type == "C"
                || current_type == "♬"
                || current_type == "₣"
                || current_type == "ζ"
                || current_type == "P"
                || current_type == "⚓"
                || current_type == "∃"
                || current_type == "χ"
                || current_type == "🕷"
                || current_type == "✂"
                || current_type == "🔗"
                || current_type == "🦠"
                || current_type == "🛠"
                || current_type == "🎓"
                || current_type == "🌀"
                || current_type == "⚗"
                || current_type == "ð"
                || current_type == "♣"
                || current_type == "🧙"
                || current_type == "💤"
                || current_type == "👹"
                || current_type == "★"
                || current_type == "🤖"
                || current_type == "🗿"
            {
                vm.prologue_state.registers.insert(
                    (ny, nx),
                    pack_agent_data(agent.state.clone(), agent.stack.clone()),
                );
            }

            if !written_cells.contains(&(y, x)) {
                if let Value::Str(s) = &vm.grid[y][x] {
                    if s == &current_type {
                        vm.grid[y][x] = Value::Int(0);
                    }
                }
                if current_type == "C"
                    || current_type == "♬"
                    || current_type == "₣"
                    || current_type == "ζ"
                    || current_type == "P"
                    || current_type == "⚓"
                    || current_type == "∃"
                    || current_type == "χ"
                    || current_type == "🕷"
                    || current_type == "✂"
                    || current_type == "🔗"
                    || current_type == "🦠"
                    || current_type == "🛠"
                    || current_type == "🎓"
                    || current_type == "🌀"
                    || current_type == "⚗"
                    || current_type == "ð"
                    || current_type == "♣"
                    || current_type == "🧙"
                    || current_type == "💤"
                    || current_type == "👹"
                    || current_type == "★"
                    || current_type == "🤖"
                    || current_type == "🗿"
                {
                    vm.prologue_state.registers.remove(&(y, x));
                }
            }

            new_agents.push(PrologueAgent {
                x: nx,
                y: ny,
                state: agent.state,
                stack: agent.stack,
            });
        } else {
            if current_type == "C"
                || current_type == "♬"
                || current_type == "₣"
                || current_type == "ζ"
                || current_type == "P"
                || current_type == "⚓"
                || current_type == "∃"
                || current_type == "χ"
                || current_type == "🕷"
                || current_type == "✂"
                || current_type == "🔗"
                || current_type == "🦠"
                || current_type == "🛠"
                || current_type == "🎓"
                || current_type == "🌀"
                || current_type == "⚗"
                || current_type == "ð"
                || current_type == "♣"
                || current_type == "🧙"
                || current_type == "💤"
                || current_type == "👹"
                || current_type == "★"
                || current_type == "🤖"
                || current_type == "🗿"
            {
                vm.prologue_state.registers.insert(
                    (y, x),
                    pack_agent_data(agent.state.clone(), agent.stack.clone()),
                );
            }
            new_agents.push(agent);
        }
    }
    vm.prologue_state.agents = new_agents;
}

fn pack_agent_data(state: Value, stack: Vec<Value>) -> Value {
    if stack.is_empty() {
        state
    } else {
        Value::Junction(
            crate::ast::JunctionType::All,
            vec![state, Value::Junction(crate::ast::JunctionType::All, stack)],
        )
    }
}

fn unpack_agent_data(val: Value) -> (Value, Vec<Value>) {
    match val {
        Value::Junction(crate::ast::JunctionType::All, list) => {
            if list.len() == 2 {
                if let Value::Junction(crate::ast::JunctionType::All, ref stack) = list[1] {
                    return (list[0].clone(), stack.clone());
                }
            }
            (Value::Junction(crate::ast::JunctionType::All, list), vec![])
        }
        v => (v, vec![]),
    }
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
            evolution_config: None,
            helix: Helix { strands: vec![] },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.prologue_state.active = true;

        vm.grid[5][4] = Value::Int(42);
        vm.grid[5][5] = Value::Str("!".to_string());
        vm.grid[6][5] = Value::Str("~".to_string());
        vm.grid[7][5] = Value::Str("?".to_string());

        exec_prologue_tick(&mut vm);

        assert!(vm.prologue_state.signal_grid[6][5].is_some());
        if let Some(Value::Int(v)) = &vm.prologue_state.signal_grid[6][5] {
            assert_eq!(*v, 42);
        } else {
            panic!("Wire did not carry signal 42");
        }

        let output = vm.output.join("\n");
        assert!(output.contains("PROLOGUE: Sink at 5,7 received Int(42)"));
    }
}

#[cfg(feature = "elektra")]
#[cfg(test)]
mod prologue_elektra_test;

#[cfg(feature = "oracle")]
#[cfg(test)]
mod prologue_oracle_test;

#[cfg(test)]
mod prologue_resonance_test;

#[cfg(test)]
mod prologue_linguistics_test;

#[cfg(test)]
mod prologue_forth_test;

#[cfg(test)]
mod gardener_test;

#[cfg(test)]
mod prologue_critter_behavior_test;

#[cfg(test)]
mod prologue_chroma_test;

#[cfg(test)]
mod nova_phage_test;
#[cfg(test)]
mod prologue_chromatin_test;
#[cfg(test)]
mod prologue_echo_test;
#[cfg(test)]
mod prologue_enzymes_test;

#[cfg(test)]
mod prologue_evolution_v2_test;
#[cfg(test)]
mod prologue_forth_v2_test;
#[cfg(test)]
mod prologue_green_spore_test;
#[cfg(test)]
mod prologue_logic_agent_test;
#[cfg(test)]
mod prologue_logos_test;
#[cfg(test)]
mod prologue_mycelium_test;
#[cfg(all(test, feature = "biophysics"))]
mod prologue_neural_growth_test;
#[cfg(test)]
mod prologue_runecraft_test;
#[cfg(test)]
mod weave_test;
#[cfg(test)]
mod weaver_test;

#[cfg(test)]
mod ribozyme_agent_test;

#[cfg(test)]
mod prologue_spectral_evolution_test;

#[cfg(test)]
mod alchemist_test;
#[cfg(test)]
mod architect_test;
#[cfg(test)]
mod golem_test;
#[cfg(test)]
mod mesmerist_test;
#[cfg(test)]
mod phonetics_test;
#[cfg(test)]
mod prologue_genesis_test;
#[cfg(test)]
mod prologue_library_test;
#[cfg(test)]
mod wizard_test;
