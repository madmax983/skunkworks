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

use crate::ast::{Dna, Nucleotide};
use crate::opcode::OpCode;
use rand::Rng;
#[cfg(feature = "nova")]
use std::collections::{HashMap, HashSet, VecDeque};

pub mod cortex;
pub mod nova;

#[cfg(feature = "nova")]
use self::nova::{Organelle, Spore};

#[cfg(feature = "nova")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Topology {
    Plane,     // 0: Bounded. Edges are walls.
    Torus,     // 1: Wraps X and Y.
    CylinderH, // 2: Wraps X, Bounded Y.
    CylinderV, // 3: Bounded X, Wraps Y.
    Klein,     // 4: Wraps X, Wraps Y with twist (x' = 15-x).
    Mobius,    // 5: Wraps X with twist, Bounded Y.
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i64),
    Str(String),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(i) => write!(f, "{}", i),
            Value::Str(s) => write!(f, "\"{}\"", s),
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
    pub epigenome: HashSet<(usize, usize)>,
    #[cfg(feature = "nova")]
    pub telomeres: Vec<i64>,
    #[cfg(feature = "nova")]
    pub hormone_grid: Vec<Vec<[i64; 3]>>,
    #[cfg(feature = "nova")]
    pub waste_grid: Vec<Vec<i64>>,
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
    pub organelles: Vec<Organelle>,
    #[cfg(feature = "nova")]
    pub active_organelle_kind: Option<nova::OrganelleType>,
    #[cfg(feature = "nova")]
    pub signal_differentiation: Option<nova::OrganelleType>,
    #[cfg(feature = "nova")]
    pub sonar_target: Option<(usize, usize)>,
    #[cfg(feature = "nova")]
    pub topology: Topology,
    #[cfg(feature = "nova")]
    pub ether: HashMap<i64, VecDeque<Value>>,
    #[cfg(feature = "cortex")]
    pub synapse_map: Vec<Vec<usize>>,
    #[cfg(feature = "cortex")]
    pub activation_levels: Vec<i64>,
}

