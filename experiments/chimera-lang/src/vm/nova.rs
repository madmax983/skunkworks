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

use super::{nova_bestiary, nova_biome::Biome, ChimeraVM, Value, MAX_STRANDS};
use crate::ast::Nucleotide;
use crate::opcode::OpCode;
use crate::{ChimeraParser, Rule};
use pest::Parser;
use rand::seq::SliceRandom;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::{Hash, Hasher};

/// The physical state of the organism, affecting movement and mutation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
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

/// Defines the specialized behavior of an Organelle.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
    /// Grows procedurally based on L-System rules.
    Seed,
    /// Sings a song repeatedly.
    Choir,
    /// Moves randomly and triggers random glitches or entropy.
    Wisp,
    /// The Mad Scientist. Performs random experiments (Alchemy, Mutation, Chaos).
    MadScientist,
    /// A Viral Agent that moves on the Orca grid and injects genetic code.
    Phage,
    /// A Logic-driven agent that deduces actions from its environment using Prolog rules.
    Savant,
    /// A multicellular agent capable of forming tissues and complex organisms.
    Metazoan,
}

/// An independent execution unit spawned by the main strand.
///
/// Organelles run in parallel to the main organism (sequentially in the loop, but logically parallel).
/// They have their own stack, IP, and location, but share the organism's Energy and DNA.
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub name: String,
    pub traits: Vec<String>,
    pub id: u64,
    pub tissue_id: Option<usize>,
    pub genome_id: u64,
    pub energy: i64,
    pub experience: i64,
    pub stage: u8,
}

/// Pre-calculated neighbor offsets (dy, dx) and their corresponding bitmasks.
///
/// Optimization: Used to avoid repeated calls to `get_direction_mask` inside hot loops.
/// Mappings: N=1, S=2, E=4, W=8.
const NEIGHBOR_DIRECTIONS: [(i64, i64, u8); 4] = [
    (-1, 0, 1), // N
    (1, 0, 2),  // S
    (0, -1, 8), // W
    (0, 1, 4),  // E
];

fn get_open_neighbors(
    vm: &ChimeraVM,
    y: usize,
    x: usize,
) -> impl Iterator<Item = (usize, usize)> + '_ {
    NEIGHBOR_DIRECTIONS
        .into_iter()
        .filter_map(move |(dy, dx, mask)| {
            if (vm.membranes[y][x] & mask) != 0 {
                None
            } else {
                vm.normalize_coords(y as i64 + dy, x as i64 + dx)
            }
        })
}

/// Generic diffusion logic for scalar grids (i64).
///
/// Applies inertia, wind flow, and decay.
///
/// **Optimization:** Uses a stack-allocated buffer `[[i64; GRID_SIZE]; GRID_SIZE]` to avoid
/// repeated heap allocations (`Vec<Vec<i64>>`) every tick.
fn diffuse_scalar_grid<F>(
    source: &mut [Vec<i64>],
    biomes: &[Vec<Biome>],
    wind: &[Vec<(i8, i8)>],
    membranes: &[Vec<u8>],
    topology: &crate::vm::Topology,
    decay_fn: F,
) where
    F: Fn(usize, usize, &Biome) -> i128,
{
    let size = crate::vm::GRID_SIZE;
    let mut buffer = [[0i64; crate::vm::GRID_SIZE]; crate::vm::GRID_SIZE];

    for y in 0..size {
        for x in 0..size {
            let inertia = biomes[y][x].diffusion_inertia();
            let weight_center = 10;
            let mut sum = (source[y][x] as i128) * (inertia as i128) * weight_center;
            let mut total_weight = (inertia as i128) * weight_center;

            for (dy, dx, mask) in NEIGHBOR_DIRECTIONS {
                if (membranes[y][x] & mask) != 0 {
                    continue;
                }

                if let Some((ny, nx)) = topology.normalize(y as i64 + dy, x as i64 + dx, size, size)
                {
                    let (w_dy, w_dx) = wind[ny][nx];
                    // Wind flow from neighbor (ny, nx) to here (y, x).
                    let flow = -(w_dy as i128 * dy as i128 + w_dx as i128 * dx as i128);
                    let weight = (10 + flow).max(0);

                    sum += (source[ny][nx] as i128) * weight;
                    total_weight += weight;
                }
            }

            let decay = decay_fn(y, x, &biomes[y][x]);
            if total_weight > 0 {
                buffer[y][x] = ((sum / total_weight) * decay / 100) as i64;
            }
        }
    }

    for y in 0..size {
        // Optimization: Use copy_from_slice (memcpy) instead of element-wise loop.
        // We slice by `size` to ensure lengths match and avoid panics if vectors are oversized.
        source[y][..size].copy_from_slice(&buffer[y][..size]);
    }
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
            let weight_center = 10;
            let mut sums = [
                (vm.hormone_grid[y][x][0] as i128) * (inertia as i128) * weight_center,
                (vm.hormone_grid[y][x][1] as i128) * (inertia as i128) * weight_center,
                (vm.hormone_grid[y][x][2] as i128) * (inertia as i128) * weight_center,
            ];
            let mut total_weight = (inertia as i128) * weight_center;

            // Manual neighbor iteration to calculate wind bias
            for (dy, dx, mask) in NEIGHBOR_DIRECTIONS {
                if (vm.membranes[y][x] & mask) != 0 {
                    continue;
                }
                if let Some((ny, nx)) = vm.normalize_coords(y as i64 + dy, x as i64 + dx) {
                    let (w_dy, w_dx) = vm.wind_grid[ny][nx];
                    // Wind flow from neighbor (ny, nx) to here (y, x).
                    // Vector from neighbor to here is (-dy, -dx).
                    // Dot product: w_dy * (-dy) + w_dx * (-dx)
                    let flow = -(w_dy as i128 * dy as i128 + w_dx as i128 * dx as i128);
                    let weight = (10 + flow).max(0); // Base 10

                    for c in 0..3 {
                        sums[c] += (vm.hormone_grid[ny][nx][c] as i128) * weight;
                    }
                    total_weight += weight;
                }
            }

            if total_weight > 0 {
                for c in 0..3 {
                    buffer[y][x][c] = (sums[c] / total_weight) as i64;
                }
            }
        }
    }
    for y in 0..16 {
        // Optimization: Use copy_from_slice (memcpy).
        vm.hormone_grid[y][..16].copy_from_slice(&buffer[y][..16]);
    }
}

/// Simulates the diffusion of metabolic waste products.
///
/// Waste accumulates and spreads. High concentrations trigger damage/mutation.
#[allow(clippy::needless_range_loop)]
pub fn diffuse_waste(vm: &mut ChimeraVM) {
    diffuse_scalar_grid(
        &mut vm.waste_grid,
        &vm.biome_grid,
        &vm.wind_grid,
        &vm.membranes,
        &vm.topology,
        |_, _, b| b.decay_rate() as i128,
    );
}

/// Simulates the diffusion and decay of light.
///
/// Light spreads but decays rapidly (50% per tick), simulating absorption and scattering.
/// Chloroplasts harvest energy from this grid.
#[allow(clippy::needless_range_loop)]
pub fn diffuse_light(vm: &mut ChimeraVM) {
    super::nova_biolum::diffuse_light_color(vm);
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

            // Light interacts with Clouds (Moisture)
            let moisture = vm.moisture_grid[y][x];
            let cloud_opacity = (moisture as i64).clamp(0, 50); // Up to 50% block

            // Blur and strong decay (50% base + cloud)
            let transmission = 50 - cloud_opacity; // 50% -> 0% transmission relative to input
                                                   // Wait, previous was / 2 (50%).
                                                   // New logic: (sum / count) * transmission / 100?
                                                   // If transmission is 50 (clear sky), it matches previous.
                                                   // If transmission is 0 (thick cloud), light dies.

            buffer[y][x] = ((sum / count as i128) * transmission as i128 / 100) as i64;
        }
    }
    for y in 0..16 {
        // Optimization: Use copy_from_slice (memcpy).
        vm.light_grid[y][..16].copy_from_slice(&buffer[y][..16]);
    }
}

/// Simulates the diffusion of mutagenic radiation.
///
/// Mutagen spreads and decays slowly (90% retained per tick).
/// High levels cause random DNA mutations.
#[allow(clippy::needless_range_loop)]
pub fn diffuse_mutagen(vm: &mut ChimeraVM) {
    diffuse_scalar_grid(
        &mut vm.mutagen_grid,
        &vm.biome_grid,
        &vm.wind_grid,
        &vm.membranes,
        &vm.topology,
        |_, _, b| b.decay_rate() as i128 * 9 / 10,
    );
}

/// Simulates the diffusion of entropy.
///
/// Entropy spreads and decays slowly.
/// High levels cause Reality Decay (glitches).
#[allow(clippy::needless_range_loop)]
pub fn diffuse_entropy(vm: &mut ChimeraVM) {
    diffuse_scalar_grid(
        &mut vm.entropy_grid,
        &vm.biome_grid,
        &vm.wind_grid,
        &vm.membranes,
        &vm.topology,
        |_, _, _| 95,
    );
}

