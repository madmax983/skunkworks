//! # Chimera Lang 🧬
//!
//! Chimera is a **biologically-inspired esoteric programming language** that simulates a living organism.
//! In Chimera, code is DNA, memory is a Petri Dish, and execution is a struggle for survival.
//!
//! Unlike traditional VMs where the goal is efficient calculation, the goal of a Chimera organism is
//! to maintain its **Energy** levels, replicate via **Mitosis**, and evolve through **Mutation**.
//!
//! ## 📖 The Book of Chimera
//!
//! ### The Biological Metaphor
//!
//! | Computer Concept | Biological Metaphor | Description |
//! |------------------|---------------------|-------------|
//! | **Program**      | **DNA**             | A Helix of Strands. Read-only by default, but mutable via enzymes. |
//! | **Function**     | **Strand**          | A sequence of Genes (Instructions). |
//! | **Instruction**  | **Gene/Enzyme**     | An atomic operation (e.g., `push`, `add`, `photosynthesize`). |
//! | **Memory**       | **Petri Dish**      | A 16x16 2D Grid. Cells contain Integers, Strings, or Organelles. |
//! | **Fuel/Cycles**  | **Energy**          | Every operation costs energy. Hitting 0 means Death (Halt). |
//! | **Thread**       | **Organelle**       | Independent agents spawned by the nucleus to perform tasks. |
//! | **Exception**    | **Reflex**          | Automatic interrupts triggered by pain (damage) or hunger. |
//!
//! ### The Virtual Machine Architecture
//!
//! The `ChimeraVM` is a Stack-based machine with a Twist:
//!
//! 1.  **The Stack**: Standard LIFO structure for data manipulation.
//! 2.  **The Helix**: The code storage. Contains multiple Strands.
//! 3.  **The Grid**: A spatial memory system. Instructions like `radiate` and `siphon` operate on circular areas.
//! 4.  **The Environment**: A simulation layer handling Light, Waste, Hormones, and Entropy.
//!
//! ### Survival Guide (Quick Start)
//!
//! A minimal organism must generate energy to stay alive.
//!
//! ```rust
//! use chimera_lang::prelude::*;
//!
//! // DNA: [ photosynthesize(), jump(0) ]
//! // This organism sits in the sun and loops forever.
//! let genes = vec![
//!     Gene { op: OpCode::Photosynthesize, args: vec![] },
//!     Gene { op: OpCode::Jump, args: vec![Nucleotide::Number(0)] },
//! ];
//!
//! let dna = Dna { helix: Helix { strands: vec![Strand { genes }] } };
//! let mut vm = ChimeraVM::new(dna);
//!
//! // Run for 100 ticks
//! for _ in 0..100 {
//!     vm.step();
//!     assert!(vm.energy > 0); // Still alive!
//! }
//! ```
//!
//! ## 🛠️ Feature Flags
//!
//! Chimera is modular. Advanced features are gated to reduce compile time and complexity.
//!
//! - **`nova`** (Default): The "Expansion Pack". Adds Organelles, Epigenetics, Time Travel, Physics, and the Market.
//!   *Enables:* `OpCode::Spawn`, `OpCode::Sporulate`, `OpCode::Gravity`, etc.
//! - **`cortex`**: Adds Neural Networks. Strands can be linked by synapses, firing "spikes" to trigger execution.
//!   *Enables:* `OpCode::Link`, `OpCode::Spark`, `OpCode::Sense`.
//! - **`biophysics`**: Adds realistic Hodgkin-Huxley neuron simulation for electrophysiology experiments.
//!   *Enables:* `OpCode::NeuroGenesis`, `OpCode::Stimulate`.
//! - **`silicon`**: Adds Wireworld cellular automata for building logic gates on the grid.
//!   *Enables:* `OpCode::Wire`, `OpCode::Conduct`, `OpCode::Latch`.
//! - **`git`**: Allows the organism to inspect the git repository it lives in (Ancestry).
//!   *Enables:* `OpCode::Excavate`, `OpCode::Evolution`.
//!
//! ## 🧬 The Genetic Code
//!
//! See [`opcode::OpCode`] for the complete list of enzymes available to the organism.

