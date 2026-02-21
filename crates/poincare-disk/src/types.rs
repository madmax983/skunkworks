use num_complex::Complex;

/// A point in the Poincaré disk ($|z| < 1$).
///
/// While this is an alias for `Complex<f64>`, all functions in this crate assume
/// that the modulus (norm) of the point is strictly less than 1.0.
pub type Point = Complex<f64>;
