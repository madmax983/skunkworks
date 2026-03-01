#![cfg(feature = "nova")]
//! # Nova Extension 🌌
//!
//! The `Nova` module implements advanced biological and physics-defying capabilities for the Chimera VM.
//! It transforms the VM from a simple execution engine into a complex, evolving ecosystem.
//!
//! ## 📖 The Book of Nova
//!
//! In the beginning, there was only the Grid and the Stack. Then came **Nova**, bringing life,
//! time travel, and entropy to the machine.
//!
//! ### 1. The Living Machine (Organelles)
//! Unlike standard threads, **Organelles** are semi-autonomous agents that live on the Grid.
//! They have their own purpose:
//! - **Mitochondria** generate energy.
//! - **Ribosomes** execute code found on the grid (spatial programming).
//! - **Chloroplasts** harvest light.
//! - **The Void** consumes matter to fuel entropy.
//!
//! ### 2. The Fabric of Reality (Phases)
//! An organism can shift its physical state:
//! - **Corporeal**: Solid. Respects walls.
//! - **Ethereal**: Ghost-like. Passes through walls but cannot touch matter.
//! - **Crystalline**: Frozen. Immune to time and mutation, but immobile.
//! - **Flux**: Chaos energy. Fast, unstable, consuming immense power.
//!
//! ### 3. The Flow of Time (Chronos)
//! Nova allows manipulation of the execution timeline:
//! - **Spores** act as save states, preserving the VM state in a dormant seed.
//! - **Prophecy** allows looking into the future to avoid death.
//! - **Time Loops** reset the state while keeping memory.
//!
//! ## Key Features
//!
//! - **Organelles**: Specialized sub-processes (Mitochondria, Ribosomes, Void) that run in parallel.
//! - **Time Travel**: `Sporulate` creates full VM snapshots; `Germinate` restores them.
//! - **Epigenetics**: `Methylate`/`Demethylate` modify gene expression without changing DNA.
//! - **Spatial Physics**: Diffusion of hormones, waste, light, and mutagen across the grid.
//! - **Quantum Entanglement**: Linked strands that share mutations.
//! - **Phases of Matter**: Shift between Corporeal, Ethereal (pass walls), Crystalline (immobile), and Flux (fast).

