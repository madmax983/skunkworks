#![cfg(feature = "nova")]
//! # Nova Extension 🌌
//!
//! The `Nova` module implements advanced biological and physics-defying capabilities for the Chimera VM.
//!
//! ## Key Features
//!
//! - **Organelles**: Specialized sub-processes (Mitochondria, Ribosomes, Void) that run in parallel.
//! - **Time Travel**: `Sporulate` creates full VM snapshots; `Germinate` restores them.
//! - **Epigenetics**: `Methylate`/`Demethylate` modify gene expression without changing DNA.
//! - **Spatial Physics**: Diffusion of hormones, waste, light, and mutagen across the grid.
//! - **Quantum Entanglement**: Linked strands that share mutations.
//! - **Phases of Matter**: Shift between Corporeal, Ethereal (pass walls), Crystalline (immobile), and Flux (fast).

use super::{nova_biome::Biome, ChimeraVM, ChromaCell, Value};
use crate::ast::{Dna, Nucleotide};
use crate::opcode::OpCode;
#[cfg(feature = "nova")]
use crate::{ChimeraParser, Rule};
#[cfg(feature = "nova")]
use pest::Parser;
#[cfg(feature = "nova")]
use rand::seq::SliceRandom;
#[cfg(feature = "nova")]
use rand::Rng;
#[cfg(feature = "nova")]
use std::collections::{HashMap, HashSet, VecDeque};

const MAX_EPIGENOME_SIZE: usize = 1024;
const MAX_INCUBATE_LENGTH: usize = 1024;

/// The physical state of the organism, affecting movement and mutation.
#[cfg(feature = "nova")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Phase {
    /// Standard state. Blocks movement through walls. Normal energy costs.
    #[default]
    Corporeal,
    /// Ghost-like state. Can move through walls. Cannot write to grid.
    Ethereal,
    /// Hardened state. Immune to mutation and decay. Cannot move.
    Crystalline,
    /// High-energy state. Double execution speed. Higher energy consumption.
    Flux,
}

/// Represents a "time-travel" snapshot of the VM state.
///
/// Used by the `Sporulate` and `Germinate` opcodes to save and restore the entire simulation state.
/// This includes the grid, stack, energy, DNA, and all Nova subsystems (epigenetics, portals, etc.).
#[cfg(feature = "nova")]
#[derive(Clone)]
pub struct Spore {
    pub phase: Phase,
    pub chirality: crate::vm::Chirality,
    pub dna: Dna,
    pub stack: Vec<Value>,
    pub ip: (usize, usize),
    pub output: Vec<String>,
    pub halted: bool,
    pub energy: i64,
    pub grid: Vec<Vec<Value>>,
    pub chaos_mode: bool,
    pub recursion_depth: usize,
    pub context_loc: (usize, usize),
    pub epigenome: HashSet<(usize, usize)>,
    pub telomeres: Vec<i64>,
    pub hormone_grid: Vec<Vec<[i64; 3]>>,
    pub waste_grid: Vec<Vec<i64>>,
    pub mutagen_grid: Vec<Vec<i64>>,
    pub light_grid: Vec<Vec<i64>>,
    pub call_stack: Vec<(usize, usize)>,
    pub input_buffer: VecDeque<char>,
    pub receptors: HashMap<char, usize>,
    pub entangled_pairs: HashMap<usize, usize>,
    pub portals: HashMap<(usize, usize), (usize, usize)>,
    pub membranes: Vec<Vec<u8>>,
    pub chroma_grid: Vec<Vec<ChromaCell>>,
    pub sonar_target: Option<(usize, usize)>,
    pub symbiotes: Vec<(usize, usize)>,
    pub ether: HashMap<i64, VecDeque<Value>>,
    pub reflexes: HashMap<i64, usize>,
    pub remap_table: HashMap<OpCode, OpCode>,
    pub direction: isize,
    pub mycelium: HashMap<(usize, usize), Vec<(usize, usize)>>,
    pub immune_system: HashSet<u64>,
    pub dictionary: HashMap<String, usize>,
    pub gravity_grid: Vec<Vec<i64>>,
    pub relativity_mode: bool,
    #[cfg(feature = "cortex")]
    pub synapse_map: Vec<Vec<usize>>,
    #[cfg(feature = "cortex")]
    pub activation_levels: Vec<i64>,
}

/// Defines the specialized behavior of an Organelle.
#[cfg(feature = "nova")]
#[derive(Debug, Clone, PartialEq)]
pub enum OrganelleType {
    /// Standard execution unit. No special abilities.
    Worker,
    /// Generates energy from `light_grid` intensity at current location.
    Chloroplast,
    /// Passively generates 1 Energy per tick.
    Mitochondria,
    /// Consumes `waste_grid` to produce energy.
    Lysosome,
    /// Reads the grid cell at its location and executes it as an instruction.
    /// Acts as a "living read head".
    Ribosome,
    /// Consumes the grid cell (turns it to 0) and moves randomly (Brownian motion).
    Void,
    /// Transmutes neighbors based on elemental recipes.
    Alchemist,
}

/// An independent execution unit spawned by the main strand.
///
/// Organelles run in parallel to the main organism (sequentially in the loop, but logically parallel).
/// They have their own stack, IP, and location, but share the organism's Energy and DNA.
#[cfg(feature = "nova")]
#[derive(Debug, Clone)]
pub struct Organelle {
    /// The organelle's private stack.
    pub stack: Vec<Value>,
    /// Instruction Pointer `(strand_idx, gene_idx)`.
    pub ip: (usize, usize),
    /// Current location on the 16x16 grid.
    pub context_loc: (usize, usize),
    /// Call stack for `Call`/`Ret` operations.
    pub call_stack: Vec<(usize, usize)>,
    /// Recursion depth tracker to prevent infinite loops.
    pub recursion_depth: usize,
    /// Execution state. If true, the organelle is removed or stops processing.
    pub halted: bool,
    /// The specialization type (e.g., Chloroplast).
    pub kind: OrganelleType,
    /// Movement vector (dy, dx) used by some organelles (e.g. Ribosome, Void).
    pub direction: (i8, i8),
    pub ttl: Option<usize>,
}

#[cfg(feature = "nova")]
fn get_open_neighbors(
    vm: &ChimeraVM,
    y: usize,
    x: usize,
) -> impl Iterator<Item = (usize, usize)> + '_ {
    [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .into_iter()
        .filter_map(move |(dy, dx)| {
            let blocked = if let Some(mask) = get_direction_mask(dy, dx) {
                (vm.membranes[y][x] & mask) != 0
            } else {
                false
            };

            if !blocked {
                vm.normalize_coords(y as i64 + dy, x as i64 + dx)
            } else {
                None
            }
        })
}

/// Simulates the diffusion of chemical signals (hormones) across the grid.
///
/// Uses a simple cellular automaton model: each cell becomes the average of itself
/// and its open neighbors (neighbors not blocked by membranes).
///
/// # Examples
///
/// ```ignore
/// // Inside VM step loop
/// nova::diffuse_hormones(&mut vm);
/// ```
#[cfg(feature = "nova")]
#[allow(clippy::needless_range_loop)]
/// Simulates the diffusion of chemical signals (hormones) across the grid.
///
/// Uses a simple cellular automaton model: each cell becomes the average of itself
/// and its open neighbors (neighbors not blocked by membranes).
///
/// Optimized to avoid intermediate Vec allocations.
pub fn diffuse_hormones(vm: &mut ChimeraVM) {
    let mut buffer = [[[0i64; 3]; 16]; 16];
    for y in 0..16 {
        for x in 0..16 {
            let inertia = vm.biome_grid[y][x].diffusion_inertia();
            let mut sums = [
                (vm.hormone_grid[y][x][0] as i128) * (inertia as i128),
                (vm.hormone_grid[y][x][1] as i128) * (inertia as i128),
                (vm.hormone_grid[y][x][2] as i128) * (inertia as i128),
            ];
            let mut count = inertia;

            for (ny, nx) in get_open_neighbors(vm, y, x) {
                for c in 0..3 {
                    sums[c] += vm.hormone_grid[ny][nx][c] as i128;
                }
                count += 1;
            }

            for c in 0..3 {
                buffer[y][x][c] = (sums[c] / count as i128) as i64;
            }
        }
    }
    for y in 0..16 {
        for x in 0..16 {
            vm.hormone_grid[y][x] = buffer[y][x];
        }
    }
}

/// Simulates the diffusion of metabolic waste products.
///
/// Waste accumulates and spreads. High concentrations trigger damage/mutation.
#[cfg(feature = "nova")]
#[allow(clippy::needless_range_loop)]
pub fn diffuse_waste(vm: &mut ChimeraVM) {
    let mut buffer = [[0i64; 16]; 16];
    for y in 0..16 {
        for x in 0..16 {
            let inertia = vm.biome_grid[y][x].diffusion_inertia();
            let mut sum = (vm.waste_grid[y][x] as i128) * (inertia as i128);
            let mut count = inertia;

            for (ny, nx) in get_open_neighbors(vm, y, x) {
                sum += vm.waste_grid[ny][nx] as i128;
                count += 1;
            }

            let decay = vm.biome_grid[y][x].decay_rate() as i128;
            buffer[y][x] = ((sum / count as i128) * decay / 100) as i64;
        }
    }
    for y in 0..16 {
        for x in 0..16 {
            vm.waste_grid[y][x] = buffer[y][x];
        }
    }
}

/// Simulates the diffusion and decay of light.
///
/// Light spreads but decays rapidly (50% per tick), simulating absorption and scattering.
/// Chloroplasts harvest energy from this grid.
#[cfg(feature = "nova")]
#[allow(clippy::needless_range_loop)]
pub fn diffuse_light(vm: &mut ChimeraVM) {
    let mut buffer = [[0i64; 16]; 16];
    for y in 0..16 {
        for x in 0..16 {
            let inertia = vm.biome_grid[y][x].diffusion_inertia();
            let mut sum = (vm.light_grid[y][x] as i128) * (inertia as i128);
            let mut count = inertia;

            for (ny, nx) in get_open_neighbors(vm, y, x) {
                sum += vm.light_grid[ny][nx] as i128;
                count += 1;
            }

            // Blur and strong decay (50%)
            buffer[y][x] = ((sum / count as i128) / 2) as i64;
        }
    }
    for y in 0..16 {
        for x in 0..16 {
            vm.light_grid[y][x] = buffer[y][x];
        }
    }
}

/// Simulates the diffusion of mutagenic radiation.
///
/// Mutagen spreads and decays slowly (90% retained per tick).
/// High levels cause random DNA mutations.
#[cfg(feature = "nova")]
#[allow(clippy::needless_range_loop)]
pub fn diffuse_mutagen(vm: &mut ChimeraVM) {
    let mut buffer = [[0i64; 16]; 16];
    for y in 0..16 {
        for x in 0..16 {
            let inertia = vm.biome_grid[y][x].diffusion_inertia();
            let mut sum = (vm.mutagen_grid[y][x] as i128) * (inertia as i128);
            let mut count = inertia;

            for (ny, nx) in get_open_neighbors(vm, y, x) {
                sum += vm.mutagen_grid[ny][nx] as i128;
                count += 1;
            }

            // Blur and slow decay (based on biome)
            // Mutagen naturally decays faster than waste (90% base retention)
            let decay = vm.biome_grid[y][x].decay_rate() as i128;
            buffer[y][x] = ((sum / count as i128) * decay / 100 * 9 / 10) as i64;
        }
    }
    for y in 0..16 {
        for x in 0..16 {
            vm.mutagen_grid[y][x] = buffer[y][x];
        }
    }
}

#[cfg(feature = "nova")]
pub fn check_chorus_chords(vm: &mut ChimeraVM) -> bool {
    let buffer: Vec<&str> = vm.chorus_buffer.iter().map(|s| s.as_str()).collect();
    let len = buffer.len();
    if len < 2 {
        return false;
    }

    // Magic Chords (Spells)
    // Checks from end of buffer (most recent)

    // "Vitality": Mi Re Do -> Energy + 50
    if len >= 3 && buffer[len - 3..] == ["Mi", "Re", "Do"] {
        vm.energy = vm.energy.saturating_add(50);
        vm.chorus_buffer.clear();
        vm.output
            .push("CHORUS: Vitality Chord! Energy restored.".to_string());
        return true;
    }

    // "Genesis": Do Mi Sol -> Spawn Worker
    if len >= 3 && buffer[len - 3..] == ["Do", "Mi", "Sol"] {
        // Spawn at random location
        if vm.organelles.len() < crate::vm::MAX_ORGANELLES {
            let mut rng = rand::thread_rng();
            let rx = rng.gen_range(0..crate::vm::GRID_SIZE);
            let ry = rng.gen_range(0..crate::vm::GRID_SIZE);

            let organelle = Organelle {
                stack: Vec::new(),
                ip: (0, 0),
                context_loc: (ry, rx),
                call_stack: Vec::new(),
                recursion_depth: 0,
                halted: false,
                kind: OrganelleType::Worker,
                direction: (0, 0),
                ttl: None,
            };
            vm.organelles.push(organelle);
            vm.chorus_buffer.clear();
            vm.output
                .push("CHORUS: Genesis Chord! Life created.".to_string());
            return true;
        }
    }

    // "Apocalypse": La Sol Fa Mi Re Do -> Kill random organelle
    if len >= 6
        && buffer[len - 6..] == ["La", "Sol", "Fa", "Mi", "Re", "Do"]
        && !vm.organelles.is_empty()
    {
        let mut rng = rand::thread_rng();
        let idx = rng.gen_range(0..vm.organelles.len());
        vm.organelles.remove(idx);
        vm.chorus_buffer.clear();
        vm.output
            .push("CHORUS: Apocalypse Chord! A life was taken.".to_string());
        return true;
    }

    // "Transmute": Lead Gold -> Transmute Grid
    if len >= 2 && buffer[len - 2..] == ["Lead", "Gold"] {
        let mut count = 0;
        for row in vm.grid.iter_mut() {
            for cell in row.iter_mut() {
                if let Value::Str(s) = cell {
                    if s == "Lead" {
                        *cell = Value::Str("Gold".to_string());
                        count += 1;
                    }
                }
            }
        }
        vm.chorus_buffer.clear();
        vm.output.push(format!(
            "CHORUS: Transmute Chord! {} Lead became Gold.",
            count
        ));
        return true;
    }

    false
}

pub fn perform_alchemy(vm: &mut ChimeraVM, y: usize, x: usize) -> bool {
    // Recipes:
    // "fire" + "water" -> "steam"
    // "earth" + "fire" -> "lava"
    // "air" + "water" -> "cloud"
    // "life" + "death" -> "spirit"
    // "lead" + "energy" (center=lead) -> "gold"

    let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut ingredients = Vec::new();
    let mut coords = Vec::new();

    for (dy, dx) in neighbors {
        if let Some((ny, nx)) = vm.normalize_coords(y as i64 + dy, x as i64 + dx) {
            ingredients.push(vm.grid[ny][nx].clone());
            coords.push((ny, nx));
        }
    }

    let has_ingredient = |s: &str| -> bool {
        ingredients
            .iter()
            .any(|v| matches!(v, Value::Str(val) if val == s))
    };

    let center_val = vm.grid[y][x].clone();
    let mut transmuted = false;
    let mut result = Value::Int(0);

    if has_ingredient("fire") && has_ingredient("water") {
        result = Value::Str("steam".to_string());
        transmuted = true;
    } else if has_ingredient("earth") && has_ingredient("fire") {
        result = Value::Str("lava".to_string());
        transmuted = true;
    } else if has_ingredient("air") && has_ingredient("water") {
        result = Value::Str("cloud".to_string());
        transmuted = true;
    } else if has_ingredient("life") && has_ingredient("death") {
        result = Value::Str("spirit".to_string());
        transmuted = true;
    } else if let Value::Str(c) = center_val {
        if c == "lead" && has_ingredient("energy") {
            result = Value::Str("gold".to_string());
            transmuted = true;
        }
    }

    if transmuted {
        vm.grid[y][x] = result;
        // Consume ingredients (set to 0/void)
        for (ny, nx) in coords {
            vm.grid[ny][nx] = Value::Int(0);
        }
        vm.output
            .push(format!("ALCHEMY: Transmutation occurred at {},{}", x, y));
        return true;
    }

    false
}

#[cfg(feature = "nova")]
fn value_to_nucleotide(v: &Value, depth: usize) -> Option<Nucleotide> {
    if depth > crate::vm::MAX_RECURSION_DEPTH {
        return None;
    }
    match v {
        Value::Int(n) => Some(Nucleotide::Number(*n)),
        Value::Str(s) => Some(Nucleotide::String(s.clone())),
        Value::Junction(t, vals) => {
            let mut nuc_vals = Vec::new();
            for val in vals {
                if let Some(n) = value_to_nucleotide(val, depth + 1) {
                    nuc_vals.push(n);
                } else {
                    return None;
                }
            }
            Some(Nucleotide::Junction(*t, nuc_vals))
        }
        Value::Superposition(_) => None, // Cannot compile superposition to static AST
    }
}

