//! Core system utilities for the "Hyper" series of experiments.
//!
//! This crate provides shared functionality for 4D visualization and system monitoring,
//! ensuring consistent behavior across different visual experiments.
//!
//! # Modules
//!
//! - [`math`]: A 4D vector math library optimized for visualization.
//! - [`monitor`]: A system resource monitor with smoothed metric interpolation.
//! - [`physics`]: A shared 4D Position-Based Dynamics physics engine.

/// 4D vector math library optimized for visualization.
///
/// This module re-exports the 3D and 4D vector math primitives from the `locus` crate.
/// It acts as the mathematical foundation for positioning and projecting
/// structures in hyper-dimensional space.
pub mod math {
    pub use locus::Vec3;
    pub use locus::Vec4;
}
pub(crate) mod monitor;
pub(crate) mod physics;

pub use monitor::*;
pub use physics::*;