use super::{ChimeraVM, Value};
use crate::ast::Nucleotide;
use crate::opcode::OpCode;
use crate::{ChimeraParser, Rule};
use pest::Parser;
use rand::seq::SliceRandom;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::{HashSet, VecDeque};

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
    /// Acts as a "living read head" on the grid.
    Ribosome,
    /// Consumes the grid cell (turns it to 0) and moves randomly (Brownian motion).
    Void,
    /// Transmutes neighbors based on elemental recipes.
    Alchemist,
    /// Grows procedurally based on L-System rules.
    Seed,
    /// Sings a song repeatedly, triggering global effects via Chorus chords.
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
///
/// # Examples
///
/// ```ignore
/// // Spawning a Mitochondria at (5, 5)
/// let organelle = Organelle {
///     kind: OrganelleType::Mitochondria,
///     context_loc: (5, 5),
///     // ... other fields default
/// };
/// vm.organelles.push(organelle);
/// ```
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
    /// The specialization type (e.g., Chloroplast, Mitochondria).
    pub kind: OrganelleType,
    /// Movement vector (dy, dx) used by some organelles (e.g. Ribosome, Void).
    pub direction: (i8, i8),
    /// Time To Live. If `Some(0)`, the organelle dies.
    pub ttl: Option<usize>,
    /// Display name (flavor text).
    pub name: String,
    /// List of acquired traits or buffs.
    pub traits: Vec<String>,
    /// Unique identifier for this organelle.
    pub id: u64,
    /// ID of the tissue/colony this organelle belongs to (for Metazoan behavior).
    pub tissue_id: Option<usize>,
    /// Hash of the source genome (for identification).
    pub genome_id: u64,
    /// Internal energy reserve (currently unused/vestigial).
    /// Organelles draw from the main VM `energy` pool.
    pub energy: i64,
    /// Accumulated experience points (for leveling up behavior).
    pub experience: i64,
    /// Growth stage (0=Larva, 1=Adult, etc.).
    pub stage: u8,
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
            if buffer_slice == chord_slice.as_slice() && *strand_idx < vm.dna.helix.strands.len() {
                vm.chorus_buffer.clear();
                vm.output
                    .push(format!("CHORUS: Triggered spell -> Strand {}", strand_idx));
                return Some(*strand_idx);
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
        OpCode::Prophecy => super::nova_simulation::exec_prophecy(vm),
        OpCode::EgregoreLink => super::nova_egregore::exec_egregore_link(vm),
        OpCode::Lucid => super::nova_simulation::exec_lucid(vm),
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
        OpCode::QuantumJump => super::nova_quantum::exec_quantum_jump(vm),
        OpCode::Chronos => super::nova_chronos::exec_chronos(vm),
        OpCode::Retroscope => super::nova_relativity::exec_retroscope(vm),
        OpCode::Relativity => super::nova_physics::exec_relativity(vm),
        OpCode::Graviton => super::nova_physics::exec_graviton(vm),
        OpCode::EventHorizon => super::nova_physics::exec_event_horizon(vm),
        OpCode::Aeolus => super::nova_fluid::exec_aeolus(vm, op, args),
        OpCode::Storm => super::nova_fluid::exec_storm(vm, op, args),
        OpCode::Tsunami => super::nova_fluid::exec_tsunami(vm, op, args),
        OpCode::Dry => super::nova_fluid::exec_dry(vm, op, args),
        OpCode::SenseWind => super::nova_physics::exec_sense_wind(vm),
        OpCode::SenseMoisture => super::nova_physics::exec_sense_moisture(vm),
        OpCode::Terraform => super::nova_physics::exec_terraform(vm),
        OpCode::SenseBiome => super::nova_physics::exec_sense_biome(vm),
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
        OpCode::SelfReplicate => super::nova_genetics::exec_self_replicate(vm),
        OpCode::Meme => super::nova_genetics::exec_meme(vm),
        OpCode::Drift => super::nova_genetics::exec_drift(vm),
        OpCode::Poly => super::nova_genetics::exec_poly(vm, args),
        OpCode::Metamorphosis => super::nova_genetics::exec_metamorphosis(vm),
        OpCode::Genesis => super::nova_genetics::exec_genesis(vm),
        OpCode::Chaos => super::nova_flux::exec_chaos(vm),
        OpCode::Orca => {
            vm.prologue_state.orca_mode = !vm.prologue_state.orca_mode;
            let status = if vm.prologue_state.orca_mode {
                "ON"
            } else {
                "OFF"
            };
            vm.output.push(format!("PROLOGUE: Orca Mode {}", status));
            None
        }
        OpCode::Synthesize => super::catalyst::synthesize(vm),
        OpCode::Catalyze => super::catalyst::catalyze(vm),
        OpCode::Piet => exec_piet(vm),
        OpCode::Chronostasis => super::nova_chronos::exec_chronostasis(vm),
        OpCode::Simulate => super::nova_simulation::exec_simulate(vm),
        OpCode::SensePigment => exec_sense_pigment(vm),
        OpCode::SenseGlyph => exec_sense_glyph(vm),
        OpCode::Sing => exec_sing(vm),
        OpCode::Listen => exec_listen(vm),
        OpCode::Brainfuck => super::nova_brainfuck::exec_brainfuck(vm),
        OpCode::Spawn => super::nova_biology::exec_spawn(vm),
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
        OpCode::Secrete => super::nova_biology::exec_secrete(vm),
        OpCode::Detect => super::nova_biology::exec_detect(vm),
        OpCode::Absorb => super::nova_biology::exec_absorb(vm),
        OpCode::Migrate => super::nova_physics::exec_migrate(vm),
        OpCode::Detox => super::nova_biology::exec_detox(vm),
        OpCode::WRead => exec_w_read(vm),
        OpCode::Call => exec_call(vm, args),
        OpCode::Exec => exec_exec(vm),
        OpCode::Ret => exec_ret(vm),
        OpCode::Bind => exec_bind(vm),
        OpCode::Unbind => exec_unbind(vm),
        OpCode::Entangle => super::nova_quantum::exec_entangle(vm),
        OpCode::Decohere => super::nova_quantum::exec_decohere(vm),
        OpCode::Conjugate => exec_conjugate(vm),
        OpCode::Gravitate => super::nova_physics::exec_gravitate(vm),
        OpCode::Lumine => exec_lumine(vm),
        OpCode::SenseLight => exec_sense_light(vm),
        OpCode::Dream => super::nova_simulation::exec_dream(vm),
        OpCode::Chemotaxis => super::nova_biology::exec_chemotaxis(vm),
        OpCode::Identity => super::nova_biology::exec_identity(vm),
        OpCode::Differentiate => super::nova_biology::exec_differentiate(vm),
        OpCode::Shape => super::nova_physics::exec_shape(vm),
        OpCode::Rift => super::nova_physics::exec_rift(vm),
        OpCode::Seal => super::nova_physics::exec_seal(vm),
        OpCode::Sonar => exec_sonar(vm),
        OpCode::LispEval => super::nova_simulation::exec_lisp_eval(vm),
        OpCode::Broadcast => exec_broadcast(vm),
        OpCode::Tune => exec_tune(vm),
        OpCode::Isomerize => super::nova_physics::exec_isomerize(vm),
        OpCode::PhaseShift => super::nova_physics::exec_phase_shift(vm),
        OpCode::Membrane => super::nova_physics::exec_membrane(vm),
        OpCode::Osmosis => super::nova_physics::exec_osmosis(vm),
        OpCode::Symbiosis => super::nova_biology::exec_symbiosis(vm),
        OpCode::Reflex => exec_reflex(vm),
        OpCode::Lysis => super::nova_biology::exec_lysis(vm),
        OpCode::Compile => super::nova_genetics::exec_compile(vm),
        OpCode::Irradiate => super::nova_biology::exec_irradiate(vm),
        OpCode::SenseMutagen => super::nova_biology::exec_sense_mutagen(vm),
        OpCode::Devour => super::nova_biology::exec_devour(vm),
        OpCode::Decompile => super::nova_genetics::exec_decompile(vm),
        OpCode::Void => exec_void_op(vm),
        OpCode::VoidRift => super::nova_void::exec_void_rift(vm),
        OpCode::VoidCast => super::nova_void::exec_void_cast(vm),
        OpCode::Reactor => super::nova_reactor::exec_reactor(vm),
        OpCode::Reaction => super::nova_reactor::exec_reaction(vm),
        OpCode::Cambrian => super::nova_ecology::cambrian_explosion(vm),
        OpCode::Forge | OpCode::Speak | OpCode::Etymology => exec_verbum_op(vm, op, args),
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
        OpCode::Spirit => super::nova_biology::exec_spirit(vm),
        OpCode::Match => super::nova_biology::exec_match(vm),
        OpCode::Bury => super::nova_biology::exec_bury(vm),
        OpCode::Exhume => super::nova_biology::exec_exhume(vm),
        OpCode::Seance => super::nova_biology::exec_seance(vm),
        OpCode::Mourn => super::nova_biology::exec_mourn(vm),
        OpCode::Reincarnate => super::nova_biology::exec_reincarnate(vm),
        OpCode::Superpose => super::nova_quantum::exec_superpose(vm),
        OpCode::Collapse => super::nova_quantum::exec_collapse(vm),
        OpCode::Observe => super::nova_quantum::exec_observe(vm),
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
        OpCode::Horcrux => super::nova_quantum::exec_horcrux(vm),
        OpCode::Rebirth => super::nova_quantum::exec_rebirth(vm),
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

pub fn glob_match(pattern: &str, target: &str) -> bool {
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

fn exec_lumine(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if vm.stack.len() >= 2 {
        let intensity_val = vm.stack.pop().unwrap();
        let radius_val = vm.stack.pop().unwrap();
        if let (Value::Int(r), Value::Int(intensity)) = (radius_val, intensity_val) {
            if r > 0 && intensity > 0 {
                let (cy, cx) = vm.context_loc;
                let mut count = 0;
                crate::vm::iterate_circle(
                    #[cfg(feature = "nova")]
                    vm.topology,
                    cx as i64,
                    cy as i64,
                    r,
                    |tx, ty| {
                        vm.light_grid[ty][tx] = vm.light_grid[ty][tx].saturating_add(intensity);
                        count += 1;
                    },
                );
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

fn exec_verbum_op(vm: &mut ChimeraVM, op: OpCode, _args: &[Nucleotide]) -> Option<(usize, usize)> {
    match op {
        OpCode::Forge => {
            if vm.stack.len() >= 2 {
                let strand_val = vm.stack.pop().unwrap();
                let name_val = vm.stack.pop().unwrap();
                if let (Value::Str(name), Value::Int(idx)) = (name_val, strand_val) {
                    let s_idx = idx as usize;
                    if s_idx < vm.dna.helix.strands.len() {
                        let genes = vm.dna.helix.strands[s_idx].genes.clone();
                        // For simplicity, parent is empty for now.
                        match vm.verbum_forge.forge(name.clone(), genes, vec![]) {
                            Ok(id) => {
                                vm.stack.push(Value::Int(id as i64));
                                vm.output.push(format!("FORGE: Created word '{}'", name));
                            }
                            Err(e) => {
                                vm.output.push(format!("FORGE ERROR: {}", e));
                                vm.stack.push(Value::Int(-1));
                            }
                        }
                    } else {
                        vm.output.push("FORGE: Invalid strand index".to_string());
                    }
                } else {
                    vm.output.push("FORGE: Type mismatch".to_string());
                }
            } else {
                vm.output.push("FORGE: Stack underflow".to_string());
            }
        }
        OpCode::Speak => {
            if let Some(val) = vm.stack.pop() {
                if let Value::Str(name) = val {
                    let word_data = vm
                        .verbum_forge
                        .get(&name)
                        .map(|word| (word.genes.clone(), word.cost));

                    if let Some((genes, cost)) = word_data {
                        if vm.energy < cost {
                            vm.output
                                .push(format!("SPEAK: Insufficient energy for '{}'", name));
                        } else {
                            vm.energy -= cost;
                            vm.output.push(format!("SPEAK: Uttered '{}'", name));
                            let strand = crate::ast::Strand { genes };
                            super::nova_simulation::execute_ephemeral_strand(vm, &strand);
                        }
                    } else {
                        vm.output
                            .push(format!("SPEAK ERROR: Word '{}' unknown", name));
                    }
                } else {
                    vm.output.push("SPEAK: Type mismatch".to_string());
                }
            } else {
                vm.output.push("SPEAK: Stack underflow".to_string());
            }
        }
        OpCode::Etymology => {
            if let Some(val) = vm.stack.pop() {
                if let Value::Str(name) = val {
                    if let Some(word) = vm.verbum_forge.get(&name) {
                        let op_strings: Vec<Value> = word
                            .genes
                            .iter()
                            .map(|g| Value::Str(g.op.to_string()))
                            .collect();
                        vm.stack
                            .push(Value::Junction(crate::ast::JunctionType::All, op_strings));
                    } else {
                        vm.output
                            .push(format!("ETYMOLOGY: Unknown word '{}'", name));
                        vm.stack.push(Value::Int(0));
                    }
                } else {
                    vm.output.push("ETYMOLOGY: Type mismatch".to_string());
                }
            } else {
                vm.output.push("ETYMOLOGY: Stack underflow".to_string());
            }
        }
        _ => {}
    }
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
                            super::nova_simulation::execute_ephemeral_strand(vm, &strand);
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

        if target_val.depth() >= crate::vm::MAX_RECURSION_DEPTH {
            vm.output.push("Error: Input too deep for Map".to_string());
            return None;
        }

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
                            Ok(strand) => {
                                super::nova_simulation::execute_ephemeral_strand(vm, &strand)
                            }
                            Err(e) => vm.output.push(format!("MAP ERROR: {}", e)),
                        }
                    } else {
                        vm.output.push(format!("MAP ERROR: Parse failed for {}", s));
                    }
                }
                Value::Int(idx) => {
                    super::nova_simulation::execute_strand_sync(vm, *idx as usize);
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

        if target_val.depth() >= crate::vm::MAX_RECURSION_DEPTH
            || init_val.depth() >= crate::vm::MAX_RECURSION_DEPTH
        {
            vm.output.push("Error: Input too deep for Fold".to_string());
            return None;
        }

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
                            super::nova_simulation::execute_ephemeral_strand(vm, &strand);
                        }
                    }
                }
                Value::Int(idx) => {
                    super::nova_simulation::execute_strand_sync(vm, *idx as usize);
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

        if target_val.depth() >= crate::vm::MAX_RECURSION_DEPTH {
            vm.output
                .push("Error: Input too deep for Filter".to_string());
            return None;
        }

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
                            super::nova_simulation::execute_ephemeral_strand(vm, &strand);
                        }
                    }
                }
                Value::Int(idx) => {
                    super::nova_simulation::execute_strand_sync(vm, *idx as usize);
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

        if val_a.depth() >= crate::vm::MAX_RECURSION_DEPTH
            || val_b.depth() >= crate::vm::MAX_RECURSION_DEPTH
        {
            vm.output.push("Error: Input too deep for Zip".to_string());
            return None;
        }

        if val_a.complexity() + val_b.complexity() > crate::vm::MAX_COMPLEXITY {
            vm.output
                .push("Error: Input too complex for Zip".to_string());
            return None;
        }

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

            let mut count = 0;
            crate::vm::iterate_circle(
                #[cfg(feature = "nova")]
                vm.topology,
                cx as i64,
                cy as i64,
                r,
                |tx, ty| {
                    if rng.gen_range(0..100) < d {
                        if let std::collections::hash_map::Entry::Vacant(e) =
                            vm.mycelium.entry((ty, tx))
                        {
                            e.insert(Vec::new());
                            count += 1;
                        }
                    }
                },
            );
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
