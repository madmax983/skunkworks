//! # Poincaré Disk Types
//!
//! Core data types used throughout the `poincare-disk` crate.
//!
//! Currently, this module primarily provides the `Point` type, which represents
//! a location in the hyperbolic plane using complex numbers in the unit disk.

use num_complex::Complex;

/// A point in the Poincaré disk ($|z| < 1$).
///
/// While this is an alias for `Complex<f64>`, all functions in this crate assume
/// that the modulus (norm) of the point is strictly less than 1.0.
pub type Point = Complex<f64>;