pub fn check_chorus_chords(vm: &mut ChimeraVM) -> Option<usize> {
    let buffer: Vec<&str> = vm.chorus_buffer.iter().map(|s| s.as_str()).collect();
    let len = buffer.len();
    if len < 2 {
        return None;
    }

    // Magic Chords (Spells)
    // Checks from end of buffer (most recent)

    // Check Registry (Dynamic Chords)
    for (chord, strand_idx) in &vm.chord_registry {
        let clen = chord.len();
        if len >= clen {
            let buffer_slice = &buffer[len - clen..];
            let chord_slice: Vec<&str> = chord.iter().map(|s| s.as_str()).collect();
            // vm.output.push(format!("DEBUG: Checking {:?} vs {:?}", buffer_slice, chord_slice));
            if buffer_slice == chord_slice.as_slice() {
                if *strand_idx < vm.dna.helix.strands.len() {
                    vm.chorus_buffer.clear();
                    vm.output
                        .push(format!("CHORUS: Triggered spell -> Strand {}", strand_idx));
                    return Some(*strand_idx);
                }
            }
        }
    }

    // "Vitality": Mi Re Do -> Energy + 50
    if len >= 3 && buffer[len - 3..] == ["Mi", "Re", "Do"] {
        vm.energy = vm.energy.saturating_add(50);
        vm.chorus_buffer.clear();
        vm.output
            .push("CHORUS: Vitality Chord! Energy restored.".to_string());
        return None;
    }

    // "Genesis": Do Mi Sol -> Spawn Worker
    if len >= 3 && buffer[len - 3..] == ["Do", "Mi", "Sol"] {
        // Spawn at random location
        if vm.organelles.len() < crate::vm::MAX_ORGANELLES {
            let mut rng = rand::thread_rng();
            let rx = rng.gen_range(0..crate::vm::GRID_SIZE);
            let ry = rng.gen_range(0..crate::vm::GRID_SIZE);

            vm.organelle_id_counter += 1;
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
                name: "Genesis Wisp".to_string(),
                traits: vec!["Summoned".to_string()],
                id: vm.organelle_id_counter,
                tissue_id: None,
                genome_id: 0,
                energy: 10,
                experience: 0,
                stage: 0,
            };
            vm.organelles.push(organelle);
            vm.chorus_buffer.clear();
            vm.output
                .push("CHORUS: Genesis Chord! Life created.".to_string());
            return None;
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
        return None;
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
        return None;
    }

    None
}

