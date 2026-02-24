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
pub mod architect;
pub mod biolum;
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
pub mod genetics;
pub mod fission;
pub mod forth;
pub mod hyper;
pub mod hypnagogia;
pub mod io;
pub mod lexicon;
pub mod linguistics;
pub mod ligase;
pub mod library;
pub mod list;
pub mod logic;
pub mod logic_agent;
pub mod logos;
pub mod math;
pub mod memetics;
pub mod mycelium;
pub mod narrative;
pub mod necromancy;
#[cfg(feature = "biophysics")]
pub mod neural;
pub mod optics;
pub mod oracle;
pub mod pandemonium;
pub mod philosopher;
pub mod phage;
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
pub mod weaver;
pub mod zeta;

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
    /// Hypnagogia: Dream Intensity (0.0 - 100.0).
    #[serde(default)]
    pub dream_intensity: f32,
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
}

fn default_logos_engine() -> logos::LogosEngine {
    logos::LogosEngine::new()
}

fn default_epigenetic_grid() -> Vec<Vec<epigenetics::EpigeneticMark>> {
    vec![vec![epigenetics::EpigeneticMark::None; GRID_SIZE]; GRID_SIZE]
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
            dream_intensity: 0.0,
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
                        // Neural
                            | "♦"
                            | "•"
                            | "°"
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
                            | ")"
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
                            | "8"
                        // Teleport
                            | "{"
                            | "}"
                        // Chronos
                            | "s"
                            | "g"
                            | "r"
                            | "c"
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
                            | "Ð"
                        // Void
                            | "µ"
                            | "Ø"
                            | "§"
                        // Construct
                            | "B"
                            | "Π"
                        // Architect
                            | "⟳"
                            | "↔"
                            | "↕"
                            | "❏"
                            | "▓"
                            | "░"
                        // Virology
                            | "v"
                            | "i"
                            | "a"
                        // Biolum
                            | "Φ"
                            | "Λ"
                            | "Ω"
                        // Philosopher
                            | "🎓"
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
                            | "x"
                            | "p"
                            | "o"
                        // Pandemonium
                            | "¿"
                            | "¡"
                            | "≈"
                        // Epigenetics
                            | "."
                            | ":"
                            | ","
                            | "☣"
                            | "♻"
                            | "χ"
                        // Elemental
                            | "Δ"
                            | "∇"
                            | "◊"
                            | "○"
                            | "☆"
                            | "☿"
                        // Fission
                            | "☢"
                            | "✇"
                            | "⌘"
                            | "✦"
                        // Hypnagogia
                            | "☾"
                            | "☀"
                        // Elektra
                            | "⚡"
                            | "≡"
                            | "∿"
                            | "🔌"
                            | "💡"
                            | "🔋"
                            | "♒"
                            | "⇝"
                            | "⏧"
                        // Oracle
                            | "¶"
                            | "λ"
                            | "¥"
                            | "∃"
                        // Psionics
                            | "Θ"
                            | "Ξ"
                            | "Σ"
                        // Resonance
                            | "♪"
                            | "♫"
                            | "🥁"
                        // Prism
                            | "▲"
                            | "▼"
                            | "🧬"
                            | "⚛"
                            | "⚒"
                            | "🧶"
                            | "💉"
                        // Linguistics
                            | "\""
                            | "®"
                            | ";"
                            | "©"
                            | "↑"
                            | "↓"
                            | "≅"
                        // Memetics
                            | "ι"
                            | "κ"
                            | "ε"
                            | "σ"
                            | "φ"
                        // Narrative
                            | "α"
                            | "ω"
                            | "✍"
                            | "📖"
                            | "📚"
                            | "🔖"
                        // Chroma
                            | "🎨"
                            | "🖌"
                            | "👁"
                            | "🔴"
                            | "🟢"
                            | "🔵"
                            // Siren
                            | "♬"
                            // Logos
                            | "Γ"
                            | "«"
                            | "»"
                            | "η"
                            | "γ"
                            // Forth
                            | "₣"
                            // Pilot
                            | "⚓"
                            // Zeta (Wire Lisp)
                            | "ζ"
                            // Hyper
                            | "⇪" | "↻" | "⌖" | "▣"
                            // Rhythm
                            | "⏱️" | "🎹" | "🎚️"
                            // Sequencer
                            | "🔍" | "✏" | "🗑" | "➕"
                            // Mycelium
                            | "🍄" | "📥" | "📤" | "🦋"
                            // Neural Growth
                            | "🌱"
                            // Weaver
                            | "🕷"
                            // Runecraft
                            | "£"
                            // Splicer
                            | "✂"
                            // Ligase
                            | "🔗"
                            // Phage
                            | "🦠"
                            // Ribozyme
                            | "🛠"
                            // Genetics (Hybridize)
                            | "⨁"
                    ) {
                        self.runes.insert((y, x));

                        if s == "@"
                            || s == "K"
                            || s == "H"
                            || (s == "C" && !self.orca_mode)
                            || s == "♻"
                            || s == "♬"
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
                        {
                            // Try to retrieve persistent state
                            let raw_state = self
                                .registers
                                .get(&(y, x))
                                .cloned()
                                .unwrap_or(Value::Int(0));
                            let (state, stack) = unpack_agent_data(raw_state);

                            let final_state = if matches!(state, Value::Int(0)) {
                                if s == "C" {
                                    critter::CritterState::default().to_value()
                                } else if s == "♬" {
                                    siren::SirenState::default().to_value()
                                } else if s == "ζ" {
                                    // Zeta defaults to East (0, 1)
                                    Value::Junction(
                                        crate::ast::JunctionType::All,
                                        vec![Value::Int(0), Value::Int(1)],
                                    )
                                } else if s == "Φ" {
                                    Value::Str("exist".to_string())
                                } else if s == "⚓" {
                                    Value::Junction(
                                        crate::ast::JunctionType::All,
                                        vec![Value::Int(0), Value::Int(1), Value::Int(0)],
                                    )
                                } else if s == "∃" {
                                    Value::Junction(
                                        crate::ast::JunctionType::All,
                                        vec![Value::Str("?".to_string()), Value::Int(0), Value::Int(1)],
                                    )
                                } else if s == "χ" {
                                    Value::Junction(
                                        crate::ast::JunctionType::All,
                                        vec![Value::Int(0), Value::Str("".to_string())],
                                    )
                                } else if s == "🕷" {
                                    Value::Int(0) // Weaver default state (placeholder)
                                } else if s == "✂" || s == "🔗" {
                                    Value::Int(0)
                                } else if s == "🦠" {
                                    Value::Int(0)
                                } else if s == "🛠" {
                                    Value::Int(0)
                                } else if s == "🎓" {
                                    Value::Junction(
                                        crate::ast::JunctionType::All,
                                        vec![Value::Int(0), Value::Int(0), Value::Int(0), Value::Int(0)],
                                    )
                                } else {
                                    Value::Int(0)
                                }
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
                    } else if self.custom_runes.contains_key(s) {
                        // Custom Rune
                        self.runes.insert((y, x));
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

    // 0. Lexicon (Word Spells) - Priority over Runes
    lexicon::process_lexicon(vm);

    // 1. Scan Grid for Topology (Runes)
    let grid_snapshot = vm.grid.clone(); // Clone for read access
    vm.prologue_state.scan_grid_rules(&grid_snapshot);

    #[cfg(feature = "biophysics")]
    neural::scan_neural_grid(vm);

    // 2. Clear Signals & Apply Delays & 3. Source Emission
    prepare_signals(vm, &grid_snapshot);

    #[cfg(feature = "biophysics")]
    neural::fire_neurons(vm);

    // 4. Propagation (Wires ~ and Gates & | + * # % ^)
    process_signal_propagation(vm, &grid_snapshot);

    // 5. Sink Consumption / Actions (?, $, M, O)
    process_sinks(vm, &grid_snapshot);

    #[cfg(feature = "biophysics")]
    neural::integrate_neurons(vm);

    // 6. Agents (@)
    process_agents(vm, &grid_snapshot);

    // 7. Hypnagogia (Dream Logic)
    hypnagogia::process_dream_logic(vm);

    // 8. Oracle (Omens)
    #[cfg(feature = "oracle")]
    {
        if !vm.omens.is_empty() {
            crate::vm::oracle::process_omens(vm);
        }
    }
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
        // Epigenetic Check: Methylation silences Source
        if vm.prologue_state.epigenetic_grid[*y][*x] == epigenetics::EpigeneticMark::Methylated {
            continue;
        }

        if let Value::Str(s) = &grid[*y][*x] {
            if s == "!" && *x > 0 {
                let val = grid[*y][*x - 1].clone();
                // Only emit truthy values? Or all values?
                // Let's emit non-empty signals.
                if !is_empty_val(&val) {
                    vm.prologue_state.signal_grid[*y][*x] = Some(val);
                }
            }
            fission::prepare_fission_sources(s, *y, *x, &mut vm.prologue_state.signal_grid);
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

    // Ensure scratch buffer is ready (in case of serialization/deserialization)
    if vm.prologue_state.scratch_signal_grid.len() != GRID_SIZE {
        vm.prologue_state.scratch_signal_grid = vec![vec![None; GRID_SIZE]; GRID_SIZE];
    }

    for _ in 0..max_iterations {
        let mut changes = false;
        // Swap out the scratch buffer to use as next state
        let mut next_signals = std::mem::take(&mut vm.prologue_state.scratch_signal_grid);

        // Ensure proper sizing (std::mem::take replaces with default empty Vec)
        if next_signals.len() != GRID_SIZE {
            next_signals = vec![vec![None; GRID_SIZE]; GRID_SIZE];
        }

        // Copy current state to next state (reusing allocation)
        next_signals.clone_from(&vm.prologue_state.signal_grid);

        for (y, x) in &runes {
            // Epigenetic Check: Methylation stops propagation
            if vm.prologue_state.epigenetic_grid[*y][*x] == epigenetics::EpigeneticMark::Methylated
            {
                continue;
            }

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
                    grid,
                    &vm.light_grid,
                    &mut vm.prologue_state.dream_intensity,
                    v_grid,
                    r_grid,
                    c_grid,
                    &mut vm.energy,
                    &vm.chroma_grid,
                    &mut vm.prologue_state.logos_engine,
                    vm.prologue_state.orca_mode,
                    &mut vm.prologue_state.hyper_state,
                    &mut vm.prologue_state.rhythm_state,
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
        // Swap buffers: next_signals becomes the new signal_grid
        std::mem::swap(&mut vm.prologue_state.signal_grid, &mut next_signals);

        // Return the used buffer (now containing old state) to scratch for reuse
        vm.prologue_state.scratch_signal_grid = next_signals;

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
    echoes: &mut HashMap<(usize, usize), echo::EchoBuffer>,
    history: &mut HashMap<(usize, usize), VecDeque<Value>>,
    void_buffer: &mut VecDeque<Value>,
    mycelium_buffer: &mut VecDeque<Value>,
    grid: &[Vec<Value>],
    light_grid: &[Vec<i64>],
    dream_intensity: &mut f32,
    voltage_grid: &mut Vec<Vec<f32>>,
    resistance_grid: &mut Vec<Vec<f32>>,
    capacitance_grid: &mut Vec<Vec<f32>>,
    energy: &mut i64,
    chroma_grid: &[Vec<crate::vm::ChromaCell>],
    logos_engine: &mut logos::LogosEngine,
    orca_mode: bool,
    hyper_state: &mut hyper::HyperState,
    rhythm_state: &mut rhythm::RhythmState,
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

    if hypnagogia::apply_hypnagogia_runes(
        rune,
        y,
        x,
        current_signals,
        next_signals,
        dream_intensity,
    ) {
        return true;
    }
    if construct::apply_construct_runes(rune, y, x, current_signals, next_delayed, grid) {
        return true;
    }
    if architect::apply_architect_runes(rune, y, x, current_signals, next_signals) {
        return true;
    }
    if topology::apply_topology_runes(
        rune,
        y,
        x,
        tick,
        current_signals,
        next_signals,
        next_delayed,
        orca_mode,
    ) {
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
    if evolution::apply_evolution_runes(rune, y, x, dna, current_signals, next_signals) {
        return true;
    }
    if void::apply_void_runes(rune, y, x, current_signals, next_signals, void_buffer) {
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
    if elemental::apply_elemental_runes(rune, y, x, current_signals, next_signals) {
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
    if mycelium::apply_mycelium_runes(
        rune,
        y,
        x,
        current_signals,
        next_signals,
        mycelium_buffer,
    ) {
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

/// Executes the logic for a Sink Rune.
///
/// Handles `?` (Sink), `$` (Scribe), `M` (Mutate), `O` (Organelle), etc.
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

        if triggered {
            // Prevent re-triggering if already executing this strand
            if vm.ip.0 != strand_idx {
                vm.context_loc = (y, x);
                vm.interrupt(strand_idx);
                vm.output.push(format!("RUNECRAFT: Executed Custom Rune '{}'", rune));
                vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1));
            }
        }
        return;
    }

    match rune {
        "🎓" => {
            // Graduate: West (@), North (Goal) -> South (Φ)
            if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
                if let Value::Str(s) = &vm.grid[wy][wx] {
                    if s == "@" {
                        if let Some((ny, nx)) = normalize_coords(y as i64 - 1, x as i64) {
                            if let Some(Value::Str(goal)) = &vm.prologue_state.signal_grid[ny][nx] {
                                if let Some((sy, sx)) = normalize_coords(y as i64 + 1, x as i64) {
                                    vm.grid[sy][sx] = Value::Str("Φ".to_string());
                                    vm.prologue_state.registers.insert((sy, sx), Value::Str(goal.clone()));
                                    vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1));
                                }
                            }
                        }
                    }
                }
            }
        }
        "?" => {
            // Sink
            // Check neighbors for signal (and Self)
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
                // vm.prologue_state.signal_grid[y][x] = Some(sig.clone()); // Light up - Removed to prevent feedback loop

                if let Value::Str(name) = sig {
                    if let Some(&idx) = vm.dictionary.get(&name) {
                        vm.interrupt(idx);
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
                return None; // Dead
            }

            match action {
                critter::CritterAction::Move(ny, nx) => {
                    let dest_val = &vm.grid[ny][nx]; // Check LIVE grid
                    let mut blocked = false;
                    let mut moved_target = None;

                    if let Value::Str(s) = dest_val {
                        if s == "C" && (ny != y || nx != x) {
                            // Collided with another Critter -> Breed
                            let mut rng = rand::thread_rng();
                            let mut spawn_dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];
                            // Shuffle to maintain randomness
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
                                        break; // Spawned one child, stop
                                    }
                                }
                            }
                            blocked = true;
                        } else if s == "!" {
                            // Eat Food
                            critter.energy += 20;
                            updated_agent.state = critter.to_value();
                            // Move into it (Eat)
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
                    // Kill agent at target
                    let dest_val = vm.grid[ny][nx].clone();
                    if let Value::Str(s) = dest_val {
                        if s == "@" || s == "K" || s == "H" || s == "C" {
                            // Kill
                            vm.grid[ny][nx] = Value::Int(0); // Corpse?
                            if s == "C" {
                                vm.prologue_state.registers.remove(&(ny, nx));
                            }
                            critter.energy += 30; // Predation gain
                            updated_agent.state = critter.to_value();
                        }
                    }
                    vm.prologue_state
                        .registers
                        .insert((y, x), updated_agent.state.clone());
                    return Some((updated_agent, None)); // Stay put
                }
                critter::CritterAction::Split => {
                    // Mitosis
                    if critter.energy > 50 {
                        let mut rng = rand::thread_rng();
                        let spawn_dirs = [(-1, 0), (1, 0), (0, -1), (0, 1)];
                        let (dy, dx) = spawn_dirs[rng.gen_range(0..4)];
                        if let Some((sy, sx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
                            if matches!(vm.grid[sy][sx], Value::Int(0)) {
                                let child = critter::breed(&critter, &critter); // Self-breed
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
                    // Build rune at target
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
                    // Mark forward
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
    // Fallback if parsing fails or not a string (should not happen for C)
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

    // 1. Seek Prey
    for (dy, dx) in neighbors {
        if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
            if let Value::Str(s) = &grid_snapshot[ny][nx] {
                if s == "@" || s == "K" {
                    return Some((ny, nx));
                }
            }
        }
    }

    // 2. Seek Signal
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
    let mut written_cells = HashSet::new();

    for mut agent in agents {
        let (y, x) = (agent.y, agent.x);

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
                None => continue, // Dead
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
        } else {
            process_seeker_logic(vm, &agent, grid_snapshot)
        };

        if let Some((ny, nx)) = target {
            // Move agent
            // Set new pos FIRST (Sequential Update)
            vm.grid[ny][nx] = Value::Str(current_type.clone());
            written_cells.insert((ny, nx));

            // Move Registers
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
            {
                vm.prologue_state.registers.insert(
                    (ny, nx),
                    pack_agent_data(agent.state.clone(), agent.stack.clone()),
                );
            }

            // Clear old pos ONLY if not written to by another agent
            if !written_cells.contains(&(y, x)) {
                if let Value::Str(s) = &vm.grid[y][x] {
                    // Only clear if it matches our agent type (avoid clearing overwrites from unrelated things)
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
            // Even if not moving, we must update registers if state changed
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
                // Assume [State, Stack]
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
        let dna = Dna { evolution_config: None,
            helix: Helix { strands: vec![] },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.prologue_state.active = true;

        // Setup Circuit: 42 -> ! -> ~ -> ?
        vm.grid[5][4] = Value::Int(42);
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
mod prologue_critter_behavior_test;

#[cfg(test)]
mod prologue_chroma_test;

#[cfg(test)]
mod prologue_echo_test;
#[cfg(test)]
mod prologue_green_spore_test;
#[cfg(test)]
mod prologue_mycelium_test;
#[cfg(test)]
mod prologue_evolution_v2_test;
#[cfg(test)]
mod prologue_forth_v2_test;
#[cfg(test)]
mod prologue_logos_test;
#[cfg(test)]
mod prologue_logic_agent_test;
#[cfg(test)]
mod prologue_chromatin_test;
#[cfg(all(test, feature = "biophysics"))]
mod prologue_neural_growth_test;
#[cfg(test)]
mod weaver_test;
#[cfg(test)]
mod prologue_runecraft_test;
#[cfg(test)]
mod prologue_enzymes_test;
#[cfg(test)]
mod nova_phage_test;

#[cfg(test)]
mod ribozyme_agent_test;

#[cfg(test)]
mod prologue_spectral_evolution_test;

#[cfg(test)]
mod prologue_library_test;
#[cfg(test)]
mod architect_test;
