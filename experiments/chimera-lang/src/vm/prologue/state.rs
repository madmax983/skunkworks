use super::unpack_agent_data;
use super::{echo, epigenetics, hyper, logos, oneiric, rhythm, weave_reality};
use crate::vm::Value;
use crate::vm::GRID_SIZE;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};

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
    /// The required combination of `Value`s needed to trigger this alchemical transmutation.
    pub ingredients: Vec<Value>,
    /// The resulting `Value` produced upon a successful transmutation.
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

fn is_prologue_rune(s: &str) -> bool {
    let b = s.as_bytes();
    if b.len() == 1 {
        let ch = b[0];
        if (33..=126).contains(&ch) {
            return matches!(
                ch as char,
                '!' | '"'
                    | '#'
                    | '$'
                    | '%'
                    | '&'
                    | '('
                    | ')'
                    | '*'
                    | '+'
                    | ','
                    | '-'
                    | '.'
                    | '/'
                    | '8'
                    | ':'
                    | ';'
                    | '<'
                    | '='
                    | '>'
                    | '?'
                    | '@'
                    | 'A'
                    | 'B'
                    | 'C'
                    | 'D'
                    | 'E'
                    | 'F'
                    | 'G'
                    | 'H'
                    | 'I'
                    | 'J'
                    | 'K'
                    | 'L'
                    | 'M'
                    | 'N'
                    | 'O'
                    | 'P'
                    | 'Q'
                    | 'R'
                    | 'S'
                    | 'T'
                    | 'U'
                    | 'V'
                    | 'W'
                    | 'X'
                    | 'Y'
                    | 'Z'
                    | '['
                    | '\\'
                    | ']'
                    | '^'
                    | 'a'
                    | 'b'
                    | 'c'
                    | 'd'
                    | 'e'
                    | 'f'
                    | 'g'
                    | 'h'
                    | 'i'
                    | 'j'
                    | 'k'
                    | 'l'
                    | 'm'
                    | 'n'
                    | 'o'
                    | 'p'
                    | 'q'
                    | 'r'
                    | 's'
                    | 't'
                    | 'u'
                    | 'v'
                    | 'w'
                    | 'x'
                    | 'y'
                    | 'z'
                    | '{'
                    | '|'
                    | '}'
                    | '~'
            );
        }
    }

    matches!(
        s,
        "♦" | "•"
            | "°"
            | "∞"
            | "Ð"
            | "µ"
            | "Ø"
            | "§"
            | "ꝏ"
            | "Π"
            | "🤖"
            | "⟳"
            | "↔"
            | "↕"
            | "❏"
            | "▓"
            | "░"
            | "Φ"
            | "Λ"
            | "Ω"
            | "🎓"
            | "†"
            | "‡"
            | "Ψ"
            | "¿"
            | "¡"
            | "≈"
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
            | "®"
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
    )
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
                    if is_prologue_rune(s.as_str()) {
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

impl PrologueState {
    /// Checks if a string is a built-in agent type.
    pub fn is_agent_type(s: &str) -> bool {
        matches!(
            s,
            "C" | "♬"
                | "₣"
                | "ζ"
                | "P"
                | "⚓"
                | "∃"
                | "χ"
                | "🕷"
                | "✂"
                | "🔗"
                | "🦠"
                | "🛠"
                | "🎓"
                | "🌀"
                | "⚗"
                | "ð"
                | "♣"
                | "🧙"
                | "💤"
                | "👹"
                | "★"
                | "🤖"
                | "🗿"
        )
    }
}
