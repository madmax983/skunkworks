//! Core system utilities for the "Hyper" series of experiments.
//!
//! This crate provides shared functionality for 4D visualization and system monitoring,
//! ensuring consistent behavior across different visual experiments.
//!
//! # Modules
//!
//! - [`math`]: A 4D vector math library optimized for visualization.
//! - [`monitor`]: A system resource monitor with smoothed metric interpolation.

pub mod math;
pub mod monitor;
