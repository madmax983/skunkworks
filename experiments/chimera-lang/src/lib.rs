//! # Chimera Lang 🧬
//!
//! A biologically inspired esoteric programming language where code is DNA, memory is a grid, and execution is survival.
//!
//! ## Overview
//!
//! Chimera Lang simulates a virtual organism. Its "DNA" (code) is composed of strands of genes (instructions).
//! The organism executes these genes to manipulate its internal state, interact with a 2D grid environment,
//! and manage its energy levels to avoid starvation.
//!
//! ## Key Concepts
//!
//! - **DNA**: The program. Mutable at runtime (self-modifying code).
//! - **Grid**: 16x16 2D memory. Cells can store Integers or Strings (OpCodes).
//! - **Energy**: Every operation costs energy. `Photosynthesize` or `Consume` to survive.
//! - **Evolution**: Random mutations can alter the code during execution.
//!
//! ## Features
//!
//! - **Nova**: Adds advanced biological features like Epigenetics, Spores (Time Travel), and CRISPR (Search/Replace).
//! - **Cortex**: Adds neural network capabilities, allowing the organism to "learn" and "sense" its environment.
//! - **Biophysics**: Adds Hodgkin-Huxley neuron simulation for realistic electrophysiology.

use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct ChimeraParser;

pub mod ast;
pub mod compiler;
pub mod opcode;
pub mod tui;
pub mod vm;

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
mod havoc_repro;
#[cfg(all(test, feature = "nova"))]
mod microscope_test;
#[cfg(all(test, feature = "nova"))]
mod nova_akashic_test;
#[cfg(all(test, feature = "nova"))]
mod nova_atmosphere_test;
#[cfg(all(test, feature = "nova"))]
mod nova_botany_test;
#[cfg(all(test, feature = "nova"))]
mod nova_alchemy_test;
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
mod nova_camouflage_test;
#[cfg(all(test, feature = "nova"))]
mod nova_compose_test;
#[cfg(all(test, feature = "nova"))]
mod nova_cerebellum_test;
#[cfg(all(test, feature = "nova"))]
mod nova_chorus_test;
#[cfg(all(test, feature = "nova"))]
mod nova_chroma_test;
#[cfg(all(test, feature = "nova"))]
mod nova_conjugation_test;
#[cfg(all(test, feature = "nova"))]
mod nova_crystal_test;
#[cfg(all(test, feature = "nova"))]
mod nova_crispr_test;
#[cfg(all(test, feature = "nova"))]
mod nova_differentiation_test;
#[cfg(all(test, feature = "nova"))]
mod nova_dream_test;
#[cfg(all(test, feature = "nova"))]
mod nova_echo_test;
#[cfg(all(test, feature = "nova"))]
mod nova_dream_trace_test;
#[cfg(all(test, feature = "nova"))]
mod nova_fungi_test;
#[cfg(all(test, feature = "nova"))]
mod nova_gravity_test;
#[cfg(all(test, feature = "nova"))]
mod nova_relativity_test;
#[cfg(all(test, feature = "nova"))]
mod nova_hormone_test;
#[cfg(all(test, feature = "nova"))]
mod nova_ipc_test;
#[cfg(all(test, feature = "nova"))]
mod nova_isomer_test;
#[cfg(all(test, feature = "nova"))]
mod nova_linguistics_test;
#[cfg(all(test, feature = "nova"))]
mod nova_market_test;
#[cfg(all(test, feature = "nova"))]
mod nova_madness_test;
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
#[cfg(all(test, feature = "resonance"))]
mod nova_resonance_test;
#[cfg(all(test, feature = "nova", feature = "resonance"))]
mod nova_cymatics_test;
#[cfg(all(test, feature = "nova"))]
mod nova_security_test;
#[cfg(all(test, feature = "nova"))]
mod nova_signals_test;
#[cfg(all(test, feature = "nova"))]
mod nova_sigil_dynamic_test;
#[cfg(all(test, feature = "nova"))]
mod nova_sigil_test;
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
mod nova_alchemy_crucible_test;
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