impl ChimeraVM {
    /// Creates a new VM instance with the given DNA.
    ///
    /// Initializes the grid to zeros, energy to 50, and IP to (0,0).
    pub fn new(dna: Dna) -> Self {
        // Initialize 16x16 grid with 0s
        let grid = vec![vec![Value::Int(0); 16]; 16];
        #[cfg(any(feature = "nova", feature = "cortex"))]
        let strand_count = dna.helix.strands.len();
        #[cfg(feature = "nova")]
        let hormone_grid = vec![vec![[0, 0, 0]; 16]; 16];
        #[cfg(feature = "nova")]
        let waste_grid = vec![vec![0; 16]; 16];
        #[cfg(feature = "nova")]
        let light_grid = vec![vec![0; 16]; 16];
        #[cfg(feature = "nova")]
        let membranes = vec![vec![0; 16]; 16];
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
            energy: 50,
            grid,
            chaos_mode: false,
            recursion_depth: 0,
            context_loc: (8, 8),
            #[cfg(feature = "nova")]
            epigenome: HashSet::new(),
            #[cfg(feature = "nova")]
            telomeres: vec![50; strand_count],
            #[cfg(feature = "nova")]
            hormone_grid,
            #[cfg(feature = "nova")]
            waste_grid,
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
            organelles: Vec::new(),
            #[cfg(feature = "nova")]
            active_organelle_kind: None,
            #[cfg(feature = "nova")]
            signal_differentiation: None,
            #[cfg(feature = "nova")]
            sonar_target: None,
            #[cfg(feature = "nova")]
            topology: Topology::Torus,
            #[cfg(feature = "nova")]
            ether: HashMap::new(),
            #[cfg(feature = "cortex")]
            synapse_map,
            #[cfg(feature = "cortex")]
            activation_levels,
        }
    }

    /// Helper to normalize coordinates based on topology
    #[cfg(feature = "nova")]
    pub fn normalize_coords(&self, y: i64, x: i64) -> Option<(usize, usize)> {
        match self.topology {
            Topology::Plane => {
                if (0..16).contains(&x) && (0..16).contains(&y) {
                    Some((y as usize, x as usize))
                } else {
                    None
                }
            }
            Topology::Torus => Some((y.rem_euclid(16) as usize, x.rem_euclid(16) as usize)),
            Topology::CylinderH => {
                // Wraps X, Bounded Y
                if (0..16).contains(&y) {
                    Some((y as usize, x.rem_euclid(16) as usize))
                } else {
                    None
                }
            }
            Topology::CylinderV => {
                // Bounded X, Wraps Y
                if (0..16).contains(&x) {
                    Some((y.rem_euclid(16) as usize, x as usize))
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
                if !(0..16).contains(&ny) {
                    // How many times did we wrap?
                    // Simple case: single step
                    // General case: rem_euclid logic with flip parity
                    // Let's assume simple wrapping for simulation steps usually +/- 1

                    // Logic:
                    // y' = y mod 16
                    // if (floor(y/16)) is odd, x' = 15 - x.

                    let wrap_count = ny.div_euclid(16);
                    if wrap_count % 2 != 0 {
                        nx = 15 - nx; // Twist X
                    }
                    ny = ny.rem_euclid(16);
                }

                // Now handle X wrapping (Torus-like)
                nx = nx.rem_euclid(16);

                Some((ny as usize, nx as usize))
            }
            Topology::Mobius => {
                // Mobius Strip: Wraps X with twist, Bounded Y
                // strip [0,L]x[0,W]. (x+L, y) = (x, W-y)
                // Here X wraps with twist.
                let mut nx = x;
                let mut ny = y;

                if !(0..16).contains(&nx) {
                    let wrap_count = nx.div_euclid(16);
                    if wrap_count % 2 != 0 {
                        ny = 15 - ny; // Twist Y
                    }
                    nx = nx.rem_euclid(16);
                }

                if (0..16).contains(&ny) {
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

    /// Advances the simulation by one tick.
    ///
    /// 1. Consumes 1 energy unit.
    /// 2. Handles interrupts (Nova feature).
    /// 3. Processes biological diffusion (hormones, waste).
    /// 4. Checks for starvation (Energy <= 0).
    /// 5. Executes the gene at the current Instruction Pointer (IP).
    /// 6. Advances IP.
    pub fn step(&mut self) {
        if self.halted {
            return;
        }

        self.energy -= 1;

        #[cfg(feature = "nova")]
        {
            self.sonar_target = None;
            if let Some(key) = self.input_buffer.pop_front() {
                if let Some(&strand_idx) = self.receptors.get(&key) {
                    // Interrupt!
                    // Push current IP to call stack so we can return later (if we want)
                    // Note: We push the *current* IP. If we want to return to the *next* instruction
                    // when we are interrupted between instructions, it depends.
                    // Here we are at start of step(), so IP points to the instruction *to be executed*.
                    // So when we return, we want to execute *that* instruction.
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
        {
            for level in self.activation_levels.iter_mut() {
                if *level > 0 {
                    *level -= 1;
                }
            }
        }

        #[cfg(feature = "nova")]
        {
            // Metabolic Waste Production
            let (cy, cx) = self.context_loc;
            self.waste_grid[cy][cx] += 10;

            nova::diffuse_hormones(self);
            nova::diffuse_waste(self);
            nova::diffuse_light(self);

            // Decay hormones: reduce intensity by 1 per step
            for row in self.hormone_grid.iter_mut() {
                for cell in row.iter_mut() {
                    for val in cell.iter_mut() {
                        if *val > 0 {
                            *val -= 1;
                        }
                    }
                }
            }

            // Toxicity check
            if self.waste_grid[cy][cx] > 100 {
                let mut rng = rand::thread_rng();
                if rng.gen_bool(0.05) {
                    self.output
                        .push(format!("MUTATION: TOXICITY at {},{}", cx, cy));
                    self.mutate();
                }
            }
        }

        if self.chaos_mode {
            let mut rng = rand::thread_rng();
            if rng.gen_bool(0.1) {
                // 10% chance per step
                self.mutate();
            }
        }

        if self.energy <= 0 {
            self.halted = true;
            self.output.push("DEATH: STARVATION".to_string());
            return;
        }

        let helix_len = self.dna.helix.strands.len();
        if self.ip.0 >= helix_len {
            self.halted = true;
            return;
        }

        let strand_len = self.dna.helix.strands[self.ip.0].genes.len();
        if self.ip.1 >= strand_len {
            // End of strand, move to next strand
            self.ip.0 += 1;
            self.ip.1 = 0;
            return;
        }

        #[cfg(feature = "nova")]
        {
            // Telomere check at start of strand
            if self.ip.1 == 0 && self.ip.0 < self.telomeres.len() {
                if self.telomeres[self.ip.0] > 0 {
                    self.telomeres[self.ip.0] -= 1;
                }

                if self.telomeres[self.ip.0] <= 0 {
                    self.output
                        .push(format!("SENESCENCE: Strand {} decayed", self.ip.0));
                    self.ip.0 += 1;
                    self.ip.1 = 0;
                    return;
                }
            }

            if self.epigenome.contains(&self.ip) {
                self.ip.1 += 1;
                return;
            }
        }

        #[cfg(feature = "nova")]
        {
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
            // Move to next gene
            self.ip.1 += 1;
        }

        #[cfg(feature = "nova")]
        {
            // Execute Organelles
            let active_organelles = std::mem::take(&mut self.organelles);
            let mut next_organelles = Vec::new();

            for mut organelle in active_organelles {
                if organelle.halted {
                    continue;
                }

                // Swap state
                std::mem::swap(&mut self.stack, &mut organelle.stack);
                std::mem::swap(&mut self.ip, &mut organelle.ip);
                std::mem::swap(&mut self.context_loc, &mut organelle.context_loc);
                std::mem::swap(&mut self.call_stack, &mut organelle.call_stack);
                std::mem::swap(&mut self.recursion_depth, &mut organelle.recursion_depth);

                self.active_organelle_kind = Some(organelle.kind.clone());

                // Reduce energy for organelle metabolism
                self.energy = self.energy.saturating_sub(1);

                // Specialized Organelle Logic
                match organelle.kind {
                    nova::OrganelleType::Chloroplast => {
                        let (cy, cx) = self.context_loc;
                        let light = self.light_grid[cy][cx];
                        if light > 0 {
                            self.energy = self.energy.saturating_add(light / 10);
                        }
                    }
                    nova::OrganelleType::Mitochondria => {
                        // Refund the metabolism cost
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
                        let (cy, cx) = self.context_loc;
                        let val = self.grid[cy][cx].clone();
                        match val {
                            Value::Int(n) => self.stack.push(Value::Int(n)),
                            Value::Str(s) => match s.as_str() {
                                ">" => organelle.direction = (0, 1),
                                "<" => organelle.direction = (0, -1),
                                "^" => organelle.direction = (-1, 0),
                                "v" => organelle.direction = (1, 0),
                                "+" | "-" | "*" | "/" | "%" | "=" | "!" | ":" | ";" => {
                                    nova::exec_ribosome_glyph(self, s.as_str());
                                }
                                _ => {
                                    if let Ok(op) = s.parse::<OpCode>() {
                                        // Execute OpCode (with no args for simplicity in grid mode)
                                        // Ignoring jump targets for Ribosome as it doesn't use IP
                                        let _ = self.execute_gene_inner(op, &[]);
                                    }
                                }
                            },
                        }
                        // Move
                        let (dy, dx) = organelle.direction;

                        #[cfg(feature = "nova")]
                        let next_coords =
                            self.normalize_coords(cy as i64 + dy as i64, cx as i64 + dx as i64);

                        #[cfg(not(feature = "nova"))]
                        let next_coords = Some((
                            (cy as i64 + dy as i64).rem_euclid(16) as usize,
                            (cx as i64 + dx as i64).rem_euclid(16) as usize,
                        ));

                        if let Some((mut new_y, mut new_x)) = next_coords {
                            let mut blocked = false;
                            #[cfg(feature = "nova")]
                            {
                                let mask = match (dy, dx) {
                                    (-1, 0) => 1, // N
                                    (1, 0) => 2,  // S
                                    (0, 1) => 4,  // E
                                    (0, -1) => 8, // W
                                    _ => 0,
                                };
                                if (self.membranes[cy][cx] & mask) != 0 {
                                    blocked = true;
                                }
                            }

                            if !blocked {
                                // Check for portal
                                #[cfg(feature = "nova")]
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
                        // Else: Hit wall, stay put
                    }
                    nova::OrganelleType::Worker => {}
                }

                // Ribosomes do not execute DNA
                if !matches!(organelle.kind, nova::OrganelleType::Ribosome) {
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

                if let Some(new_kind) = self.signal_differentiation.take() {
                    organelle.kind = new_kind;
                }

                // Swap back
                std::mem::swap(&mut self.stack, &mut organelle.stack);
                std::mem::swap(&mut self.ip, &mut organelle.ip);
                std::mem::swap(&mut self.context_loc, &mut organelle.context_loc);
                std::mem::swap(&mut self.call_stack, &mut organelle.call_stack);
                std::mem::swap(&mut self.recursion_depth, &mut organelle.recursion_depth);

                if !organelle.halted {
                    next_organelles.push(organelle);
                }
            }

            // Append surviving organelles back (new ones might have been added by Spawn)
            self.organelles.extend(next_organelles);
        }
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
        if self.recursion_depth > 100 {
            self.output
                .push("Error: Recursion limit exceeded".to_string());
            return None;
        }
        self.recursion_depth += 1;
        let result = self.execute_gene_inner(op, args);
        self.recursion_depth -= 1;
        result
    }

    pub(crate) fn get_circular_coords(&self, cx: i64, cy: i64, r: i64) -> Vec<(usize, usize)> {
        let mut coords = Vec::new();
        let r_sq = r * r;
        for y in 0..16 {
            for x in 0..16 {
                let dx = x as i64 - cx;
                let dy = y as i64 - cy;
                if dx * dx + dy * dy <= r_sq {
                    coords.push((x, y));
                }
            }
        }
        coords
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
            OpCode::Rift
            | OpCode::Seal
            | OpCode::Shape
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
            | OpCode::Membrane
            | OpCode::Osmosis
            | OpCode::Sonar => nova::exec_nova_op(self, op, args),

            OpCode::Unknown(name) => {
                self.output.push(format!("Unknown enzyme: {}", name));
                None
            }
        }
    }

    fn binary_op<F>(stack: &mut Vec<Value>, output: &mut Vec<String>, op: F)
    where
        F: Fn(i64, i64) -> i64,
    {
        if stack.len() < 2 {
            output.push("Error: Stack underflow".to_string());
            return;
        }
        let b = stack.pop().unwrap();
        let a = stack.pop().unwrap();

        match (a, b) {
            (Value::Int(ia), Value::Int(ib)) => stack.push(Value::Int(op(ia, ib))),
            _ => output.push("Error: Type mismatch".to_string()),
        }
    }

    fn exec_stack_op(&mut self, op: OpCode, args: &[Nucleotide]) -> Option<(usize, usize)> {
        match op {
            OpCode::Push => {
                if let Some(arg) = args.first() {
                    match arg {
                        Nucleotide::Number(n) => self.stack.push(Value::Int(*n)),
                        Nucleotide::String(s) => self.stack.push(Value::Str(s.clone())),
                        _ => self
                            .output
                            .push(format!("Error: Invalid arg for push: {:?}", arg)),
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
                        if let Value::Int(i) = val {
                            if i == 0 {
                                return Some((*n as usize, 0));
                            }
                        } else {
                            self.output.push("Error: Type mismatch for brz".to_string());
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
                        if (0..16).contains(&y) && (0..16).contains(&x) {
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
                if self.stack.len() >= 3 {
                    let x_val = self.stack.pop().unwrap();
                    let y_val = self.stack.pop().unwrap();
                    let val = self.stack.pop().unwrap();
                    if let (Value::Int(y), Value::Int(x)) = (y_val, x_val) {
                        if (0..16).contains(&y) && (0..16).contains(&x) {
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
                        if (0..16).contains(y) && (0..16).contains(x) {
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
