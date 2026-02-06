//! Virtual Machine for the Chimera language.
//!
//! The `ChimeraVM` is the core execution engine that interprets the DNA instructions.
//! It manages the instruction pointer, stack, memory grid, and biological constraints (energy).
//!
//! # Architecture
//!
//! - **DNA**: Read-only program code organized in strands of genes.
//! - **Stack**: LIFO data structure for values (Integers and Strings).
//! - **Grid**: 16x16 mutable 2D memory space.
//! - **Energy**: The fuel for execution. Operations cost energy; running out causes death (halt).
//! - **IP (Instruction Pointer)**: Tracks current `(strand_idx, gene_idx)`.
//!
//! # Execution Cycle
//!
//! 1. Check constraints (Energy > 0, IP bounds).
//! 2. Execute Gene at IP.
//! 3. Update State (Stack, Grid, Energy).
//! 4. Advance IP (or Jump).
//!
//! # Features
//!
//! - **Nova**: Epigenetics, Spores (Time Travel), Quantum Entanglement.
//! - **Cortex**: Neural Network simulation (Synapses, Activation).

use crate::ast::{Dna, JunctionType, Nucleotide};
use crate::opcode::OpCode;
use rand::Rng;
use serde::{Deserialize, Serialize};
#[cfg(feature = "nova")]
use std::collections::{HashMap, HashSet, VecDeque};

pub const MAX_RECURSION_DEPTH: usize = 100;
pub const MAX_CALL_STACK_DEPTH: usize = 100;
pub const MAX_SPORES: usize = 64;
pub const MAX_ORGANELLES: usize = 256;
pub const MAX_CHORUS_SIZE: usize = 8;
pub const MAX_JUNCTION_SIZE: usize = 1024;
pub const GRID_SIZE: usize = 16;
pub const INITIAL_ENERGY: i64 = 50;

#[cfg(feature = "nova")]
pub mod akashic;
pub mod bard;
#[cfg(feature = "nova")]
pub mod blackbox;
pub mod cortex;
pub mod microscope;
#[cfg(feature = "biophysics")]
pub mod neuron;
pub mod nova;
#[cfg(feature = "nova")]
pub mod nova_sigil;
pub mod oracle;
pub mod resonance;
#[cfg(feature = "silicon")]
pub mod silicon;

#[cfg(feature = "resonance")]
use crossbeam_channel::Sender;
#[cfg(feature = "resonance")]
use resonance_audio::audio::AudioCommand;

#[cfg(feature = "nova")]
use self::nova::{Organelle, Spore};

#[cfg(feature = "nova")]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct ChromaCell {
    pub char: Option<char>,
    pub fg: Option<(u8, u8, u8)>,
}

#[cfg(any(feature = "nova", feature = "silicon"))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Topology {
    Plane,     // 0: Bounded. Edges are walls.
    Torus,     // 1: Wraps X and Y.
    CylinderH, // 2: Wraps X, Bounded Y.
    CylinderV, // 3: Bounded X, Wraps Y.
    Klein,     // 4: Wraps X, Wraps Y with twist (x' = 15-x).
    Mobius,    // 5: Wraps X with twist, Bounded Y.
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Value {
    Int(i64),
    Str(String),
    Junction(JunctionType, Vec<Value>),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(i) => write!(f, "{}", i),
            Value::Str(s) => write!(f, "\"{}\"", s),
            Value::Junction(t, vals) => {
                let t_str = match t {
                    JunctionType::Any => "any",
                    JunctionType::All => "all",
                };
                write!(f, "{}(", t_str)?;
                for (i, v) in vals.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", v)?;
                }
                write!(f, ")")
            }
        }
    }
}

/// The execution engine for the Chimera language.
///
/// Holds the entire state of a simulation instance.
///
/// # Examples
///
/// ```rust
/// use chimera_lang::vm::ChimeraVM;
/// use chimera_lang::ast::{Dna, Helix, Strand, Gene, Nucleotide};
/// use chimera_lang::opcode::OpCode;
///
/// // Create a simple genome: [ push(10) push(20) add() ]
/// let genes = vec![
///     Gene { op: OpCode::Push, args: vec![Nucleotide::Number(10)] },
///     Gene { op: OpCode::Push, args: vec![Nucleotide::Number(20)] },
///     Gene { op: OpCode::Add, args: vec![] },
/// ];
/// let dna = Dna { helix: Helix { strands: vec![Strand { genes }] } };
///
/// let mut vm = ChimeraVM::new(dna);
///
/// // Execute until halted or finished
/// while !vm.halted && vm.ip.0 < 1 { // Simple loop guard
///     vm.step();
/// }
///
/// // Check result on stack
/// // Note: We need to access the public `stack` field.
/// // Values are wrapped in `Value::Int`.
/// assert_eq!(vm.stack.len(), 1);
/// ```
#[derive(Clone)]
pub struct ChimeraVM {
    /// The read-only DNA program.
    pub dna: Dna,
    /// The LIFO stack for data manipulation.
    pub stack: Vec<Value>,
    /// Instruction Pointer: `(strand_index, gene_index)`.
    pub ip: (usize, usize),
    /// Standard Output buffer (silent, accumulates strings).
    pub output: Vec<String>,
    /// Execution flag. If true, `step()` does nothing.
    pub halted: bool,
    /// Biological fuel. Starts at 50. Decreases by 1 per step.
    pub energy: i64,
    /// 16x16 2D memory grid.
    pub grid: Vec<Vec<Value>>,
    /// If true, random mutations occur frequently.
    pub chaos_mode: bool,
    /// Current recursion depth (limit 100).
    pub recursion_depth: usize,
    /// "Cursor" location on the grid for spatial operations.
    pub context_loc: (usize, usize),
    #[cfg(feature = "nova")]
    pub phase: nova::Phase,
    #[cfg(feature = "nova")]
    pub epigenome: HashSet<(usize, usize)>,
    #[cfg(feature = "nova")]
    pub telomeres: Vec<i64>,
    #[cfg(feature = "nova")]
    pub hormone_grid: Vec<Vec<[i64; 3]>>,
    #[cfg(feature = "nova")]
    pub waste_grid: Vec<Vec<i64>>,
    #[cfg(feature = "nova")]
    pub mutagen_grid: Vec<Vec<i64>>,
    #[cfg(feature = "nova")]
    pub light_grid: Vec<Vec<i64>>,
    #[cfg(feature = "nova")]
    pub spores: Vec<Spore>,
    #[cfg(feature = "nova")]
    pub call_stack: Vec<(usize, usize)>,
    #[cfg(feature = "nova")]
    pub input_buffer: VecDeque<char>,
    #[cfg(feature = "nova")]
    pub receptors: HashMap<char, usize>,
    #[cfg(feature = "nova")]
    pub entangled_pairs: HashMap<usize, usize>,
    #[cfg(feature = "nova")]
    pub portals: HashMap<(usize, usize), (usize, usize)>,
    #[cfg(feature = "nova")]
    pub membranes: Vec<Vec<u8>>,
    #[cfg(feature = "nova")]
    pub chroma_grid: Vec<Vec<ChromaCell>>,
    #[cfg(feature = "nova")]
    pub organelles: Vec<Organelle>,
    #[cfg(feature = "nova")]
    pub active_organelle_kind: Option<nova::OrganelleType>,
    #[cfg(feature = "nova")]
    pub signal_differentiation: Option<nova::OrganelleType>,
    #[cfg(feature = "nova")]
    pub sonar_target: Option<(usize, usize)>,
    #[cfg(feature = "nova")]
    pub symbiotes: Vec<(usize, usize)>,
    #[cfg(any(feature = "nova", feature = "silicon"))]
    pub topology: Topology,
    #[cfg(feature = "nova")]
    pub ether: HashMap<i64, VecDeque<Value>>,
    #[cfg(feature = "cortex")]
    pub synapse_map: Vec<Vec<usize>>,
    #[cfg(feature = "cortex")]
    pub activation_levels: Vec<i64>,
    #[cfg(feature = "nova")]
    pub reflexes: HashMap<i64, usize>,
    #[cfg(feature = "nova")]
    pub remap_table: HashMap<OpCode, OpCode>,
    #[cfg(feature = "nova")]
    pub direction: isize,
    #[cfg(feature = "nova")]
    pub mycelium: HashMap<(usize, usize), Vec<(usize, usize)>>,
    #[cfg(feature = "nova")]
    pub chorus_buffer: VecDeque<String>,
    #[cfg(feature = "nova")]
    pub score: Vec<bard::Note>,
    #[cfg(feature = "oracle")]
    pub knowledge_base: Vec<Value>,
    #[cfg(feature = "resonance")]
    pub audio_tx: Option<Sender<AudioCommand>>,
    #[cfg(feature = "biophysics")]
    pub neurons: std::collections::HashMap<(usize, usize), neuron::Neuron>,
    #[cfg(feature = "nova")]
    pub blackbox: blackbox::Blackbox,
    #[cfg(feature = "silicon")]
    pub silicon_mode: bool,
}