/// Executes a Nova-specific OpCode.
///
/// This function handles the dispatch for all biological and advanced physics operations.
///
/// # Returns
///
/// Returns `Some((strand_idx, gene_idx))` if the operation triggered a jump or call that
/// modifies the Instruction Pointer (IP). Returns `None` if execution should proceed sequentially.
#[cfg(feature = "nova")]
#[allow(clippy::needless_range_loop)]
fn exec_prophecy(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // stack: ticks (top)
    if let Some(val) = vm.stack.pop() {
        if let Value::Int(ticks) = val {
            if ticks > 0 {
                let safe_ticks = ticks.min(1000);

                // Clone VM
                let mut sim_vm = vm.clone();
                sim_vm.output.clear(); // Silence output
                sim_vm.halted = false; // Ensure it can run (unless already dead?)

                // Advance IP to avoid infinite recursion (executing prophecy again)
                // We assume standard sequential flow (IP.1 + 1)
                sim_vm.ip.1 += 1;

                if vm.energy <= 0 {
                    // If already dead, prophecy is 1
                    vm.stack.push(Value::Int(1));
                } else {
                    // Run simulation loop
                    for _ in 0..safe_ticks {
                        sim_vm.step();
                        if sim_vm.halted {
                            break;
                        }
                    }

                    // Result: 1 if Dead (halted), 0 if Alive
                    let result = if sim_vm.halted { 1 } else { 0 };
                    vm.stack.push(Value::Int(result));

                    // Cost
                    let cost = 50 + (safe_ticks / 2);
                    vm.energy = vm.energy.saturating_sub(cost);
                    vm.output.push(format!(
                        "PROPHECY: Predicted {} (1=Death, 0=Life) in {} ticks",
                        result, safe_ticks
                    ));
                }
            } else {
                vm.output
                    .push("Error: Invalid ticks for prophecy".to_string());
            }
        } else {
            vm.output
                .push("Error: Type mismatch for prophecy".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for prophecy".to_string());
    }
    None
}

#[cfg(feature = "nova")]
fn exec_metamorphosis(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    let rows = vm.grid.len();
    let cols = if rows > 0 { vm.grid[0].len() } else { 0 };

    let mut new_strands = Vec::new();

    for row in &vm.grid {
        let mut genes = Vec::new();
        let mut x = 0;
        while x < cols {
            let val = &row[x];
            match val {
                Value::Int(n) => {
                    genes.push(crate::ast::Gene {
                        op: OpCode::Push,
                        args: vec![crate::ast::Nucleotide::Number(*n)],
                    });
                    x += 1;
                }
                Value::Junction(t, vals) => {
                    let mut nuc_vals = Vec::new();
                    for v in vals {
                        if let Some(n) = value_to_nucleotide(v, 0) {
                            nuc_vals.push(n);
                        } else {
                            vm.output.push(
                                "METAMORPHOSIS: Warning: Recursion limit exceeded".to_string(),
                            );
                        }
                    }
                    genes.push(crate::ast::Gene {
                        op: OpCode::Push,
                        args: vec![crate::ast::Nucleotide::Junction(*t, nuc_vals)],
                    });
                    x += 1;
                }
                Value::Superposition(_) => {
                    x += 1; // Skip
                }
                Value::Str(s) => {
                    let op = s.parse().unwrap_or(OpCode::Unknown(s.clone()));
                    let mut args = Vec::new();
                    match op {
                        OpCode::Push | OpCode::Jump | OpCode::Brz | OpCode::Call => {
                            if x + 1 < cols {
                                if let Some(arg) = value_to_nucleotide(&row[x + 1], 0) {
                                    args.push(arg);
                                } else {
                                    vm.output.push(
                                        "METAMORPHOSIS: Warning: Recursion limit exceeded"
                                            .to_string(),
                                    );
                                }
                                x += 1;
                            }
                        }
                        #[cfg(feature = "cortex")]
                        OpCode::Gate => {
                            if x + 1 < cols {
                                if let Some(arg) = value_to_nucleotide(&row[x + 1], 0) {
                                    args.push(arg);
                                } else {
                                    vm.output.push(
                                        "METAMORPHOSIS: Warning: Recursion limit exceeded"
                                            .to_string(),
                                    );
                                }
                                x += 1;
                            }
                        }
                        _ => {}
                    }
                    genes.push(crate::ast::Gene { op, args });
                    x += 1;
                }
            }
        }
        if !genes.is_empty() {
            new_strands.push(crate::ast::Strand { genes });
        }
    }

    if !new_strands.is_empty() {
        vm.dna.helix.strands = new_strands;

        // Reset State
        vm.energy = 50;
        vm.ip = (0, 0);
        vm.stack.clear();
        vm.telomeres = vec![50; vm.dna.helix.strands.len()];
        vm.epigenome.clear();
        vm.receptors.clear();
        #[cfg(feature = "cortex")]
        {
            vm.activation_levels = vec![0; vm.dna.helix.strands.len()];
            vm.synapse_map = vec![Vec::new(); vm.dna.helix.strands.len()];
        }
        vm.market.clear();

        vm.output
            .push("METAMORPHOSIS: The chrysalis breaks...".to_string());
        Some((0, 0))
    } else {
        vm.output
            .push("METAMORPHOSIS: Failed (Grid empty/invalid)".to_string());
        Some((0, 0))
    }
}

#[cfg(feature = "nova")]
fn exec_simulate(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // stack: ticks, strand_idx (bottom)
    if vm.stack.len() >= 2 {
        let ticks_val = vm.stack.pop().unwrap();
        let s_val = vm.stack.pop().unwrap();

        if let (Value::Int(ticks), Value::Int(s_idx)) = (ticks_val, s_val) {
            let idx = s_idx as usize;
            if idx < vm.dna.helix.strands.len() && ticks > 0 {
                if vm.recursion_depth > 100 {
                    vm.output
                        .push("Error: Recursion limit exceeded".to_string());
                    return None;
                }

                // Cap ticks to prevent DoS
                let safe_ticks = ticks.min(1000);

                // Fork VM
                // Cloning `vm` clones everything, which provides an accurate snapshot.
                let mut sim_vm = vm.clone();

                // Setup simulation context
                sim_vm.ip = (idx, 0);
                sim_vm.output.clear(); // Silence output
                sim_vm.halted = false;

                // Run simulation loop
                for _ in 0..safe_ticks {
                    sim_vm.step();
                    if sim_vm.halted {
                        break;
                    }
                }

                // Collect Results
                // 1. Top of stack (or 0 if empty)
                let top_val = sim_vm.stack.last().cloned().unwrap_or(Value::Int(0));
                // 2. Final Energy
                let energy = sim_vm.energy;
                // 3. Status (1 = Alive, 0 = Halted/Dead)
                let status = if sim_vm.halted { 0 } else { 1 };

                // Push results to original VM stack
                vm.stack.push(top_val);
                vm.stack.push(Value::Int(energy));
                vm.stack.push(Value::Int(status));

                // Deduct Energy Cost: Base cost + duration cost
                let cost = safe_ticks.saturating_add(50);
                vm.energy = vm.energy.saturating_sub(cost);

                vm.output.push(format!(
                    "SIMULATE: Ran strand {} for {} ticks. Status: {}",
                    idx, safe_ticks, status
                ));
            } else {
                vm.output
                    .push("Error: Invalid args for simulate".to_string());
            }
        } else {
            vm.output
                .push("Error: Type mismatch for simulate".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for simulate".to_string());
    }
    None
}

#[cfg(feature = "nova")]
fn exec_brainfuck(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // stack: bf_code_string, input_string (top)
    if vm.stack.len() >= 2 {
        let input_val = vm.stack.pop().unwrap();
        let code_val = vm.stack.pop().unwrap();

        if let (Value::Str(code), Value::Str(input)) = (code_val, input_val) {
            let code_chars: Vec<char> = code.chars().collect();
            let mut input_chars: VecDeque<u8> = input.bytes().collect::<VecDeque<_>>();
            let mut output_bytes: Vec<u8> = Vec::new();

            let mut tape = vec![0u8; 30000];
            let mut ptr = 0;
            let mut pc = 0;
            let mut cycles = 0;
            let max_cycles = 10000; // Safety limit

            // Precompute jump targets
            let mut jumps = HashMap::new();
            let mut loop_stack = Vec::new();
            for (i, &c) in code_chars.iter().enumerate() {
                if c == '[' {
                    loop_stack.push(i);
                } else if c == ']' {
                    if let Some(start) = loop_stack.pop() {
                        jumps.insert(start, i);
                        jumps.insert(i, start);
                    }
                }
            }

            while pc < code_chars.len() && cycles < max_cycles {
                match code_chars[pc] {
                    '>' => {
                        if ptr < tape.len() - 1 {
                            ptr += 1;
                        } else {
                            ptr = 0;
                        } // Wrap
                    }
                    '<' => {
                        if ptr > 0 {
                            ptr -= 1;
                        } else {
                            ptr = tape.len() - 1;
                        } // Wrap
                    }
                    '+' => tape[ptr] = tape[ptr].wrapping_add(1),
                    '-' => tape[ptr] = tape[ptr].wrapping_sub(1),
                    '.' => output_bytes.push(tape[ptr]),
                    ',' => {
                        tape[ptr] = input_chars.pop_front().unwrap_or(0);
                    }
                    '[' => {
                        if tape[ptr] == 0 {
                            if let Some(&target) = jumps.get(&pc) {
                                pc = target;
                            }
                        }
                    }
                    ']' => {
                        if tape[ptr] != 0 {
                            if let Some(&target) = jumps.get(&pc) {
                                pc = target;
                            }
                        }
                    }
                    _ => {} // Ignore non-BF chars
                }
                pc += 1;
                cycles += 1;
            }

            let output_str = String::from_utf8_lossy(&output_bytes).to_string();
            vm.stack.push(Value::Str(output_str));
            vm.energy = vm.energy.saturating_sub((cycles / 100) as i64);
            vm.output.push(format!("BRAINFUCK: Ran {} cycles", cycles));
        } else {
            vm.output
                .push("Error: Type mismatch for brainfuck".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for brainfuck".to_string());
    }
    None
}

#[cfg(feature = "nova")]
fn exec_splice(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // stack: method, strand_b, strand_a (bottom)
    if vm.stack.len() >= 3 {
        let method_val = vm.stack.pop().unwrap();
        let strand_b_val = vm.stack.pop().unwrap();
        let strand_a_val = vm.stack.pop().unwrap();

        if let (Value::Int(s_a), Value::Int(s_b), Value::Int(method)) =
            (strand_a_val, strand_b_val, method_val)
        {
            let idx_a = s_a as usize;
            let idx_b = s_b as usize;
            let helix_len = vm.dna.helix.strands.len();

            if s_a >= 0 && s_b >= 0 && idx_a < helix_len && idx_b < helix_len {
                let genes_a = &vm.dna.helix.strands[idx_a].genes;
                let genes_b = &vm.dna.helix.strands[idx_b].genes;
                let len_a = genes_a.len();
                let len_b = genes_b.len();
                let max_len = len_a.max(len_b);

                let mut new_genes = Vec::new();
                let mut rng = rand::thread_rng();

                match method {
                    0 => {
                        // Interleave
                        for i in 0..max_len {
                            if i < len_a {
                                new_genes.push(genes_a[i].clone());
                            }
                            if i < len_b {
                                new_genes.push(genes_b[i].clone());
                            }
                        }
                        vm.output
                            .push(format!("SPLICE: Interleaved strands {} and {}", s_a, s_b));
                    }
                    1 => {
                        // Uniform Crossover
                        for i in 0..max_len {
                            if i < len_a && i < len_b {
                                if rng.gen_bool(0.5) {
                                    new_genes.push(genes_a[i].clone());
                                } else {
                                    new_genes.push(genes_b[i].clone());
                                }
                            } else if i < len_a {
                                new_genes.push(genes_a[i].clone());
                            } else if i < len_b {
                                new_genes.push(genes_b[i].clone());
                            }
                        }
                        vm.output
                            .push(format!("SPLICE: Crossover strands {} and {}", s_a, s_b));
                    }
                    2 => {
                        // Midpoint Split (Head A + Tail B)
                        let mid_a = len_a / 2;
                        let mid_b = len_b / 2;
                        for gene in genes_a.iter().take(mid_a) {
                            new_genes.push(gene.clone());
                        }
                        for gene in genes_b.iter().skip(mid_b) {
                            new_genes.push(gene.clone());
                        }
                        vm.output
                            .push(format!("SPLICE: Hybridized strands {} and {}", s_a, s_b));
                    }
                    _ => {
                        vm.output.push("Error: Invalid splice method".to_string());
                    }
                }

                if !new_genes.is_empty() {
                    vm.dna
                        .helix
                        .strands
                        .push(crate::ast::Strand { genes: new_genes });
                    vm.telomeres.push(50);
                    #[cfg(feature = "cortex")]
                    {
                        vm.activation_levels.push(0);
                        vm.synapse_map.push(Vec::new());
                    }
                    let new_idx = vm.dna.helix.strands.len() - 1;
                    vm.stack.push(Value::Int(new_idx as i64));
                    vm.energy = vm.energy.saturating_sub(30);
                } else if method <= 2 {
                    // If result empty but method valid (e.g. empty parents)
                    vm.stack.push(Value::Int(-1));
                }
            } else {
                vm.output
                    .push("Error: Strand index out of bounds for splice".to_string());
            }
        } else {
            vm.output
                .push("Error: Type mismatch for splice".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for splice".to_string());
    }
    None
}

#[cfg(feature = "nova")]
fn exec_recombine(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // stack: split_point, strand_b, strand_a (bottom)
    if vm.stack.len() >= 3 {
        let split_val = vm.stack.pop().unwrap();
        let strand_b_val = vm.stack.pop().unwrap();
        let strand_a_val = vm.stack.pop().unwrap();

        match (strand_a_val, strand_b_val, split_val) {
            (Value::Int(sa), Value::Int(sb), Value::Int(split)) => {
                let helix_len = vm.dna.helix.strands.len();
                let sa_idx = sa as usize;
                let sb_idx = sb as usize;
                let split_idx = split as usize;

                if sa >= 0 && sb >= 0 && split >= 0 && sa_idx < helix_len && sb_idx < helix_len {
                    // We need to check split bounds for both strands
                    let len_a = vm.dna.helix.strands[sa_idx].genes.len();
                    let len_b = vm.dna.helix.strands[sb_idx].genes.len();

                    if split_idx <= len_a && split_idx <= len_b {
                        // Perform recombination
                        // We need to borrow strands mutably.
                        // Since they are in the same Vec, we need split_at_mut or similar trickery,
                        // or just use indices if we can modify the Vec safely.
                        // We can't get two mutable references to the same Vec at different indices directly.
                        // So we'll use `split_at_mut` if they are different indices, or just do nothing if same.

                        if sa_idx == sb_idx {
                            // Recombining same strand with itself at same point is a no-op.
                            vm.output
                                .push("Warning: Recombining strand with itself".to_string());
                        } else {
                            // Ensure ordered access to avoid panic
                            let (lower, upper) = if sa_idx < sb_idx {
                                (sa_idx, sb_idx)
                            } else {
                                (sb_idx, sa_idx)
                            };

                            let (first_slice, second_slice) =
                                vm.dna.helix.strands.split_at_mut(upper);
                            let strand_low = &mut first_slice[lower];
                            let strand_high = &mut second_slice[0]; // relative index 0 is absolute 'upper'

                            // Identify which is A and B
                            let (strand_a, strand_b) = if sa_idx < sb_idx {
                                (strand_low, strand_high)
                            } else {
                                (strand_high, strand_low)
                            };

                            let mut tail_a = strand_a.genes.split_off(split_idx);
                            let mut tail_b = strand_b.genes.split_off(split_idx);

                            strand_a.genes.append(&mut tail_b);
                            strand_b.genes.append(&mut tail_a);

                            vm.output.push(format!(
                                "RECOMBINATION: Swapped tails of strand {} and {} at {}",
                                sa, sb, split
                            ));
                        }
                    } else {
                        vm.output
                            .push("Error: Split point out of bounds".to_string());
                    }
                } else {
                    vm.output
                        .push("Error: Strand index out of bounds".to_string());
                }
            }
            _ => vm
                .output
                .push("Error: Type mismatch for recombine".to_string()),
        }
    } else {
        vm.output
            .push("Error: Stack underflow for recombine".to_string());
    }
    None
}

#[cfg(feature = "nova")]
fn exec_crispr_scan(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // stack: guide_idx, target_idx (bottom)
    if vm.stack.len() >= 2 {
        let guide_val = vm.stack.pop().unwrap();
        let target_val = vm.stack.pop().unwrap();

        if let (Value::Int(g_idx), Value::Int(t_idx)) = (guide_val, target_val) {
            let g_idx = g_idx as usize;
            let t_idx = t_idx as usize;
            let helix_len = vm.dna.helix.strands.len();

            if g_idx < helix_len && t_idx < helix_len {
                let guide_strand = &vm.dna.helix.strands[g_idx];
                let target_strand = &vm.dna.helix.strands[t_idx];

                // We need to match sequence of gene names
                // Use op.to_string() for comparison
                let guide_names: Vec<String> = guide_strand
                    .genes
                    .iter()
                    .map(|g| g.op.to_string())
                    .collect();
                let target_names: Vec<String> = target_strand
                    .genes
                    .iter()
                    .map(|g| g.op.to_string())
                    .collect();

                let mut found_idx: i64 = -1;

                if !guide_names.is_empty() && guide_names.len() <= target_names.len() {
                    for i in 0..=(target_names.len() - guide_names.len()) {
                        if target_names[i..i + guide_names.len()] == guide_names[..] {
                            found_idx = i as i64;
                            break;
                        }
                    }
                } else if guide_names.is_empty() {
                    // Explicitly handle empty guide to ensure -1 (though init value handles it)
                    found_idx = -1;
                }

                vm.stack.push(Value::Int(found_idx));
                vm.energy = vm.energy.saturating_sub(5);
                vm.output.push(format!(
                    "CRISPR_SCAN: Scanned strand {} for pattern from {} -> {}",
                    t_idx, g_idx, found_idx
                ));
            } else {
                vm.output
                    .push("Error: Strand index out of bounds for crispr_scan".to_string());
            }
        } else {
            vm.output
                .push("Error: Type mismatch for crispr_scan".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for crispr_scan".to_string());
    }
    None
}

#[cfg(feature = "nova")]
fn exec_cas9_cut(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // stack: cut_index, strand_idx (bottom)
    if vm.stack.len() >= 2 {
        let cut_val = vm.stack.pop().unwrap();
        let strand_val = vm.stack.pop().unwrap();

        if let (Value::Int(cut), Value::Int(s_idx)) = (cut_val, strand_val) {
            let s_idx = s_idx as usize;
            let cut_idx = cut as usize;
            let helix_len = vm.dna.helix.strands.len();

            if s_idx < helix_len {
                let strand_len = vm.dna.helix.strands[s_idx].genes.len();
                if cut >= 0 && cut_idx <= strand_len {
                    // Perform split
                    // We need to mutate the strand.
                    let strand = &mut vm.dna.helix.strands[s_idx];
                    let tail_genes = strand.genes.split_off(cut_idx);

                    // Create new strand
                    vm.dna
                        .helix
                        .strands
                        .push(crate::ast::Strand { genes: tail_genes });
                    vm.telomeres.push(50);

                    #[cfg(feature = "cortex")]
                    {
                        vm.activation_levels.push(0);
                        vm.synapse_map.push(Vec::new());
                    }

                    let new_strand_idx = vm.dna.helix.strands.len() - 1;

                    vm.stack.push(Value::Int(new_strand_idx as i64));
                    vm.energy = vm.energy.saturating_sub(10);

                    vm.output.push(format!(
                        "CAS9_CUT: Cut strand {} at {}, created strand {}",
                        s_idx, cut_idx, new_strand_idx
                    ));

                    // Handling Self-Modification Safety:
                    // If we cut the strand we are currently executing,
                    // and the cut point is BEFORE or AT our current IP, execution context moves.
                    if s_idx == vm.ip.0 && vm.ip.1 >= cut_idx {
                        let new_gene_idx = vm.ip.1 - cut_idx;
                        // We jump to the next instruction in the NEW strand
                        return Some((new_strand_idx, new_gene_idx + 1));
                    }
                } else {
                    vm.output.push("Error: Cut index out of bounds".to_string());
                }
            } else {
                vm.output
                    .push("Error: Strand index out of bounds for cas9_cut".to_string());
            }
        } else {
            vm.output
                .push("Error: Type mismatch for cas9_cut".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for cas9_cut".to_string());
    }
    None
}

#[cfg(feature = "nova")]
#[allow(clippy::needless_range_loop)]
pub fn exec_nova_op(vm: &mut ChimeraVM, op: OpCode, args: &[Nucleotide]) -> Option<(usize, usize)> {
    match op {
        #[cfg(feature = "nova")]
        OpCode::Prophecy => exec_prophecy(vm),
        #[cfg(feature = "nova")]
        OpCode::Offer => {
            // stack: price, item
            if vm.stack.len() >= 2 {
                let item_val = vm.stack.pop().unwrap();
                let price_val = vm.stack.pop().unwrap();
                if let Value::Int(price) = price_val {
                    if price > 0 {
                        // Store item as string representation for now
                        let item_str = match &item_val {
                            Value::Str(s) => s.clone(),
                            _ => format!("{}", item_val),
                        };
                        let order_id = vm.market.place_ask(vm.ip.0, item_str, price);
                        vm.stack.push(Value::Int(order_id as i64));
                        vm.output
                            .push(format!("OFFER: Sell '{}' for {}", item_val, price));
                    } else {
                        vm.output.push("OFFER: Price must be positive".to_string());
                    }
                } else {
                    vm.output.push("Error: Type mismatch for offer".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for offer".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Buy => {
            // stack: max_price, query
            if vm.stack.len() >= 2 {
                let query_val = vm.stack.pop().unwrap();
                let max_price_val = vm.stack.pop().unwrap();
                if let (Value::Int(max_price), Value::Str(query)) = (max_price_val, query_val) {
                    if let Some((item_str, cost)) =
                        vm.market.match_buy(vm.ip.0, query.clone(), max_price)
                    {
                        vm.stack.push(Value::Str(item_str));
                        vm.stack.push(Value::Int(cost));
                        vm.output
                            .push(format!("BUY: Bought '{}' for {}", query, cost));
                    } else {
                        vm.stack.push(Value::Int(0)); // Failed
                        vm.output
                            .push(format!("BUY: No match for '{}' <= {}", query, max_price));
                    }
                } else {
                    vm.output.push("Error: Type mismatch for buy".to_string());
                }
            } else {
                vm.output.push("Error: Stack underflow for buy".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Invest => {
            // stack: amount
            if let Some(val) = vm.stack.pop() {
                if let Value::Int(amount) = val {
                    if amount > 0 && vm.energy >= amount {
                        vm.energy -= amount;
                        vm.market.credit(vm.ip.0, amount);
                        vm.output
                            .push(format!("INVEST: Converted {} Energy to Credits", amount));
                    } else {
                        vm.output.push("INVEST: Insufficient energy".to_string());
                    }
                } else {
                    vm.output.push("Error: Type mismatch for invest".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for invest".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Divest => {
            // stack: amount
            if let Some(val) = vm.stack.pop() {
                if let Value::Int(amount) = val {
                    if amount > 0 {
                        if vm.market.debit(vm.ip.0, amount) {
                            vm.energy = vm.energy.saturating_add(amount);
                            vm.output
                                .push(format!("DIVEST: Converted {} Credits to Energy", amount));
                        } else {
                            vm.output.push("DIVEST: Insufficient credits".to_string());
                        }
                    }
                } else {
                    vm.output.push("Error: Type mismatch for divest".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for divest".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Balance => {
            let bal = vm.market.get_balance(vm.ip.0);
            vm.stack.push(Value::Int(bal));
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Ticker => {
            if let Some((_, price)) = vm.market.history.back() {
                vm.stack.push(Value::Int(*price));
            } else {
                vm.stack.push(Value::Int(0));
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::TimeWarp => {
            // stack: radius, factor (top)
            if vm.stack.len() >= 2 {
                let factor_val = vm.stack.pop().unwrap();
                let radius_val = vm.stack.pop().unwrap();

                if let (Value::Int(r), Value::Int(f)) = (radius_val, factor_val) {
                    if r > 0 {
                        let factor = f.clamp(0, 10) as u8;
                        let (cy, cx) = vm.context_loc;
                        let coords = vm.get_circular_coords(cx as i64, cy as i64, r);
                        let count = coords.len();

                        for (tx, ty) in coords {
                            vm.time_grid[ty][tx] = factor;
                        }

                        // Cost depends on area and factor magnitude
                        let cost_multiplier = if factor == 0 { 2 } else { factor as i64 };
                        vm.energy = vm
                            .energy
                            .saturating_sub((count as i64 * cost_multiplier) / 2);
                        vm.output.push(format!(
                            "TIME_WARP: Set time factor {} at {},{} r={}",
                            factor, cx, cy, r
                        ));
                    }
                } else {
                    vm.output
                        .push("Error: Type mismatch for time_warp".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for time_warp".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::RetinaDraw => {
            // Stack: [ ..., packed_color, char_code, y, x ]
            if vm.stack.len() >= 4 {
                let x_val = vm.stack.pop().unwrap();
                let y_val = vm.stack.pop().unwrap();
                let char_val = vm.stack.pop().unwrap();
                let color_val = vm.stack.pop().unwrap();

                if let (Value::Int(x), Value::Int(y), Value::Int(c), Value::Int(rgb)) =
                    (x_val, y_val, char_val, color_val)
                {
                    let r = ((rgb >> 16) & 0xFF) as u8;
                    let g = ((rgb >> 8) & 0xFF) as u8;
                    let b = (rgb & 0xFF) as u8;
                    let ch = (c as u8) as char;

                    vm.retina.draw(y as usize, x as usize, ch, r, g, b);
                    vm.energy = vm.energy.saturating_sub(1);
                } else {
                    vm.output
                        .push("Error: Type mismatch for retina_draw".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for retina_draw".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::RetinaClear => {
            // Stack: [ ..., packed_color ]
            if let Some(val) = vm.stack.pop() {
                if let Value::Int(rgb) = val {
                    let r = ((rgb >> 16) & 0xFF) as u8;
                    let g = ((rgb >> 8) & 0xFF) as u8;
                    let b = (rgb & 0xFF) as u8;
                    vm.retina.clear(r, g, b);
                    vm.energy = vm.energy.saturating_sub(10);
                } else {
                    vm.output
                        .push("Error: Type mismatch for retina_clear".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for retina_clear".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::RetinaSize => {
            vm.stack.push(Value::Int(vm.retina.width as i64));
            vm.stack.push(Value::Int(vm.retina.height as i64));
            None
        }
        #[cfg(feature = "nova")]
        OpCode::QuantumJump => {
            let s_idx = vm.ip.0;
            if let Some(&partner_idx) = vm.entangled_pairs.get(&s_idx) {
                if partner_idx < vm.dna.helix.strands.len() {
                    let gene_idx = vm.ip.1;
                    // Check if partner has enough genes
                    let p_len = vm.dna.helix.strands[partner_idx].genes.len();
                    let target_gene = if gene_idx < p_len {
                        gene_idx
                    } else {
                        p_len.saturating_sub(1)
                    };

                    vm.energy = vm.energy.saturating_sub(10);
                    vm.output
                        .push(format!("QUANTUM_JUMP: {} -> {}", s_idx, partner_idx));
                    return Some((partner_idx, target_gene));
                }
            } else {
                vm.output
                    .push("QUANTUM_JUMP: No entangled partner".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Chronos => {
            let (cy, cx) = vm.context_loc;
            let factor = vm.time_grid[cy][cx];
            vm.stack.push(Value::Int(factor as i64));
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Relativity => {
            vm.relativity_mode = !vm.relativity_mode;
            let status = if vm.relativity_mode { "ON" } else { "OFF" };
            vm.output
                .push(format!("RELATIVITY: Physics engine {}", status));
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Graviton => {
            let (cy, cx) = vm.context_loc;
            vm.gravity_grid[cy][cx] = vm.gravity_grid[cy][cx].saturating_add(50);
            vm.energy = vm.energy.saturating_sub(10);
            vm.output.push(format!("GRAVITON: Emitted at {},{}", cx, cy));
            None
        }
        #[cfg(feature = "nova")]
        OpCode::EventHorizon => {
            let (cy, cx) = vm.context_loc;
            let g = vm.gravity_grid[cy][cx];
            vm.stack.push(Value::Int(g));
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Terraform => {
            // stack: biome_id, radius (top)
            if vm.stack.len() >= 2 {
                let radius_val = vm.stack.pop().unwrap();
                let id_val = vm.stack.pop().unwrap();

                if let (Value::Int(r), Value::Int(id)) = (radius_val, id_val) {
                    if r > 0 {
                        let biome = match id {
                            0 => Biome::Plains,
                            1 => Biome::Swamp,
                            2 => Biome::Desert,
                            3 => Biome::Tundra,
                            4 => Biome::Volcanic,
                            _ => Biome::Plains,
                        };

                        let (cy, cx) = vm.context_loc;
                        let coords = vm.get_circular_coords(cx as i64, cy as i64, r);
                        let count = coords.len();

                        for (tx, ty) in coords {
                            vm.biome_grid[ty][tx] = biome;
                        }

                        // Terraforming is expensive
                        vm.energy = vm.energy.saturating_sub(count as i64 * 5);
                        vm.output.push(format!(
                            "TERRAFORM: Changed {} cells to {:?} at {},{}",
                            count, biome, cx, cy
                        ));
                    }
                } else {
                    vm.output
                        .push("Error: Type mismatch for terraform".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for terraform".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::SenseBiome => {
            let (cy, cx) = vm.context_loc;
            let biome = vm.biome_grid[cy][cx];
            let id = match biome {
                Biome::Plains => 0,
                Biome::Swamp => 1,
                Biome::Desert => 2,
                Biome::Tundra => 3,
                Biome::Volcanic => 4,
            };
            vm.stack.push(Value::Int(id));
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Alchemy => {
            let (cy, cx) = vm.context_loc;
            perform_alchemy(vm, cy, cx);
            vm.energy = vm.energy.saturating_sub(5);
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Meme => {
            if let Some(gene) = &vm.last_gene {
                let gene_clone = gene.clone();
                let helix_len = vm.dna.helix.strands.len();
                if helix_len > 0 {
                    let mut rng = rand::thread_rng();
                    let target_strand = rng.gen_range(0..helix_len);
                    let strand_len = vm.dna.helix.strands[target_strand].genes.len();
                    let insert_pos = rng.gen_range(0..=strand_len); // Can insert at end

                    vm.dna.helix.strands[target_strand]
                        .genes
                        .insert(insert_pos, gene_clone);

                    vm.energy = vm.energy.saturating_sub(20);
                    vm.output.push(format!(
                        "MEME: Infected strand {} at {} with {}",
                        target_strand, insert_pos, gene.op
                    ));
                }
            } else {
                vm.output
                    .push("MEME: No previous gene to spread".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Drift => {
            if let Some(val) = vm.stack.pop() {
                if let Value::Int(prob) = val {
                    let p = prob.clamp(0, 100);
                    let mut rng = rand::thread_rng();
                    let mut changes = 0;

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
                        OpCode::GRead,
                        OpCode::GWrite,
                        OpCode::Genome,
                        OpCode::Meme,
                        OpCode::Poly, // Self-reference!
                    ];

                    for strand in &mut vm.dna.helix.strands {
                        for gene in &mut strand.genes {
                            if rng.gen_range(0..100) < p {
                                let new_op = enzymes[rng.gen_range(0..enzymes.len())].clone();
                                gene.op = new_op;
                                changes += 1;
                            }
                        }
                    }

                    vm.energy = vm.energy.saturating_sub(50 + changes);
                    vm.output.push(format!("DRIFT: Mutated {} genes", changes));
                } else {
                    vm.output.push("Error: Type mismatch for drift".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for drift".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Poly => {
            // stack: [ val ] (peek)
            if let Some(val) = vm.stack.last() {
                // args: [ op_int, op_str ]
                if args.len() >= 2 {
                    let op_to_run = match val {
                        Value::Int(_) => Some(&args[0]),
                        Value::Str(_) => Some(&args[1]),
                        _ => None,
                    };

                    if let Some(nucleotide) = op_to_run {
                        let op_str = match nucleotide {
                            Nucleotide::String(s) => Some(s.clone()),
                            Nucleotide::Identifier(s) => Some(s.clone()),
                            _ => None,
                        };

                        if let Some(s) = op_str {
                            if let Ok(op) = s.parse::<OpCode>() {
                                return vm.execute_gene_inner(op, &[]);
                            } else {
                                vm.output.push(format!("POLY: Invalid OpCode {}", s));
                            }
                        } else {
                            vm.output
                                .push("POLY: Arg must be String or Identifier".to_string());
                        }
                    }
                } else {
                    vm.output.push("POLY: Requires 2 arguments".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for poly".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Metamorphosis => exec_metamorphosis(vm),
        #[cfg(feature = "nova")]
        OpCode::Piet => {
            if let Some(val) = vm.stack.pop() {
                if let Value::Int(steps) = val {
                    if steps > 0 {
                        super::piet::exec_piet(vm, steps);
                    } else {
                        vm.output.push("PIET: Steps must be positive".to_string());
                    }
                } else {
                    vm.output.push("Error: Type mismatch for piet".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for piet".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Chronostasis => {
            if let Some(val) = vm.stack.pop() {
                if let Value::Int(ticks) = val {
                    if ticks > 0 {
                        vm.chronostasis_timer = ticks as usize;
                        let cost = 50 + ticks;
                        vm.energy = vm.energy.saturating_sub(cost);
                        vm.output
                            .push(format!("CHRONOSTASIS: Time frozen for {} ticks", ticks));
                    } else {
                        vm.output
                            .push("Error: Invalid ticks for chronostasis".to_string());
                    }
                } else {
                    vm.output
                        .push("Error: Type mismatch for chronostasis".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for chronostasis".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Simulate => exec_simulate(vm),
        #[cfg(feature = "nova")]
        OpCode::SensePigment => {
            let (cy, cx) = vm.context_loc;
            if let Some((r, g, b)) = vm.chroma_grid[cy][cx].fg {
                vm.stack.push(Value::Int(r as i64));
                vm.stack.push(Value::Int(g as i64));
                vm.stack.push(Value::Int(b as i64));
            } else {
                vm.stack.push(Value::Int(0));
                vm.stack.push(Value::Int(0));
                vm.stack.push(Value::Int(0));
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::SenseGlyph => {
            let (cy, cx) = vm.context_loc;
            if let Some(c) = vm.chroma_grid[cy][cx].char {
                vm.stack.push(Value::Int(c as u8 as i64));
            } else {
                vm.stack.push(Value::Int(-1));
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Sing => {
            // stack: note (top)
            if let Some(val) = vm.stack.pop() {
                if let Value::Str(note) = val {
                    vm.chorus_buffer.push_back(note.clone());
                    if vm.chorus_buffer.len() > crate::vm::MAX_CHORUS_SIZE {
                        vm.chorus_buffer.pop_front();
                    }
                    vm.energy = vm.energy.saturating_sub(2);
                    vm.output.push(format!("SING: {}", note));

                    check_chorus_chords(vm);
                } else {
                    vm.output.push("Error: Type mismatch for sing".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for sing".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Listen => {
            let notes: Vec<Value> = vm
                .chorus_buffer
                .iter()
                .map(|s| Value::Str(s.clone()))
                .collect();
            vm.stack
                .push(Value::Junction(crate::ast::JunctionType::All, notes));
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Brainfuck => exec_brainfuck(vm),
        #[cfg(feature = "nova")]
        OpCode::Spawn => {
            // stack: type, strand_idx (bottom)
            if vm.stack.len() >= 2 {
                let type_val = vm.stack.pop().unwrap();
                let idx_val = vm.stack.pop().unwrap();

                if let (Value::Int(t), Value::Int(idx)) = (type_val, idx_val) {
                    let s_idx = idx as usize;
                    if s_idx < vm.dna.helix.strands.len() {
                        if vm.organelles.len() >= crate::vm::MAX_ORGANELLES {
                            vm.output
                                .push("Error: Organelle limit exceeded".to_string());
                            return None;
                        }

                        let (kind, direction) = match t {
                            1 => (OrganelleType::Chloroplast, (0, 0)),
                            2 => (OrganelleType::Mitochondria, (0, 0)),
                            3 => (OrganelleType::Lysosome, (0, 0)),
                            4 => (OrganelleType::Ribosome, (0, 1)), // Default East
                            6 => (OrganelleType::Alchemist, (0, 0)),
                            _ => (OrganelleType::Worker, (0, 0)),
                        };

                        let organelle = Organelle {
                            stack: Vec::new(),
                            ip: (s_idx, 0),
                            context_loc: vm.context_loc,
                            call_stack: Vec::new(),
                            recursion_depth: 0,
                            halted: false,
                            kind: kind.clone(),
                            direction,
                            ttl: None,
                        };
                        vm.organelles.push(organelle);
                        vm.energy = vm.energy.saturating_sub(20);
                        vm.output.push(format!(
                            "SPAWN: Created {:?} Organelle executing strand {}",
                            kind, s_idx
                        ));
                    } else {
                        vm.output
                            .push("Error: Strand index out of bounds for spawn".to_string());
                    }
                } else {
                    vm.output.push("Error: Type mismatch for spawn".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for spawn".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Sporulate => {
            if vm.spores.len() >= crate::vm::MAX_SPORES {
                vm.output.push("Error: Spore limit exceeded".to_string());
                return None;
            }

            // Create snapshot
            let spore = Spore {
                phase: vm.phase,
                chirality: vm.chirality,
                dna: vm.dna.clone(),
                stack: vm.stack.clone(),
                ip: vm.ip,
                output: vm.output.clone(),
                halted: vm.halted,
                energy: vm.energy,
                grid: vm.grid.clone(),
                chaos_mode: vm.chaos_mode,
                recursion_depth: vm.recursion_depth,
                context_loc: vm.context_loc,
                epigenome: vm.epigenome.clone(),
                telomeres: vm.telomeres.clone(),
                hormone_grid: vm.hormone_grid.clone(),
                waste_grid: vm.waste_grid.clone(),
                mutagen_grid: vm.mutagen_grid.clone(),
                light_grid: vm.light_grid.clone(),
                call_stack: vm.call_stack.clone(),
                input_buffer: vm.input_buffer.clone(),
                receptors: vm.receptors.clone(),
                entangled_pairs: vm.entangled_pairs.clone(),
                portals: vm.portals.clone(),
                membranes: vm.membranes.clone(),
                chroma_grid: vm.chroma_grid.clone(),
                sonar_target: vm.sonar_target,
                symbiotes: vm.symbiotes.clone(),
                ether: vm.ether.clone(),
                reflexes: vm.reflexes.clone(),
                remap_table: vm.remap_table.clone(),
                direction: vm.direction,
                mycelium: vm.mycelium.clone(),
                immune_system: vm.immune_system.clone(),
                dictionary: vm.dictionary.clone(),
                gravity_grid: vm.gravity_grid.clone(),
                relativity_mode: vm.relativity_mode,
                #[cfg(feature = "cortex")]
                synapse_map: vm.synapse_map.clone(),
                #[cfg(feature = "cortex")]
                activation_levels: vm.activation_levels.clone(),
            };

            let id = vm.spores.len();
            vm.spores.push(spore);
            vm.stack.push(Value::Int(id as i64));
            vm.energy = vm.energy.saturating_sub(50); // High cost for time travel
            vm.output.push(format!("SPORULATE: Created Spore {}", id));
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Germinate => {
            // stack: spore_id
            if let Some(val) = vm.stack.pop() {
                if let Value::Int(id) = val {
                    let idx = id as usize;
                    if idx < vm.spores.len() {
                        let spore = &vm.spores[idx];
                        // Restore state
                        vm.phase = spore.phase;
                        vm.chirality = spore.chirality;
                        vm.dna = spore.dna.clone();
                        vm.stack = spore.stack.clone();
                        vm.ip = spore.ip;
                        vm.output = spore.output.clone();
                        vm.halted = spore.halted;
                        vm.energy = spore.energy;
                        vm.grid = spore.grid.clone();
                        vm.chaos_mode = spore.chaos_mode;
                        vm.recursion_depth = spore.recursion_depth;
                        vm.context_loc = spore.context_loc;
                        vm.epigenome = spore.epigenome.clone();
                        vm.telomeres = spore.telomeres.clone();
                        vm.hormone_grid = spore.hormone_grid.clone();
                        vm.waste_grid = spore.waste_grid.clone();
                        vm.mutagen_grid = spore.mutagen_grid.clone();
                        vm.light_grid = spore.light_grid.clone();
                        vm.call_stack = spore.call_stack.clone();
                        vm.input_buffer = spore.input_buffer.clone();
                        vm.receptors = spore.receptors.clone();
                        vm.entangled_pairs = spore.entangled_pairs.clone();
                        vm.portals = spore.portals.clone();
                        vm.membranes = spore.membranes.clone();
                        vm.chroma_grid = spore.chroma_grid.clone();
                        vm.sonar_target = spore.sonar_target;
                        vm.symbiotes = spore.symbiotes.clone();
                        vm.ether = spore.ether.clone();
                        vm.reflexes = spore.reflexes.clone();
                        vm.remap_table = spore.remap_table.clone();
                        vm.direction = spore.direction;
                        vm.mycelium = spore.mycelium.clone();
                        vm.immune_system = spore.immune_system.clone();
                        vm.dictionary = spore.dictionary.clone();
                        vm.gravity_grid = spore.gravity_grid.clone();
                        vm.relativity_mode = spore.relativity_mode;
                        #[cfg(feature = "cortex")]
                        {
                            vm.synapse_map = spore.synapse_map.clone();
                            vm.activation_levels = spore.activation_levels.clone();
                        }

                        vm.output.push(format!("GERMINATE: Restored Spore {}", idx));
                    } else {
                        vm.output
                            .push("Error: Spore index out of bounds".to_string());
                    }
                } else {
                    vm.output
                        .push("Error: Type mismatch for germinate".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for germinate".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Incubate => {
            // stack: len, y, x (top)
            if vm.stack.len() >= 3 {
                let x_val = vm.stack.pop().unwrap();
                let y_val = vm.stack.pop().unwrap();
                let len_val = vm.stack.pop().unwrap();

                if let (Value::Int(x), Value::Int(y), Value::Int(len)) = (x_val, y_val, len_val) {
                    if len > 0 {
                        let mut genes = Vec::new();
                        let mut valid = true;
                        let max_len = (len as usize).min(MAX_INCUBATE_LENGTH);

                        // Read sequence from grid
                        let mut sequence = Vec::new();
                        for k in 0..max_len {
                            if let Some((ny, nx)) = vm.normalize_coords(y, x + k as i64) {
                                sequence.push(vm.grid[ny][nx].clone());
                            } else {
                                valid = false;
                                vm.output
                                    .push("Error: Incubate range out of bounds".to_string());
                                break;
                            }
                        }

                        if valid {
                            let mut k = 0;
                            while k < sequence.len() {
                                match &sequence[k] {
                                    Value::Int(n) => {
                                        // Treated as push(n)
                                        genes.push(crate::ast::Gene {
                                            op: OpCode::Push,
                                            args: vec![crate::ast::Nucleotide::Number(*n)],
                                        });
                                        k += 1;
                                    }
                                    Value::Junction(t, vals) => {
                                        // Treated as push(junction)
                                        let mut nuc_vals = Vec::new();
                                        for v in vals {
                                            if let Some(n) = value_to_nucleotide(v, 0) {
                                                nuc_vals.push(n);
                                            } else {
                                                vm.output.push(
                                                    "INCUBATE: Warning: Recursion limit exceeded"
                                                        .to_string(),
                                                );
                                            }
                                        }
                                        genes.push(crate::ast::Gene {
                                            op: OpCode::Push,
                                            args: vec![crate::ast::Nucleotide::Junction(
                                                *t, nuc_vals,
                                            )],
                                        });
                                        k += 1;
                                    }
                                    Value::Superposition(_) => {
                                        k += 1; // Skip
                                    }
                                    Value::Str(s) => {
                                        let op = s.parse().unwrap_or(OpCode::Unknown(s.clone()));
                                        let mut args = Vec::new();
                                        match op {
                                            OpCode::Push
                                            | OpCode::Jump
                                            | OpCode::Brz
                                            | OpCode::Call
                                            | OpCode::Bind
                                            | OpCode::Unbind
                                            | OpCode::Decohere
                                            | OpCode::Telomerase
                                            | OpCode::Mitosis
                                            | OpCode::Apoptosis
                                            | OpCode::Cas9Cut
                                            | OpCode::Ligase
                                            | OpCode::Entangle => {
                                                #[cfg(feature = "cortex")]
                                                {
                                                    // Gate is handled by cortex feature, but we can't easily conditionally match inside match arm list
                                                    // So we'll just check if it's one of these.
                                                    // Actually, this match arm structure is tricky with cfg flags.
                                                    // Let's use if statements or just be permissive.
                                                }
                                                // 1 Arg
                                                if k + 1 < sequence.len() {
                                                    if let Some(arg) =
                                                        value_to_nucleotide(&sequence[k + 1], 0)
                                                    {
                                                        args.push(arg);
                                                    } else {
                                                        vm.output.push(
                                                            "INCUBATE: Warning: Recursion limit exceeded".to_string(),
                                                        );
                                                    }
                                                    k += 1; // Consume arg
                                                }
                                            }
                                            #[cfg(feature = "cortex")]
                                            OpCode::Gate
                                            | OpCode::Link
                                            | OpCode::Sever
                                            | OpCode::Spark => {
                                                if k + 1 < sequence.len() {
                                                    if let Some(arg) =
                                                        value_to_nucleotide(&sequence[k + 1], 0)
                                                    {
                                                        args.push(arg);
                                                    } else {
                                                        vm.output.push(
                                                            "INCUBATE: Warning: Recursion limit exceeded".to_string(),
                                                        );
                                                    }
                                                    k += 1; // Consume arg
                                                }
                                            }
                                            _ => {}
                                        }

                                        genes.push(crate::ast::Gene { op, args });
                                        k += 1;
                                    }
                                }
                            }

                            vm.dna.helix.strands.push(crate::ast::Strand { genes });
                            vm.telomeres.push(50);
                            #[cfg(feature = "cortex")]
                            {
                                vm.activation_levels.push(0);
                                vm.synapse_map.push(Vec::new());
                            }
                            vm.energy = vm.energy.saturating_sub(20); // Cost
                            vm.output.push(format!(
                                "INCUBATE: Created new strand {} from grid",
                                vm.dna.helix.strands.len() - 1
                            ));
                        }
                    } else {
                        vm.output
                            .push("Error: Invalid length for incubate".to_string());
                    }
                } else {
                    vm.output
                        .push("Error: Type mismatch for incubate".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for incubate".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Methylate => {
            if vm.stack.len() >= 2 {
                let gene_val = vm.stack.pop().unwrap();
                let strand_val = vm.stack.pop().unwrap();
                if let (Value::Int(g_idx), Value::Int(s_idx)) = (gene_val, strand_val) {
                    if vm.epigenome.len() < MAX_EPIGENOME_SIZE {
                        vm.epigenome.insert((s_idx as usize, g_idx as usize));
                        vm.output.push(format!("METHYLATED: {}:{}", s_idx, g_idx));
                    } else {
                        vm.output
                            .push("Error: Epigenome size limit exceeded".to_string());
                    }
                } else {
                    vm.output
                        .push("Error: Invalid args for methylate".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for methylate".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Demethylate => {
            if vm.stack.len() >= 2 {
                let gene_val = vm.stack.pop().unwrap();
                let strand_val = vm.stack.pop().unwrap();
                if let (Value::Int(g_idx), Value::Int(s_idx)) = (gene_val, strand_val) {
                    vm.epigenome.remove(&(s_idx as usize, g_idx as usize));
                    vm.output.push(format!("DEMETHYLATED: {}:{}", s_idx, g_idx));
                } else {
                    vm.output
                        .push("Error: Invalid args for demethylate".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for demethylate".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Telomerase => {
            if let Some(val) = vm.stack.pop() {
                match val {
                    Value::Int(amount) => {
                        if amount > 0 {
                            let idx = vm.ip.0;
                            if idx < vm.telomeres.len() {
                                vm.telomeres[idx] = vm.telomeres[idx].saturating_add(amount);
                                vm.energy = vm.energy.saturating_sub(25); // High cost
                                vm.output.push(format!(
                                    "TELOMERASE: Extended strand {} by {}",
                                    idx, amount
                                ));
                            }
                        }
                    }
                    _ => vm
                        .output
                        .push("Error: Invalid arg for telomerase".to_string()),
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for telomerase".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::TLen => {
            let idx = vm.ip.0;
            if idx < vm.telomeres.len() {
                vm.stack.push(Value::Int(vm.telomeres[idx]));
            } else {
                vm.stack.push(Value::Int(0));
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Splice => exec_splice(vm),
        #[cfg(feature = "nova")]
        OpCode::Recombine => exec_recombine(vm),
        #[cfg(feature = "nova")]
        OpCode::SIndex => {
            vm.stack.push(Value::Int(vm.ip.0 as i64));
            None
        }
        #[cfg(feature = "nova")]
        OpCode::CrisprScan => exec_crispr_scan(vm),
        #[cfg(feature = "nova")]
        OpCode::Cas9Cut => exec_cas9_cut(vm),
        #[cfg(feature = "nova")]
        OpCode::Ligase => {
            // stack: donor_idx, recipient_idx (bottom)
            if vm.stack.len() >= 2 {
                let donor_val = vm.stack.pop().unwrap();
                let recipient_val = vm.stack.pop().unwrap();

                if let (Value::Int(d_idx), Value::Int(r_idx)) = (donor_val, recipient_val) {
                    let d_idx = d_idx as usize;
                    let r_idx = r_idx as usize;
                    let helix_len = vm.dna.helix.strands.len();

                    if d_idx < helix_len && r_idx < helix_len {
                        if d_idx == r_idx {
                            vm.output
                                .push("Warning: Ligase on same strand is no-op".to_string());
                        } else {
                            // We need to move genes from donor to recipient.
                            let (lower, upper) = if d_idx < r_idx {
                                (d_idx, r_idx)
                            } else {
                                (r_idx, d_idx)
                            };

                            let (first_slice, second_slice) =
                                vm.dna.helix.strands.split_at_mut(upper);
                            let strand_low = &mut first_slice[lower];
                            let strand_high = &mut second_slice[0];

                            let (strand_d, strand_r) = if d_idx < r_idx {
                                (strand_low, strand_high)
                            } else {
                                (strand_high, strand_low)
                            };

                            strand_r.genes.append(&mut strand_d.genes);
                            // donor genes are now empty.

                            vm.energy = vm.energy.saturating_sub(10);
                            vm.output
                                .push(format!("LIGASE: Appended strand {} to {}", d_idx, r_idx));
                        }
                    } else {
                        vm.output
                            .push("Error: Strand index out of bounds for ligase".to_string());
                    }
                } else {
                    vm.output
                        .push("Error: Type mismatch for ligase".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for ligase".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Mitosis => {
            // stack: strand_idx (target to clone)
            if let Some(val) = vm.stack.pop() {
                match val {
                    Value::Int(idx) => {
                        let s_idx = idx as usize;
                        if s_idx < vm.dna.helix.strands.len() {
                            // Clone the strand
                            let new_strand = vm.dna.helix.strands[s_idx].clone();
                            vm.dna.helix.strands.push(new_strand);
                            vm.telomeres.push(50); // Default life

                            #[cfg(feature = "cortex")]
                            {
                                vm.activation_levels.push(0);
                                vm.synapse_map.push(Vec::new());
                            }

                            // Inherit epigenetics
                            // We need to find all keys (s_idx, g_idx) and insert (new_idx, g_idx)
                            let new_s_idx = vm.dna.helix.strands.len() - 1;
                            let genes_to_methylate: Vec<usize> = vm
                                .epigenome
                                .iter()
                                .filter(|(s, _)| *s == s_idx)
                                .map(|(_, g)| *g)
                                .collect();

                            for g_idx in genes_to_methylate {
                                vm.epigenome.insert((new_s_idx, g_idx));
                            }

                            vm.energy = vm.energy.saturating_sub(30); // Cost
                            vm.output
                                .push(format!("MITOSIS: Cloned strand {} to {}", s_idx, new_s_idx));
                        } else {
                            vm.output
                                .push("Error: Strand index out of bounds for mitosis".to_string());
                        }
                    }
                    _ => vm
                        .output
                        .push("Error: Type mismatch for mitosis".to_string()),
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for mitosis".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Apoptosis => {
            // stack: strand_idx
            if let Some(val) = vm.stack.pop() {
                match val {
                    Value::Int(idx) => {
                        let s_idx = idx as usize;
                        if s_idx < vm.dna.helix.strands.len() {
                            // Backup to graveyard
                            let strand = vm.dna.helix.strands[s_idx].clone();
                            vm.graveyard.push(strand);

                            vm.dna.helix.strands[s_idx].genes.clear();

                            // Remove associated epigenetics
                            vm.epigenome.retain(|(s, _)| *s != s_idx);

                            vm.energy = vm.energy.saturating_sub(10);
                            vm.output
                                .push(format!("APOPTOSIS: Cleared strand {}", s_idx));
                        } else {
                            vm.output.push(
                                "Error: Strand index out of bounds for apoptosis".to_string(),
                            );
                        }
                    }
                    _ => vm
                        .output
                        .push("Error: Type mismatch for apoptosis".to_string()),
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for apoptosis".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Integrase => {
            // stack: arg, name, gene_idx, strand_idx (bottom)
            if vm.stack.len() >= 4 {
                let arg_val = vm.stack.pop().unwrap();
                let name_val = vm.stack.pop().unwrap();
                let gene_idx_val = vm.stack.pop().unwrap();
                let strand_idx_val = vm.stack.pop().unwrap();

                match (strand_idx_val, gene_idx_val, name_val, arg_val) {
                    (Value::Int(s), Value::Int(g), Value::Str(name), arg) => {
                        let s_idx = s as usize;
                        let g_idx = g as usize;
                        let helix_len = vm.dna.helix.strands.len();

                        if s >= 0 && s_idx < helix_len {
                            let strand_len = vm.dna.helix.strands[s_idx].genes.len();
                            if g >= 0 && g_idx <= strand_len {
                                // Create Gene
                                let new_gene = crate::ast::Gene {
                                    op: name.parse().unwrap_or(OpCode::Unknown(name.clone())),
                                    args: if let Some(n) = value_to_nucleotide(&arg, 0) {
                                        vec![n]
                                    } else {
                                        vm.output.push(
                                            "INTEGRASE: Warning: Recursion limit exceeded"
                                                .to_string(),
                                        );
                                        vec![]
                                    },
                                };

                                // Insert
                                vm.dna.helix.strands[s_idx].genes.insert(g_idx, new_gene);

                                // Update Epigenome: Shift all markers at (s_idx, k >= g_idx) to k+1
                                let mut new_markers = Vec::new();
                                let mut to_remove = Vec::new();
                                for &(ms, mg) in vm.epigenome.iter() {
                                    if ms == s_idx && mg >= g_idx {
                                        to_remove.push((ms, mg));
                                        new_markers.push((ms, mg + 1));
                                    }
                                }
                                for marker in to_remove {
                                    vm.epigenome.remove(&marker);
                                }
                                for marker in new_markers {
                                    vm.epigenome.insert(marker);
                                }

                                vm.energy = vm.energy.saturating_sub(20);
                                vm.output
                                    .push(format!("INTEGRASE: Inserted {} at {}:{}", name, s, g));

                                // Update IP if we inserted before or at current execution
                                if vm.ip.0 == s_idx && vm.ip.1 >= g_idx {
                                    vm.ip.1 += 1;
                                }
                                // Default None means step() will increment IP +1.
                                // If we shifted IP +1 here, total is +2.
                                // This skips the inserted gene (if at g_idx) and the current gene (now at g_idx+1).
                                // Correct.
                            } else {
                                vm.output.push(
                                    "Error: Gene index out of bounds for integrase".to_string(),
                                );
                            }
                        } else {
                            vm.output.push(
                                "Error: Strand index out of bounds for integrase".to_string(),
                            );
                        }
                    }
                    _ => vm
                        .output
                        .push("Error: Type mismatch for integrase".to_string()),
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for integrase".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Excision => {
            // stack: gene_idx, strand_idx (bottom)
            if vm.stack.len() >= 2 {
                let gene_idx_val = vm.stack.pop().unwrap();
                let strand_idx_val = vm.stack.pop().unwrap();

                match (strand_idx_val, gene_idx_val) {
                    (Value::Int(s), Value::Int(g)) => {
                        let s_idx = s as usize;
                        let g_idx = g as usize;
                        let helix_len = vm.dna.helix.strands.len();

                        if s >= 0 && s_idx < helix_len {
                            let strand_len = vm.dna.helix.strands[s_idx].genes.len();
                            if g >= 0 && g_idx < strand_len {
                                // Remove
                                vm.dna.helix.strands[s_idx].genes.remove(g_idx);

                                // Update Epigenome: Remove marker at g_idx, shift k > g_idx to k-1
                                vm.epigenome.remove(&(s_idx, g_idx));
                                let mut new_markers = Vec::new();
                                let mut to_remove = Vec::new();
                                for &(ms, mg) in vm.epigenome.iter() {
                                    if ms == s_idx && mg > g_idx {
                                        to_remove.push((ms, mg));
                                        new_markers.push((ms, mg - 1));
                                    }
                                }
                                for marker in to_remove {
                                    vm.epigenome.remove(&marker);
                                }
                                for marker in new_markers {
                                    vm.epigenome.insert(marker);
                                }

                                vm.energy = vm.energy.saturating_sub(15);
                                vm.output.push(format!("EXCISION: Removed {}:{}", s, g));

                                // Update IP
                                if vm.ip.0 == s_idx {
                                    if g_idx < vm.ip.1 {
                                        // Removed before current. Shift IP left.
                                        vm.ip.1 -= 1;
                                        return None; // step() increments +1. Net 0 change (but content moved left, so we execute next).
                                    } else if g_idx == vm.ip.1 {
                                        // Removed current.
                                        // Next gene slid into current slot.
                                        // We want to execute it.
                                        // So return IP as is, prevent step() increment.
                                        return Some(vm.ip);
                                    }
                                }
                            } else {
                                vm.output.push(
                                    "Error: Gene index out of bounds for excision".to_string(),
                                );
                            }
                        } else {
                            vm.output
                                .push("Error: Strand index out of bounds for excision".to_string());
                        }
                    }
                    _ => vm
                        .output
                        .push("Error: Type mismatch for excision".to_string()),
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for excision".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Secrete => {
            // stack: channel, amount (top)
            if vm.stack.len() >= 2 {
                let amount_val = vm.stack.pop().unwrap();
                let channel_val = vm.stack.pop().unwrap();
                if let (Value::Int(c), Value::Int(a)) = (channel_val, amount_val) {
                    if a > 0 {
                        let (cy, cx) = vm.context_loc;
                        let channel_idx = (c.unsigned_abs() as usize) % 3;
                        vm.hormone_grid[cy][cx][channel_idx] =
                            vm.hormone_grid[cy][cx][channel_idx].saturating_add(a);
                        vm.output.push(format!(
                            "SECRETE: Added {} to channel {} at {},{}",
                            a, c, cx, cy
                        ));
                    }
                } else {
                    vm.output
                        .push("Error: Type mismatch for secrete".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for secrete".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Detect => {
            // stack: channel
            if let Some(val) = vm.stack.pop() {
                if let Value::Int(c) = val {
                    let (cy, cx) = vm.context_loc;
                    let channel_idx = (c.unsigned_abs() as usize) % 3;
                    let intensity = vm.hormone_grid[cy][cx][channel_idx];
                    vm.stack.push(Value::Int(intensity));
                } else {
                    vm.output
                        .push("Error: Type mismatch for detect".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for detect".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Absorb => {
            // stack: channel, amount (top)
            if vm.stack.len() >= 2 {
                let amount_val = vm.stack.pop().unwrap();
                let channel_val = vm.stack.pop().unwrap();
                if let (Value::Int(c), Value::Int(a)) = (channel_val, amount_val) {
                    let (cy, cx) = vm.context_loc;
                    let channel_idx = (c.unsigned_abs() as usize) % 3;
                    let intensity = &mut vm.hormone_grid[cy][cx][channel_idx];
                    let absorbed = if *intensity >= a {
                        *intensity = intensity.saturating_sub(a);
                        a
                    } else {
                        let v = *intensity;
                        *intensity = 0;
                        v
                    };
                    vm.stack.push(Value::Int(absorbed));
                    vm.output.push(format!(
                        "ABSORB: Consumed {} from channel {} at {},{}",
                        absorbed, c, cx, cy
                    ));
                } else {
                    vm.output
                        .push("Error: Type mismatch for absorb".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for absorb".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Migrate => {
            // stack: dy, dx (top)
            if vm.stack.len() >= 2 {
                let dx_val = vm.stack.pop().unwrap();
                let dy_val = vm.stack.pop().unwrap();
                if let (Value::Int(mut dy), Value::Int(mut dx)) = (dy_val, dx_val) {
                    if vm.chirality == crate::vm::Chirality::Right {
                        dy = -dy;
                        dx = -dx;
                    }

                    if vm.phase == Phase::Crystalline {
                        vm.output
                            .push("Error: Crystalline phase is immobile".to_string());
                        return None;
                    }

                    let (cy, cx) = vm.context_loc;

                    let mut blocked = false;
                    if vm.phase != Phase::Ethereal {
                        if let Some(mask) = get_direction_mask(dy, dx) {
                            if (vm.membranes[cy][cx] & mask) != 0 {
                                blocked = true;
                            }
                        }
                    }

                    if !blocked {
                        if let Some((mut new_y, mut new_x)) =
                            vm.normalize_coords(cy as i64 + dy, cx as i64 + dx)
                        {
                            // Check for portal
                            if let Some(&(py, px)) = vm.portals.get(&(new_y, new_x)) {
                                vm.output.push(format!(
                                    "PORTAL: Teleported from {},{} to {},{}",
                                    new_x, new_y, px, py
                                ));
                                new_y = py;
                                new_x = px;
                            }

                            vm.context_loc = (new_y, new_x);
                            vm.energy = vm.energy.saturating_sub(5);
                            vm.output
                                .push(format!("MIGRATE: moved to {},{}", new_x, new_y));
                        } else {
                            // Hit boundary
                            vm.energy = vm.energy.saturating_sub(2);
                            vm.output.push("MIGRATE: Blocked by boundary".to_string());
                            if vm.trigger_reflex(0) {
                                return Some(vm.ip);
                            }
                        }
                    } else {
                        // Blocked by membrane
                        vm.energy = vm.energy.saturating_sub(2);
                        vm.output.push("MIGRATE: Blocked by membrane".to_string());
                        if vm.trigger_reflex(0) {
                            return Some(vm.ip);
                        }
                    }
                } else {
                    vm.output
                        .push("Error: Type mismatch for migrate".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for migrate".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Detox => {
            // stack: radius (top)
            if let Some(val) = vm.stack.pop() {
                if let Value::Int(r) = val {
                    let (cy, cx) = vm.context_loc;
                    let coords = vm.get_circular_coords(cx as i64, cy as i64, r);
                    for (tx, ty) in coords {
                        vm.waste_grid[ty][tx] = 0;
                    }
                    vm.energy = vm.energy.saturating_sub((r * r + 1).clamp(5, 50)); // Cost proportional to area
                    vm.output
                        .push(format!("DETOX: Cleansed radius {} at {},{}", r, cx, cy));
                } else {
                    vm.output.push("Error: Type mismatch for detox".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for detox".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::WRead => {
            let (cy, cx) = vm.context_loc;
            let waste = vm.waste_grid[cy][cx];
            vm.stack.push(Value::Int(waste));
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Call => {
            if let Some(Nucleotide::Number(idx)) = args.first() {
                let strand_idx = *idx as usize;
                if strand_idx < vm.dna.helix.strands.len() {
                    if vm.call_stack.len() >= crate::vm::MAX_CALL_STACK_DEPTH {
                        vm.output.push("Error: Call stack overflow".to_string());
                        return None;
                    }
                    // Push return address (current strand, next gene)
                    // ip points to Call instruction. Step loop will increment it.
                    // But if we jump, step loop sets ip to target.
                    // So we need to push (ip.0, ip.1 + 1).
                    vm.call_stack.push((vm.ip.0, vm.ip.1 + 1));
                    return Some((strand_idx, 0));
                } else {
                    vm.output
                        .push("Error: Invalid strand index for call".to_string());
                }
            } else {
                vm.output.push("Error: Invalid arg for call".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Exec => {
            if let Some(val) = vm.stack.pop() {
                if let Value::Int(idx) = val {
                    let strand_idx = idx as usize;
                    if strand_idx < vm.dna.helix.strands.len() {
                        if vm.call_stack.len() >= crate::vm::MAX_CALL_STACK_DEPTH {
                            vm.output.push("Error: Call stack overflow".to_string());
                            return None;
                        }
                        vm.call_stack.push((vm.ip.0, vm.ip.1 + 1));
                        return Some((strand_idx, 0));
                    } else {
                        vm.output.push("Error: Invalid strand index for exec".to_string());
                    }
                } else {
                    vm.output.push("Error: Type mismatch for exec".to_string());
                }
            } else {
                vm.output.push("Error: Stack underflow for exec".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Ret => {
            if let Some(ret_addr) = vm.call_stack.pop() {
                return Some(ret_addr);
            } else {
                vm.output
                    .push("Warning: Return with empty stack".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Bind => {
            // stack: strand_idx (top), char_code
            if vm.stack.len() >= 2 {
                let s_val = vm.stack.pop().unwrap();
                let c_val = vm.stack.pop().unwrap();
                if let (Value::Int(s), Value::Int(c)) = (s_val, c_val) {
                    let strand_idx = s as usize;
                    let key = (c as u8) as char;
                    if strand_idx < vm.dna.helix.strands.len() {
                        vm.receptors.insert(key, strand_idx);
                        vm.output
                            .push(format!("BIND: '{}' -> Strand {}", key, strand_idx));
                    } else {
                        vm.output
                            .push("Error: Invalid strand index for bind".to_string());
                    }
                } else {
                    vm.output.push("Error: Type mismatch for bind".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for bind".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Unbind => {
            if let Some(val) = vm.stack.pop() {
                if let Value::Int(c) = val {
                    let key = (c as u8) as char;
                    vm.receptors.remove(&key);
                    vm.output.push(format!("UNBIND: '{}'", key));
                } else {
                    vm.output
                        .push("Error: Type mismatch for unbind".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for unbind".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Entangle => {
            if vm.stack.len() >= 2 {
                let s_val2 = vm.stack.pop().unwrap();
                let s_val1 = vm.stack.pop().unwrap();
                if let (Value::Int(s1), Value::Int(s2)) = (s_val1, s_val2) {
                    let idx1 = s1 as usize;
                    let idx2 = s2 as usize;
                    let len = vm.dna.helix.strands.len();
                    if idx1 < len && idx2 < len {
                        if idx1 != idx2 {
                            // Remove existing links
                            if let Some(old) = vm.entangled_pairs.remove(&idx1) {
                                vm.entangled_pairs.remove(&old);
                            }
                            if let Some(old) = vm.entangled_pairs.remove(&idx2) {
                                vm.entangled_pairs.remove(&old);
                            }

                            // Add new links
                            vm.entangled_pairs.insert(idx1, idx2);
                            vm.entangled_pairs.insert(idx2, idx1);
                            vm.output.push(format!("ENTANGLE: {} <-> {}", idx1, idx2));
                        } else {
                            vm.output
                                .push("Warning: Cannot entangle strand with itself".to_string());
                        }
                    } else {
                        vm.output
                            .push("Error: Strand index out of bounds for entangle".to_string());
                    }
                } else {
                    vm.output
                        .push("Error: Type mismatch for entangle".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for entangle".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Decohere => {
            if let Some(val) = vm.stack.pop() {
                if let Value::Int(s) = val {
                    let idx = s as usize;
                    if let Some(partner) = vm.entangled_pairs.remove(&idx) {
                        vm.entangled_pairs.remove(&partner);
                        vm.output
                            .push(format!("DECOHERE: Broken link {} <-> {}", idx, partner));
                    } else {
                        vm.output
                            .push(format!("DECOHERE: No link found for {}", idx));
                    }
                } else {
                    vm.output
                        .push("Error: Type mismatch for decohere".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for decohere".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Conjugate => {
            // stack: direction (0=R, 1=D, 2=L, 3=U), y, x, strand_idx (bottom)
            if vm.stack.len() >= 4 {
                let dir_val = vm.stack.pop().unwrap();
                let x_val = vm.stack.pop().unwrap();
                let y_val = vm.stack.pop().unwrap();
                let s_val = vm.stack.pop().unwrap();

                if let (Value::Int(s), Value::Int(y), Value::Int(x), Value::Int(dir)) =
                    (s_val, y_val, x_val, dir_val)
                {
                    let s_idx = s as usize;
                    if s_idx < vm.dna.helix.strands.len() {
                        let strand = &vm.dna.helix.strands[s_idx];
                        let mut curr_x = x;
                        let mut curr_y = y;
                        let (dx, dy) = match dir.rem_euclid(4) {
                            0 => (1, 0),
                            1 => (0, 1),
                            2 => (-1, 0),
                            3 => (0, -1),
                            _ => (0, 0),
                        };

                        let mut success_count = 0;
                        let mut cells_to_write = Vec::new();

                        for gene in &strand.genes {
                            cells_to_write.push(Value::Str(gene.op.to_string()));
                            for arg in &gene.args {
                                match arg {
                                    crate::ast::Nucleotide::Number(n) => {
                                        cells_to_write.push(Value::Int(*n));
                                    }
                                    crate::ast::Nucleotide::String(s) => {
                                        cells_to_write.push(Value::Str(s.clone()));
                                    }
                                    _ => {}
                                }
                            }
                        }

                        for val in cells_to_write {
                            if let Some((ny, nx)) = vm.normalize_coords(curr_y, curr_x) {
                                vm.grid[ny][nx] = val;
                                success_count += 1;

                                // Advance
                                if let Some((next_y, next_x)) =
                                    vm.normalize_coords(ny as i64 + dy, nx as i64 + dx)
                                {
                                    curr_y = next_y as i64;
                                    curr_x = next_x as i64;
                                } else {
                                    // Hit wall, stop writing
                                    break;
                                }
                            } else {
                                break; // Start out of bounds
                            }
                        }

                        vm.energy -= success_count; // Cost 1 per cell
                        vm.output.push(format!(
                            "CONJUGATE: Wrote {} cells from strand {} at {},{}",
                            success_count, s_idx, x, y
                        ));
                    } else {
                        vm.output
                            .push("Error: Invalid strand index for conjugate".to_string());
                    }
                } else {
                    vm.output
                        .push("Error: Type mismatch for conjugate".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for conjugate".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Gravitate => {
            // stack: radius
            if let Some(val) = vm.stack.pop() {
                if let Value::Int(r) = val {
                    if r > 0 {
                        let (cy, cx) = vm.context_loc;
                        // Get coordinates within radius
                        // Note: get_circular_coords uses Euclidean distance on Plane.
                        // For Torus, we should use toroidal distance, but for simplicity we keep it local.
                        // However, valid coordinates are returned.
                        let coords = vm.get_circular_coords(cx as i64, cy as i64, r);

                        // Calculate distances and sort
                        let mut coords_with_dist: Vec<((usize, usize), i64)> = coords
                            .into_iter()
                            .map(|(x, y)| {
                                let dx = x as i64 - cx as i64;
                                let dy = y as i64 - cy as i64;
                                // Squared distance is sufficient for sorting
                                ((x, y), dx * dx + dy * dy)
                            })
                            .collect();

                        // Sort by distance (ascending)
                        coords_with_dist.sort_by_key(|&(_, d)| d);

                        let mut moved_count = 0;

                        for ((tx, ty), dist_sq) in coords_with_dist {
                            if dist_sq == 0 {
                                continue; // Skip center
                            }

                            // If empty, skip
                            if matches!(vm.grid[ty][tx], Value::Int(0)) {
                                continue;
                            }

                            // Calculate target (one step closer to center)
                            let dx = cx as i64 - tx as i64;
                            let dy = cy as i64 - ty as i64;

                            let sx = if dx > 0 {
                                1
                            } else if dx < 0 {
                                -1
                            } else {
                                0
                            };
                            let sy = if dy > 0 {
                                1
                            } else if dy < 0 {
                                -1
                            } else {
                                0
                            };

                            // Use normalize_coords to find valid target
                            if let Some((target_y, target_x)) =
                                vm.normalize_coords(ty as i64 + sy, tx as i64 + sx)
                            {
                                // Check if target is empty
                                if matches!(vm.grid[target_y][target_x], Value::Int(0)) {
                                    // Move
                                    vm.grid[target_y][target_x] = vm.grid[ty][tx].clone();
                                    vm.grid[ty][tx] = Value::Int(0);
                                    moved_count += 1;
                                }
                            }
                        }

                        vm.energy = vm.energy.saturating_sub(moved_count + 5); // Base cost + variable
                        vm.output.push(format!(
                            "GRAVITATE: Pulled {} items towards {},{}",
                            moved_count, cx, cy
                        ));
                    } else {
                        // Negative or zero radius is no-op
                    }
                } else {
                    vm.output
                        .push("Error: Type mismatch for gravitate".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for gravitate".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Lumine => {
            // stack: intensity, radius (bottom)
            if vm.stack.len() >= 2 {
                let intensity_val = vm.stack.pop().unwrap();
                let radius_val = vm.stack.pop().unwrap();
                if let (Value::Int(r), Value::Int(intensity)) = (radius_val, intensity_val) {
                    if r > 0 && intensity > 0 {
                        let (cy, cx) = vm.context_loc;
                        let coords = vm.get_circular_coords(cx as i64, cy as i64, r);
                        let count = coords.len();
                        for (tx, ty) in coords {
                            vm.light_grid[ty][tx] = vm.light_grid[ty][tx].saturating_add(intensity);
                        }
                        vm.energy = vm.energy.saturating_sub((count / 2) as i64);
                        vm.output.push(format!(
                            "LUMINE: Emitted {} light at {},{} r={}",
                            intensity, cx, cy, r
                        ));
                    }
                } else {
                    vm.output
                        .push("Error: Type mismatch for lumine".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for lumine".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::SenseLight => {
            let (cy, cx) = vm.context_loc;
            let intensity = vm.light_grid[cy][cx];
            vm.stack.push(Value::Int(intensity));
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Dream => {
            // stack: ticks, strand_idx (bottom)
            if vm.stack.len() >= 2 {
                let ticks_val = vm.stack.pop().unwrap();
                let s_val = vm.stack.pop().unwrap();

                if let (Value::Int(ticks), Value::Int(s_idx)) = (ticks_val, s_val) {
                    let idx = s_idx as usize;
                    if idx < vm.dna.helix.strands.len() && ticks > 0 {
                        // Cap ticks
                        let safe_ticks = ticks.min(1000);

                        // Clone VM
                        let mut dream_vm = vm.clone();

                        // Force a mutation
                        dream_vm.mutate();
                        let mutation_desc = dream_vm
                            .output
                            .last()
                            .cloned()
                            .unwrap_or_else(|| "Unknown Mutation".to_string());

                        // Capture mutated strand
                        let mutated_strand = if idx < dream_vm.dna.helix.strands.len() {
                            Some(dream_vm.dna.helix.strands[idx].clone())
                        } else {
                            None
                        };

                        // Run simulation
                        dream_vm.ip = (idx, 0);
                        dream_vm.output.clear();
                        dream_vm.halted = false;

                        for _ in 0..safe_ticks {
                            dream_vm.step();
                            if dream_vm.halted {
                                break;
                            }
                        }

                        // Evaluate
                        let success = dream_vm.energy > vm.energy;

                        // Pay Cost (Base 50 + ticks/2)
                        let cost = 50 + (safe_ticks / 2);

                        let trace = crate::vm::dream::DreamTrace::new(
                            0,
                            idx,
                            safe_ticks as usize,
                            cost,
                            dream_vm.energy,
                            if dream_vm.halted { 0 } else { 1 },
                            mutation_desc,
                            mutated_strand,
                            success,
                            dream_vm.output.clone(),
                            None,
                        );
                        vm.dream_traces.push(trace);

                        if success {
                            // Adopt DNA
                            vm.dna = dream_vm.dna;
                            vm.stack.push(Value::Int(1)); // Success
                            vm.output.push("DREAM: Mutation accepted".to_string());
                        } else {
                            vm.stack.push(Value::Int(0)); // Failure
                            vm.output.push("DREAM: Mutation discarded".to_string());
                        }

                        vm.energy = vm.energy.saturating_sub(cost);
                    } else {
                        vm.output.push("Error: Invalid args for dream".to_string());
                    }
                } else {
                    vm.output.push("Error: Type mismatch for dream".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for dream".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Chemotaxis => {
            if let Some(val) = vm.stack.pop() {
                if let Value::Int(c) = val {
                    let (cy, cx) = vm.context_loc;
                    let channel_idx = (c.unsigned_abs() as usize) % 3;

                    let mut max_intensity = -1;
                    let mut best_dy = 0;
                    let mut best_dx = 0;

                    for dy in -1..=1 {
                        for dx in -1..=1 {
                            if let Some((ny, nx)) =
                                vm.normalize_coords(cy as i64 + dy, cx as i64 + dx)
                            {
                                let intensity = vm.hormone_grid[ny][nx][channel_idx];
                                if intensity > max_intensity {
                                    max_intensity = intensity;
                                    best_dy = dy;
                                    best_dx = dx;
                                }
                            }
                        }
                    }

                    vm.stack.push(Value::Int(best_dy));
                    vm.stack.push(Value::Int(best_dx));
                    vm.energy = vm.energy.saturating_sub(5);
                } else {
                    vm.output
                        .push("Error: Type mismatch for chemotaxis".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for chemotaxis".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Identity => {
            let id = match &vm.active_organelle_kind {
                None => -1, // Nucleus
                Some(OrganelleType::Worker) => 0,
                Some(OrganelleType::Chloroplast) => 1,
                Some(OrganelleType::Mitochondria) => 2,
                Some(OrganelleType::Lysosome) => 3,
                Some(OrganelleType::Ribosome) => 4,
                Some(OrganelleType::Void) => 5,
                Some(OrganelleType::Alchemist) => 6,
            };
            vm.stack.push(Value::Int(id));
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Differentiate => {
            if let Some(val) = vm.stack.pop() {
                if vm.active_organelle_kind.is_some() {
                    if let Value::Int(t) = val {
                        let new_kind = match t {
                            1 => Some(OrganelleType::Chloroplast),
                            2 => Some(OrganelleType::Mitochondria),
                            3 => Some(OrganelleType::Lysosome),
                            4 => Some(OrganelleType::Ribosome),
                            5 => Some(OrganelleType::Void),
                            6 => Some(OrganelleType::Alchemist),
                            _ => Some(OrganelleType::Worker), // 0 or others fallback to Worker
                        };

                        if let Some(kind) = new_kind {
                            vm.signal_differentiation = Some(kind.clone());
                            vm.energy = vm.energy.saturating_sub(50); // High cost to re-specialize
                            vm.output
                                .push(format!("DIFFERENTIATE: Requesting change to {:?}", kind));
                        }
                    } else {
                        vm.output
                            .push("Error: Type mismatch for differentiate".to_string());
                    }
                } else {
                    vm.output
                        .push("Error: Nucleus cannot differentiate".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for differentiate".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Shape => {
            if let Some(val) = vm.stack.pop() {
                if let Value::Int(t) = val {
                    let new_topology = match t {
                        0 => Some(super::Topology::Plane),
                        1 => Some(super::Topology::Torus),
                        2 => Some(super::Topology::CylinderH),
                        3 => Some(super::Topology::CylinderV),
                        4 => Some(super::Topology::Klein),
                        5 => Some(super::Topology::Mobius),
                        6 => Some(super::Topology::Hyperbolic),
                        _ => None,
                    };

                    if let Some(topo) = new_topology {
                        vm.topology = topo;
                        vm.output
                            .push(format!("SHAPE: Changed topology to {:?}", topo));
                        vm.energy = vm.energy.saturating_sub(100); // Massive energy cost to reshape reality
                    } else {
                        vm.output
                            .push(format!("Error: Invalid topology index {}", t));
                    }
                } else {
                    vm.output.push("Error: Type mismatch for shape".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for shape".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Rift => {
            // stack: y2, x2, y1, x1 (bottom)
            if vm.stack.len() >= 4 {
                let x2_val = vm.stack.pop().unwrap();
                let y2_val = vm.stack.pop().unwrap();
                let x1_val = vm.stack.pop().unwrap();
                let y1_val = vm.stack.pop().unwrap();

                if let (Value::Int(x1), Value::Int(y1), Value::Int(x2), Value::Int(y2)) =
                    (x1_val, y1_val, x2_val, y2_val)
                {
                    if (0..16).contains(&x1)
                        && (0..16).contains(&y1)
                        && (0..16).contains(&x2)
                        && (0..16).contains(&y2)
                    {
                        vm.portals
                            .insert((y1 as usize, x1 as usize), (y2 as usize, x2 as usize));
                        vm.energy = vm.energy.saturating_sub(50);
                        vm.output.push(format!(
                            "RIFT: Opened portal from {},{} to {},{}",
                            x1, y1, x2, y2
                        ));
                    } else {
                        vm.output
                            .push("Error: Coordinates out of bounds for rift".to_string());
                    }
                } else {
                    vm.output.push("Error: Type mismatch for rift".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for rift".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Seal => {
            // stack: y, x (top)
            if vm.stack.len() >= 2 {
                let x_val = vm.stack.pop().unwrap();
                let y_val = vm.stack.pop().unwrap();
                if let (Value::Int(x), Value::Int(y)) = (x_val, y_val) {
                    if (0..16).contains(&x) && (0..16).contains(&y) {
                        if vm.portals.remove(&(y as usize, x as usize)).is_some() {
                            vm.energy = vm.energy.saturating_sub(10);
                            vm.output
                                .push(format!("SEAL: Closed portal at {},{}", x, y));
                        } else {
                            vm.output
                                .push(format!("SEAL: No portal found at {},{}", x, y));
                        }
                    } else {
                        vm.output
                            .push("Error: Coordinates out of bounds for seal".to_string());
                    }
                } else {
                    vm.output.push("Error: Type mismatch for seal".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for seal".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Sonar => {
            // stack: dy, dx (top)
            if vm.stack.len() >= 2 {
                let dx_val = vm.stack.pop().unwrap();
                let dy_val = vm.stack.pop().unwrap();
                if let (Value::Int(dy), Value::Int(dx)) = (dy_val, dx_val) {
                    let (cy, cx) = vm.context_loc;
                    let mut found = false;

                    for d in 1..=16 {
                        if let Some((ny, nx)) =
                            vm.normalize_coords(cy as i64 + dy * d, cx as i64 + dx * d)
                        {
                            if !matches!(vm.grid[ny][nx], Value::Int(0)) {
                                vm.stack.push(Value::Int(d));
                                vm.stack.push(vm.grid[ny][nx].clone());
                                vm.sonar_target = Some((ny, nx));
                                found = true;
                                break;
                            }
                        } else {
                            // Hit wall - stop scanning
                            break;
                        }
                    }

                    if !found {
                        vm.stack.push(Value::Int(16));
                        vm.stack.push(Value::Int(0));
                    }

                    vm.energy = vm.energy.saturating_sub(2);
                } else {
                    vm.output.push("Error: Type mismatch for sonar".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for sonar".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Broadcast => {
            // stack: channel, value (top)
            if vm.stack.len() >= 2 {
                let value = vm.stack.pop().unwrap();
                let channel_val = vm.stack.pop().unwrap();
                if let Value::Int(channel) = channel_val {
                    let queue = vm.ether.entry(channel).or_default();
                    if queue.len() < 100 {
                        queue.push_back(value);
                        vm.energy = vm.energy.saturating_sub(1);
                        vm.output
                            .push(format!("BROADCAST: Sent to channel {}", channel));
                    } else {
                        vm.output
                            .push(format!("BROADCAST: Channel {} full", channel));
                    }
                } else {
                    vm.output
                        .push("Error: Type mismatch for broadcast".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for broadcast".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Tune => {
            // stack: channel
            if let Some(val) = vm.stack.pop() {
                if let Value::Int(channel) = val {
                    let mut value = Value::Int(0);
                    if let Some(queue) = vm.ether.get_mut(&channel) {
                        if let Some(v) = queue.pop_front() {
                            value = v;
                            vm.output
                                .push(format!("TUNE: Received from channel {}", channel));
                        }
                    }
                    vm.stack.push(value);
                    vm.energy = vm.energy.saturating_sub(1);
                } else {
                    vm.output.push("Error: Type mismatch for tune".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for tune".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Isomerize => {
            vm.chirality = match vm.chirality {
                crate::vm::Chirality::Left => crate::vm::Chirality::Right,
                crate::vm::Chirality::Right => crate::vm::Chirality::Left,
            };
            vm.output
                .push(format!("ISOMERIZE: Switched to {:?}", vm.chirality));
            None
        }
        #[cfg(feature = "nova")]
        OpCode::PhaseShift => {
            if let Some(val) = vm.stack.pop() {
                if let Value::Int(id) = val {
                    let phase = match id {
                        1 => Phase::Ethereal,
                        2 => Phase::Crystalline,
                        3 => Phase::Flux,
                        _ => Phase::Corporeal,
                    };
                    vm.phase = phase;
                    vm.energy = vm.energy.saturating_sub(50);
                    vm.output
                        .push(format!("PHASE_SHIFT: Transformed to {:?}", phase));
                } else {
                    vm.output
                        .push("Error: Type mismatch for phase_shift".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for phase_shift".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Membrane => {
            // stack: mask (1=N, 2=S, 4=E, 8=W)
            if let Some(val) = vm.stack.pop() {
                if let Value::Int(mask_val) = val {
                    let mask = mask_val as u8;
                    let (cy, cx) = vm.context_loc;

                    // Toggle bits
                    vm.membranes[cy][cx] ^= mask;

                    // Handle reciprocity
                    // North
                    if (mask & 1) != 0 {
                        if let Some((ny, nx)) = vm.normalize_coords(cy as i64 - 1, cx as i64) {
                            vm.membranes[ny][nx] ^= 2;
                        }
                    }
                    // South
                    if (mask & 2) != 0 {
                        if let Some((ny, nx)) = vm.normalize_coords(cy as i64 + 1, cx as i64) {
                            vm.membranes[ny][nx] ^= 1;
                        }
                    }
                    // East
                    if (mask & 4) != 0 {
                        if let Some((ny, nx)) = vm.normalize_coords(cy as i64, cx as i64 + 1) {
                            vm.membranes[ny][nx] ^= 8;
                        }
                    }
                    // West
                    if (mask & 8) != 0 {
                        if let Some((ny, nx)) = vm.normalize_coords(cy as i64, cx as i64 - 1) {
                            vm.membranes[ny][nx] ^= 4;
                        }
                    }

                    vm.energy = vm.energy.saturating_sub(10);
                    vm.output
                        .push(format!("MEMBRANE: Toggled mask {} at {},{}", mask, cx, cy));
                } else {
                    vm.output
                        .push("Error: Type mismatch for membrane".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for membrane".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Osmosis => {
            // stack: dy, dx (top)
            if vm.stack.len() >= 2 {
                let dx_val = vm.stack.pop().unwrap();
                let dy_val = vm.stack.pop().unwrap();
                if let (Value::Int(dy), Value::Int(dx)) = (dy_val, dx_val) {
                    let (cy, cx) = vm.context_loc;
                    if let Some((mut new_y, mut new_x)) =
                        vm.normalize_coords(cy as i64 + dy, cx as i64 + dx)
                    {
                        // Check for portal
                        if let Some(&(py, px)) = vm.portals.get(&(new_y, new_x)) {
                            vm.output.push(format!(
                                "PORTAL: Teleported from {},{} to {},{}",
                                new_x, new_y, px, py
                            ));
                            new_y = py;
                            new_x = px;
                        }

                        vm.context_loc = (new_y, new_x);
                        vm.energy = vm.energy.saturating_sub(20); // High cost
                        vm.output
                            .push(format!("OSMOSIS: Moved to {},{}", new_x, new_y));
                    } else {
                        vm.energy = vm.energy.saturating_sub(5);
                        vm.output.push("OSMOSIS: Blocked by boundary".to_string());
                    }
                } else {
                    vm.output
                        .push("Error: Type mismatch for osmosis".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for osmosis".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Symbiosis => {
            // stack: dy, dx (top)
            if vm.stack.len() >= 2 {
                let dx_val = vm.stack.pop().unwrap();
                let dy_val = vm.stack.pop().unwrap();
                if let (Value::Int(dy), Value::Int(dx)) = (dy_val, dx_val) {
                    let (cy, cx) = vm.context_loc;
                    if let Some((ny, nx)) = vm.normalize_coords(cy as i64 + dy, cx as i64 + dx) {
                        // Find organelle at (ny, nx)
                        let mut found_idx = None;
                        for (i, org) in vm.organelles.iter().enumerate() {
                            if org.context_loc == (ny, nx) {
                                found_idx = Some(i);
                                break;
                            }
                        }

                        if let Some(idx) = found_idx {
                            let organelle = vm.organelles.remove(idx);

                            // Absorb IP
                            vm.symbiotes.push(organelle.ip);

                            // Absorb Stack
                            vm.stack.extend(organelle.stack);

                            vm.energy = vm.energy.saturating_sub(20);
                            vm.output
                                .push(format!("SYMBIOSIS: Absorbed organelle at {},{}", nx, ny));
                        } else {
                            vm.output.push("SYMBIOSIS: No organelle found".to_string());
                        }
                    } else {
                        vm.output
                            .push("Error: Coordinates out of bounds for symbiosis".to_string());
                    }
                } else {
                    vm.output
                        .push("Error: Type mismatch for symbiosis".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for symbiosis".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Reflex => {
            // stack: event_id, strand_idx (bottom)
            if vm.stack.len() >= 2 {
                let event_val = vm.stack.pop().unwrap();
                let strand_val = vm.stack.pop().unwrap();

                if let (Value::Int(s), Value::Int(e)) = (strand_val, event_val) {
                    let s_idx = s as usize;
                    if s_idx < vm.dna.helix.strands.len() {
                        vm.reflexes.insert(e, s_idx);
                        vm.output
                            .push(format!("REFLEX: Bound event {} to strand {}", e, s_idx));
                    } else {
                        vm.output
                            .push("Error: Strand index out of bounds for reflex".to_string());
                    }
                } else {
                    vm.output
                        .push("Error: Type mismatch for reflex".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for reflex".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Lysis => {
            // Eject the last symbiote
            if let Some(sip) = vm.symbiotes.pop() {
                if vm.organelles.len() >= crate::vm::MAX_ORGANELLES {
                    vm.symbiotes.push(sip); // Put it back
                    vm.output
                        .push("Error: Organelle limit exceeded".to_string());
                    return None;
                }

                let (cy, cx) = vm.context_loc;

                // Create organelle
                let organelle = Organelle {
                    stack: Vec::new(),
                    ip: sip,
                    context_loc: (cy, cx),
                    call_stack: Vec::new(),
                    recursion_depth: 0,
                    halted: false,
                    kind: OrganelleType::Worker, // Default
                    direction: (0, 0),
                    ttl: None,
                };
                vm.organelles.push(organelle);
                vm.energy = vm.energy.saturating_sub(10);
                vm.output
                    .push(format!("LYSIS: Ejected symbiote to {},{}", cx, cy));
            } else {
                vm.output.push("LYSIS: No symbiotes to eject".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Compile => {
            if let Some(val) = vm.stack.pop() {
                if let Value::Str(s) = val {
                    match ChimeraParser::parse(Rule::strand, &s) {
                        Ok(mut pairs) => {
                            let pair = pairs.next().unwrap();
                            match crate::ast::Strand::try_from_pair(pair) {
                                Ok(strand) => {
                                    vm.dna.helix.strands.push(strand);
                                    vm.telomeres.push(50);
                                    #[cfg(feature = "cortex")]
                                    {
                                        vm.activation_levels.push(0);
                                        vm.synapse_map.push(Vec::new());
                                    }
                                    vm.stack
                                        .push(Value::Int((vm.dna.helix.strands.len() - 1) as i64));
                                    vm.energy = vm.energy.saturating_sub(50);
                                    vm.output.push("COMPILE: Success".to_string());
                                }
                                Err(e) => {
                                    vm.output.push(format!("COMPILE ERROR: {}", e));
                                }
                            }
                        }
                        Err(e) => {
                            vm.output.push(format!("COMPILE ERROR: {}", e));
                        }
                    }
                } else {
                    vm.output
                        .push("Error: Type mismatch for compile".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for compile".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Irradiate => {
            // stack: amount, radius (top)
            if vm.stack.len() >= 2 {
                let radius_val = vm.stack.pop().unwrap();
                let amount_val = vm.stack.pop().unwrap();
                if let (Value::Int(r), Value::Int(amount)) = (radius_val, amount_val) {
                    if r > 0 && amount > 0 {
                        let (cy, cx) = vm.context_loc;
                        let coords = vm.get_circular_coords(cx as i64, cy as i64, r);
                        for (tx, ty) in coords {
                            vm.mutagen_grid[ty][tx] =
                                vm.mutagen_grid[ty][tx].saturating_add(amount);
                        }
                        // Cost is proportional to amount and area
                        vm.energy = vm
                            .energy
                            .saturating_sub((r * r + 1).clamp(5, 50) + amount / 10);
                        vm.output.push(format!(
                            "IRRADIATE: Added {} mutagen at {},{} r={}",
                            amount, cx, cy, r
                        ));
                    }
                } else {
                    vm.output
                        .push("Error: Type mismatch for irradiate".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for irradiate".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::SenseMutagen => {
            let (cy, cx) = vm.context_loc;
            let level = vm.mutagen_grid[cy][cx];
            vm.stack.push(Value::Int(level));
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Devour => {
            let (cy, cx) = vm.context_loc;
            let level = vm.mutagen_grid[cy][cx];
            if level > 0 {
                vm.mutagen_grid[cy][cx] = 0;
                let energy_gain = level / 2; // 50% efficiency
                vm.energy = vm.energy.saturating_add(energy_gain);
                vm.stack.push(Value::Int(energy_gain));
                vm.output.push(format!(
                    "DEVOUR: Consumed {} mutagen, gained {} energy",
                    level, energy_gain
                ));
            } else {
                vm.stack.push(Value::Int(0));
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Decompile => {
            if let Some(val) = vm.stack.pop() {
                if let Value::Int(idx) = val {
                    let s_idx = idx as usize;
                    if s_idx < vm.dna.helix.strands.len() {
                        let strand = &vm.dna.helix.strands[s_idx];

                        fn format_nucleotide(n: &crate::ast::Nucleotide, depth: usize) -> String {
                            if depth > crate::vm::MAX_RECURSION_DEPTH {
                                return "...".to_string();
                            }
                            match n {
                                crate::ast::Nucleotide::Number(i) => i.to_string(),
                                crate::ast::Nucleotide::String(s) => format!("\"{}\"", s),
                                crate::ast::Nucleotide::Identifier(s) => s.clone(),
                                crate::ast::Nucleotide::Junction(t, args) => {
                                    let t_str = match t {
                                        crate::ast::JunctionType::Any => "any",
                                        crate::ast::JunctionType::All => "all",
                                    };
                                    let args_str: Vec<String> = args
                                        .iter()
                                        .map(|arg| format_nucleotide(arg, depth + 1))
                                        .collect();
                                    format!("{}({})", t_str, args_str.join(" "))
                                }
                            }
                        }

                        let mut s = String::from("[ ");
                        for gene in &strand.genes {
                            s.push_str(gene.op.as_ref());
                            s.push('(');
                            for (i, arg) in gene.args.iter().enumerate() {
                                if i > 0 {
                                    s.push(' ');
                                }
                                s.push_str(&format_nucleotide(arg, 0));
                            }
                            s.push_str(") ");
                        }
                        s.push(']');
                        vm.stack.push(Value::Str(s));
                        vm.energy = vm.energy.saturating_sub(10);
                    } else {
                        vm.output
                            .push("Error: Strand index out of bounds for decompile".to_string());
                    }
                } else {
                    vm.output
                        .push("Error: Type mismatch for decompile".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for decompile".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Void => {
            if vm.organelles.len() >= crate::vm::MAX_ORGANELLES {
                vm.output
                    .push("Error: Organelle limit exceeded".to_string());
                return None;
            }

            let (cy, cx) = vm.context_loc;
            let organelle = Organelle {
                stack: Vec::new(),
                ip: (0, 0), // Void has no IP
                context_loc: (cy, cx),
                call_stack: Vec::new(),
                recursion_depth: 0,
                halted: false,
                kind: OrganelleType::Void,
                direction: (0, 0),
                ttl: None,
            };
            vm.organelles.push(organelle);
            vm.energy = vm.energy.saturating_sub(50);
            vm.output.push(format!("VOID: Spawned at {},{}", cx, cy));
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Supernova => {
            let s_idx = vm.ip.0;
            if s_idx < vm.dna.helix.strands.len() {
                // Backup to graveyard
                let strand = vm.dna.helix.strands[s_idx].clone();
                vm.graveyard.push(strand);

                // Remove genes from strand
                let genes = std::mem::take(&mut vm.dna.helix.strands[s_idx].genes);
                let mut rng = rand::thread_rng();

                let rows = vm.grid.len();
                let cols = if rows > 0 { vm.grid[0].len() } else { 0 };

                if rows > 0 && cols > 0 {
                    // Helper to format gene with args
                    fn format_nucleotide(n: &Nucleotide, depth: usize) -> String {
                        if depth > crate::vm::MAX_RECURSION_DEPTH {
                            return "...".to_string();
                        }
                        match n {
                            Nucleotide::Number(i) => i.to_string(),
                            Nucleotide::String(s) => format!("\"{}\"", s),
                            Nucleotide::Identifier(s) => s.clone(),
                            Nucleotide::Junction(t, args) => {
                                let t_str = match t {
                                    crate::ast::JunctionType::Any => "any",
                                    crate::ast::JunctionType::All => "all",
                                };
                                let args_str: Vec<String> = args
                                    .iter()
                                    .map(|arg| format_nucleotide(arg, depth + 1))
                                    .collect();
                                format!("{}({})", t_str, args_str.join(" "))
                            }
                        }
                    }

                    for gene in genes {
                        let mut s = gene.op.as_ref().to_string();
                        if !gene.args.is_empty() {
                            s.push('(');
                            for (i, arg) in gene.args.iter().enumerate() {
                                if i > 0 {
                                    s.push(' ');
                                }
                                s.push_str(&format_nucleotide(arg, 0));
                            }
                            s.push(')');
                        }

                        // Scatter gene to random grid location
                        let rx = rng.gen_range(0..cols);
                        let ry = rng.gen_range(0..rows);
                        vm.grid[ry][rx] = Value::Str(s);
                    }
                }

                vm.output
                    .push(format!("SUPERNOVA: Strand {} exploded", s_idx));
                vm.halted = true; // Suicide
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Singularity => {
            let mut merged_genes = Vec::new();
            // Take all strands
            let strands = std::mem::take(&mut vm.dna.helix.strands);
            for mut strand in strands {
                merged_genes.append(&mut strand.genes);
            }

            // Create single new strand
            vm.dna.helix.strands.push(crate::ast::Strand {
                genes: merged_genes,
            });

            // Reset state that depends on strand indices
            vm.telomeres = vec![100]; // Reset telomeres for the new massive strand
            #[cfg(feature = "cortex")]
            {
                vm.activation_levels = vec![0];
                vm.synapse_map = vec![Vec::new()];
            }
            // Clear other specific mappings?
            vm.epigenome.clear();
            // We should ideally remap epigenome, but Singularity is destructive/transformative.
            vm.market.clear();

            // Reset IP to start of new strand
            vm.ip = (0, 0);

            vm.output
                .push("SINGULARITY: All strands merged".to_string());
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Eval => {
            if let Some(val) = vm.stack.pop() {
                if let Value::Str(s) = val {
                    // Parse string to strand
                    match ChimeraParser::parse(Rule::strand, &s) {
                        Ok(mut pairs) => {
                            let pair = pairs.next().unwrap();
                            match crate::ast::Strand::try_from_pair(pair) {
                                Ok(strand) => {
                                    execute_ephemeral_strand(vm, &strand);
                                    vm.output.push("EVAL: Success".to_string());
                                }
                                Err(e) => {
                                    vm.output.push(format!("EVAL ERROR: {}", e));
                                }
                            }
                        }
                        Err(e) => {
                            vm.output.push(format!("EVAL ERROR: {}", e));
                        }
                    }
                } else {
                    vm.output.push("Error: Type mismatch for eval".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for eval".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Map => {
            // stack: junction, function (top)
            if vm.stack.len() >= 2 {
                let func_val = vm.stack.pop().unwrap();
                let target_val = vm.stack.pop().unwrap();

                let inputs = match target_val {
                    Value::Junction(_, vals) => vals,
                    scalar => vec![scalar],
                };

                let mut results = Vec::new();

                for input in inputs {
                    let stack_depth = vm.stack.len();
                    vm.stack.push(input);

                    match &func_val {
                        Value::Str(s) => {
                            // Parse and eval
                            if let Ok(mut pairs) = ChimeraParser::parse(Rule::strand, s) {
                                let pair = pairs.next().unwrap();
                                match crate::ast::Strand::try_from_pair(pair) {
                                    Ok(strand) => execute_ephemeral_strand(vm, &strand),
                                    Err(e) => vm.output.push(format!("MAP ERROR: {}", e)),
                                }
                            } else {
                                vm.output.push(format!("MAP ERROR: Parse failed for {}", s));
                            }
                        }
                        Value::Int(idx) => {
                            // Sync call strand
                            execute_strand_sync(vm, *idx as usize);
                        }
                        _ => {
                            vm.output
                                .push("Error: Invalid function for map".to_string());
                        }
                    }

                    if vm.stack.len() > stack_depth {
                        let new_items = vm.stack.split_off(stack_depth);
                        if results.len() + new_items.len() > crate::vm::MAX_JUNCTION_SIZE {
                            vm.output
                                .push("Error: Junction size limit exceeded in Map".to_string());
                            return None;
                        }
                        results.extend(new_items);
                    }
                }

                vm.stack
                    .push(Value::Junction(crate::ast::JunctionType::Any, results));
                vm.energy = vm.energy.saturating_sub(10);
            } else {
                vm.output.push("Error: Stack underflow for map".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Fold => {
            // stack: junction, init, function (top)
            if vm.stack.len() >= 3 {
                let func_val = vm.stack.pop().unwrap();
                let init_val = vm.stack.pop().unwrap();
                let target_val = vm.stack.pop().unwrap();

                let inputs = match target_val {
                    Value::Junction(_, vals) => vals,
                    scalar => vec![scalar],
                };

                let mut acc = init_val;

                for input in inputs {
                    vm.stack.push(acc.clone());
                    vm.stack.push(input);

                    match &func_val {
                        Value::Str(s) => {
                            if let Ok(mut pairs) = ChimeraParser::parse(Rule::strand, s) {
                                let pair = pairs.next().unwrap();
                                if let Ok(strand) = crate::ast::Strand::try_from_pair(pair) {
                                    execute_ephemeral_strand(vm, &strand);
                                }
                            }
                        }
                        Value::Int(idx) => {
                            execute_strand_sync(vm, *idx as usize);
                        }
                        _ => {}
                    }

                    if let Some(res) = vm.stack.pop() {
                        acc = res;
                    }
                    // Discard extra items if any
                }

                vm.stack.push(acc);
                vm.energy = vm.energy.saturating_sub(10);
            } else {
                vm.output
                    .push("Error: Stack underflow for fold".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Filter => {
            // stack: junction, predicate (top)
            if vm.stack.len() >= 2 {
                let func_val = vm.stack.pop().unwrap();
                let target_val = vm.stack.pop().unwrap();

                let inputs = match target_val {
                    Value::Junction(_, vals) => vals,
                    scalar => vec![scalar],
                };

                let mut results = Vec::new();

                for input in inputs {
                    vm.stack.push(input.clone());

                    match &func_val {
                        Value::Str(s) => {
                            if let Ok(mut pairs) = ChimeraParser::parse(Rule::strand, s) {
                                let pair = pairs.next().unwrap();
                                if let Ok(strand) = crate::ast::Strand::try_from_pair(pair) {
                                    execute_ephemeral_strand(vm, &strand);
                                }
                            }
                        }
                        Value::Int(idx) => {
                            execute_strand_sync(vm, *idx as usize);
                        }
                        _ => {}
                    }

                    if let Some(res) = vm.stack.pop() {
                        let keep = match res {
                            Value::Int(i) => i != 0,
                            Value::Str(s) => !s.is_empty(),
                            _ => false,
                        };
                        if keep {
                            if results.len() >= crate::vm::MAX_JUNCTION_SIZE {
                                vm.output.push(
                                    "Error: Junction size limit exceeded in Filter".to_string(),
                                );
                                return None;
                            }
                            results.push(input);
                        }
                    }
                }

                vm.stack
                    .push(Value::Junction(crate::ast::JunctionType::Any, results));
                vm.energy = vm.energy.saturating_sub(10);
            } else {
                vm.output
                    .push("Error: Stack underflow for filter".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Zip => {
            // stack: junction_a, junction_b (top)
            if vm.stack.len() >= 2 {
                let val_b = vm.stack.pop().unwrap();
                let val_a = vm.stack.pop().unwrap();

                let inputs_a = match val_a {
                    Value::Junction(_, vals) => vals,
                    scalar => vec![scalar],
                };
                let inputs_b = match val_b {
                    Value::Junction(_, vals) => vals,
                    scalar => vec![scalar],
                };

                let len = inputs_a.len().min(inputs_b.len());
                if len > crate::vm::MAX_JUNCTION_SIZE {
                    vm.output
                        .push("Error: Junction size limit exceeded in Zip".to_string());
                    return None;
                }

                let mut results = Vec::new();

                for i in 0..len {
                    results.push(Value::Junction(
                        crate::ast::JunctionType::All,
                        vec![inputs_a[i].clone(), inputs_b[i].clone()],
                    ));
                }

                vm.stack
                    .push(Value::Junction(crate::ast::JunctionType::Any, results));
                vm.energy = vm.energy.saturating_sub(5);
            } else {
                vm.output.push("Error: Stack underflow for zip".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Pigment => {
            // stack: r, g, b, y, x (top)
            if vm.stack.len() >= 5 {
                let x_val = vm.stack.pop().unwrap();
                let y_val = vm.stack.pop().unwrap();
                let b_val = vm.stack.pop().unwrap();
                let g_val = vm.stack.pop().unwrap();
                let r_val = vm.stack.pop().unwrap();

                if let (Value::Int(x), Value::Int(y), Value::Int(r), Value::Int(g), Value::Int(b)) =
                    (x_val, y_val, r_val, g_val, b_val)
                {
                    if let Some((ny, nx)) = vm.normalize_coords(y, x) {
                        if r < 0 || g < 0 || b < 0 {
                            vm.chroma_grid[ny][nx].fg = None;
                            vm.output
                                .push(format!("PIGMENT: Cleared color at {},{}", nx, ny));
                        } else {
                            let rc = r.clamp(0, 255) as u8;
                            let gc = g.clamp(0, 255) as u8;
                            let bc = b.clamp(0, 255) as u8;
                            vm.chroma_grid[ny][nx].fg = Some((rc, gc, bc));
                        }
                        vm.energy = vm.energy.saturating_sub(2);
                    } else {
                        vm.output
                            .push("Error: Coordinates out of bounds for pigment".to_string());
                    }
                } else {
                    vm.output
                        .push("Error: Type mismatch for pigment".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for pigment".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Glyph => {
            // stack: char_code, y, x (top)
            if vm.stack.len() >= 3 {
                let x_val = vm.stack.pop().unwrap();
                let y_val = vm.stack.pop().unwrap();
                let c_val = vm.stack.pop().unwrap();

                if let (Value::Int(x), Value::Int(y), Value::Int(c)) = (x_val, y_val, c_val) {
                    if let Some((ny, nx)) = vm.normalize_coords(y, x) {
                        if c < 0 {
                            vm.chroma_grid[ny][nx].char = None;
                            vm.output
                                .push(format!("GLYPH: Cleared char at {},{}", nx, ny));
                        } else {
                            let ch = (c as u8) as char;
                            vm.chroma_grid[ny][nx].char = Some(ch);
                        }
                        vm.energy = vm.energy.saturating_sub(2);
                    } else {
                        vm.output
                            .push("Error: Coordinates out of bounds for glyph".to_string());
                    }
                } else {
                    vm.output.push("Error: Type mismatch for glyph".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for glyph".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Evolve => {
            let mut birth_rules = vec![3];
            let mut survival_rules = vec![2, 3];
            let mut used_custom_rule = false;

            // Check if top of stack is a rule string
            if let Some(Value::Str(s)) = vm.stack.last() {
                if let Some((b, s)) = parse_life_rule(s) {
                    birth_rules = b;
                    survival_rules = s;
                    used_custom_rule = true;
                }
            }

            if used_custom_rule {
                vm.stack.pop(); // Consume the rule string
                vm.output.push(format!(
                    "EVOLVE: Using rule B{:?}/S{:?}",
                    birth_rules, survival_rules
                ));
            } else {
                vm.output
                    .push("EVOLVE: Using default rule B3/S23".to_string());
            }

            // Cellular Automata (Game of Life variant)
            let rows = vm.grid.len();
            let cols = if rows > 0 { vm.grid[0].len() } else { 0 };
            let mut next_grid = vm.grid.clone();

            for y in 0..rows {
                for x in 0..cols {
                    // Count neighbors
                    let mut neighbors = 0;
                    for dy in -1..=1 {
                        for dx in -1..=1 {
                            if dy == 0 && dx == 0 {
                                continue;
                            }
                            if let Some((ny, nx)) =
                                vm.normalize_coords(y as i64 + dy, x as i64 + dx)
                            {
                                if !matches!(vm.grid[ny][nx], Value::Int(0)) {
                                    neighbors += 1;
                                }
                            }
                        }
                    }

                    let is_alive = !matches!(vm.grid[y][x], Value::Int(0));

                    if !is_alive && birth_rules.contains(&neighbors) {
                        // Birth: Becomes 1
                        next_grid[y][x] = Value::Int(1);
                    } else if is_alive && !survival_rules.contains(&neighbors) {
                        // Death (Overpopulation or Underpopulation)
                        next_grid[y][x] = Value::Int(0);
                    }
                    // Else survive (keep value)
                }
            }
            vm.grid = next_grid;
            vm.energy = vm.energy.saturating_sub(20);
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Glitch => {
            if let Some(val) = vm.stack.pop() {
                if let Value::Int(severity) = val {
                    let mut rng = rand::thread_rng();
                    let sev = severity.clamp(1, 100) as usize;

                    // Corrupt grid
                    let rows = vm.grid.len();
                    if rows > 0 {
                        let cols = vm.grid[0].len();
                        for _ in 0..sev {
                            let rx = rng.gen_range(0..cols);
                            let ry = rng.gen_range(0..rows);
                            if rng.gen_bool(0.5) {
                                vm.grid[ry][rx] = Value::Int(rng.gen_range(0..10));
                            } else {
                                vm.grid[ry][rx] = Value::Int(0);
                            }
                        }
                    }

                    // Corrupt stack
                    if !vm.stack.is_empty() {
                        let changes = sev.min(vm.stack.len());
                        for _ in 0..changes {
                            let idx = rng.gen_range(0..vm.stack.len());
                            if rng.gen_bool(0.3) {
                                // Mutate value
                                vm.stack[idx] = Value::Int(rng.gen_range(0..100));
                            }
                        }
                    }

                    vm.energy = vm.energy.saturating_sub(severity);
                    vm.output.push(format!("GLITCH: Severity {}", severity));
                } else {
                    vm.output
                        .push("Error: Type mismatch for glitch".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for glitch".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Scramble => {
            let mut rng = rand::thread_rng();
            vm.stack.shuffle(&mut rng);
            vm.energy = vm.energy.saturating_sub(10);
            vm.output.push("SCRAMBLE: Stack shuffled".to_string());
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Hyphae => {
            let (cy, cx) = vm.context_loc;
            if let std::collections::hash_map::Entry::Vacant(e) = vm.mycelium.entry((cy, cx)) {
                e.insert(Vec::new());
                vm.energy = vm.energy.saturating_sub(20);
                vm.output.push(format!("HYPHAE: Sprouted at {},{}", cx, cy));
            } else {
                vm.output
                    .push(format!("HYPHAE: Node already exists at {},{}", cx, cy));
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Connect => {
            // stack: y, x (top)
            if vm.stack.len() >= 2 {
                let x_val = vm.stack.pop().unwrap();
                let y_val = vm.stack.pop().unwrap();
                if let (Value::Int(x), Value::Int(y)) = (x_val, y_val) {
                    if let Some((ty, tx)) = vm.normalize_coords(y, x) {
                        let (cy, cx) = vm.context_loc;
                        if vm.mycelium.contains_key(&(cy, cx))
                            && vm.mycelium.contains_key(&(ty, tx))
                        {
                            // Add undirected edge
                            vm.mycelium.get_mut(&(cy, cx)).unwrap().push((ty, tx));
                            vm.mycelium.get_mut(&(ty, tx)).unwrap().push((cy, cx));
                            vm.energy = vm.energy.saturating_sub(10);
                            vm.output.push(format!(
                                "CONNECT: Mycelium linked {},{} <-> {},{}",
                                cx, cy, tx, ty
                            ));
                        } else {
                            vm.output
                                .push("CONNECT: Both ends must be Hyphae".to_string());
                        }
                    } else {
                        vm.output
                            .push("Error: Coordinates out of bounds for connect".to_string());
                    }
                } else {
                    vm.output
                        .push("Error: Type mismatch for connect".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for connect".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Transport => {
            // stack: val, y, x (top)
            if vm.stack.len() >= 3 {
                let x_val = vm.stack.pop().unwrap();
                let y_val = vm.stack.pop().unwrap();
                let val = vm.stack.pop().unwrap();

                if let (Value::Int(x), Value::Int(y)) = (x_val, y_val) {
                    if let Some((ty, tx)) = vm.normalize_coords(y, x) {
                        let (cy, cx) = vm.context_loc;

                        // BFS to find path
                        let mut queue = VecDeque::new();
                        let mut visited = HashSet::new();
                        queue.push_back((cy, cx));
                        visited.insert((cy, cx));

                        let mut found = false;
                        while let Some(curr) = queue.pop_front() {
                            if curr == (ty, tx) {
                                found = true;
                                break;
                            }
                            if let Some(neighbors) = vm.mycelium.get(&curr) {
                                for &next in neighbors {
                                    if !visited.contains(&next) {
                                        visited.insert(next);
                                        queue.push_back(next);
                                    }
                                }
                            }
                        }

                        if found {
                            vm.grid[ty][tx] = val;
                            vm.energy = vm.energy.saturating_sub(5);
                            vm.output
                                .push(format!("TRANSPORT: Sent value to {},{}", tx, ty));
                        } else {
                            vm.output
                                .push("TRANSPORT: No mycelial path found".to_string());
                        }
                    } else {
                        vm.output
                            .push("Error: Coordinates out of bounds for transport".to_string());
                    }
                } else {
                    vm.output
                        .push("Error: Type mismatch for transport".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for transport".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::SporeCloud => {
            // stack: radius, density (top)
            if vm.stack.len() >= 2 {
                let dens_val = vm.stack.pop().unwrap();
                let rad_val = vm.stack.pop().unwrap();
                if let (Value::Int(r), Value::Int(d)) = (rad_val, dens_val) {
                    let (cy, cx) = vm.context_loc;
                    let mut rng = rand::thread_rng();
                    let coords = vm.get_circular_coords(cx as i64, cy as i64, r);

                    let mut count = 0;
                    for (tx, ty) in coords {
                        if rng.gen_range(0..100) < d {
                            if let std::collections::hash_map::Entry::Vacant(e) =
                                vm.mycelium.entry((ty, tx))
                            {
                                e.insert(Vec::new());
                                count += 1;
                            }
                        }
                    }
                    vm.energy = vm.energy.saturating_sub(count * 5);
                    vm.output
                        .push(format!("SPORE_CLOUD: Sprouted {} hyphae", count));
                } else {
                    vm.output
                        .push("Error: Type mismatch for spore_cloud".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for spore_cloud".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Signal => {
            super::ipc::signal(vm);
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Receive => {
            super::ipc::receive(vm);
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Spirit => {
            if let Some(val) = vm.stack.pop() {
                if let Value::Str(msg) = val {
                    vm.spirit_message = Some(msg);
                } else {
                    // Put it back if not a string (optional prompt)
                    vm.stack.push(val);
                    vm.spirit_message = None;
                }
            } else {
                vm.spirit_message = None;
            }
            vm.spirit_request = true;
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Match => {
            // stack: pattern, target (top)
            if vm.stack.len() >= 2 {
                let target_val = vm.stack.pop().unwrap();
                let pattern_val = vm.stack.pop().unwrap();

                if let (Value::Str(p), Value::Str(t)) = (pattern_val, target_val) {
                    let is_match = glob_match(&p, &t);
                    vm.stack.push(Value::Int(if is_match { 1 } else { 0 }));
                    vm.energy = vm.energy.saturating_sub(5);
                } else {
                    vm.output.push("Error: Type mismatch for match".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for match".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Bury => {
            // stack: strand_idx
            if let Some(val) = vm.stack.pop() {
                match val {
                    Value::Int(idx) => {
                        let s_idx = idx as usize;
                        if s_idx < vm.dna.helix.strands.len() {
                            // Copy to graveyard
                            let strand = vm.dna.helix.strands[s_idx].clone();
                            vm.graveyard.push(strand);

                            // Clear original
                            vm.dna.helix.strands[s_idx].genes.clear();
                            // Remove associated epigenetics
                            vm.epigenome.retain(|(s, _)| *s != s_idx);

                            vm.energy = vm.energy.saturating_sub(10);
                            vm.output
                                .push(format!("BURY: Buried strand {} in graveyard", s_idx));
                        } else {
                            vm.output
                                .push("Error: Strand index out of bounds for bury".to_string());
                        }
                    }
                    _ => vm.output.push("Error: Type mismatch for bury".to_string()),
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for bury".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Exhume => {
            if let Some(strand) = vm.graveyard.pop() {
                vm.dna.helix.strands.push(strand);
                vm.telomeres.push(50);
                #[cfg(feature = "cortex")]
                {
                    vm.activation_levels.push(0);
                    vm.synapse_map.push(Vec::new());
                }

                let new_idx = vm.dna.helix.strands.len() - 1;
                vm.stack.push(Value::Int(new_idx as i64));
                vm.energy = vm.energy.saturating_sub(25);
                vm.output
                    .push(format!("EXHUME: Resurrected strand as {}", new_idx));
            } else {
                vm.stack.push(Value::Int(-1));
                vm.output.push("EXHUME: Graveyard empty".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Seance => {
            if let Some(strand) = vm.graveyard.last() {
                let ghost_strand = strand.clone();
                execute_ephemeral_strand(vm, &ghost_strand);
                vm.energy = vm.energy.saturating_sub(15);
                vm.output.push("SEANCE: Communed with the dead".to_string());
            } else {
                vm.output.push("SEANCE: Graveyard empty".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Mourn => {
            let count = vm.graveyard.len() as i64;
            let energy_gain = count * 2;
            vm.energy = vm.energy.saturating_add(energy_gain);
            vm.stack.push(Value::Int(energy_gain));
            vm.output.push(format!(
                "MOURN: Gained {} energy from {} ancestors",
                energy_gain, count
            ));
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Reincarnate => {
            // stack: strand_idx
            if let Some(val) = vm.stack.pop() {
                match val {
                    Value::Int(idx) => {
                        let s_idx = idx as usize;
                        if s_idx < vm.dna.helix.strands.len() {
                            // 1. Clone strand
                            let mut new_strand = vm.dna.helix.strands[s_idx].clone();
                            let mut rng = rand::thread_rng();

                            // 2. Drift Mutations
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
                                OpCode::GRead,
                                OpCode::GWrite,
                                OpCode::Genome,
                                OpCode::Meme,
                                OpCode::Poly,
                            ];

                            // Higher mutation rate for reincarnation (e.g. 10%)
                            for gene in &mut new_strand.genes {
                                if rng.gen_bool(0.1) {
                                    let new_op = enzymes[rng.gen_range(0..enzymes.len())].clone();
                                    gene.op = new_op;
                                }
                            }

                            // 3. Push new strand
                            vm.dna.helix.strands.push(new_strand);
                            let new_idx = vm.dna.helix.strands.len() - 1;
                            vm.telomeres.push(50);
                            #[cfg(feature = "cortex")]
                            {
                                vm.activation_levels.push(0);
                                vm.synapse_map.push(Vec::new());
                            }

                            // 4. Apoptosis of old strand
                            // Clear original
                            vm.dna.helix.strands[s_idx].genes.clear();
                            // Remove associated epigenetics
                            vm.epigenome.retain(|(s, _)| *s != s_idx);

                            // 5. Push new index
                            vm.stack.push(Value::Int(new_idx as i64));

                            vm.energy = vm.energy.saturating_sub(50);
                            vm.output.push(format!(
                                "REINCARNATE: Strand {} reborn as {}",
                                s_idx, new_idx
                            ));
                        } else {
                            vm.output.push(
                                "Error: Strand index out of bounds for reincarnate".to_string(),
                            );
                        }
                    }
                    _ => vm
                        .output
                        .push("Error: Type mismatch for reincarnate".to_string()),
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for reincarnate".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Superpose => {
            if vm.stack.len() >= 2 {
                let b = vm.stack.pop().unwrap();
                let a = vm.stack.pop().unwrap();
                // 50/50 split
                vm.stack
                    .push(Value::Superposition(vec![(a, 0.5), (b, 0.5)]));
                vm.energy = vm.energy.saturating_sub(10);
            } else {
                vm.output
                    .push("Error: Stack underflow for superpose".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Collapse => {
            if let Some(val) = vm.stack.pop() {
                match val {
                    Value::Superposition(states) => {
                        let mut rng = rand::thread_rng();
                        let r: f64 = rng.gen();
                        let mut sum = 0.0;
                        let mut collapsed = states[0].0.clone(); // Default

                        for (v, p) in states {
                            sum += p;
                            if r <= sum {
                                collapsed = v;
                                break;
                            }
                        }
                        vm.stack.push(collapsed);
                        vm.energy = vm.energy.saturating_sub(5);
                    }
                    other => {
                        // Scalar stays scalar
                        vm.stack.push(other);
                    }
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for collapse".to_string());
            }
            None
        }
        #[cfg(feature = "nova")]
        OpCode::Observe => {
            if let Some(val) = vm.stack.pop() {
                match val {
                    Value::Superposition(states) => {
                        let mut rng = rand::thread_rng();
                        let r: f64 = rng.gen();
                        let mut sum = 0.0;
                        let mut collapsed = states[0].0.clone();

                        for (v, p) in states {
                            sum += p;
                            if r <= sum {
                                collapsed = v;
                                break;
                            }
                        }
                        vm.stack.push(collapsed.clone());
                        vm.energy = vm.energy.saturating_sub(5);
                        vm.output.push(format!("OBSERVED: {}", collapsed));
                    }
                    other => {
                        vm.stack.push(other.clone());
                        vm.output.push(format!("OBSERVED: {}", other));
                    }
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for observe".to_string());
            }
            None
        }
        _ => None,
    }
}

#[cfg(feature = "nova")]
fn parse_life_rule(rule: &str) -> Option<(Vec<u8>, Vec<u8>)> {
    let parts: Vec<&str> = rule.split('/').collect();
    if parts.len() != 2 {
        return None;
    }

    let parse_part = |s: &str, prefix: char| -> Vec<u8> {
        let s = s.trim();
        let nums = if s.starts_with(prefix) { &s[1..] } else { s };

        let mut digits = Vec::new();
        for c in nums.chars() {
            if let Some(d) = c.to_digit(10) {
                digits.push(d as u8);
            }
        }
        digits
    };

    let birth = parse_part(parts[0], 'B');
    let survival = parse_part(parts[1], 'S');

    Some((birth, survival))
}

#[cfg(feature = "nova")]
fn glob_match(pattern: &str, target: &str) -> bool {
    if let Some((p_head, p_tail)) = pattern.split_once('*') {
        if !target.starts_with(p_head) {
            return false;
        }
        let t_rest = &target[p_head.len()..];
        if p_tail.is_empty() {
            return true;
        }

        for i in 0..=t_rest.len() {
            // Optimization: if p_tail doesn't have *, we can check ends_with directly?
            // But p_tail might have *. Recursion handles it.
            // Only optimize matching char boundaries to avoid panic?
            // split_once uses byte indices but string slicing requires char boundaries.
            // split_once returns valid &str so p_head is valid.
            // t_rest slicing [i..] needs to be on char boundary.
            if t_rest.is_char_boundary(i) && glob_match(p_tail, &t_rest[i..]) {
                return true;
            }
        }
        false
    } else {
        pattern == target
    }
}

#[cfg(feature = "nova")]
fn execute_ephemeral_strand(vm: &mut ChimeraVM, strand: &crate::ast::Strand) {
    if vm.recursion_depth > crate::vm::MAX_RECURSION_DEPTH {
        vm.output
            .push("Error: Recursion limit exceeded in ephemeral execution".to_string());
        return;
    }
    vm.recursion_depth += 1;

    for gene in &strand.genes {
        let result = vm.execute_gene_inner(gene.op.clone(), &gene.args);
        if let Some(target) = result {
            vm.ip = target;
            // Jump occurred! Stop ephemeral execution and let the main loop continue from new IP.
            break;
        }
    }

    vm.recursion_depth -= 1;
}

#[cfg(feature = "nova")]
fn execute_strand_sync(vm: &mut ChimeraVM, strand_idx: usize) {
    if strand_idx < vm.dna.helix.strands.len() {
        let strand = vm.dna.helix.strands[strand_idx].clone();
        execute_ephemeral_strand(vm, &strand);
    } else {
        vm.output
            .push("Error: Invalid strand index for sync execution".to_string());
    }
}

/// Converts a direction vector (dy, dx) into a bitmask for membrane checking.
///
/// Mappings:
/// - (-1, 0) North -> 1
/// - (1, 0) South -> 2
/// - (0, 1) East -> 4
/// - (0, -1) West -> 8
#[cfg(feature = "nova")]
pub fn get_direction_mask(dy: i64, dx: i64) -> Option<u8> {
    match (dy, dx) {
        (-1, 0) => Some(1), // N
        (1, 0) => Some(2),  // S
        (0, 1) => Some(4),  // E
        (0, -1) => Some(8), // W
        _ => None,
    }
}

#[cfg(feature = "nova")]
impl ChimeraVM {
    pub fn resurrect_from_graveyard(&mut self, index: usize) -> Result<usize, String> {
        if index < self.graveyard.len() {
            let strand = self.graveyard.remove(index);
            self.dna.helix.strands.push(strand);
            self.telomeres.push(50);
            #[cfg(feature = "cortex")]
            {
                self.activation_levels.push(0);
                self.synapse_map.push(Vec::new());
            }
            let new_idx = self.dna.helix.strands.len() - 1;
            Ok(new_idx)
        } else {
            Err("Invalid graveyard index".to_string())
        }
    }
}
