//! Core library for magnetic field and fluid density simulations in the Ferrous ecosystem.
//!
//! `ferrous-core` provides shared data structures and algorithms used by various experiments
//! such as `ferrous-chimera`, `ferrous-fluid`, and `ferrous-graph`.
//!
//! The central component is the [`Platter`], a 2D grid that can simulate:
//! - Magnetic field strength (clamped values).
//! - Fluid density (unbounded accumulation).
//! - Pheromone trails (with decay).

pub mod platter;
pub use platter::Platter;