impl ChimeraVM {
    /// Creates a new VM instance with the given DNA.
    ///
    /// Initializes the grid to zeros, energy to INITIAL_ENERGY, and IP to (0,0).
    pub fn new(dna: Dna) -> Self {
        // Initialize grid with 0s
        let grid = vec![vec![Value::Int(0); GRID_SIZE]; GRID_SIZE];
        #[cfg(any(feature = "nova", feature = "cortex"))]
        let strand_count = dna.helix.strands.len();
        #[cfg(feature = "nova")]
        let hormone_grid = vec![vec![[0, 0, 0]; GRID_SIZE]; GRID_SIZE];
        #[cfg(feature = "nova")]
        let waste_grid = vec![vec![0; GRID_SIZE]; GRID_SIZE];
        #[cfg(feature = "nova")]
        let mutagen_grid = vec![vec![0; GRID_SIZE]; GRID_SIZE];
        #[cfg(feature = "nova")]
        let light_grid = vec![vec![0; GRID_SIZE]; GRID_SIZE];
        #[cfg(feature = "nova")]
        let membranes = vec![vec![0; GRID_SIZE]; GRID_SIZE];
        #[cfg(feature = "nova")]
        let chroma_grid = vec![vec![ChromaCell::default(); GRID_SIZE]; GRID_SIZE];
        #[cfg(feature = "cortex")]
        let synapse_map = vec![vec![]; strand_count];
        #[cfg(feature = "cortex")]
        let activation_levels = vec![0; strand_count];

        Self {
            dna,
            stack: Vec::new(),
            ip: (0, 0),
            output: Vec::new(),
            halted: false,
            energy: INITIAL_ENERGY,
            grid,
            chaos_mode: false,
            recursion_depth: 0,
            context_loc: (8, 8),
            #[cfg(feature = "nova")]
            phase: nova::Phase::default(),
            #[cfg(feature = "nova")]
            epigenome: HashSet::new(),
            #[cfg(feature = "nova")]
            telomeres: vec![50; strand_count],
            #[cfg(feature = "nova")]
            hormone_grid,
            #[cfg(feature = "nova")]
            waste_grid,
            #[cfg(feature = "nova")]
            mutagen_grid,
            #[cfg(feature = "nova")]
            light_grid,
            #[cfg(feature = "nova")]
            spores: Vec::new(),
            #[cfg(feature = "nova")]
            call_stack: Vec::new(),
            #[cfg(feature = "nova")]
            input_buffer: VecDeque::new(),
            #[cfg(feature = "nova")]
            receptors: HashMap::new(),
            #[cfg(feature = "nova")]
            entangled_pairs: HashMap::new(),
            #[cfg(feature = "nova")]
            portals: HashMap::new(),
            #[cfg(feature = "nova")]
            membranes,
            #[cfg(feature = "nova")]
            chroma_grid,
            #[cfg(feature = "nova")]
            organelles: Vec::new(),
            #[cfg(feature = "nova")]
            active_organelle_kind: None,
            #[cfg(feature = "nova")]
            signal_differentiation: None,
            #[cfg(feature = "nova")]
            sonar_target: None,
            #[cfg(feature = "nova")]
            symbiotes: Vec::new(),
            #[cfg(any(feature = "nova", feature = "silicon"))]
            topology: Topology::Torus,
            #[cfg(feature = "nova")]
            ether: HashMap::new(),
            #[cfg(feature = "cortex")]
            synapse_map,
            #[cfg(feature = "cortex")]
            activation_levels,
            #[cfg(feature = "nova")]
            reflexes: HashMap::new(),
            #[cfg(feature = "nova")]
            remap_table: HashMap::new(),
            #[cfg(feature = "nova")]
            direction: 1,
            #[cfg(feature = "nova")]
            mycelium: HashMap::new(),
            #[cfg(feature = "nova")]
            chorus_buffer: VecDeque::new(),
            #[cfg(feature = "nova")]
            score: Vec::new(),
            #[cfg(feature = "oracle")]
            knowledge_base: Vec::new(),
            #[cfg(feature = "resonance")]
            audio_tx: None,
            #[cfg(feature = "biophysics")]
            neurons: std::collections::HashMap::new(),
            #[cfg(feature = "nova")]
            blackbox: blackbox::Blackbox::new(),
            #[cfg(feature = "silicon")]
            silicon_mode: false,
        }
    }

    #[cfg(feature = "resonance")]
    pub fn set_audio_tx(&mut self, tx: Sender<AudioCommand>) {
        self.audio_tx = Some(tx);
    }

    /// Triggers an internal reflex event (interrupt).
    ///
    /// If a handler is registered for the event ID, execution jumps to that strand.
    /// Returns `true` if the reflex was triggered.
    #[cfg(feature = "nova")]
    pub fn trigger_reflex(&mut self, event_id: i64) -> bool {
        if let Some(&strand_idx) = self.reflexes.get(&event_id) {
            if strand_idx < self.dna.helix.strands.len() {
                if self.call_stack.len() >= MAX_CALL_STACK_DEPTH {
                    self.output
                        .push("Error: Reflex ignored, call stack full".to_string());
                    return false;
                }

                // Push return address (current strand, next gene)
                // We use ip.1 because usually this is called between instructions or during an instruction
                // that hasn't advanced IP yet.
                // If called during step() before gene execution, IP is valid.
                self.call_stack.push(self.ip);
                self.ip = (strand_idx, 0);
                self.output.push(format!(
                    "REFLEX: Triggered event {} -> Strand {}",
                    event_id, strand_idx
                ));
                return true;
            }
        }
        false
    }

    #[inline]
    fn is_valid_coord(&self, y: i64, x: i64) -> bool {
        let size = GRID_SIZE as i64;
        (0..size).contains(&y) && (0..size).contains(&x)
    }

    /// Helper to normalize coordinates based on topology
    #[cfg(any(feature = "nova", feature = "silicon"))]
    pub fn normalize_coords(&self, y: i64, x: i64) -> Option<(usize, usize)> {
        let size = GRID_SIZE as i64;
        match self.topology {
            Topology::Plane => {
                if self.is_valid_coord(y, x) {
                    Some((y as usize, x as usize))
                } else {
                    None
                }
            }
            Topology::Torus => Some((y.rem_euclid(size) as usize, x.rem_euclid(size) as usize)),
            Topology::CylinderH => {
                // Wraps X, Bounded Y
                if (0..size).contains(&y) {
                    Some((y as usize, x.rem_euclid(size) as usize))
                } else {
                    None
                }
            }
            Topology::CylinderV => {
                // Bounded X, Wraps Y
                if (0..size).contains(&x) {
                    Some((y.rem_euclid(size) as usize, x as usize))
                } else {
                    None
                }
            }
            Topology::Klein => {
                // Wraps X normal, Y wraps with X-twist
                // Standard Klein bottle: (x, y+H) = (W-x, y)
                // Let's implement: X wraps normally. Y wraps with twist.
                let mut nx = x;
                let mut ny = y;

                // First handle Y wrapping (the twisty one)
                // If we go off top or bottom, we flip X and wrap Y
                if !(0..size).contains(&ny) {
                    // How many times did we wrap?
                    // Simple case: single step
                    // General case: rem_euclid logic with flip parity
                    // Let's assume simple wrapping for simulation steps usually +/- 1

                    // Logic:
                    // y' = y mod 16
                    // if (floor(y/16)) is odd, x' = 15 - x.

                    let wrap_count = ny.div_euclid(size);
                    if wrap_count % 2 != 0 {
                        nx = (size - 1) - nx; // Twist X
                    }
                    ny = ny.rem_euclid(size);
                }

                // Now handle X wrapping (Torus-like)
                nx = nx.rem_euclid(size);

                Some((ny as usize, nx as usize))
            }
            Topology::Mobius => {
                // Mobius Strip: Wraps X with twist, Bounded Y
                // strip [0,L]x[0,W]. (x+L, y) = (x, W-y)
                // Here X wraps with twist.
                let mut nx = x;
                let mut ny = y;

                if !(0..size).contains(&nx) {
                    let wrap_count = nx.div_euclid(size);
                    if wrap_count % 2 != 0 {
                        ny = (size - 1) - ny; // Twist Y
                    }
                    nx = nx.rem_euclid(size);
                }

                if (0..size).contains(&ny) {
                    Some((ny as usize, nx as usize))
                } else {
                    None
                }
            }
        }
    }

    /// Handles a character input event.
    ///
    /// If a receptor is bound to this key, the input is buffered and `true` is returned.
    #[cfg(feature = "nova")]
    pub fn handle_input(&mut self, key: char) -> bool {
        if self.receptors.contains_key(&key) {
            self.input_buffer.push_back(key);
            true
        } else {
            false
        }
    }

    #[cfg(feature = "nova")]
    fn handle_input_interrupts(&mut self) {
        self.sonar_target = None;
        if let Some(key) = self.input_buffer.pop_front() {
            if let Some(&strand_idx) = self.receptors.get(&key) {
                self.call_stack.push(self.ip);

                if strand_idx < self.dna.helix.strands.len() {
                    self.ip = (strand_idx, 0);
                    self.output.push(format!(
                        "INTERRUPT: Signal '{}' -> Strand {}",
                        key, strand_idx
                    ));
                }
            }
        }
    }

    #[cfg(feature = "cortex")]
    fn update_cortex_state(&mut self) {
        for level in self.activation_levels.iter_mut() {
            if *level > 0 {
                *level -= 1;
            }
        }
    }

    #[cfg(feature = "nova")]
    fn process_environment(&mut self) {
        let (cy, cx) = self.context_loc;
        self.waste_grid[cy][cx] += 10;

        nova::diffuse_hormones(self);
        nova::diffuse_waste(self);
        nova::diffuse_light(self);
        nova::diffuse_mutagen(self);

        for row in self.hormone_grid.iter_mut() {
            for cell in row.iter_mut() {
                for val in cell.iter_mut() {
                    if *val > 0 {
                        *val -= 1;
                    }
                }
            }
        }

        if self.waste_grid[cy][cx] > 100 {
            let mut rng = rand::thread_rng();
            if rng.gen_bool(0.05) {
                self.output
                    .push(format!("MUTATION: TOXICITY at {},{}", cx, cy));
                self.mutate();
            }
        }

        if self.mutagen_grid[cy][cx] > 50 {
            let mut rng = rand::thread_rng();
            // Higher probability for mutagen (10%)
            if rng.gen_bool(0.10) {
                self.output
                    .push(format!("MUTATION: RADIATION at {},{}", cx, cy));
                self.mutate();
            }
        }
    }

    fn check_starvation(&mut self) -> bool {
        if self.energy <= 0 {
            self.halted = true;
            self.output.push("DEATH: STARVATION".to_string());
            true
        } else {
            #[cfg(feature = "nova")]
            if self.energy < 10 {
                self.trigger_reflex(2); // Event 2: Low Energy
            }
            false
        }
    }