use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct ChimeraParser;

pub mod ast;
pub mod compiler;
pub mod opcode;
pub mod tui;
pub mod vm;

/// Common imports for Chimera Language.
///
/// Use `use chimera_lang::prelude::*;` to import common types.
pub mod prelude {
    pub use crate::ast::{Dna, Gene, Helix, JunctionType, Nucleotide, Strand};
    pub use crate::opcode::OpCode;
    pub use crate::vm::{ChimeraVM, Value};
}

#[cfg(all(test, feature = "nova"))]
mod babel_test;
#[cfg(all(test, feature = "biophysics"))]
mod biophysics_test;
#[cfg(all(test, feature = "nova"))]
mod chaos_test;
#[cfg(all(test, feature = "nova"))]
mod cladistics_test;
#[cfg(test)]
mod cortex_test;
#[cfg(all(test, feature = "biophysics"))]
mod cortex_view_test;
#[cfg(all(test, feature = "nova"))]
mod havoc_poly_crash;
#[cfg(all(test, feature = "nova"))]
mod havoc_repro;
#[cfg(all(test, feature = "nova"))]
mod microscope_test;
#[cfg(all(test, feature = "nova"))]
mod nova_akashic_test;
#[cfg(all(test, feature = "nova"))]
mod nova_alchemy_crucible_test;
#[cfg(all(test, feature = "nova"))]
mod nova_alchemy_test;
#[cfg(all(test, feature = "nova"))]
mod nova_astrology_test;
#[cfg(all(test, feature = "nova"))]
mod nova_atmosphere_test;
#[cfg(all(test, feature = "nova"))]
mod nova_ballistics_test;
#[cfg(all(test, feature = "nova"))]
mod nova_bang_test;
#[cfg(all(test, feature = "nova"))]
mod nova_bard_test;
#[cfg(all(test, feature = "nova"))]
mod nova_biolum_test;
#[cfg(all(test, feature = "nova"))]
mod nova_blackbox_test;
#[cfg(all(test, feature = "nova"))]
mod nova_botany_test;
#[cfg(all(test, feature = "nova"))]
mod nova_bureaucracy_test;
#[cfg(all(test, feature = "nova"))]
mod nova_camouflage_test;
#[cfg(all(test, feature = "nova"))]
mod nova_cartography_test;
#[cfg(all(test, feature = "nova"))]
mod nova_cerebellum_test;
#[cfg(all(test, feature = "nova"))]
mod nova_chemistry_test;
#[cfg(all(test, feature = "nova"))]
mod nova_chorus_test;
#[cfg(all(test, feature = "nova"))]
mod nova_chroma_test;
#[cfg(all(test, feature = "nova"))]
mod nova_compose_test;
#[cfg(all(test, feature = "nova"))]
mod nova_conjugation_test;
#[cfg(all(test, feature = "nova"))]
mod nova_crispr_test;
#[cfg(all(test, feature = "nova"))]
mod nova_crystal_test;
#[cfg(all(test, feature = "nova", feature = "resonance"))]
mod nova_cymatics_test;
#[cfg(all(test, feature = "nova"))]
mod nova_differentiation_test;
#[cfg(all(test, feature = "nova"))]
mod nova_dream_test;
#[cfg(all(test, feature = "nova"))]
mod nova_dream_trace_test;
#[cfg(all(test, feature = "nova"))]
mod nova_fungi_test;
#[cfg(all(test, feature = "nova"))]
mod nova_gastronomy_test;
#[cfg(all(test, feature = "nova"))]
mod nova_gravity_test;
#[cfg(all(test, feature = "nova"))]
mod nova_hormone_test;
#[cfg(all(test, feature = "nova"))]
mod nova_ipc_test;
#[cfg(all(test, feature = "nova"))]
mod nova_isomer_test;
#[cfg(all(test, feature = "nova"))]
mod nova_linguistics_test;
#[cfg(all(test, feature = "nova"))]
mod nova_logistics_test;
#[cfg(all(test, feature = "nova"))]
mod nova_madness_test;
#[cfg(all(test, feature = "nova"))]
mod nova_market_test;
#[cfg(all(test, feature = "nova"))]
mod nova_metamorphism_test;
#[cfg(all(test, feature = "nova"))]
mod nova_membrane_test;
#[cfg(all(test, feature = "nova"))]
mod nova_meta_test;
#[cfg(all(test, feature = "nova"))]
mod nova_morphogenesis_test;
#[cfg(all(test, feature = "nova"))]
mod nova_mutagen_test;
#[cfg(all(test, feature = "nova"))]
mod nova_necromancy_test;
#[cfg(all(test, feature = "oracle"))]
mod nova_omen_test;
#[cfg(all(test, feature = "nova"))]
mod nova_organelle_test;
#[cfg(all(test, feature = "nova"))]
mod nova_organelle_types_test;
#[cfg(all(test, feature = "nova"))]
mod nova_phase_test;
#[cfg(all(test, feature = "nova"))]
mod nova_piet_test;
#[cfg(all(test, feature = "nova"))]
mod nova_portal_test;
#[cfg(all(test, feature = "nova"))]
mod nova_probabilistic_test;
#[cfg(all(test, feature = "nova"))]
mod nova_quantum_test;
#[cfg(all(test, feature = "nova"))]
mod nova_radio_test;
#[cfg(all(test, feature = "nova"))]
mod nova_reflex_test;
#[cfg(all(test, feature = "nova"))]
mod nova_relativity_test;
#[cfg(all(test, feature = "resonance"))]
mod nova_resonance_test;
#[cfg(all(test, feature = "nova"))]
mod nova_resonance_war_test;
#[cfg(all(test, feature = "nova"))]
mod nova_scent_test;
#[cfg(all(test, feature = "nova"))]
mod nova_security_test;
#[cfg(all(test, feature = "nova"))]
mod nova_sigil_dynamic_test;
#[cfg(all(test, feature = "nova"))]
mod nova_sigil_test;
#[cfg(all(test, feature = "nova"))]
mod nova_signals_test;
#[cfg(all(test, feature = "nova"))]
mod nova_simulate_test;
#[cfg(all(test, feature = "nova"))]
mod nova_sonar_test;
#[cfg(all(test, feature = "nova"))]
mod nova_splice_test;
#[cfg(all(test, feature = "nova"))]
mod nova_spore_test;
#[cfg(all(test, feature = "nova"))]
mod nova_superposition_test;
#[cfg(all(test, feature = "nova"))]
mod nova_symbiosis_test;
#[cfg(all(test, feature = "nova"))]
mod nova_taxis_test;
#[cfg(all(test, feature = "nova"))]
mod nova_test;
#[cfg(all(test, feature = "nova"))]
mod nova_topology_test;
#[cfg(all(test, feature = "nova"))]
mod nova_true_alchemy_test;
#[cfg(all(test, feature = "nova"))]
mod nova_virus_test;
#[cfg(all(test, feature = "nova"))]
mod nova_void_test;
#[cfg(all(test, feature = "nova"))]
mod nova_waste_test;
#[cfg(all(test, feature = "oracle"))]
mod oracle_test;
#[cfg(all(test, feature = "nova"))]
mod prion_test;
#[cfg(all(test, feature = "resonance"))]
mod resonance_test;
#[cfg(all(test, feature = "nova"))]
mod ribosome_test;
#[cfg(all(test, feature = "nova"))]
mod ribozyme_test;
#[cfg(all(test, feature = "nova"))]
mod sentry_brainfuck_test;
#[cfg(test)]
mod sentry_nova_test;
#[cfg(all(test, feature = "silicon"))]
mod silicon_test;
#[cfg(all(test, feature = "nova"))]
mod song_test;
#[cfg(test)]
mod warden_dos_test;
#[cfg(test)]
mod warden_exploit_test;
#[cfg(test)]
mod warden_memory_test;
#[cfg(test)]
mod warden_parser_test;
#[cfg(test)]
mod warden_phylogeny_test;
#[cfg(all(test, feature = "elektra"))]
mod elektra_test;