/// Runs a predictive simulation to see if the current path leads to death.
///
/// **OpCode:** `Prophecy`
/// **Stack:** `[ ..., ticks ] -> [ ..., result (1=Death, 0=Life) ]`
#[allow(clippy::needless_range_loop)]
fn exec_prophecy(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // stack: ticks (top)
    let ticks = vm.pop_int("prophecy")?;

    if ticks > 0 {
        let safe_ticks = ticks.min(1000);

        // Clone VM
        let mut sim_vm = vm.clone();

        // Inherit and increment recursion depth to prevent infinite prophecy loops
        sim_vm.recursion_depth += 1;
        if sim_vm.recursion_depth > crate::vm::MAX_SIMULATION_DEPTH {
            vm.output
                .push("Error: Simulation depth limit exceeded in prophecy".to_string());
            return None;
        }
        if sim_vm.recursion_depth > crate::vm::MAX_RECURSION_DEPTH {
            vm.output
                .push("Error: Recursion limit exceeded in prophecy".to_string());
            return None;
        }

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
    None
}

pub fn exec_lisp_eval(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    let s = vm.pop_str("lisp_eval")?;

    match crate::lisp::compile_fragment(&s) {
        Ok(genes) => {
            let strand = crate::ast::Strand { genes };
            execute_ephemeral_strand(vm, &strand);
            vm.output.push("LISP_EVAL: Success".to_string());
        }
        Err(e) => {
            vm.output.push(format!("LISP_EVAL ERROR: {}", e));
        }
    }
    None
}

/// Runs a sandboxed simulation of a specific strand.
///
/// **OpCode:** `Simulate`
/// **Stack:** `[ ..., strand_idx, ticks ] -> [ ..., top_val, final_energy, status ]`
fn exec_simulate(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // stack: ticks, strand_idx (bottom)
    let ticks = vm.pop_int("simulate")?;
    let s_idx = vm.pop_int("simulate")?;

    let idx = s_idx as usize;
    if idx < vm.dna.helix.strands.len() && ticks > 0 {
        if vm.recursion_depth > crate::vm::MAX_SIMULATION_DEPTH {
            vm.output
                .push("Error: Simulation depth limit exceeded".to_string());
            return None;
        }
        if vm.recursion_depth > crate::vm::MAX_RECURSION_DEPTH {
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
    None
}

/// Executes a Brainfuck program string with input.
///
/// **OpCode:** `Brainfuck`
/// **Stack:** `[ ..., bf_code, input ] -> [ ..., output ]`
fn exec_brainfuck(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // stack: bf_code_string, input_string (top)
    let input = vm.pop_str("brainfuck")?;
    let code = vm.pop_str("brainfuck")?;

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

    None
}

/// Executes a Nova-specific OpCode.
///
/// This function acts as the central dispatcher for all advanced features:
/// Biology, Physics, Metaphysics, Market, and more.
///
/// # Returns
///
/// Returns `Some((strand_idx, gene_idx))` if the operation triggered a jump or call that
/// modifies the Instruction Pointer (IP). Returns `None` if execution should proceed sequentially.
#[allow(clippy::needless_range_loop)]
pub fn exec_operator(vm: &mut ChimeraVM, _args: &[Nucleotide]) -> Option<(usize, usize)> {
    // Stack: [ ..., char_str, strand_idx ]
    // BUT OpCode usually takes stack args.
    // Let's check opcode.rs. Stack: [ ..., char_str, strand_idx ] -> [ ... ]
    // So we pop from stack.
    let idx = vm.pop_int("Operator")?;
    let s = vm.pop_str("Operator")?;

    if let Some(c) = s.chars().next() {
        if idx >= 0 && (idx as usize) < vm.dna.helix.strands.len() {
            vm.custom_operators.insert(c, idx as usize);
            vm.output
                .push(format!("OPERATOR: Defined '{}' -> Strand {}", c, idx));
        } else {
            vm.output
                .push("Error: Invalid strand index for Operator".to_string());
        }
    } else {
        vm.output
            .push("Error: Empty string for Operator char".to_string());
    }
    None
}

pub fn exec_nova_op(vm: &mut ChimeraVM, op: OpCode, args: &[Nucleotide]) -> Option<(usize, usize)> {
    match op {
        OpCode::Levenshtein => super::nova_linguistics::exec_levenshtein(vm),
        OpCode::Soundex => super::nova_linguistics::exec_soundex(vm),
        OpCode::Anagram => super::nova_linguistics::exec_anagram(vm),
        OpCode::Cipher => super::nova_linguistics::exec_cipher(vm),
        OpCode::Pangram => super::nova_linguistics::exec_pangram(vm),
        OpCode::Resonate => super::nova_resonance_war::exec_resonate(vm),
        OpCode::SonicClaim => super::nova_resonance_war::exec_sonic_claim(vm),
        OpCode::Dampen => super::nova_resonance_war::exec_dampen(vm),
        OpCode::ListenFreq => super::nova_resonance_war::exec_listen_freq(vm),
        OpCode::Prophecy => exec_prophecy(vm),
        OpCode::EgregoreLink => super::nova_egregore::exec_egregore_link(vm),
        OpCode::Lucid => exec_lucid(vm),
        OpCode::ChronosSplice => super::nova_genetics::exec_chronos_splice(vm),
        OpCode::Claim => super::nova_sovereignty::exec_claim(vm),
        OpCode::Cede => super::nova_sovereignty::exec_cede(vm),
        OpCode::Sovereignty => super::nova_sovereignty::exec_sovereignty(vm),
        OpCode::Tax => super::nova_sovereignty::exec_tax(vm),
        OpCode::Pocket => super::nova_pocket::exec_pocket(vm),
        OpCode::Unpocket => super::nova_pocket::exec_unpocket(vm),
        OpCode::Logistics => super::nova_logistics::exec_logistics(vm, op, args),
        OpCode::Harmonize => exec_harmonize(vm),
        OpCode::Choir => exec_choir(vm),
        OpCode::Fire => super::nova_ballistics::exec_fire(vm),
        OpCode::Salvo => super::nova_ballistics::exec_salvo(vm),
        OpCode::Reflector => super::nova_optics::exec_reflector(vm),
        OpCode::Prism => super::nova_optics::exec_prism(vm),
        OpCode::Lens => super::nova_optics::exec_lens(vm),
        OpCode::Sacrifice => super::nova_egregore::exec_sacrifice(vm),
        OpCode::Pray => super::nova_egregore::exec_pray(vm),
        OpCode::EgregoreTithe => super::nova_egregore::exec_egregore_tithe(vm),
        OpCode::EgregoreChannel => super::nova_egregore::exec_egregore_channel(vm),
        OpCode::EgregoreDictate => super::nova_egregore::exec_egregore_dictate(vm),
        OpCode::EgregoreQuery => super::nova_egregore::exec_egregore_query(vm),
        OpCode::EgregoreSummon => super::nova_egregore::exec_egregore_summon(vm),
        OpCode::Knot => super::nova_quipu::exec_knot(vm),
        OpCode::Unknot => super::nova_quipu::exec_unknot(vm),
        OpCode::Cord => super::nova_quipu::exec_cord(vm),
        OpCode::ReadCord => super::nova_quipu::exec_read_cord(vm),
        OpCode::Tangle => super::nova_quipu::exec_tangle(vm),
        OpCode::Offer => super::nova_market::exec_offer(vm),
        OpCode::Buy => super::nova_market::exec_buy(vm),
        OpCode::Invest => super::nova_market::exec_invest(vm),
        OpCode::Divest => super::nova_market::exec_divest(vm),
        OpCode::Balance => super::nova_market::exec_balance(vm),
        OpCode::Ticker => super::nova_market::exec_ticker(vm),
        OpCode::TimeWarp => super::nova_chronos::exec_time_warp(vm),
        OpCode::RetinaDraw => super::retina::exec_retina_draw(vm),
        OpCode::RetinaClear => super::retina::exec_retina_clear(vm),
        OpCode::RetinaSize => super::retina::exec_retina_size(vm),
        OpCode::Scanline => super::retina::exec_scanline(vm),
        OpCode::Rasterize => super::retina::exec_rasterize(vm),
        OpCode::QuantumJump => exec_quantum_jump(vm),
        OpCode::Chronos => super::nova_chronos::exec_chronos(vm),
        OpCode::Retroscope => super::nova_relativity::exec_retroscope(vm),
        OpCode::Relativity => exec_relativity(vm),
        OpCode::Graviton => exec_graviton(vm),
        OpCode::EventHorizon => exec_event_horizon(vm),
        OpCode::Aeolus => super::nova_fluid::exec_aeolus(vm, op, args),
        OpCode::Storm => super::nova_fluid::exec_storm(vm, op, args),
        OpCode::Tsunami => super::nova_fluid::exec_tsunami(vm, op, args),
        OpCode::Dry => super::nova_fluid::exec_dry(vm, op, args),
        OpCode::SenseWind => exec_sense_wind(vm),
        OpCode::SenseMoisture => exec_sense_moisture(vm),
        OpCode::Terraform => exec_terraform(vm),
        OpCode::SenseBiome => exec_sense_biome(vm),
        OpCode::Alchemy => exec_alchemy_op(vm),
        OpCode::Mix => super::nova_chemistry::exec_mix(vm),
        OpCode::Brew => super::nova_chemistry::exec_brew(vm),
        OpCode::Splash => super::nova_chemistry::exec_splash(vm),
        OpCode::Fossilize => super::nova_paleontology::exec_fossilize(vm),
        OpCode::Unearth => super::nova_paleontology::exec_unearth(vm),
        OpCode::CarbonDate => super::nova_paleontology::exec_carbon_date(vm),
        OpCode::Emit | OpCode::Smell | OpCode::Track => {
            super::nova_scent::exec_scent_op(vm, op, args)
        }
        OpCode::Conceive
        | OpCode::Propagate
        | OpCode::Forget
        | OpCode::Shibboleth
        | OpCode::Infect
        | OpCode::Outbreak
        | OpCode::Sanitize
        | OpCode::BioHack => super::memetics::exec_memetics_op(vm, op, args),
        OpCode::Meme => super::nova_genetics::exec_meme(vm),
        OpCode::Drift => super::nova_genetics::exec_drift(vm),
        OpCode::Poly => super::nova_genetics::exec_poly(vm, args),
        OpCode::Metamorphosis => super::nova_genetics::exec_metamorphosis(vm),
        OpCode::Genesis => super::nova_genetics::exec_genesis(vm),
        OpCode::Chaos => super::nova_flux::exec_chaos(vm),
        OpCode::Synthesize => super::catalyst::synthesize(vm),
        OpCode::Catalyze => super::catalyst::catalyze(vm),
        OpCode::Piet => exec_piet(vm),
        OpCode::Chronostasis => super::nova_chronos::exec_chronostasis(vm),
        OpCode::Simulate => exec_simulate(vm),
        OpCode::SensePigment => exec_sense_pigment(vm),
        OpCode::SenseGlyph => exec_sense_glyph(vm),
        OpCode::Sing => exec_sing(vm),
        OpCode::Listen => exec_listen(vm),
        OpCode::Brainfuck => exec_brainfuck(vm),
        OpCode::Spawn => exec_spawn(vm),
        OpCode::Entropy => exec_entropy(vm),
        OpCode::Stabilize => exec_stabilize(vm),
        OpCode::Disintegrate => exec_disintegrate(vm),
        OpCode::Sporulate => super::nova_chronos::exec_sporulate(vm),
        OpCode::TimeLoop => super::nova_chronos::exec_time_loop(vm),
        OpCode::Germinate => super::nova_chronos::exec_germinate(vm),
        OpCode::Paradox => super::nova_chronos::exec_paradox(vm),
        #[cfg(feature = "oracle")]
        OpCode::Divergence => super::nova_chronos::exec_divergence(vm),
        OpCode::Retrograde => super::nova_chronos::exec_retrograde(vm),
        OpCode::Incubate => super::nova_genetics::exec_incubate(vm),
        OpCode::Methylate => super::nova_genetics::exec_methylate(vm),
        OpCode::Demethylate => super::nova_genetics::exec_demethylate(vm),
        OpCode::Telomerase => super::nova_genetics::exec_telomerase(vm),
        OpCode::TLen => super::nova_genetics::exec_tlen(vm),
        OpCode::Splice => super::nova_genetics::exec_splice(vm),
        OpCode::Frankenstein => super::nova_genetics::exec_frankenstein(vm),
        OpCode::Recombine => super::nova_genetics::exec_recombine(vm),
        OpCode::SIndex => exec_s_index(vm),
        OpCode::CrisprScan => super::nova_genetics::exec_crispr_scan(vm),
        OpCode::Cas9Cut => super::nova_genetics::exec_cas9_cut(vm),
        OpCode::Ligase => super::nova_genetics::exec_ligase(vm),
        OpCode::Mitosis => super::nova_genetics::exec_mitosis(vm),
        OpCode::Apoptosis => super::nova_genetics::exec_apoptosis(vm),
        OpCode::Integrase => super::nova_genetics::exec_integrase(vm),
        OpCode::Excision => super::nova_genetics::exec_excision(vm),
        OpCode::Secrete => exec_secrete(vm),
        OpCode::Detect => exec_detect(vm),
        OpCode::Absorb => exec_absorb(vm),
        OpCode::Migrate => exec_migrate(vm),
        OpCode::Detox => exec_detox(vm),
        OpCode::WRead => exec_w_read(vm),
        OpCode::Call => exec_call(vm, args),
        OpCode::Exec => exec_exec(vm),
        OpCode::Ret => exec_ret(vm),
        OpCode::Bind => exec_bind(vm),
        OpCode::Unbind => exec_unbind(vm),
        OpCode::Entangle => exec_entangle(vm),
        OpCode::Decohere => exec_decohere(vm),
        OpCode::Conjugate => exec_conjugate(vm),
        OpCode::Gravitate => exec_gravitate(vm),
        OpCode::Lumine => exec_lumine(vm),
        OpCode::SenseLight => exec_sense_light(vm),
        OpCode::Dream => exec_dream(vm),
        OpCode::Chemotaxis => exec_chemotaxis(vm),
        OpCode::Identity => exec_identity(vm),
        OpCode::Differentiate => exec_differentiate(vm),
        OpCode::Shape => exec_shape(vm),
        OpCode::Rift => exec_rift(vm),
        OpCode::Seal => exec_seal(vm),
        OpCode::Sonar => exec_sonar(vm),
        OpCode::LispEval => exec_lisp_eval(vm),
        OpCode::Broadcast => exec_broadcast(vm),
        OpCode::Tune => exec_tune(vm),
        OpCode::Isomerize => exec_isomerize(vm),
        OpCode::PhaseShift => exec_phase_shift(vm),
        OpCode::Membrane => exec_membrane(vm),
        OpCode::Osmosis => exec_osmosis(vm),
        OpCode::Symbiosis => exec_symbiosis(vm),
        OpCode::Reflex => exec_reflex(vm),
        OpCode::Lysis => exec_lysis(vm),
        OpCode::Compile => super::nova_genetics::exec_compile(vm),
        OpCode::Irradiate => exec_irradiate(vm),
        OpCode::SenseMutagen => exec_sense_mutagen(vm),
        OpCode::Devour => exec_devour(vm),
        OpCode::Decompile => super::nova_genetics::exec_decompile(vm),
        OpCode::Void => exec_void_op(vm),
        OpCode::VoidRift => super::nova_void::exec_void_rift(vm),
        OpCode::VoidCast => super::nova_void::exec_void_cast(vm),
        OpCode::Reactor => super::nova_reactor::exec_reactor(vm),
        OpCode::Reaction => super::nova_reactor::exec_reaction(vm),
        OpCode::Cambrian => super::nova_ecology::cambrian_explosion(vm),
        OpCode::Supernova => exec_supernova(vm),
        OpCode::Singularity => exec_singularity(vm),
        OpCode::Eval => exec_eval(vm),
        OpCode::Map => exec_map(vm),
        OpCode::Fold => exec_fold(vm),
        OpCode::Filter => exec_filter(vm),
        OpCode::Zip => exec_zip(vm),
        OpCode::Pigment => exec_pigment(vm),
        OpCode::Glyph => exec_glyph(vm),
        OpCode::Evolve => super::nova_garden::exec_evolve(vm),
        OpCode::Sow => super::nova_garden::exec_sow(vm),
        OpCode::Harvest => super::nova_garden::exec_harvest(vm),
        OpCode::Draw => {
            super::nova_arcana::exec_draw(vm);
            None
        }
        OpCode::Fate => {
            super::nova_arcana::exec_fate(vm);
            None
        }
        OpCode::Shuffle => {
            super::nova_arcana::exec_shuffle(vm);
            None
        }
        OpCode::Glitch => exec_glitch(vm),
        OpCode::Scramble => exec_scramble(vm),
        OpCode::Hyphae => exec_hyphae(vm),
        OpCode::Connect => exec_connect(vm),
        OpCode::Transport => exec_transport(vm),
        OpCode::SporeCloud => exec_spore_cloud(vm),
        OpCode::Signal => {
            super::ipc::signal(vm);
            None
        }
        OpCode::Receive => {
            super::ipc::receive(vm);
            None
        }
        OpCode::Spirit => exec_spirit(vm),
        OpCode::Match => exec_match(vm),
        OpCode::Bury => exec_bury(vm),
        OpCode::Exhume => exec_exhume(vm),
        OpCode::Seance => exec_seance(vm),
        OpCode::Mourn => exec_mourn(vm),
        OpCode::Reincarnate => exec_reincarnate(vm),
        OpCode::Superpose => exec_superpose(vm),
        OpCode::Collapse => exec_collapse(vm),
        OpCode::Observe => exec_observe(vm),
        OpCode::MeshNet => super::nova_biomesh::exec_mesh_net(vm, args),
        OpCode::MeshSend => super::nova_biomesh::exec_mesh_send(vm),
        OpCode::MeshRecv => super::nova_biomesh::exec_mesh_recv(vm),
        OpCode::MeshGrow => super::nova_biomesh::exec_mesh_grow(vm),
        OpCode::MeshPrune => super::nova_biomesh::exec_mesh_prune(vm),
        OpCode::Luciferin => super::nova_biolum::exec_luciferin(vm),
        OpCode::Photophore => super::nova_biolum::exec_photophore(vm),
        OpCode::Symbolize => super::nova_semiotics::exec_symbolize(vm),
        OpCode::Interpret => super::nova_semiotics::exec_interpret(vm),
        OpCode::ContextShift => super::nova_semiotics::exec_context_shift(vm),
        OpCode::Deconstruct => super::nova_semiotics::exec_deconstruct(vm),
        OpCode::TuiMod => exec_tui_mod(vm),
        OpCode::Horcrux => exec_horcrux(vm),
        OpCode::Rebirth => exec_rebirth(vm),
        OpCode::Prologue => exec_prologue(vm),
        OpCode::Rune => exec_rune(vm),
        _ => None,
    }
}

fn exec_prologue(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    vm.prologue_state.active = !vm.prologue_state.active;
    let status = if vm.prologue_state.active {
        "ON"
    } else {
        "OFF"
    };
    vm.output.push(format!("PROLOGUE: Rune Logic {}", status));
    None
}

fn exec_rune(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if vm.stack.len() >= 3 {
        let x_val = vm.stack.pop().unwrap();
        let y_val = vm.stack.pop().unwrap();
        let c_val = vm.stack.pop().unwrap();
        if let (Value::Int(c), Value::Int(y), Value::Int(x)) = (c_val, y_val, x_val) {
            if let Some((ny, nx)) = vm.normalize_coords(y, x) {
                if let Some(ch) = char::from_u32(c as u32) {
                    vm.grid[ny][nx] = Value::Str(ch.to_string());
                    vm.output
                        .push(format!("RUNE: Placed '{}' at {},{}", ch, nx, ny));
                }
            }
        }
    }
    None
}

fn exec_tui_mod(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if vm.stack.len() >= 2 {
        let mode_val = vm.stack.pop().unwrap();
        let val_val = vm.stack.pop().unwrap();

        match (mode_val, val_val) {
            (Value::Int(mode), Value::Int(val)) => match mode {
                0 => {
                    let intensity = (val as f32) / 100.0;
                    if vm.tui_events.len() < crate::vm::MAX_TUI_EVENTS {
                        vm.tui_events.push(crate::vm::TuiEvent::Glitch(intensity));
                        vm.output
                            .push(format!("TUI: Glitch set to {:.2}", intensity));
                    } else {
                        vm.output.push("TUI: Event queue full".to_string());
                    }
                }
                1 => {
                    let intensity = (val as f32) / 10.0;
                    if vm.tui_events.len() < crate::vm::MAX_TUI_EVENTS {
                        vm.tui_events.push(crate::vm::TuiEvent::Shake(intensity));
                        vm.output
                            .push(format!("TUI: Screen Shake {:.2}", intensity));
                    } else {
                        vm.output.push("TUI: Event queue full".to_string());
                    }
                }
                _ => {
                    vm.output.push("TUI: Unknown mode".to_string());
                }
            },
            (Value::Int(mode), Value::Str(s)) => {
                if mode == 2 {
                    if vm.tui_events.len() < crate::vm::MAX_TUI_EVENTS {
                        vm.tui_events.push(crate::vm::TuiEvent::Message(s.clone()));
                        vm.output.push(format!("TUI: Message '{}'", s));
                    } else {
                        vm.output.push("TUI: Event queue full".to_string());
                    }
                } else {
                    vm.output
                        .push("Error: TuiMod mode requires Int value".to_string());
                }
            }
            _ => {
                vm.output
                    .push("Error: Type mismatch for TuiMod".to_string());
            }
        }
    } else {
        vm.output
            .push("Error: Stack underflow for TuiMod".to_string());
    }
    None
}

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

fn execute_strand_sync(vm: &mut ChimeraVM, strand_idx: usize) {
    if strand_idx < vm.dna.helix.strands.len() {
        let strand = vm.dna.helix.strands[strand_idx].clone();
        execute_ephemeral_strand(vm, &strand);
    } else {
        vm.output
            .push("Error: Invalid strand index for sync execution".to_string());
    }
}

fn exec_gravitate(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // stack: radius
    let r = vm.pop_int("gravitate")?;

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
            if let Some((target_y, target_x)) = vm.normalize_coords(ty as i64 + sy, tx as i64 + sx)
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
    None
}

fn exec_spawn(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // stack: type, strand_idx (bottom)
    let t = vm.pop_int("spawn")?;
    let idx = vm.pop_int("spawn")?;

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
            5 => (OrganelleType::Void, (0, 0)),
            6 => (OrganelleType::Alchemist, (0, 0)),
            10 => (OrganelleType::MadScientist, (0, 0)),
            11 => (OrganelleType::Phage, (0, 1)), // Default East
            12 => (OrganelleType::Savant, (0, 0)),
            13 => (OrganelleType::Metazoan, (0, 0)),
            _ => (OrganelleType::Worker, (0, 0)),
        };

        let strand = &vm.dna.helix.strands[s_idx];
        let mut hasher = DefaultHasher::new();
        strand.hash(&mut hasher);
        let genome_id = hasher.finish();
        let traits = nova_bestiary::analyze_traits(strand);
        let name = nova_bestiary::generate_name(genome_id, &traits);

        vm.organelle_id_counter += 1;
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
            name,
            traits,
            id: vm.organelle_id_counter,
            tissue_id: None,
            genome_id,
            energy: 50,
            experience: 0,
            stage: 0,
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
    None
}

fn exec_migrate(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // stack: dy, dx (top)
    let mut dx = vm.pop_int("migrate")?;
    let mut dy = vm.pop_int("migrate")?;

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
        if let Some((mut new_y, mut new_x)) = vm.normalize_coords(cy as i64 + dy, cx as i64 + dx) {
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

            if let Some(target) = super::nova_ward::check_ward_trigger(vm) {
                return Some(target);
            }
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
    None
}

fn exec_conjugate(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // stack: direction (0=R, 1=D, 2=L, 3=U), y, x, strand_idx (bottom)
    let dir = vm.pop_int("conjugate")?;
    let x = vm.pop_int("conjugate")?;
    let y = vm.pop_int("conjugate")?;
    let s = vm.pop_int("conjugate")?;

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
                if let Some((next_y, next_x)) = vm.normalize_coords(ny as i64 + dy, nx as i64 + dx)
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
    None
}

fn exec_dream(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // stack: ticks, strand_idx (bottom)
    let ticks = vm.pop_int("dream")?;
    let s_idx = vm.pop_int("dream")?;

    let idx = s_idx as usize;
    if idx < vm.dna.helix.strands.len() && ticks > 0 {
        if vm.recursion_depth > crate::vm::MAX_SIMULATION_DEPTH {
            vm.output
                .push("Error: Simulation depth limit exceeded".to_string());
            return None;
        }

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
        let mut success = dream_vm.energy > vm.energy;

        // Nightmare Check
        let (cy, cx) = vm.context_loc;
        let entropy = vm.entropy_grid[cy][cx];
        let is_nightmare = entropy > 50;

        if is_nightmare {
            success = true; // Nightmares are forced
            vm.output
                .push("NIGHTMARE: The Void invades the dream...".to_string());
        }

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
            is_nightmare,
            dream_vm.output.clone(),
            None,
        );
        vm.dream_traces.push(trace);

        if success {
            // Adopt DNA
            vm.dna = dream_vm.dna;
            vm.stack.push(Value::Int(1)); // Success
            if is_nightmare {
                vm.output.push("DREAM: Nightmare realized!".to_string());
            } else {
                vm.output.push("DREAM: Mutation accepted".to_string());
            }
        } else {
            vm.stack.push(Value::Int(0)); // Failure
            vm.output.push("DREAM: Mutation discarded".to_string());
        }

        vm.energy = vm.energy.saturating_sub(cost);
    } else {
        vm.output.push("Error: Invalid args for dream".to_string());
    }
    None
}

/// Converts a direction vector (dy, dx) into a bitmask for membrane checking.
///
/// Mappings:
/// - (-1, 0) North -> 1
/// - (1, 0) South -> 2
/// - (0, 1) East -> 4
/// - (0, -1) West -> 8
pub fn get_direction_mask(dy: i64, dx: i64) -> Option<u8> {
    match (dy, dx) {
        (-1, 0) => Some(1), // N
        (1, 0) => Some(2),  // S
        (0, 1) => Some(4),  // E
        (0, -1) => Some(8), // W
        _ => None,
    }
}

fn exec_lucid(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    let amount = vm.pop_int("lucid")?;

    if amount > 0 {
        let cost = amount;
        if vm.energy >= cost {
            vm.energy -= cost;
            let (cy, cx) = vm.context_loc;
            vm.entropy_grid[cy][cx] = vm.entropy_grid[cy][cx].saturating_sub(amount).max(0);
            vm.output.push("LUCIDITY: Clarity restored.".to_string());
        } else {
            vm.output.push("LUCID: Insufficient energy".to_string());
        }
    }
    None
}

fn exec_harmonize(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if vm.stack.len() >= 2 {
        let s_val = vm.stack.pop().unwrap();
        let chord_val = vm.stack.pop().unwrap();

        if let (Value::Int(s_idx), Value::Junction(_, notes)) = (s_val, chord_val) {
            let idx = s_idx as usize;
            if idx < vm.dna.helix.strands.len() {
                let mut chord_str = Vec::new();
                let mut valid = true;
                for note in notes {
                    if let Value::Str(s) = note {
                        chord_str.push(s);
                    } else {
                        valid = false;
                        break;
                    }
                }

                if valid && !chord_str.is_empty() {
                    if vm.chord_registry.len() >= crate::vm::MAX_CHORD_REGISTRY {
                        vm.output
                            .push("Error: Chord registry limit exceeded".to_string());
                        return None;
                    }
                    vm.chord_registry.insert(chord_str.clone(), idx);
                    vm.output.push(format!(
                        "HARMONIZE: Registered chord {:?} -> Strand {}",
                        chord_str, idx
                    ));
                } else {
                    vm.output
                        .push("HARMONIZE: Invalid chord (must be strings)".to_string());
                }
            } else {
                vm.output
                    .push("HARMONIZE: Invalid strand index".to_string());
            }
        } else {
            vm.output.push("HARMONIZE: Type mismatch".to_string());
        }
    } else {
        vm.output.push("HARMONIZE: Stack underflow".to_string());
    }
    None
}

fn exec_choir(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if let Some(Value::Junction(_, notes)) = vm.stack.pop() {
        if vm.organelles.len() < crate::vm::MAX_ORGANELLES {
            let mut song = Vec::new();
            for note in notes {
                if let Value::Str(s) = note {
                    song.push(s);
                }
            }

            if !song.is_empty() {
                vm.organelle_id_counter += 1;
                let organelle = Organelle {
                    stack: Vec::new(),
                    ip: (0, 0),
                    context_loc: vm.context_loc,
                    call_stack: Vec::new(),
                    recursion_depth: 0,
                    halted: false,
                    kind: OrganelleType::Choir,
                    direction: (0, 0),
                    ttl: None,
                    name: "Seraphim".to_string(),
                    traits: song,
                    id: vm.organelle_id_counter,
                    tissue_id: None,
                    genome_id: 0,
                    energy: 50,
                    experience: 0,
                    stage: 0,
                };
                vm.organelles.push(organelle);
                vm.energy = vm.energy.saturating_sub(50);
                vm.output.push("CHOIR: Spun into existence".to_string());
            }
        }
    }
    None
}

fn exec_quantum_jump(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    let s_idx = vm.ip.0;
    if let Some(&partner_idx) = vm.entangled_pairs.get(&s_idx) {
        if partner_idx < vm.dna.helix.strands.len() {
            let gene_idx = vm.ip.1;
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

fn exec_relativity(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    vm.relativity_mode = !vm.relativity_mode;
    let status = if vm.relativity_mode { "ON" } else { "OFF" };
    vm.output
        .push(format!("RELATIVITY: Physics engine {}", status));
    None
}

fn exec_graviton(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    let (cy, cx) = vm.context_loc;
    vm.gravity_grid[cy][cx] = vm.gravity_grid[cy][cx].saturating_add(50);
    vm.energy = vm.energy.saturating_sub(10);
    vm.output
        .push(format!("GRAVITON: Emitted at {},{}", cx, cy));
    None
}

fn exec_event_horizon(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    let (cy, cx) = vm.context_loc;
    let g = vm.gravity_grid[cy][cx];
    vm.stack.push(Value::Int(g));
    None
}

fn exec_sense_wind(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    let (cy, cx) = vm.context_loc;
    let (dy, dx) = vm.wind_grid[cy][cx];
    vm.stack.push(Value::Int(dy as i64));
    vm.stack.push(Value::Int(dx as i64));
    None
}

fn exec_sense_moisture(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    let (cy, cx) = vm.context_loc;
    vm.stack.push(Value::Int(vm.moisture_grid[cy][cx]));
    None
}

fn exec_terraform(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
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
                    5 => Biome::Glitch,
                    6 => Biome::Aether,
                    7 => Biome::Silicon,
                    8 => Biome::Garden,
                    _ => Biome::Plains,
                };

                let (cy, cx) = vm.context_loc;
                let coords = vm.get_circular_coords(cx as i64, cy as i64, r);
                let count = coords.len();

                for (tx, ty) in coords {
                    vm.biome_grid[ty][tx] = biome;
                }

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

fn exec_sense_biome(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    let (cy, cx) = vm.context_loc;
    let biome = vm.biome_grid[cy][cx];
    let id = match biome {
        Biome::Plains => 0,
        Biome::Swamp => 1,
        Biome::Desert => 2,
        Biome::Tundra => 3,
        Biome::Volcanic => 4,
        Biome::Glitch => 5,
        Biome::Aether => 6,
        Biome::Silicon => 7,
        Biome::Garden => 8,
    };
    vm.stack.push(Value::Int(id));
    None
}

fn exec_alchemy_op(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    let (cy, cx) = vm.context_loc;
    crate::vm::alchemy::perform_alchemy(vm, cy, cx);
    vm.energy = vm.energy.saturating_sub(5);
    None
}

fn exec_piet(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if let Some(val) = vm.stack.pop() {
        if let Value::Int(steps) = val {
            if steps > 0 {
                if vm.piet_state.is_none() {
                    vm.piet_state = Some(super::piet::init_piet(vm));
                }

                let mut remaining = steps;
                let mut active = true;

                if let Some(mut state) = vm.piet_state.take() {
                    while remaining > 0 {
                        if !super::piet::step_piet_once(vm, &mut state) {
                            active = false;
                            break;
                        }
                        remaining -= 1;
                    }

                    if active {
                        vm.piet_state = Some(state);
                    } else {
                        vm.piet_state = None;
                    }
                }
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

fn exec_sense_pigment(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
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

fn exec_sense_glyph(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    let (cy, cx) = vm.context_loc;
    if let Some(c) = vm.chroma_grid[cy][cx].char {
        vm.stack.push(Value::Int(c as u8 as i64));
    } else {
        vm.stack.push(Value::Int(-1));
    }
    None
}

fn exec_sing(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if let Some(val) = vm.stack.pop() {
        if let Value::Str(note) = val {
            vm.chorus_buffer.push_back(note.clone());
            if vm.chorus_buffer.len() > crate::vm::MAX_CHORUS_SIZE {
                vm.chorus_buffer.pop_front();
            }
            vm.energy = vm.energy.saturating_sub(2);
            vm.output.push(format!("SING: {}", note));

            if let Some(target) = check_chorus_chords(vm) {
                if vm.call_stack.len() < crate::vm::MAX_CALL_STACK_DEPTH {
                    vm.call_stack.push(vm.ip);
                    vm.ip = (target, 0);
                }
            }
        } else {
            vm.output.push("Error: Type mismatch for sing".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for sing".to_string());
    }
    None
}

fn exec_listen(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    let notes: Vec<Value> = vm
        .chorus_buffer
        .iter()
        .map(|s| Value::Str(s.clone()))
        .collect();
    vm.stack
        .push(Value::Junction(crate::ast::JunctionType::All, notes));
    None
}

fn exec_entropy(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    let (cy, cx) = vm.context_loc;
    let level = vm.entropy_grid[cy][cx];
    vm.stack.push(Value::Int(level));
    None
}

fn exec_stabilize(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if let Some(val) = vm.stack.pop() {
        if let Value::Int(amount) = val {
            if amount > 0 {
                let cost = amount;
                if vm.energy >= cost {
                    vm.energy -= cost;
                    let (cy, cx) = vm.context_loc;
                    vm.entropy_grid[cy][cx] = vm.entropy_grid[cy][cx].saturating_sub(amount);
                    vm.glitch_level = (vm.glitch_level - (amount as f32 / 10.0)).max(0.0);
                    vm.output.push(format!(
                        "STABILIZE: Reduced entropy by {} at {},{}",
                        amount, cx, cy
                    ));
                } else {
                    vm.output.push("STABILIZE: Insufficient energy".to_string());
                }
            }
        } else {
            vm.output
                .push("Error: Type mismatch for stabilize".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for stabilize".to_string());
    }
    None
}

fn exec_disintegrate(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if vm.stack.len() >= 2 {
        let x_val = vm.stack.pop().unwrap();
        let y_val = vm.stack.pop().unwrap();
        if let (Value::Int(y), Value::Int(x)) = (y_val, x_val) {
            if let Some((ny, nx)) = vm.normalize_coords(y, x) {
                vm.entropy_grid[ny][nx] = 100;
                vm.grid[ny][nx] = Value::Int(0);
                vm.glitch_level = (vm.glitch_level + 0.5).clamp(0.0, 1.0);
                vm.energy = vm.energy.saturating_sub(10);
                vm.output
                    .push(format!("DISINTEGRATE: Cell at {},{}", nx, ny));
            } else {
                vm.output
                    .push("Error: Coordinates out of bounds for disintegrate".to_string());
            }
        } else {
            vm.output
                .push("Error: Type mismatch for disintegrate".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for disintegrate".to_string());
    }
    None
}

fn exec_s_index(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    vm.stack.push(Value::Int(vm.ip.0 as i64));
    None
}

fn exec_secrete(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
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

fn exec_detect(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
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

fn exec_absorb(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
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

pub(crate) fn exec_detox(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if let Some(val) = vm.stack.pop() {
        if let Value::Int(r) = val {
            let (cy, cx) = vm.context_loc;
            let coords = vm.get_circular_coords(cx as i64, cy as i64, r);
            for (tx, ty) in coords {
                vm.waste_grid[ty][tx] = 0;
            }
            let r_sq = (r as i128).saturating_mul(r as i128);
            let cost = (r_sq + 1).clamp(5, 50) as i64;
            vm.energy = vm.energy.saturating_sub(cost);
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

fn exec_w_read(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    let (cy, cx) = vm.context_loc;
    let waste = vm.waste_grid[cy][cx];
    vm.stack.push(Value::Int(waste));
    None
}

fn exec_call(vm: &mut ChimeraVM, args: &[Nucleotide]) -> Option<(usize, usize)> {
    if let Some(Nucleotide::Number(idx)) = args.first() {
        let strand_idx = *idx as usize;
        if strand_idx < vm.dna.helix.strands.len() {
            if vm.call_stack.len() >= crate::vm::MAX_CALL_STACK_DEPTH {
                vm.output.push("Error: Call stack overflow".to_string());
                return None;
            }
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

fn exec_exec(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
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
                vm.output
                    .push("Error: Invalid strand index for exec".to_string());
            }
        } else {
            vm.output.push("Error: Type mismatch for exec".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for exec".to_string());
    }
    None
}

fn exec_ret(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if let Some(ret_addr) = vm.call_stack.pop() {
        return Some(ret_addr);
    } else {
        vm.output
            .push("Warning: Return with empty stack".to_string());
    }
    None
}

fn exec_bind(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
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

fn exec_unbind(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
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

fn exec_entangle(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if vm.stack.len() >= 2 {
        let s_val2 = vm.stack.pop().unwrap();
        let s_val1 = vm.stack.pop().unwrap();
        if let (Value::Int(s1), Value::Int(s2)) = (s_val1, s_val2) {
            let idx1 = s1 as usize;
            let idx2 = s2 as usize;
            let len = vm.dna.helix.strands.len();
            if idx1 < len && idx2 < len {
                if idx1 != idx2 {
                    if let Some(old) = vm.entangled_pairs.remove(&idx1) {
                        vm.entangled_pairs.remove(&old);
                    }
                    if let Some(old) = vm.entangled_pairs.remove(&idx2) {
                        vm.entangled_pairs.remove(&old);
                    }

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

fn exec_decohere(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
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

fn exec_lumine(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
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

fn exec_sense_light(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    let (cy, cx) = vm.context_loc;
    let intensity = vm.light_grid[cy][cx];
    vm.stack.push(Value::Int(intensity));
    None
}

fn exec_chemotaxis(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if let Some(val) = vm.stack.pop() {
        if let Value::Int(c) = val {
            let (cy, cx) = vm.context_loc;
            let channel_idx = (c.unsigned_abs() as usize) % 3;

            let mut max_intensity = -1;
            let mut best_dy = 0;
            let mut best_dx = 0;

            for dy in -1..=1 {
                for dx in -1..=1 {
                    if let Some((ny, nx)) = vm.normalize_coords(cy as i64 + dy, cx as i64 + dx) {
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

fn exec_identity(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    let id = match &vm.active_organelle_kind {
        None => -1, // Nucleus
        Some(OrganelleType::Worker) => 0,
        Some(OrganelleType::Chloroplast) => 1,
        Some(OrganelleType::Mitochondria) => 2,
        Some(OrganelleType::Lysosome) => 3,
        Some(OrganelleType::Ribosome) => 4,
        Some(OrganelleType::Void) => 5,
        Some(OrganelleType::Alchemist) => 6,
        Some(OrganelleType::Seed) => 7,
        Some(OrganelleType::Choir) => 8,
        Some(OrganelleType::Wisp) => 9,
        Some(OrganelleType::MadScientist) => 10,
        Some(OrganelleType::Phage) => 11,
        Some(OrganelleType::Savant) => 12,
        Some(OrganelleType::Metazoan) => 13,
    };
    vm.stack.push(Value::Int(id));
    None
}

fn exec_differentiate(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
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
                    7 => Some(OrganelleType::Seed),
                    8 => Some(OrganelleType::Choir),
                    9 => Some(OrganelleType::Wisp),
                    10 => Some(OrganelleType::MadScientist),
                    11 => Some(OrganelleType::Phage),
                    12 => Some(OrganelleType::Savant),
                    13 => Some(OrganelleType::Metazoan),
                    _ => Some(OrganelleType::Worker),
                };

                if let Some(kind) = new_kind {
                    vm.signal_differentiation = Some(kind.clone());
                    vm.energy = vm.energy.saturating_sub(50);
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

fn exec_shape(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
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
                7 => Some(super::Topology::Sphere),
                8 => Some(super::Topology::Projective),
                _ => None,
            };

            if let Some(topo) = new_topology {
                vm.topology = topo;
                vm.output
                    .push(format!("SHAPE: Changed topology to {:?}", topo));
                vm.energy = vm.energy.saturating_sub(100);
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

fn exec_rift(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
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

fn exec_seal(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
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

fn exec_sonar(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if vm.stack.len() >= 2 {
        let dx_val = vm.stack.pop().unwrap();
        let dy_val = vm.stack.pop().unwrap();
        if let (Value::Int(dy), Value::Int(dx)) = (dy_val, dx_val) {
            let (cy, cx) = vm.context_loc;
            let mut found = false;

            for d in 1..=16 {
                if let Some((ny, nx)) = vm.normalize_coords(cy as i64 + dy * d, cx as i64 + dx * d)
                {
                    if !matches!(vm.grid[ny][nx], Value::Int(0)) {
                        vm.stack.push(Value::Int(d));
                        vm.stack.push(vm.grid[ny][nx].clone());
                        vm.sonar_target = Some((ny, nx));
                        found = true;
                        break;
                    }
                } else {
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

fn exec_broadcast(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if vm.stack.len() >= 2 {
        let value = vm.stack.pop().unwrap();
        let channel_val = vm.stack.pop().unwrap();
        if let Value::Int(channel) = channel_val {
            if vm.ether.len() >= crate::vm::MAX_ETHER_CHANNELS && !vm.ether.contains_key(&channel) {
                vm.output
                    .push("Error: Ether channel limit exceeded".to_string());
                return None;
            }
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

fn exec_tune(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
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

fn exec_isomerize(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    vm.chirality = match vm.chirality {
        crate::vm::Chirality::Left => crate::vm::Chirality::Right,
        crate::vm::Chirality::Right => crate::vm::Chirality::Left,
    };
    vm.output
        .push(format!("ISOMERIZE: Switched to {:?}", vm.chirality));
    None
}

fn exec_phase_shift(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
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

fn exec_membrane(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    let mask_val = vm.pop_int("membrane")?;

    let mask = mask_val as u8;
    let (cy, cx) = vm.context_loc;

    vm.membranes[cy][cx] ^= mask;

    if (mask & 1) != 0 {
        if let Some((ny, nx)) = vm.normalize_coords(cy as i64 - 1, cx as i64) {
            vm.membranes[ny][nx] ^= 2;
        }
    }
    if (mask & 2) != 0 {
        if let Some((ny, nx)) = vm.normalize_coords(cy as i64 + 1, cx as i64) {
            vm.membranes[ny][nx] ^= 1;
        }
    }
    if (mask & 4) != 0 {
        if let Some((ny, nx)) = vm.normalize_coords(cy as i64, cx as i64 + 1) {
            vm.membranes[ny][nx] ^= 8;
        }
    }
    if (mask & 8) != 0 {
        if let Some((ny, nx)) = vm.normalize_coords(cy as i64, cx as i64 - 1) {
            vm.membranes[ny][nx] ^= 4;
        }
    }

    vm.energy = vm.energy.saturating_sub(10);
    vm.output
        .push(format!("MEMBRANE: Toggled mask {} at {},{}", mask, cx, cy));

    None
}

fn exec_osmosis(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    let dx = vm.pop_int("osmosis")?;
    let dy = vm.pop_int("osmosis")?;

    let (cy, cx) = vm.context_loc;
    if let Some((mut new_y, mut new_x)) = vm.normalize_coords(cy as i64 + dy, cx as i64 + dx) {
        if let Some(&(py, px)) = vm.portals.get(&(new_y, new_x)) {
            vm.output.push(format!(
                "PORTAL: Teleported from {},{} to {},{}",
                new_x, new_y, px, py
            ));
            new_y = py;
            new_x = px;
        }

        vm.context_loc = (new_y, new_x);
        vm.energy = vm.energy.saturating_sub(20);
        vm.output
            .push(format!("OSMOSIS: Moved to {},{}", new_x, new_y));

        if let Some(target) = super::nova_ward::check_ward_trigger(vm) {
            return Some(target);
        }
    } else {
        vm.energy = vm.energy.saturating_sub(5);
        vm.output.push("OSMOSIS: Blocked by boundary".to_string());
    }
    None
}

fn exec_symbiosis(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    let dx = vm.pop_int("symbiosis")?;
    let dy = vm.pop_int("symbiosis")?;

    let (cy, cx) = vm.context_loc;
    if let Some((ny, nx)) = vm.normalize_coords(cy as i64 + dy, cx as i64 + dx) {
        let mut found_idx = None;
        for (i, org) in vm.organelles.iter().enumerate() {
            if org.context_loc == (ny, nx) {
                found_idx = Some(i);
                break;
            }
        }

        if let Some(idx) = found_idx {
            let organelle = vm.organelles.remove(idx);
            vm.symbiotes.push(organelle.ip);
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
    None
}

fn exec_reflex(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    let e = vm.pop_int("reflex")?;
    let s = vm.pop_int("reflex")?;

    let s_idx = s as usize;
    if s_idx < vm.dna.helix.strands.len() {
        if vm.reflexes.len() >= crate::vm::MAX_REFLEXES {
            vm.output.push("Error: Reflex limit exceeded".to_string());
            return None;
        }
        vm.reflexes.insert(e, s_idx);
        vm.output
            .push(format!("REFLEX: Bound event {} to strand {}", e, s_idx));
    } else {
        vm.output
            .push("Error: Strand index out of bounds for reflex".to_string());
    }
    None
}

fn exec_lysis(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if let Some(sip) = vm.symbiotes.pop() {
        if vm.organelles.len() >= crate::vm::MAX_ORGANELLES {
            vm.symbiotes.push(sip);
            vm.output
                .push("Error: Organelle limit exceeded".to_string());
            return None;
        }

        let (cy, cx) = vm.context_loc;
        vm.organelle_id_counter += 1;
        let organelle = Organelle {
            stack: Vec::new(),
            ip: sip,
            context_loc: (cy, cx),
            call_stack: Vec::new(),
            recursion_depth: 0,
            halted: false,
            kind: OrganelleType::Worker,
            direction: (0, 0),
            ttl: None,
            name: "Symbiote Spawn".to_string(),
            traits: vec!["Ejected".to_string()],
            id: vm.organelle_id_counter,
            tissue_id: None,
            genome_id: 0,
            energy: 25,
            experience: 0,
            stage: 0,
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

pub(crate) fn exec_irradiate(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    let r = vm.pop_int("irradiate")?;
    let amount = vm.pop_int("irradiate")?;

    if r > 0 && amount > 0 {
        let (cy, cx) = vm.context_loc;
        let coords = vm.get_circular_coords(cx as i64, cy as i64, r);
        for (tx, ty) in coords {
            vm.mutagen_grid[ty][tx] = vm.mutagen_grid[ty][tx].saturating_add(amount);
        }
        let r_sq = (r as i128).saturating_mul(r as i128);
        let cost = (r_sq + 1).clamp(5, 50) as i64 + amount / 10;
        vm.energy = vm.energy.saturating_sub(cost);
        vm.output.push(format!(
            "IRRADIATE: Added {} mutagen at {},{} r={}",
            amount, cx, cy, r
        ));
    }
    None
}

fn exec_sense_mutagen(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    let (cy, cx) = vm.context_loc;
    let level = vm.mutagen_grid[cy][cx];
    vm.stack.push(Value::Int(level));
    None
}

fn exec_devour(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    let (cy, cx) = vm.context_loc;
    let level = vm.mutagen_grid[cy][cx];
    if level > 0 {
        vm.mutagen_grid[cy][cx] = 0;
        let energy_gain = level / 2;
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

fn exec_void_op(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if vm.organelles.len() >= crate::vm::MAX_ORGANELLES {
        vm.output
            .push("Error: Organelle limit exceeded".to_string());
        return None;
    }

    let (cy, cx) = vm.context_loc;
    vm.organelle_id_counter += 1;
    let organelle = Organelle {
        stack: Vec::new(),
        ip: (0, 0),
        context_loc: (cy, cx),
        call_stack: Vec::new(),
        recursion_depth: 0,
        halted: false,
        kind: OrganelleType::Void,
        direction: (0, 0),
        ttl: None,
        name: "Voidwalker".to_string(),
        traits: vec!["Nihilistic".to_string()],
        id: vm.organelle_id_counter,
        tissue_id: None,
        genome_id: 0,
        energy: 100,
        experience: 0,
        stage: 0,
    };
    vm.organelles.push(organelle);
    vm.energy = vm.energy.saturating_sub(50);
    vm.output.push(format!("VOID: Spawned at {},{}", cx, cy));
    None
}

fn exec_supernova(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    let s_idx = vm.ip.0;
    if s_idx < vm.dna.helix.strands.len() {
        let strand = vm.dna.helix.strands[s_idx].clone();
        vm.graveyard.push(strand);

        let genes = std::mem::take(&mut vm.dna.helix.strands[s_idx].genes);
        let mut rng = rand::thread_rng();

        let rows = vm.grid.len();
        let cols = if rows > 0 { vm.grid[0].len() } else { 0 };

        if rows > 0 && cols > 0 {
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
                            crate::ast::JunctionType::Dish => "dish",
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

                let rx = rng.gen_range(0..cols);
                let ry = rng.gen_range(0..rows);
                vm.grid[ry][rx] = Value::Str(s);
            }
        }

        vm.cladistics.kill_strand(s_idx, vm.tick_counter);
        vm.output
            .push(format!("SUPERNOVA: Strand {} exploded", s_idx));
        vm.halted = true;
    }
    None
}

fn exec_singularity(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    let mut merged_genes = Vec::new();
    let strands = std::mem::take(&mut vm.dna.helix.strands);
    for mut strand in strands {
        merged_genes.append(&mut strand.genes);
    }

    vm.dna.helix.strands.push(crate::ast::Strand {
        genes: merged_genes,
    });

    vm.telomeres = vec![100];
    #[cfg(feature = "cortex")]
    {
        vm.activation_levels = vec![0];
        vm.synapse_map = vec![Vec::new()];
    }
    vm.epigenome.clear();
    vm.market.clear();

    vm.ip = (0, 0);

    vm.output
        .push("SINGULARITY: All strands merged".to_string());
    None
}

fn exec_eval(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if let Some(val) = vm.stack.pop() {
        if let Value::Str(s) = val {
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

fn exec_map(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
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

        let result = Value::Junction(crate::ast::JunctionType::Any, results);
        if result.depth() > crate::vm::MAX_RECURSION_DEPTH {
            vm.output
                .push("Error: Map result depth limit exceeded".to_string());
        } else {
            vm.stack.push(result);
            vm.energy = vm.energy.saturating_sub(10);
        }
    } else {
        vm.output.push("Error: Stack underflow for map".to_string());
    }
    None
}

fn exec_fold(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
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
        }

        if acc.depth() > crate::vm::MAX_RECURSION_DEPTH {
            vm.output
                .push("Error: Fold result depth limit exceeded".to_string());
        } else {
            vm.stack.push(acc);
            vm.energy = vm.energy.saturating_sub(10);
        }
    } else {
        vm.output
            .push("Error: Stack underflow for fold".to_string());
    }
    None
}

fn exec_filter(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
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
                        vm.output
                            .push("Error: Junction size limit exceeded in Filter".to_string());
                        return None;
                    }
                    results.push(input);
                }
            }
        }

        let result = Value::Junction(crate::ast::JunctionType::Any, results);
        if result.depth() > crate::vm::MAX_RECURSION_DEPTH {
            vm.output
                .push("Error: Filter result depth limit exceeded".to_string());
        } else {
            vm.stack.push(result);
            vm.energy = vm.energy.saturating_sub(10);
        }
    } else {
        vm.output
            .push("Error: Stack underflow for filter".to_string());
    }
    None
}

fn exec_zip(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
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

        let result = Value::Junction(crate::ast::JunctionType::Any, results);
        if result.depth() > crate::vm::MAX_RECURSION_DEPTH {
            vm.output
                .push("Error: Zip result depth limit exceeded".to_string());
        } else {
            vm.stack.push(result);
            vm.energy = vm.energy.saturating_sub(5);
        }
    } else {
        vm.output.push("Error: Stack underflow for zip".to_string());
    }
    None
}

fn exec_pigment(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
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

fn exec_glyph(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
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

fn exec_glitch(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if let Some(val) = vm.stack.pop() {
        if let Value::Int(severity) = val {
            let mut rng = rand::thread_rng();
            let sev = severity.clamp(1, 100) as usize;

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

            if !vm.stack.is_empty() {
                let changes = sev.min(vm.stack.len());
                for _ in 0..changes {
                    let idx = rng.gen_range(0..vm.stack.len());
                    if rng.gen_bool(0.3) {
                        vm.stack[idx] = Value::Int(rng.gen_range(0..100));
                    }
                }
            }

            vm.glitch_level = (vm.glitch_level + (sev as f32 / 10.0)).clamp(0.0, 1.0);
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

fn exec_scramble(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    let mut rng = rand::thread_rng();
    vm.stack.shuffle(&mut rng);
    vm.energy = vm.energy.saturating_sub(10);
    vm.output.push("SCRAMBLE: Stack shuffled".to_string());
    None
}

fn exec_hyphae(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
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

fn exec_connect(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if vm.stack.len() >= 2 {
        let x_val = vm.stack.pop().unwrap();
        let y_val = vm.stack.pop().unwrap();
        if let (Value::Int(x), Value::Int(y)) = (x_val, y_val) {
            if let Some((ty, tx)) = vm.normalize_coords(y, x) {
                let (cy, cx) = vm.context_loc;
                if vm.mycelium.contains_key(&(cy, cx)) && vm.mycelium.contains_key(&(ty, tx)) {
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

fn exec_transport(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if vm.stack.len() >= 3 {
        let x_val = vm.stack.pop().unwrap();
        let y_val = vm.stack.pop().unwrap();
        let val = vm.stack.pop().unwrap();

        if let (Value::Int(x), Value::Int(y)) = (x_val, y_val) {
            if let Some((ty, tx)) = vm.normalize_coords(y, x) {
                let (cy, cx) = vm.context_loc;

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

fn exec_spore_cloud(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
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

fn exec_spirit(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if let Some(val) = vm.stack.pop() {
        if let Value::Str(msg) = val {
            vm.spirit_message = Some(msg);
        } else {
            vm.stack.push(val);
            vm.spirit_message = None;
        }
    } else {
        vm.spirit_message = None;
    }
    vm.spirit_request = true;
    None
}

fn exec_match(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
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

fn exec_bury(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if let Some(val) = vm.stack.pop() {
        match val {
            Value::Int(idx) => {
                let s_idx = idx as usize;
                if s_idx < vm.dna.helix.strands.len() {
                    if vm.graveyard.len() >= crate::vm::MAX_GRAVEYARD_SIZE {
                        vm.output.push("BURY: Graveyard limit reached".to_string());
                        return None;
                    }

                    let strand = vm.dna.helix.strands[s_idx].clone();
                    vm.graveyard.push(strand);

                    vm.dna.helix.strands[s_idx].genes.clear();
                    vm.epigenome.retain(|(s, _)| *s != s_idx);

                    vm.cladistics.kill_strand(s_idx, vm.tick_counter);

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

fn exec_exhume(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if let Some(strand) = vm.graveyard.pop() {
        vm.dna.helix.strands.push(strand);
        vm.telomeres.push(50);
        #[cfg(feature = "cortex")]
        {
            vm.activation_levels.push(0);
            vm.synapse_map.push(Vec::new());
        }

        let new_idx = vm.dna.helix.strands.len() - 1;

        vm.cladistics.register_strand(
            new_idx,
            Some(vm.ip.0),
            vm.tick_counter,
            "Exhume".to_string(),
        );

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

fn exec_seance(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
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

fn exec_mourn(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
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

fn exec_reincarnate(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if let Some(val) = vm.stack.pop() {
        match val {
            Value::Int(idx) => {
                let s_idx = idx as usize;
                if s_idx < vm.dna.helix.strands.len() {
                    let mut new_strand = vm.dna.helix.strands[s_idx].clone();
                    let mut rng = rand::thread_rng();

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

                    for gene in &mut new_strand.genes {
                        if rng.gen_bool(0.1) {
                            let new_op = enzymes[rng.gen_range(0..enzymes.len())].clone();
                            gene.op = new_op;
                        }
                    }

                    if vm.dna.helix.strands.len() >= MAX_STRANDS {
                        vm.output
                            .push("Error: Strand limit exceeded for reincarnate".to_string());
                        return None;
                    }
                    vm.dna.helix.strands.push(new_strand);
                    let new_idx = vm.dna.helix.strands.len() - 1;
                    vm.telomeres.push(50);
                    #[cfg(feature = "cortex")]
                    {
                        vm.activation_levels.push(0);
                        vm.synapse_map.push(Vec::new());
                    }

                    vm.cladistics.register_strand(
                        new_idx,
                        Some(s_idx),
                        vm.tick_counter,
                        "Reincarnate".to_string(),
                    );

                    vm.dna.helix.strands[s_idx].genes.clear();
                    vm.epigenome.retain(|(s, _)| *s != s_idx);

                    vm.cladistics.kill_strand(s_idx, vm.tick_counter);

                    vm.stack.push(Value::Int(new_idx as i64));

                    vm.energy = vm.energy.saturating_sub(50);
                    vm.output.push(format!(
                        "REINCARNATE: Strand {} reborn as {}",
                        s_idx, new_idx
                    ));
                } else {
                    vm.output
                        .push("Error: Strand index out of bounds for reincarnate".to_string());
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

fn exec_superpose(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if vm.stack.len() >= 2 {
        let b = vm.stack.pop().unwrap();
        let a = vm.stack.pop().unwrap();

        let depth_a = a.depth();
        let depth_b = b.depth();
        if depth_a.max(depth_b) + 1 > crate::vm::MAX_RECURSION_DEPTH {
            vm.output
                .push("Error: Superpose depth limit exceeded".to_string());
        } else {
            vm.stack
                .push(Value::Superposition(vec![(a, 0.5), (b, 0.5)]));
            vm.energy = vm.energy.saturating_sub(10);
        }
    } else {
        vm.output
            .push("Error: Stack underflow for superpose".to_string());
    }
    None
}

fn exec_collapse(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
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
                vm.stack.push(collapsed);
                vm.energy = vm.energy.saturating_sub(5);
            }
            other => {
                vm.stack.push(other);
            }
        }
    } else {
        vm.output
            .push("Error: Stack underflow for collapse".to_string());
    }
    None
}

fn exec_observe(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
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

fn exec_horcrux(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if vm.stack.len() >= 3 {
        let x_val = vm.stack.pop().unwrap();
        let y_val = vm.stack.pop().unwrap();
        let s_val = vm.stack.pop().unwrap();

        if let (Value::Int(s_idx), Value::Int(y), Value::Int(x)) = (s_val, y_val, x_val) {
            let idx = s_idx as usize;
            if idx < vm.dna.helix.strands.len() {
                if let Some((ny, nx)) = vm.normalize_coords(y, x) {
                    let strand = &vm.dna.helix.strands[idx];
                    let mut hasher = DefaultHasher::new();
                    strand.hash(&mut hasher);
                    let hash = hasher.finish();
                    let gene_str = super::nova_genetics::strand_to_string(strand);
                    let horcrux_str = format!("Horcrux:{:x}:{}", hash, gene_str);

                    vm.grid[ny][nx] = Value::Str(horcrux_str);

                    // Kill source strand
                    vm.dna.helix.strands[idx].genes.clear();
                    vm.epigenome.retain(|(s, _)| *s != idx);
                    vm.cladistics.kill_strand(idx, vm.tick_counter);

                    vm.energy = vm.energy.saturating_sub(50);
                    vm.output.push(format!(
                        "HORCRUX: Strand {} soul bound to {},{}",
                        idx, nx, ny
                    ));
                } else {
                    vm.output
                        .push("Error: Coordinates out of bounds for horcrux".to_string());
                }
            } else {
                vm.output
                    .push("Error: Invalid strand index for horcrux".to_string());
            }
        } else {
            vm.output
                .push("Error: Type mismatch for horcrux".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for horcrux".to_string());
    }
    None
}

fn exec_rebirth(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if vm.stack.len() >= 2 {
        let x_val = vm.stack.pop().unwrap();
        let y_val = vm.stack.pop().unwrap();

        if let (Value::Int(y), Value::Int(x)) = (y_val, x_val) {
            if let Some((ny, nx)) = vm.normalize_coords(y, x) {
                if let Value::Str(s) = &vm.grid[ny][nx] {
                    if s.starts_with("Horcrux:") {
                        let parts: Vec<&str> = s.splitn(3, ':').collect();
                        if parts.len() == 3 {
                            let _hash = parts[1];
                            let gene_src = parts[2];

                            match ChimeraParser::parse(Rule::strand, gene_src) {
                                Ok(mut pairs) => {
                                    let pair = pairs.next().unwrap();
                                    match crate::ast::Strand::try_from_pair(pair) {
                                        Ok(strand) => {
                                            if vm.dna.helix.strands.len() >= MAX_STRANDS {
                                                vm.output.push(
                                                    "REBIRTH ERROR: Strand limit exceeded"
                                                        .to_string(),
                                                );
                                            } else {
                                                vm.dna.helix.strands.push(strand);
                                                vm.telomeres.push(50);
                                                #[cfg(feature = "cortex")]
                                                {
                                                    vm.activation_levels.push(0);
                                                    vm.synapse_map.push(Vec::new());
                                                }
                                                let new_idx = vm.dna.helix.strands.len() - 1;

                                                vm.cladistics.register_strand(
                                                    new_idx,
                                                    Some(vm.ip.0),
                                                    vm.tick_counter,
                                                    "Rebirth".to_string(),
                                                );

                                                vm.grid[ny][nx] = Value::Int(0); // Consume Horcrux
                                                vm.stack.push(Value::Int(new_idx as i64));
                                                vm.energy = vm.energy.saturating_sub(25);
                                                vm.output.push(format!(
                                                    "REBIRTH: Soul restored as strand {}",
                                                    new_idx
                                                ));
                                            }
                                        }
                                        Err(e) => {
                                            vm.output
                                                .push(format!("REBIRTH ERROR: Parse failed {}", e));
                                        }
                                    }
                                }
                                Err(e) => {
                                    vm.output.push(format!("REBIRTH ERROR: Syntax error {}", e));
                                }
                            }
                        } else {
                            vm.output
                                .push("REBIRTH ERROR: Malformed Horcrux".to_string());
                        }
                    } else {
                        vm.output.push("REBIRTH: Not a Horcrux".to_string());
                        vm.stack.push(Value::Int(-1));
                    }
                } else {
                    vm.output
                        .push("REBIRTH: Cell does not contain string".to_string());
                    vm.stack.push(Value::Int(-1));
                }
            } else {
                vm.output
                    .push("Error: Coordinates out of bounds for rebirth".to_string());
            }
        } else {
            vm.output
                .push("Error: Type mismatch for rebirth".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for rebirth".to_string());
    }
    None
}

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