    #[cfg(feature = "nova")]
    fn check_telomeres(&mut self) -> bool {
        if self.ip.1 == 0 && self.ip.0 < self.telomeres.len() {
            let degrade = self.phase != nova::Phase::Crystalline;

            if degrade && self.telomeres[self.ip.0] > 0 {
                self.telomeres[self.ip.0] -= 1;
            }

            if self.telomeres[self.ip.0] <= 0 {
                self.output
                    .push(format!("SENESCENCE: Strand {} decayed", self.ip.0));
                self.ip.0 += 1;
                self.ip.1 = 0;
                return true;
            }
        }
        false
    }

    #[cfg(feature = "nova")]
    fn process_symbiotes(&mut self) {
        let count = self.symbiotes.len();
        for i in 0..count {
            if self.energy <= 0 {
                break;
            }

            // Swap execution context
            let mut sym_ip = self.symbiotes[i];
            std::mem::swap(&mut self.ip, &mut sym_ip);

            // Execute logic (simplified version of step checks)
            let helix_len = self.dna.helix.strands.len();
            if self.ip.0 < helix_len {
                let strand_len = self.dna.helix.strands[self.ip.0].genes.len();
                if self.ip.1 < strand_len {
                    // Check telomeres/epigenetics? For now, skip for symbiotes to avoid complexity
                    let (gene_op, gene_args) = {
                        let gene = &self.dna.helix.strands[self.ip.0].genes[self.ip.1];
                        (gene.op.clone(), gene.args.clone())
                    };

                    let jump_target = self.execute_gene(gene_op, &gene_args);
                    if let Some(target) = jump_target {
                        self.ip = target;
                    } else {
                        self.ip.1 += 1;
                    }
                }
            }

            // Swap back
            std::mem::swap(&mut self.ip, &mut sym_ip);
            self.symbiotes[i] = sym_ip;
        }
    }

    #[cfg(feature = "nova")]
    fn process_organelles(&mut self) {
        let active_organelles = std::mem::take(&mut self.organelles);
        let mut next_organelles = Vec::new();

        for mut organelle in active_organelles {
            if self.tick_organelle(&mut organelle) {
                next_organelles.push(organelle);
            }
        }
        self.organelles.extend(next_organelles);
    }

    #[cfg(feature = "nova")]
    fn tick_organelle(&mut self, organelle: &mut Organelle) -> bool {
        if organelle.halted {
            return false;
        }

        std::mem::swap(&mut self.stack, &mut organelle.stack);
        std::mem::swap(&mut self.ip, &mut organelle.ip);
        std::mem::swap(&mut self.context_loc, &mut organelle.context_loc);
        std::mem::swap(&mut self.call_stack, &mut organelle.call_stack);
        std::mem::swap(&mut self.recursion_depth, &mut organelle.recursion_depth);

        self.active_organelle_kind = Some(organelle.kind.clone());
        self.energy = self.energy.saturating_sub(1);

        match organelle.kind {
            nova::OrganelleType::Chloroplast => {
                let (cy, cx) = self.context_loc;
                let light = self.light_grid[cy][cx];
                if light > 0 {
                    self.energy = self.energy.saturating_add(light / 10);
                }
            }
            nova::OrganelleType::Mitochondria => {
                self.energy = self.energy.saturating_add(1);
            }
            nova::OrganelleType::Lysosome => {
                let (cy, cx) = self.context_loc;
                let waste = self.waste_grid[cy][cx];
                if waste > 0 {
                    let consumed = waste.min(10);
                    self.waste_grid[cy][cx] -= consumed;
                    self.energy = self.energy.saturating_add(consumed / 5);
                }
            }
            nova::OrganelleType::Ribosome => {
                self.process_ribosome(organelle);
            }
            nova::OrganelleType::Void => {
                let (cy, cx) = self.context_loc;
                // Void consumes grid cell if not 0
                let val = self.grid[cy][cx].clone();
                if !matches!(val, Value::Int(0)) {
                    self.grid[cy][cx] = Value::Int(0);
                    self.energy = self.energy.saturating_add(1);
                    self.output.push(format!("VOID: Consumed at {},{}", cx, cy));

                    // Void Song: Check for elemental strings
                    if let Value::Str(s) = val {
                        let note = match s.as_str() {
                            "Fire" => Some("Do"),
                            "Water" => Some("Re"),
                            "Earth" => Some("Mi"),
                            "Air" => Some("Fa"),
                            "Spirit" => Some("Sol"),
                            _ => None,
                        };

                        if let Some(n) = note {
                            self.chorus_buffer.push_back(n.to_string());
                            if self.chorus_buffer.len() > MAX_CHORUS_SIZE {
                                self.chorus_buffer.pop_front();
                            }
                            self.output.push(format!("VOID SONG: {}", n));
                            nova::check_chorus_chords(self);
                        }
                    }
                }

                // Brownian Motion
                let mut rng = rand::thread_rng();
                let dy = rng.gen_range(-1..=1);
                let dx = rng.gen_range(-1..=1);
                organelle.direction = (dy, dx);
            }
            nova::OrganelleType::Alchemist => {
                let (cy, cx) = self.context_loc;
                if nova::perform_alchemy(self, cy, cx) {
                    self.energy = self.energy.saturating_sub(5);
                }

                // Brownian Motion
                let mut rng = rand::thread_rng();
                let dy = rng.gen_range(-1..=1);
                let dx = rng.gen_range(-1..=1);
                organelle.direction = (dy, dx);
            }
            nova::OrganelleType::Worker => {}
        }

        if !matches!(
            organelle.kind,
            nova::OrganelleType::Ribosome
                | nova::OrganelleType::Void
                | nova::OrganelleType::Alchemist
        ) {
            self.execute_organelle_dna(organelle);
        }

        if let Some(new_kind) = self.signal_differentiation.take() {
            organelle.kind = new_kind;
        }

        std::mem::swap(&mut self.stack, &mut organelle.stack);
        std::mem::swap(&mut self.ip, &mut organelle.ip);
        std::mem::swap(&mut self.context_loc, &mut organelle.context_loc);
        std::mem::swap(&mut self.call_stack, &mut organelle.call_stack);
        std::mem::swap(&mut self.recursion_depth, &mut organelle.recursion_depth);

        if let Some(ttl) = organelle.ttl {
            if ttl <= 1 {
                return false;
            }
            organelle.ttl = Some(ttl - 1);
        }

        !organelle.halted
    }

    #[cfg(feature = "nova")]
    fn ribosome_binary_op<F>(&mut self, op: F)
    where
        F: Fn(i64, i64) -> Option<i64>,
    {
        if self.stack.len() >= 2 {
            let b = self.stack.pop().unwrap();
            let a = self.stack.pop().unwrap();
            if let (Value::Int(ia), Value::Int(ib)) = (a, b) {
                if let Some(res) = op(ia, ib) {
                    self.stack.push(Value::Int(res));
                }
            }
        }
    }

