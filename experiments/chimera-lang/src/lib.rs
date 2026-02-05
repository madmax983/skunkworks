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

use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct ChimeraParser;

pub mod ast;
pub mod opcode;
pub mod tui;
pub mod vm;

#[cfg(test)]
mod cortex_test;
mod havoc_repro;
#[cfg(all(test, feature = "nova"))]
mod nova_biolum_test;
#[cfg(test)]
mod nova_cerebellum_test;
#[cfg(all(test, feature = "nova"))]
mod nova_conjugation_test;
#[cfg(test)]
mod nova_crispr_test;
#[cfg(all(test, feature = "nova"))]
mod nova_differentiation_test;
#[cfg(all(test, feature = "nova"))]
mod nova_dream_test;
#[cfg(all(test, feature = "nova"))]
mod nova_gravity_test;
#[cfg(test)]
mod nova_hormone_test;
#[cfg(all(test, feature = "nova"))]
mod nova_membrane_test;
#[cfg(all(test, feature = "nova"))]
mod nova_organelle_test;
#[cfg(all(test, feature = "nova"))]
mod nova_organelle_types_test;
#[cfg(all(test, feature = "nova"))]
mod nova_portal_test;
#[cfg(test)]
mod nova_quantum_test;
#[cfg(all(test, feature = "nova"))]
mod nova_radio_test;
#[cfg(all(test, feature = "nova"))]
mod nova_simulate_test;
#[cfg(all(test, feature = "nova"))]
mod nova_sonar_test;
#[cfg(test)]
mod nova_spore_test;
#[cfg(all(test, feature = "nova"))]
mod nova_taxis_test;
mod nova_test;
#[cfg(all(test, feature = "nova"))]
mod nova_topology_test;
#[cfg(test)]
mod nova_waste_test;
#[cfg(all(test, feature = "nova"))]
mod ribosome_test;
#[cfg(test)]
mod sentry_nova_test;