    #[cfg(feature = "nova")]
    fn process_ribosome(&mut self, organelle: &mut Organelle) {
        let (cy, cx) = self.context_loc;
        let val = self.grid[cy][cx].clone();
        match val {
            Value::Int(n) => self.stack.push(Value::Int(n)),
            Value::Junction(t, vals) => self.stack.push(Value::Junction(t, vals)),
            Value::Str(s) => match s.as_str() {
                ">" => organelle.direction = (0, 1),
                "<" => organelle.direction = (0, -1),
                "^" => organelle.direction = (-1, 0),
                "v" => organelle.direction = (1, 0),
                "+" => self.ribosome_binary_op(|a, b| Some(a.wrapping_add(b))),
                "-" => self.ribosome_binary_op(|a, b| Some(a.wrapping_sub(b))),
                "*" => {
                    // Bang: Trigger all neighbors
                    let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
                    for (dy, dx) in neighbors {
                        if let Some((ny, nx)) =
                            self.normalize_coords(cy as i64 + dy, cx as i64 + dx)
                        {
                            let mask = match (dy, dx) {
                                (-1, 0) => 1,
                                (1, 0) => 2,
                                (0, 1) => 4,
                                (0, -1) => 8,
                                _ => 0,
                            };
                            if (self.membranes[cy][cx] & mask) == 0 {
                                // Spawn ephemeral Ribosome
                                let new_org = Organelle {
                                    stack: Vec::new(),
                                    ip: (0, 0),
                                    context_loc: (ny, nx),
                                    call_stack: Vec::new(),
                                    recursion_depth: 0,
                                    halted: false,
                                    kind: nova::OrganelleType::Ribosome,
                                    direction: (dy as i8, dx as i8),
                                    ttl: Some(1),
                                };
                                self.organelles.push(new_org);
                            }
                        }
                    }
                }
                "o" => {
                    // Offset Read: [dy, dx] -> [val]
                    if self.stack.len() >= 2 {
                        let x_off = self.stack.pop().unwrap();
                        let y_off = self.stack.pop().unwrap();
                        if let (Value::Int(dx), Value::Int(dy)) = (x_off, y_off) {
                            if let Some((ny, nx)) =
                                self.normalize_coords(cy as i64 + dy, cx as i64 + dx)
                            {
                                self.stack.push(self.grid[ny][nx].clone());
                            } else {
                                self.stack.push(Value::Int(0));
                            }
                        }
                    }
                }
                "x" => {
                    // Offset Write: [val, dy, dx] -> []
                    if self.stack.len() >= 3 {
                        let x_off = self.stack.pop().unwrap();
                        let y_off = self.stack.pop().unwrap();
                        let val = self.stack.pop().unwrap();
                        if let (Value::Int(dx), Value::Int(dy)) = (x_off, y_off) {
                            if let Some((ny, nx)) =
                                self.normalize_coords(cy as i64 + dy, cx as i64 + dx)
                            {
                                self.grid[ny][nx] = val;
                            }
                        }
                    }
                }
                "?" => {
                    let mut rng = rand::thread_rng();
                    self.stack.push(Value::Int(rng.gen_range(0..10)));
                }
                "mul" => {
                    self.ribosome_binary_op(|a, b| Some(a.wrapping_mul(b)));
                }
                "/" => self.ribosome_binary_op(|a, b| {
                    if b != 0 {
                        Some(a.wrapping_div(b))
                    } else {
                        None
                    }
                }),
                "%" => self.ribosome_binary_op(|a, b| {
                    if b != 0 {
                        Some(a.wrapping_rem(b))
                    } else {
                        None
                    }
                }),
                "=" => {
                    if self.stack.len() >= 2 {
                        let b = self.stack.pop().unwrap();
                        let a = self.stack.pop().unwrap();
                        if a == b {
                            self.stack.push(Value::Int(1));
                        } else {
                            self.stack.push(Value::Int(0));
                        }
                    }
                }
                "!" => {
                    if let Some(Value::Int(i)) = self.stack.pop() {
                        self.stack.push(Value::Int(if i == 0 { 1 } else { 0 }));
                    }
                }
                ":" => {
                    if self.stack.len() >= 2 {
                        let x_off = self.stack.pop().unwrap();
                        let y_off = self.stack.pop().unwrap();
                        if let (Value::Int(dx), Value::Int(dy)) = (x_off, y_off) {
                            if let Some((ny, nx)) =
                                self.normalize_coords(cy as i64 + dy, cx as i64 + dx)
                            {
                                self.stack.push(self.grid[ny][nx].clone());
                            }
                        }
                    }
                }
                ";" => {
                    if self.stack.len() >= 3 {
                        let x_off = self.stack.pop().unwrap();
                        let y_off = self.stack.pop().unwrap();
                        let val = self.stack.pop().unwrap();
                        if let (Value::Int(dx), Value::Int(dy)) = (x_off, y_off) {
                            if let Some((ny, nx)) =
                                self.normalize_coords(cy as i64 + dy, cx as i64 + dx)
                            {
                                self.grid[ny][nx] = val;
                            }
                        }
                    }
                }
                _ => {
                    if let Ok(op) = s.parse::<OpCode>() {
                        let _ = self.execute_gene_inner(op, &[]);
                    }
                }
            },
        }

        let (dy, dx) = organelle.direction;
        if let Some((mut new_y, mut new_x)) =
            self.normalize_coords(cy as i64 + dy as i64, cx as i64 + dx as i64)
        {
            let mask = match (dy, dx) {
                (-1, 0) => 1,
                (1, 0) => 2,
                (0, 1) => 4,
                (0, -1) => 8,
                _ => 0,
            };
            if (self.membranes[cy][cx] & mask) == 0 {
                if let Some(&(py, px)) = self.portals.get(&(new_y, new_x)) {
                    self.output.push(format!(
                        "PORTAL: Teleported from {},{} to {},{}",
                        new_x, new_y, px, py
                    ));
                    new_y = py;
                    new_x = px;
                }
                self.context_loc = (new_y, new_x);
            }
        }
    }

    #[cfg(feature = "nova")]
    fn execute_organelle_dna(&mut self, organelle: &mut Organelle) {
        if self.energy > 0 && self.ip.0 < self.dna.helix.strands.len() {
            let strand_len = self.dna.helix.strands[self.ip.0].genes.len();
            if self.ip.1 < strand_len {
                let (gene_op, gene_args) = {
                    let gene = &self.dna.helix.strands[self.ip.0].genes[self.ip.1];
                    (gene.op.clone(), gene.args.clone())
                };

                let jump_target = self.execute_gene(gene_op, &gene_args);

                if let Some(target) = jump_target {
                    self.ip = target;
                } else {
                    self.ip.1 += 1;
                }
            } else {
                organelle.halted = true;
            }
        } else {
            organelle.halted = true;
        }
    }

    /// Advances the simulation by one tick.
    pub fn step(&mut self) {
        if self.halted {
            return;
        }

        #[cfg(feature = "nova")]
        self.blackbox.record(
            &self.dna,
            self.ip,
            &self.stack,
            self.energy,
            self.context_loc,
        );

        self.energy -= 1;

        #[cfg(feature = "nova")]
        self.handle_input_interrupts();

        #[cfg(feature = "cortex")]
        self.update_cortex_state();

        #[cfg(feature = "nova")]
        self.process_environment();

        #[cfg(feature = "biophysics")]
        {
            for neuron in self.neurons.values_mut() {
                neuron.step(0.1);
            }
        }

        #[cfg(feature = "silicon")]
        if self.silicon_mode {
            silicon::step_wireworld(self);
        }

        if self.chaos_mode {
            let mut rng = rand::thread_rng();
            if rng.gen_bool(0.1) {
                self.mutate();
            }
        }

        if self.check_starvation() {
            return;
        }

        #[cfg(feature = "nova")]
        let iterations = if self.phase == nova::Phase::Flux {
            self.energy -= 1;
            2
        } else {
            1
        };
        #[cfg(not(feature = "nova"))]
        let iterations = 1;

        for _ in 0..iterations {
            let helix_len = self.dna.helix.strands.len();
            if self.ip.0 >= helix_len {
                self.halted = true;
                return;
            }

            let strand_len = self.dna.helix.strands[self.ip.0].genes.len();
            if self.ip.1 >= strand_len {
                self.ip.0 += 1;
                self.ip.1 = 0;
                return;
            }

            #[cfg(feature = "nova")]
            {
                if self.check_telomeres() {
                    return;
                }
                if self.epigenome.contains(&self.ip) {
                    self.ip.1 += 1;
                    continue;
                }
                self.active_organelle_kind = None;
            }

            // Clone gene info to release borrow on self.dna
            let (gene_op, gene_args) = {
                let gene = &self.dna.helix.strands[self.ip.0].genes[self.ip.1];
                (gene.op.clone(), gene.args.clone())
            };

            let jump_target = self.execute_gene(gene_op, &gene_args);

            if let Some(target) = jump_target {
                self.ip = target;
            } else {
                #[cfg(feature = "nova")]
                {
                    if self.direction >= 0 {
                        self.ip.1 += 1;
                    } else if self.ip.1 > 0 {
                        self.ip.1 -= 1;
                    } else {
                        // Move to previous non-empty strand
                        let start_strand = self.ip.0;
                        let helix_len = self.dna.helix.strands.len();
                        loop {
                            if self.ip.0 > 0 {
                                self.ip.0 -= 1;
                            } else {
                                self.ip.0 = helix_len.saturating_sub(1);
                            }

                            let len = self.dna.helix.strands[self.ip.0].genes.len();
                            if len > 0 {
                                self.ip.1 = len - 1;
                                break;
                            }

                            if self.ip.0 == start_strand {
                                // All strands empty or cycled back
                                break;
                            }
                        }
                    }
                }
                #[cfg(not(feature = "nova"))]
                {
                    self.ip.1 += 1;
                }
            }
        }

        #[cfg(feature = "nova")]
        self.process_symbiotes();

        #[cfg(feature = "nova")]
        self.process_organelles();
    }

    /// Executes a single gene operation.
    ///
    /// Returns `Some((new_strand, new_gene))` if a jump occurred, or `None` to continue sequentially.
    ///
    /// # Errors
    ///
    /// Runtime errors (stack underflow, type mismatch, division by zero) are silent:
    /// they push an error message to `self.output` and return gracefully, mimicking biological resilience.
    fn execute_gene(&mut self, op: OpCode, args: &[Nucleotide]) -> Option<(usize, usize)> {
        if self.recursion_depth > MAX_RECURSION_DEPTH {
            self.output
                .push("Error: Recursion limit exceeded".to_string());
            return None;
        }
        self.recursion_depth += 1;

        #[cfg(feature = "nova")]
        let effective_op = self.remap_table.get(&op).unwrap_or(&op).clone();
        #[cfg(not(feature = "nova"))]
        let effective_op = op;

        let result = self.execute_gene_inner(effective_op, args);
        self.recursion_depth -= 1;
        result
    }

    pub(crate) fn get_circular_coords(&self, cx: i64, cy: i64, r: i64) -> Vec<(usize, usize)> {
        let mut coords = Vec::new();
        let r_sq = (r as i128).saturating_mul(r as i128);
        for y in 0..GRID_SIZE {
            for x in 0..GRID_SIZE {
                let dx = (x as i64).saturating_sub(cx) as i128;
                let dy = (y as i64).saturating_sub(cy) as i128;
                let dist_sq = dx.saturating_mul(dx).saturating_add(dy.saturating_mul(dy));

                if dist_sq <= r_sq {
                    coords.push((x, y));
                }
            }
        }
        coords
    }

    #[cfg(feature = "nova")]
    fn exec_prion_op(&mut self, op: OpCode, _args: &[Nucleotide]) -> Option<(usize, usize)> {
        match op {
            OpCode::Remap => {
                if self.stack.len() >= 2 {
                    let to_val = self.stack.pop().unwrap();
                    let from_val = self.stack.pop().unwrap();
                    if let (Value::Str(from), Value::Str(to)) = (from_val, to_val) {
                        if let (Ok(from_op), Ok(to_op)) =
                            (from.parse::<OpCode>(), to.parse::<OpCode>())
                        {
                            self.remap_table.insert(from_op.clone(), to_op.clone());
                            self.output.push(format!("REMAP: {} -> {}", from_op, to_op));
                        } else {
                            self.output
                                .push("Error: Invalid OpCode string for remap".to_string());
                        }
                    } else {
                        self.output
                            .push("Error: Type mismatch for remap".to_string());
                    }
                } else {
                    self.output
                        .push("Error: Stack underflow for remap".to_string());
                }
            }
            OpCode::Restore => {
                if let Some(val) = self.stack.pop() {
                    if let Value::Str(s) = val {
                        if let Ok(op) = s.parse::<OpCode>() {
                            if self.remap_table.remove(&op).is_some() {
                                self.output.push(format!("RESTORE: {}", op));
                            } else {
                                self.output
                                    .push(format!("RESTORE: {} was not remapped", op));
                            }
                        } else {
                            self.output
                                .push("Error: Invalid OpCode string for restore".to_string());
                        }
                    } else {
                        self.output
                            .push("Error: Type mismatch for restore".to_string());
                    }
                } else {
                    self.output
                        .push("Error: Stack underflow for restore".to_string());
                }
            }
            OpCode::Mirror => {
                self.direction *= -1;
                self.output
                    .push(format!("MIRROR: Direction {}", self.direction));
            }
            _ => {}
        }
        None
    }

    pub(crate) fn execute_gene_inner(
        &mut self,
        op: OpCode,
        args: &[Nucleotide],
    ) -> Option<(usize, usize)> {
        match op {
            OpCode::Push => self.exec_stack_op(op, args),
            OpCode::Add | OpCode::Sub | OpCode::Mul | OpCode::Div => {
                self.exec_math_op(op);
                None
            }
            OpCode::Dup | OpCode::Swap | OpCode::Drop => self.exec_stack_op(op, args),
            OpCode::Print => {
                self.exec_io_op(op);
                None
            }
            OpCode::Jump | OpCode::Brz => self.exec_flow_op(op, args),
            OpCode::Photosynthesize | OpCode::Consume => self.exec_bio_op(op, args),
            OpCode::GRead | OpCode::GWrite | OpCode::Radiate | OpCode::Siphon => {
                self.exec_grid_op(op)
            }
            OpCode::Genome | OpCode::Transcribe => self.exec_bio_op(op, args),
            OpCode::Virus => self.exec_grid_op(op),
            OpCode::JumpS | OpCode::BrzS => self.exec_flow_op(op, args),
            OpCode::SLen | OpCode::HelixLen | OpCode::GeneLen => self.exec_stack_op(op, args),
            #[cfg(feature = "cortex")]
            OpCode::Link | OpCode::Sever | OpCode::Spark | OpCode::Sense | OpCode::Gate => {
                cortex::exec_cortex_op(self, op, args);
                None
            }

            #[cfg(feature = "nova")]
            OpCode::Remap | OpCode::Restore | OpCode::Mirror => self.exec_prion_op(op, args),

            #[cfg(feature = "nova")]
            OpCode::AkashicWrite | OpCode::AkashicRead => {
                akashic::exec_akashic_op(self, op, args);
                None
            }

            #[cfg(feature = "nova")]
            OpCode::Blackbox => {
                let dump = self.blackbox.dump();
                self.stack.push(Value::Str(dump));
                None
            }

            #[cfg(feature = "nova")]
            OpCode::Invoke => nova_sigil::exec_invoke(self, op, args),

            #[cfg(feature = "nova")]
            OpCode::Alchemy
            | OpCode::Prophecy
            | OpCode::Sing
            | OpCode::Listen
            | OpCode::Hyphae
            | OpCode::Connect
            | OpCode::Transport
            | OpCode::SporeCloud
            | OpCode::Brainfuck
            | OpCode::Irradiate
            | OpCode::SenseMutagen
            | OpCode::Devour
            | OpCode::Evolve
            | OpCode::Glitch
            | OpCode::Scramble
            | OpCode::Pigment
            | OpCode::Glyph
            | OpCode::SensePigment
            | OpCode::SenseGlyph
            | OpCode::Rift
            | OpCode::Seal
            | OpCode::Shape
            | OpCode::Void
            | OpCode::Supernova
            | OpCode::Singularity
            | OpCode::Simulate
            | OpCode::Dream
            | OpCode::Chemotaxis
            | OpCode::Identity
            | OpCode::Differentiate
            | OpCode::Sporulate
            | OpCode::Germinate
            | OpCode::Spawn
            | OpCode::Incubate
            | OpCode::Methylate
            | OpCode::Demethylate
            | OpCode::Telomerase
            | OpCode::TLen
            | OpCode::Recombine
            | OpCode::SIndex
            | OpCode::CrisprScan
            | OpCode::Cas9Cut
            | OpCode::Ligase
            | OpCode::Mitosis
            | OpCode::Apoptosis
            | OpCode::Integrase
            | OpCode::Excision
            | OpCode::Secrete
            | OpCode::Detect
            | OpCode::Absorb
            | OpCode::Migrate
            | OpCode::Detox
            | OpCode::WRead
            | OpCode::Call
            | OpCode::Ret
            | OpCode::Bind
            | OpCode::Unbind
            | OpCode::Entangle
            | OpCode::Decohere
            | OpCode::Conjugate
            | OpCode::Gravitate
            | OpCode::Lumine
            | OpCode::SenseLight
            | OpCode::Broadcast
            | OpCode::Tune
            | OpCode::PhaseShift
            | OpCode::Membrane
            | OpCode::Osmosis
            | OpCode::Symbiosis
            | OpCode::Lysis
            | OpCode::Reflex
            | OpCode::Compile
            | OpCode::Decompile
            | OpCode::Sonar
            | OpCode::Eval
            | OpCode::Map
            | OpCode::Fold
            | OpCode::Filter
            | OpCode::Zip => nova::exec_nova_op(self, op, args),

            #[cfg(feature = "nova")]
            OpCode::Note | OpCode::Rest | OpCode::Tempo | OpCode::Perform => {
                bard::exec_bard_op(self, op, args);
                None
            }

            #[cfg(feature = "oracle")]
            OpCode::Assert | OpCode::Retract | OpCode::Query => {
                oracle::exec_oracle_op(self, op, args);
                None
            }

            #[cfg(feature = "resonance")]
            OpCode::Pluck => {
                resonance::exec_resonance_op(self, op, args);
                None
            }

            #[cfg(feature = "biophysics")]
            OpCode::NeuroGenesis | OpCode::Stimulate | OpCode::Dendrite | OpCode::Axon => {
                neuron::exec_biophysics_op(self, op, args);
                None
            }

            #[cfg(feature = "silicon")]
            OpCode::Conduct | OpCode::Wire | OpCode::Pulse | OpCode::Silicon => {
                silicon::exec_silicon_op(self, op, args);
                None
            }

            OpCode::Unknown(name) => {
                self.output.push(format!("Unknown enzyme: {}", name));
                None
            }
        }
    }

    fn binary_op<F>(stack: &mut Vec<Value>, output: &mut Vec<String>, op: F)
    where
        F: Fn(i64, i64) -> i64 + Copy,
    {
        if stack.len() < 2 {
            output.push("Error: Stack underflow".to_string());
            return;
        }
        let b = stack.pop().unwrap();
        let a = stack.pop().unwrap();

        fn apply<F>(a: Value, b: Value, op: F, depth: usize) -> Option<Value>
        where
            F: Fn(i64, i64) -> i64 + Copy,
        {
            if depth > MAX_RECURSION_DEPTH {
                return None;
            }
            match (a, b) {
                (Value::Int(ia), Value::Int(ib)) => Some(Value::Int(op(ia, ib))),
                (Value::Junction(t, vals), scalar @ Value::Int(_)) => {
                    let mut res = Vec::new();
                    for v in vals {
                        if res.len() >= MAX_JUNCTION_SIZE {
                            return None;
                        }
                        if let Some(r) = apply(v, scalar.clone(), op, depth + 1) {
                            res.push(r);
                        } else {
                            return None;
                        }
                    }
                    Some(Value::Junction(t, res))
                }
                (scalar @ Value::Int(_), Value::Junction(t, vals)) => {
                    let mut res = Vec::new();
                    for v in vals {
                        if res.len() >= MAX_JUNCTION_SIZE {
                            return None;
                        }
                        if let Some(r) = apply(scalar.clone(), v, op, depth + 1) {
                            res.push(r);
                        } else {
                            return None;
                        }
                    }
                    Some(Value::Junction(t, res))
                }
                (Value::Junction(ta, va), Value::Junction(_tb, vb)) => {
                    // Cross product, defaulting to type of A
                    let mut res = Vec::new();
                    for xa in va {
                        for xb in &vb {
                            if res.len() >= MAX_JUNCTION_SIZE {
                                return None;
                            }
                            if let Some(r) = apply(xa.clone(), xb.clone(), op, depth + 1) {
                                res.push(r);
                            }
                        }
                    }
                    Some(Value::Junction(ta, res))
                }
                _ => None,
            }
        }

        if let Some(res) = apply(a, b, op, 0) {
            stack.push(res);
        } else {
            output.push("Error: Type mismatch or complexity limit".to_string());
        }
    }

    fn exec_stack_op(&mut self, op: OpCode, args: &[Nucleotide]) -> Option<(usize, usize)> {
        match op {
            OpCode::Push => {
                if let Some(arg) = args.first() {
                    fn nuc_to_val(n: &Nucleotide, depth: usize) -> Option<Value> {
                        if depth > MAX_RECURSION_DEPTH {
                            return None;
                        }
                        match n {
                            Nucleotide::Number(v) => Some(Value::Int(*v)),
                            Nucleotide::String(s) => Some(Value::Str(s.clone())),
                            Nucleotide::Junction(t, list) => {
                                let mut vals = Vec::new();
                                for item in list {
                                    if let Some(v) = nuc_to_val(item, depth + 1) {
                                        vals.push(v);
                                    } else {
                                        return None;
                                    }
                                }
                                Some(Value::Junction(*t, vals))
                            }
                            _ => None,
                        }
                    }

                    if let Some(val) = nuc_to_val(arg, 0) {
                        self.stack.push(val);
                    } else {
                        self.output
                            .push(format!("Error: Invalid arg for push: {:?}", arg));
                    }
                }
            }
            OpCode::Dup => {
                if let Some(val) = self.stack.last() {
                    self.stack.push(val.clone());
                }
            }
            OpCode::Swap => {
                let len = self.stack.len();
                if len >= 2 {
                    self.stack.swap(len - 1, len - 2);
                } else {
                    self.output
                        .push("Error: Stack underflow for swap".to_string());
                }
            }
            OpCode::Drop => {
                self.stack.pop();
            }
            OpCode::SLen => {
                self.stack.push(Value::Int(self.stack.len() as i64));
            }
            OpCode::HelixLen => {
                self.stack
                    .push(Value::Int(self.dna.helix.strands.len() as i64));
            }
            OpCode::GeneLen => {
                if let Some(val) = self.stack.pop() {
                    match val {
                        Value::Int(idx) => {
                            if idx >= 0 && (idx as usize) < self.dna.helix.strands.len() {
                                let len = self.dna.helix.strands[idx as usize].genes.len();
                                self.stack.push(Value::Int(len as i64));
                            } else {
                                self.output.push(
                                    "Error: Strand index out of bounds for gene_len".to_string(),
                                );
                            }
                        }
                        _ => self
                            .output
                            .push("Error: Type mismatch for gene_len".to_string()),
                    }
                } else {
                    self.output
                        .push("Error: Stack underflow for gene_len".to_string());
                }
            }
            _ => {}
        }
        None
    }

    fn exec_math_op(&mut self, op: OpCode) {
        match op {
            OpCode::Add => {
                Self::binary_op(&mut self.stack, &mut self.output, |a, b| a.wrapping_add(b));
            }
            OpCode::Sub => {
                Self::binary_op(&mut self.stack, &mut self.output, |a, b| a.wrapping_sub(b));
            }
            OpCode::Mul => {
                Self::binary_op(&mut self.stack, &mut self.output, |a, b| a.wrapping_mul(b));
            }
            OpCode::Div => {
                if self.stack.len() < 2 {
                    self.output.push("Error: Stack underflow".to_string());
                } else {
                    let b_val = self.stack.pop().unwrap();
                    let a_val = self.stack.pop().unwrap();
                    match (a_val, b_val) {
                        (Value::Int(a), Value::Int(b)) => {
                            if b == 0 {
                                self.output.push("Error: Division by zero".to_string());
                            } else if a == i64::MIN && b == -1 {
                                self.output.push("Error: Division overflow".to_string());
                            } else {
                                self.stack.push(Value::Int(a / b));
                            }
                        }
                        _ => self.output.push("Error: Type mismatch".to_string()),
                    }
                }
            }
            _ => {}
        }
    }

    fn exec_io_op(&mut self, op: OpCode) {
        if let OpCode::Print = op {
            if let Some(val) = self.stack.pop() {
                self.output.push(format!("{}", val));
            }
        }
    }

    fn exec_flow_op(&mut self, op: OpCode, args: &[Nucleotide]) -> Option<(usize, usize)> {
        match op {
            OpCode::Jump => {
                if let Some(Nucleotide::Number(n)) = args.first() {
                    Some((*n as usize, 0))
                } else {
                    self.output.push("Error: Invalid arg for jump".to_string());
                    None
                }
            }
            OpCode::Brz => {
                if let Some(Nucleotide::Number(n)) = args.first() {
                    if let Some(val) = self.stack.pop() {
                        fn check_zero(v: &Value) -> bool {
                            match v {
                                Value::Int(i) => *i == 0,
                                Value::Junction(t, vals) => match t {
                                    JunctionType::Any => vals.iter().any(check_zero),
                                    JunctionType::All => vals.iter().all(check_zero),
                                },
                                _ => false,
                            }
                        }

                        if check_zero(&val) {
                            return Some((*n as usize, 0));
                        }
                    } else {
                        self.output
                            .push("Error: Stack underflow for brz".to_string());
                    }
                } else {
                    self.output.push("Error: Invalid arg for brz".to_string());
                }
                None
            }
            OpCode::JumpS => {
                if let Some(val) = self.stack.pop() {
                    match val {
                        Value::Int(target) => {
                            if target >= 0 {
                                return Some((target as usize, 0));
                            } else {
                                self.output.push("Error: Negative jump target".to_string());
                            }
                        }
                        _ => self
                            .output
                            .push("Error: Type mismatch for jump_s".to_string()),
                    }
                } else {
                    self.output
                        .push("Error: Stack underflow for jump_s".to_string());
                }
                None
            }
            OpCode::BrzS => {
                if self.stack.len() >= 2 {
                    let target_val = self.stack.pop().unwrap();
                    let cond_val = self.stack.pop().unwrap();

                    match (target_val, cond_val) {
                        (Value::Int(target), Value::Int(cond)) => {
                            if cond == 0 {
                                if target >= 0 {
                                    return Some((target as usize, 0));
                                } else {
                                    self.output.push("Error: Negative jump target".to_string());
                                }
                            }
                        }
                        _ => self
                            .output
                            .push("Error: Type mismatch for brz_s".to_string()),
                    }
                } else {
                    self.output
                        .push("Error: Stack underflow for brz_s".to_string());
                }
                None
            }
            _ => None,
        }
    }

    fn exec_grid_op(&mut self, op: OpCode) -> Option<(usize, usize)> {
        match op {
            OpCode::GRead => {
                if self.stack.len() >= 2 {
                    let x_val = self.stack.pop().unwrap();
                    let y_val = self.stack.pop().unwrap();
                    if let (Value::Int(y), Value::Int(x)) = (y_val, x_val) {
                        if self.is_valid_coord(y, x) {
                            self.stack.push(self.grid[y as usize][x as usize].clone());
                        } else {
                            self.output
                                .push("Error: Grid index out of bounds".to_string());
                        }
                    } else {
                        self.output
                            .push("Error: Type mismatch for g_read".to_string());
                    }
                } else {
                    self.output
                        .push("Error: Stack underflow for g_read".to_string());
                }
            }
            OpCode::GWrite => {
                #[cfg(feature = "nova")]
                if self.phase == nova::Phase::Ethereal {
                    self.output
                        .push("Error: Ethereal phase prevents GWrite".to_string());
                    return None;
                }

                if self.stack.len() >= 3 {
                    let x_val = self.stack.pop().unwrap();
                    let y_val = self.stack.pop().unwrap();
                    let val = self.stack.pop().unwrap();
                    if let (Value::Int(y), Value::Int(x)) = (y_val, x_val) {
                        if self.is_valid_coord(y, x) {
                            self.grid[y as usize][x as usize] = val;
                        } else {
                            self.output
                                .push("Error: Grid index out of bounds".to_string());
                        }
                    } else {
                        self.output
                            .push("Error: Type mismatch for g_write".to_string());
                    }
                } else {
                    self.output
                        .push("Error: Stack underflow for g_write".to_string());
                }
            }
            OpCode::Radiate => {
                if self.stack.len() >= 4 {
                    let x_val = self.stack.pop().unwrap();
                    let y_val = self.stack.pop().unwrap();
                    let r_val = self.stack.pop().unwrap();
                    let val = self.stack.pop().unwrap();

                    if let (Value::Int(x), Value::Int(y), Value::Int(r)) = (x_val, y_val, r_val) {
                        let coords = self.get_circular_coords(x, y, r);
                        let count = coords.len();
                        for (cx, cy) in coords {
                            self.grid[cy][cx] = val.clone();
                        }
                        self.energy = self.energy.saturating_sub((count / 2) as i64);
                        self.output.push(format!(
                            "RADIATE: Affected {} cells at {},{} r={}",
                            count, x, y, r
                        ));
                    } else {
                        self.output
                            .push("Error: Type mismatch for radiate".to_string());
                    }
                } else {
                    self.output
                        .push("Error: Stack underflow for radiate".to_string());
                }
            }
            OpCode::Siphon => {
                if self.stack.len() >= 3 {
                    let x_val = self.stack.pop().unwrap();
                    let y_val = self.stack.pop().unwrap();
                    let r_val = self.stack.pop().unwrap();

                    if let (Value::Int(x), Value::Int(y), Value::Int(r)) = (x_val, y_val, r_val) {
                        let coords = self.get_circular_coords(x, y, r);
                        let count = coords.len();
                        let mut sum: i64 = 0;
                        for (cx, cy) in coords {
                            if let Value::Int(n) = self.grid[cy][cx] {
                                sum = sum.saturating_add(n);
                            }
                            self.grid[cy][cx] = Value::Int(0);
                        }
                        self.stack.push(Value::Int(sum));
                        self.energy = self.energy.saturating_sub(5);
                        self.output
                            .push(format!("SIPHON: Absorbed {} from {} cells", sum, count));
                    } else {
                        self.output
                            .push("Error: Type mismatch for siphon".to_string());
                    }
                } else {
                    self.output
                        .push("Error: Stack underflow for siphon".to_string());
                }
            }
            OpCode::Virus => {
                if self.stack.len() >= 2 {
                    let x_val = self.stack.pop().unwrap();
                    let y_val = self.stack.pop().unwrap();

                    let coords = if let (Value::Int(y), Value::Int(x)) = (&y_val, &x_val) {
                        if self.is_valid_coord(*y, *x) {
                            Some((*y, *x))
                        } else {
                            self.output
                                .push("Error: Grid index out of bounds".to_string());
                            None
                        }
                    } else {
                        self.output
                            .push("Error: Type mismatch for virus".to_string());
                        None
                    };

                    if let Some((y, x)) = coords {
                        let val = self.grid[y as usize][x as usize].clone();
                        match val {
                            Value::Int(n) => self.stack.push(Value::Int(n)),
                            Value::Str(s) => {
                                let old_loc = self.context_loc;
                                self.context_loc = (y as usize, x as usize);
                                let op = s.parse().unwrap_or(OpCode::Unknown(s.clone()));
                                let result = self.execute_gene(op, &[]);
                                self.context_loc = old_loc;
                                return result;
                            }
                            Value::Junction(_, _) => {
                                self.output
                                    .push("Error: Virus cannot execute junction".to_string());
                            }
                        }
                    }
                } else {
                    self.output
                        .push("Error: Stack underflow for virus".to_string());
                }
            }
            _ => {}
        }
        None
    }

    fn exec_bio_op(&mut self, op: OpCode, _args: &[Nucleotide]) -> Option<(usize, usize)> {
        match op {
            OpCode::Photosynthesize => {
                self.energy = self.energy.saturating_add(5);
            }
            OpCode::Consume => {
                if let Some(val) = self.stack.pop() {
                    match val {
                        Value::Int(n) => self.energy = self.energy.saturating_add(n),
                        Value::Str(s) => self.energy = self.energy.saturating_add(s.len() as i64),
                        Value::Junction(_, _) => {
                            self.output
                                .push("Error: Cannot consume junction".to_string());
                        }
                    }
                } else {
                    self.output
                        .push("Error: Stack underflow for consume".to_string());
                }
            }
            OpCode::Genome => {
                if self.ip.0 < self.dna.helix.strands.len() {
                    let strand = &self.dna.helix.strands[self.ip.0];
                    self.stack.push(Value::Int(strand.genes.len() as i64));
                    for gene in &strand.genes {
                        self.stack.push(Value::Str(gene.op.to_string()));
                    }
                }
            }
            OpCode::Transcribe => {
                if self.stack.len() < 4 {
                    self.output
                        .push("Error: Stack underflow for transcribe".to_string());
                    return None;
                }
                let val = self.stack.pop().unwrap();
                let arg_idx_val = self.stack.pop().unwrap();
                let gene_idx_val = self.stack.pop().unwrap();
                let strand_idx_val = self.stack.pop().unwrap();

                match (val, arg_idx_val, gene_idx_val, strand_idx_val) {
                    (Value::Int(v), Value::Int(ai), Value::Int(gi), Value::Int(si)) => {
                        let si_idx = si as usize;
                        #[cfg(feature = "nova")]
                        let mut success = false;
                        if si >= 0 && si_idx < self.dna.helix.strands.len() {
                            let strand = &mut self.dna.helix.strands[si_idx];
                            if gi >= 0 && (gi as usize) < strand.genes.len() {
                                let gene = &mut strand.genes[gi as usize];
                                if ai >= 0 && (ai as usize) < gene.args.len() {
                                    gene.args[ai as usize] = Nucleotide::Number(v);
                                    self.output.push(format!(
                                        "TRANSCRIBE: strand {} gene {} arg {} -> {}",
                                        si, gi, ai, v
                                    ));
                                    #[cfg(feature = "nova")]
                                    {
                                        success = true;
                                    }
                                } else {
                                    self.output
                                        .push("Error: Arg index out of bounds".to_string());
                                }
                            } else {
                                self.output
                                    .push("Error: Gene index out of bounds".to_string());
                            }
                        } else {
                            self.output
                                .push("Error: Strand index out of bounds".to_string());
                        }

                        #[cfg(feature = "nova")]
                        if success {
                            if let Some(&partner_idx) = self.entangled_pairs.get(&si_idx) {
                                if partner_idx < self.dna.helix.strands.len() {
                                    let p_strand = &mut self.dna.helix.strands[partner_idx];
                                    if (gi as usize) < p_strand.genes.len() {
                                        let p_gene = &mut p_strand.genes[gi as usize];
                                        if (ai as usize) < p_gene.args.len() {
                                            p_gene.args[ai as usize] = Nucleotide::Number(v);
                                            self.output.push(format!(
                                                "ENTANGLEMENT: Transcribed partner {} gene {} arg {}",
                                                partner_idx, gi, ai
                                            ));
                                        }
                                    }
                                }
                            }
                        }
                    }
                    _ => self
                        .output
                        .push("Error: Type mismatch for transcribe args".to_string()),
                }
            }
            _ => {}
        }
        None
    }

    pub fn mutate(&mut self) {
        #[cfg(feature = "nova")]
        if self.phase == nova::Phase::Crystalline {
            return;
        }

        let mut rng = rand::thread_rng();
        let helix_len = self.dna.helix.strands.len();
        if helix_len == 0 {
            return;
        }

        let strand_idx = rng.gen_range(0..helix_len);
        let gene_count = self.dna.helix.strands[strand_idx].genes.len();
        if gene_count == 0 {
            return;
        }

        let gene_idx = rng.gen_range(0..gene_count);

        // 50% chance to change name, 50% to change arg
        if rng.gen_bool(0.5) {
            let enzymes = [
                OpCode::Push,
                OpCode::Add,
                OpCode::Sub,
                OpCode::Mul,
                OpCode::Div,
                OpCode::Dup,
                OpCode::Print,
                OpCode::Swap,
                OpCode::Drop,
                OpCode::Jump,
                OpCode::Brz,
                OpCode::Photosynthesize,
                OpCode::Consume,
                OpCode::Transcribe,
                OpCode::SLen,
                OpCode::HelixLen,
                OpCode::GeneLen,
                OpCode::GRead,
                OpCode::GWrite,
                OpCode::Radiate,
                OpCode::Siphon,
                OpCode::Genome,
                #[cfg(feature = "nova")]
                OpCode::Telomerase,
                #[cfg(feature = "nova")]
                OpCode::TLen,
                #[cfg(feature = "nova")]
                OpCode::SIndex,
                #[cfg(feature = "nova")]
                OpCode::Mitosis,
                #[cfg(feature = "nova")]
                OpCode::Apoptosis,
                #[cfg(feature = "nova")]
                OpCode::CrisprScan,
                #[cfg(feature = "nova")]
                OpCode::Cas9Cut,
                #[cfg(feature = "nova")]
                OpCode::Ligase,
                #[cfg(feature = "nova")]
                OpCode::Entangle,
                #[cfg(feature = "nova")]
                OpCode::Decohere,
            ];
            let new_op = enzymes[rng.gen_range(0..enzymes.len())].clone();

            // Apply to primary
            let old_op = self.dna.helix.strands[strand_idx].genes[gene_idx]
                .op
                .clone();
            self.dna.helix.strands[strand_idx].genes[gene_idx].op = new_op.clone();
            self.output
                .push(format!("MUTATION: {} -> {}", old_op, new_op));

            #[cfg(feature = "nova")]
            {
                self.trigger_reflex(1); // Event 1: Mutation
            }

            #[cfg(feature = "nova")]
            if let Some(&partner_idx) = self.entangled_pairs.get(&strand_idx) {
                if partner_idx < self.dna.helix.strands.len()
                    && gene_idx < self.dna.helix.strands[partner_idx].genes.len()
                {
                    self.dna.helix.strands[partner_idx].genes[gene_idx].op = new_op;
                    self.output.push(format!(
                        "ENTANGLEMENT: Mutated partner {} gene {} op",
                        partner_idx, gene_idx
                    ));
                }
            }
        } else {
            let has_args = !self.dna.helix.strands[strand_idx].genes[gene_idx]
                .args
                .is_empty();
            if has_args {
                let old_n = match &self.dna.helix.strands[strand_idx].genes[gene_idx].args[0] {
                    Nucleotide::Number(n) => *n,
                    _ => return, // Skip non-number args for simplicity
                };
                let new_n = rng.gen_range(0..100);

                // Apply
                self.dna.helix.strands[strand_idx].genes[gene_idx].args[0] =
                    Nucleotide::Number(new_n);
                self.output
                    .push(format!("MUTATION: arg {} -> {}", old_n, new_n));

                #[cfg(feature = "nova")]
                {
                    self.trigger_reflex(1); // Event 1: Mutation
                }

                #[cfg(feature = "nova")]
                if let Some(&partner_idx) = self.entangled_pairs.get(&strand_idx) {
                    if partner_idx < self.dna.helix.strands.len()
                        && gene_idx < self.dna.helix.strands[partner_idx].genes.len()
                        && !self.dna.helix.strands[partner_idx].genes[gene_idx]
                            .args
                            .is_empty()
                    {
                        self.dna.helix.strands[partner_idx].genes[gene_idx].args[0] =
                            Nucleotide::Number(new_n);
                        self.output.push(format!(
                            "ENTANGLEMENT: Mutated partner {} gene {} arg",
                            partner_idx, gene_idx
                        ));
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};

    fn make_dna(genes: Vec<Gene>) -> Dna {
        Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        }
    }

    #[test]
    fn test_add() {
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(10)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(20)],
            },
            Gene {
                op: OpCode::Add,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));
        while !vm.halted {
            vm.step();
        }
        assert_eq!(vm.stack.len(), 1);
        match vm.stack[0] {
            Value::Int(30) => (),
            _ => panic!("Expected 30"),
        }
    }

    #[test]
    fn test_jump() {
        // [ jump(1) push(100) ] [ push(200) ]
        let strand0 = Strand {
            genes: vec![
                Gene {
                    op: OpCode::Jump,
                    args: vec![Nucleotide::Number(1)],
                },
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(100)],
                },
            ],
        };
        let strand1 = Strand {
            genes: vec![Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(200)],
            }],
        };

        let dna = Dna {
            helix: Helix {
                strands: vec![strand0, strand1],
            },
        };
        let mut vm = ChimeraVM::new(dna);

        // Step 1: jump(1)
        vm.step();
        assert_eq!(vm.ip, (1, 0));

        // Step 2: push(200)
        vm.step();
        assert_eq!(vm.stack.len(), 1);
        if let Value::Int(i) = vm.stack[0] {
            assert_eq!(i, 200);
        } else {
            panic!("Expected 200");
        }
    }

    #[test]
    fn test_brz() {
        // [ push(0) brz(1) push(100) ] [ push(200) ]
        let strand0 = Strand {
            genes: vec![
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(0)],
                },
                Gene {
                    op: OpCode::Brz,
                    args: vec![Nucleotide::Number(1)],
                },
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(100)],
                },
            ],
        };
        let strand1 = Strand {
            genes: vec![Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(200)],
            }],
        };

        let dna = Dna {
            helix: Helix {
                strands: vec![strand0, strand1],
            },
        };
        let mut vm = ChimeraVM::new(dna);

        // Step 1: push(0)
        vm.step();

        // Step 2: brz(1) -> pops 0, jumps to (1, 0)
        vm.step();
        assert_eq!(vm.ip, (1, 0));
        assert_eq!(vm.stack.len(), 0);

        // Step 3: push(200)
        vm.step();
        if let Value::Int(i) = vm.stack[0] {
            assert_eq!(i, 200);
        } else {
            panic!("Expected 200");
        }
    }

    #[test]
    fn test_transcribe() {
        // [ push(0) push(0) push(0) push(99) transcribe() push(0) ]
        // The last push(0) should be modified to push(99)
        // stack order: strand, gene, arg, value
        // strand 0, gene 5 (the last push), arg 0 -> 99
        let strand0 = Strand {
            genes: vec![
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(0)],
                }, // 0: strand idx
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(5)],
                }, // 1: gene idx
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(0)],
                }, // 2: arg idx
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(99)],
                }, // 3: value
                Gene {
                    op: OpCode::Transcribe,
                    args: vec![],
                }, // 4: transcribe
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(0)],
                }, // 5: target to be modified
            ],
        };
        let dna = Dna {
            helix: Helix {
                strands: vec![strand0],
            },
        };
        let mut vm = ChimeraVM::new(dna);

        for _ in 0..5 {
            vm.step();
        }

        // After transcribe, check if the last gene is modified
        if let Nucleotide::Number(n) = vm.dna.helix.strands[0].genes[5].args[0] {
            assert_eq!(n, 99);
        } else {
            panic!("Gene not modified");
        }

        // Execute the modified gene
        vm.step();
        assert_eq!(vm.stack.last().unwrap(), &Value::Int(99));
    }

    #[test]
    fn test_div_by_zero() {
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(10)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            },
            Gene {
                op: OpCode::Div,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));
        while !vm.halted {
            vm.step();
        }

        // We expect an error message instead of a panic
        assert!(
            vm.output.iter().any(|s| s.contains("Division by zero")),
            "Expected 'Division by zero' error, got: {:?}",
            vm.output
        );
    }

    #[test]
    fn test_div_overflow() {
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(i64::MIN)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(-1)],
            },
            Gene {
                op: OpCode::Div,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));
        while !vm.halted {
            vm.step();
        }

        assert!(
            vm.output.iter().any(|s| s.contains("Division overflow")),
            "Expected 'Division overflow' error, got: {:?}",
            vm.output
        );
    }

    #[test]
    fn test_stack_underflow() {
        let genes = vec![Gene {
            op: OpCode::Add, // Requires 2 args, stack has 0
            args: vec![],
        }];
        let mut vm = ChimeraVM::new(make_dna(genes));
        while !vm.halted {
            vm.step();
        }

        assert!(
            vm.output.iter().any(|s| s.contains("Stack underflow")),
            "Expected 'Stack underflow' error, got: {:?}",
            vm.output
        );
    }

    #[test]
    fn test_type_mismatch() {
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(10)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("foo".to_string())],
            },
            Gene {
                op: OpCode::Add,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));
        while !vm.halted {
            vm.step();
        }

        assert!(
            vm.output.iter().any(|s| s.contains("Type mismatch")),
            "Expected 'Type mismatch' error, got: {:?}",
            vm.output
        );
    }

    #[test]
    fn test_starvation() {
        // [ jump(0) ] - infinite loop, no food
        let genes = vec![Gene {
            op: OpCode::Jump,
            args: vec![Nucleotide::Number(0)],
        }];
        let mut vm = ChimeraVM::new(make_dna(genes));
        // Start energy is 50. Should die after 50 steps.
        for _ in 0..60 {
            vm.step();
        }
        assert!(vm.halted);
        assert!(vm.output.contains(&"DEATH: STARVATION".to_string()));
    }

    #[test]
    fn test_metabolism() {
        // [ photosynthesize() jump(0) ]
        // Cost: 2 per loop. Gain: 5 per loop. Net +3.
        let genes = vec![
            Gene {
                op: OpCode::Photosynthesize,
                args: vec![],
            },
            Gene {
                op: OpCode::Jump,
                args: vec![Nucleotide::Number(0)],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));

        #[cfg(feature = "nova")]
        {
            // Give enough telomeres for the test
            vm.telomeres[0] = 1000;
        }

        for _ in 0..100 {
            vm.step();
        }
        assert!(!vm.halted);
        assert!(vm.energy > 50);
    }

    #[test]
    fn test_consume_survival() {
        // [ push(10) consume() jump(0) ]
        // Cost: 3 per loop. Gain: 10 per loop. Net +7.
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(10)],
            },
            Gene {
                op: OpCode::Consume,
                args: vec![],
            },
            Gene {
                op: OpCode::Jump,
                args: vec![Nucleotide::Number(0)],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));

        for _ in 0..100 {
            vm.step();
        }
        assert!(!vm.halted);
        assert!(vm.energy > 50);
    }

    #[test]
    fn test_grid_ops() {
        // [ push(42) push(10) push(10) g_write() push(10) push(10) g_read() ]
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(42)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(10)],
            }, // y
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(10)],
            }, // x
            Gene {
                op: OpCode::GWrite,
                args: vec![],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(10)],
            }, // y
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(10)],
            }, // x
            Gene {
                op: OpCode::GRead,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));
        while !vm.halted {
            vm.step();
        }
        assert_eq!(vm.stack.len(), 1);
        match vm.stack[0] {
            Value::Int(42) => (),
            _ => panic!("Expected 42"),
        }

        // Verify grid state directly
        match vm.grid[10][10] {
            Value::Int(42) => (),
            _ => panic!("Grid not updated"),
        }
    }

    #[test]
    fn test_virus_int() {
        // [ push(42) push(5) push(5) g_write() push(5) push(5) virus() ]
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(42)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(5)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(5)],
            },
            Gene {
                op: OpCode::GWrite,
                args: vec![],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(5)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(5)],
            },
            Gene {
                op: OpCode::Virus,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));
        while !vm.halted {
            vm.step();
        }
        assert_eq!(vm.stack.last().unwrap(), &Value::Int(42));
    }

    #[test]
    fn test_virus_enzyme() {
        // [ push("add") push(6) push(6) g_write() push(10) push(20) push(6) push(6) virus() ]
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String("add".to_string())],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(6)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(6)],
            },
            Gene {
                op: OpCode::GWrite,
                args: vec![],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(10)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(20)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(6)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(6)],
            },
            Gene {
                op: OpCode::Virus,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));
        while !vm.halted {
            vm.step();
        }
        assert_eq!(vm.stack.len(), 1);
        assert_eq!(vm.stack[0], Value::Int(30));
    }

    #[test]
    fn test_jump_s() {
        // [ push(1) jump_s() push(100) ] [ push(200) ]
        let strand0 = Strand {
            genes: vec![
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(1)],
                },
                Gene {
                    op: OpCode::JumpS,
                    args: vec![],
                },
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(100)],
                },
            ],
        };
        let strand1 = Strand {
            genes: vec![Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(200)],
            }],
        };
        let dna = Dna {
            helix: Helix {
                strands: vec![strand0, strand1],
            },
        };
        let mut vm = ChimeraVM::new(dna);

        vm.step(); // push(1)
        vm.step(); // jump_s
        assert_eq!(vm.ip, (1, 0));

        vm.step(); // push(200)
        assert_eq!(vm.stack.pop(), Some(Value::Int(200)));
    }

    #[test]
    fn test_radiate() {
        // [ push(100) push(2) push(8) push(8) radiate() ]
        // Writes 100 to circle radius 2 at 8,8
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(100)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(2)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(8)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(8)],
            },
            Gene {
                op: OpCode::Radiate,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));
        while !vm.halted {
            vm.step();
        }

        // Center should be 100
        assert_eq!(vm.grid[8][8], Value::Int(100));
        // (8+1, 8) should be 100
        assert_eq!(vm.grid[8][9], Value::Int(100));
        // (8+3, 8) should be 0 (out of radius 2)
        assert_eq!(vm.grid[8][11], Value::Int(0));
    }

    #[test]
    fn test_siphon() {
        // First radiate 10s
        // [ push(10) push(1) push(5) push(5) radiate() push(1) push(5) push(5) siphon() ]
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(10)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(5)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(5)],
            },
            Gene {
                op: OpCode::Radiate,
                args: vec![],
            },
            // Siphon same area
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(5)],
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(5)],
            },
            Gene {
                op: OpCode::Siphon,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));
        while !vm.halted {
            vm.step();
        }

        // Center (5,5), (5,4), (5,6), (4,5), (6,5) = 5 cells. 5 * 10 = 50.

        if let Some(Value::Int(sum)) = vm.stack.pop() {
            assert_eq!(sum, 50);
        } else {
            panic!("Expected sum on stack");
        }

        // Grid should be cleared
        assert_eq!(vm.grid[5][5], Value::Int(0));
    }

    #[test]
    fn test_genome() {
        // [ genome() ]
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            },
            Gene {
                op: OpCode::Genome,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));
        while !vm.halted {
            vm.step();
        }

        // Stack should contain:
        // 1 (from push)
        // 2 (length)
        // "push"
        // "genome"

        let last = vm.stack.pop().unwrap();
        assert_eq!(last, Value::Str("genome".to_string()));
        let second = vm.stack.pop().unwrap();
        assert_eq!(second, Value::Str("push".to_string()));
        let len = vm.stack.pop().unwrap();
        assert_eq!(len, Value::Int(2));
    }

    #[test]
    fn test_consume_overflow() {
        // [ push(i64::MAX) consume() ]
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(i64::MAX)],
            },
            Gene {
                op: OpCode::Consume,
                args: vec![],
            },
        ];
        let mut vm = ChimeraVM::new(make_dna(genes));
        // Initial energy is 50. Adding MAX should saturate.
        vm.step(); // push
        vm.step(); // consume
        assert_eq!(vm.energy, i64::MAX);
    }
}
