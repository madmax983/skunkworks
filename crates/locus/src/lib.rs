//! # Locus
//!
//! A lightweight 2D geometry library for TUI applications.
//!
//! This crate provides a simple [`Vec2`] struct for position, velocity, and force calculations.
//! It supports standard arithmetic operations (`+`, `-`, `*`, `/`) and common vector
//! operations (magnitude, normalize, dot, reflect).
//!
//! It also includes a [`Topology`] enum for handling coordinate wrapping in various
//! 2D manifolds (Torus, Klein Bottle, Möbius Strip).
//!
//! ## Features
//!
//! - `serde`: Enables `Serialize` and `Deserialize` implementation for [`Vec2`] and [`Topology`].

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// A simple 2D vector with `f64` components.
///
/// This struct is the workhorse of position, velocity, and force calculations.
/// It supports standard arithmetic operations and common vector operations.
///
/// # Examples
///
/// ```
/// use locus::Vec2;
///
/// let v1 = Vec2::new(3.0, 4.0);
/// let v2 = Vec2::new(1.0, 2.0);
///
/// let sum = v1 + v2;
/// assert_eq!(sum, Vec2::new(4.0, 6.0));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Vec2 {
    /// The X component (horizontal).
    pub x: f64,
    /// The Y component (vertical).
    pub y: f64,
}

impl Vec2 {
    /// Creates a new vector with the given coordinates.
    ///
    /// # Examples
    ///
    /// ```
    /// use locus::Vec2;
    /// let v = Vec2::new(10.5, -5.2);
    /// ```
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /// Creates a vector with all components set to zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use locus::Vec2;
    /// let v = Vec2::zero();
    /// assert_eq!(v.x, 0.0);
    /// assert_eq!(v.y, 0.0);
    /// ```
    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    /// Calculates the squared magnitude (length) of the vector.
    ///
    /// This is faster than `magnitude()` because it avoids a square root calculation.
    /// Use this when you only need to compare lengths (e.g., checking if distance < radius).
    ///
    /// # Examples
    ///
    /// ```
    /// use locus::Vec2;
    /// let v = Vec2::new(3.0, 4.0);
    /// assert_eq!(v.magnitude_squared(), 25.0);
    /// ```
    pub fn magnitude_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }

    /// Calculates the magnitude (length) of the vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use locus::Vec2;
    /// let v = Vec2::new(3.0, 4.0);
    /// assert_eq!(v.magnitude(), 5.0);
    /// ```
    pub fn magnitude(&self) -> f64 {
        self.magnitude_squared().sqrt()
    }

    /// Returns a normalized version of the vector (length of 1.0).
    ///
    /// If the vector is zero, it returns the zero vector.
    /// If the vector has infinite components, it attempts to handle them gracefully
    /// by returning a unit vector along the major axis.
    ///
    /// # Examples
    ///
    /// ```
    /// use locus::Vec2;
    /// let v = Vec2::new(10.0, 0.0);
    /// assert_eq!(v.normalize(), Vec2::new(1.0, 0.0));
    ///
    /// let zero = Vec2::zero();
    /// assert_eq!(zero.normalize(), Vec2::zero());
    /// ```
    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        if mag == 0.0 {
            Self::zero()
        } else if mag.is_infinite() {
            if self.x.is_infinite() || self.y.is_infinite() {
                // If components are infinite, normalize by treating infinite components as +/- 1.0
                let x = if self.x.is_infinite() {
                    self.x.signum()
                } else {
                    0.0
                };
                let y = if self.y.is_infinite() {
                    self.y.signum()
                } else {
                    0.0
                };
                Vec2::new(x, y).normalize()
            } else {
                // If magnitude is infinite but components are finite (overflow), scale down
                let max_comp = self.x.abs().max(self.y.abs());
                let scaled = Vec2::new(self.x / max_comp, self.y / max_comp);
                scaled.normalize()
            }
        } else {
            *self / mag
        }
    }

    /// Limits the vector's magnitude to a maximum value.
    ///
    /// If the vector's length is greater than `max`, it is scaled down to `max`.
    /// Otherwise, it is returned unchanged.
    ///
    /// # Examples
    ///
    /// ```
    /// use locus::Vec2;
    /// let v = Vec2::new(10.0, 0.0);
    /// assert_eq!(v.limit(5.0), Vec2::new(5.0, 0.0));
    /// ```
    pub fn limit(&self, max: f64) -> Self {
        if self.magnitude_squared() > max * max {
            self.normalize() * max
        } else {
            *self
        }
    }

    /// Calculates the squared Euclidean distance between two vectors.
    ///
    /// Faster than `distance()` as it avoids the square root.
    ///
    /// # Examples
    ///
    /// ```
    /// use locus::Vec2;
    /// let v1 = Vec2::zero();
    /// let v2 = Vec2::new(3.0, 4.0);
    /// assert_eq!(v1.distance_squared(v2), 25.0);
    /// ```
    pub fn distance_squared(&self, other: Vec2) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        dx * dx + dy * dy
    }

    /// Calculates the Euclidean distance between two vectors.
    ///
    /// # Examples
    ///
    /// ```
    /// use locus::Vec2;
    /// let v1 = Vec2::zero();
    /// let v2 = Vec2::new(3.0, 4.0);
    /// assert_eq!(v1.distance(v2), 5.0);
    /// ```
    pub fn distance(&self, other: Vec2) -> f64 {
        self.distance_squared(other).sqrt()
    }

    /// Calculates the dot product of two vectors.
    ///
    /// The dot product is the sum of the products of the corresponding components.
    /// It can be used to find the angle between two vectors or to project one vector onto another.
    ///
    /// # Examples
    ///
    /// ```
    /// use locus::Vec2;
    /// let v1 = Vec2::new(1.0, 0.0);
    /// let v2 = Vec2::new(0.0, 1.0);
    /// assert_eq!(v1.dot(v2), 0.0); // Perpendicular
    ///
    /// let v3 = Vec2::new(2.0, 0.0);
    /// assert_eq!(v1.dot(v3), 2.0); // Parallel
    /// ```
    pub fn dot(&self, other: Vec2) -> f64 {
        self.x * other.x + self.y * other.y
    }

    /// Reflects the vector off a surface with the given normal.
    ///
    /// The normal vector does not need to be normalized, but `reflect` assumes the surface behaves
    /// like a standard mirror.
    ///
    /// Formula: `r = d - 2 * (d . n) / (n . n) * n`
    ///
    /// # Examples
    ///
    /// ```
    /// use locus::Vec2;
    /// let v = Vec2::new(1.0, -1.0);
    /// let normal = Vec2::new(0.0, 1.0);
    /// let reflected = v.reflect(normal);
    /// assert_eq!(reflected, Vec2::new(1.0, 1.0));
    /// ```
    pub fn reflect(&self, normal: Vec2) -> Self {
        let n_sq = normal.magnitude_squared();
        if n_sq == 0.0 {
            return *self;
        }
        let dot = self.dot(normal);
        let factor = 2.0 * dot / n_sq;
        Self {
            x: self.x - factor * normal.x,
            y: self.y - factor * normal.y,
        }
    }

    /// Rotates the vector by the given angle (in radians).
    ///
    /// A positive angle rotates counter-clockwise (in standard math coordinates).
    ///
    /// # Examples
    ///
    /// ```
    /// use locus::Vec2;
    /// use std::f64::consts::PI;
    ///
    /// let v = Vec2::new(1.0, 0.0);
    /// let rotated = v.rotate(PI / 2.0);
    /// assert!((rotated.x - 0.0).abs() < 1e-10);
    /// assert!((rotated.y - 1.0).abs() < 1e-10);
    /// ```
    pub fn rotate(self, angle: f64) -> Self {
        let (sin, cos) = angle.sin_cos();
        Self {
            x: self.x * cos - self.y * sin,
            y: self.x * sin + self.y * cos,
        }
    }
}

// Implement arithmetic traits

impl Add for Vec2 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl Sub for Vec2 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl Mul<f64> for Vec2 {
    type Output = Self;
    fn mul(self, scalar: f64) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl MulAssign<f64> for Vec2 {
    fn mul_assign(&mut self, scalar: f64) {
        self.x *= scalar;
        self.y *= scalar;
    }
}

impl Div<f64> for Vec2 {
    type Output = Self;
    fn div(self, scalar: f64) -> Self {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
        }
    }
}

impl DivAssign<f64> for Vec2 {
    fn div_assign(&mut self, scalar: f64) {
        self.x /= scalar;
        self.y /= scalar;
    }
}

impl Neg for Vec2 {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl From<(f64, f64)> for Vec2 {
    fn from((x, y): (f64, f64)) -> Self {
        Self::new(x, y)
    }
}

impl From<Vec2> for (f64, f64) {
    fn from(v: Vec2) -> Self {
        (v.x, v.y)
    }
}

/// Represents the topology of a grid or space.
///
/// Determines how coordinates wrap or bound at the edges.
/// Assumes a square grid of size `size x size`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Topology {
    /// Bounded grid. Edges are walls.
    Plane,
    /// Standard wrap. Top connects to Bottom, Left connects to Right.
    /// Used for "Pac-Man" worlds or standard tori.
    Torus,
    /// Wraps X (Left-Right), Bounded Y (Top-Bottom).
    CylinderH,
    /// Bounded X (Left-Right), Wraps Y (Top-Bottom).
    CylinderV,
    /// Wraps X normally. Wraps Y with a twist (flip X).
    /// Moving off the top/bottom edge flips your X coordinate.
    Klein,
    /// Wraps X with a twist (flip Y). Bounded Y.
    /// Moving off the left/right edge flips your Y coordinate.
    Mobius,
    /// Poincaré Disk model placeholder. Typically handled as bounded or custom logic.
    Hyperbolic,
}

impl Topology {
    /// Normalizes coordinates based on the topology and grid size.
    ///
    /// This function takes arbitrary `i64` coordinates (which may be negative or
    /// larger than the grid) and maps them to a valid `(usize, usize)` index inside
    /// the grid `[0, size) x [0, size)`.
    ///
    /// Returns `Some((y, x))` if the coordinates land on the grid.
    /// Returns `None` if the coordinates are out of bounds (for bounded topologies).
    ///
    /// # Arguments
    ///
    /// * `y` - The Y coordinate (row).
    /// * `x` - The X coordinate (column).
    /// * `size` - The size of the grid (assumed square).
    ///
    /// # Examples
    ///
    /// ```
    /// use locus::Topology;
    /// let size = 10;
    ///
    /// // Torus Wrap
    /// let t = Topology::Torus;
    /// assert_eq!(t.normalize(10, -1, size), Some((0, 9)));
    ///
    /// // Plane Bounds
    /// let p = Topology::Plane;
    /// assert_eq!(p.normalize(10, 5, size), None); // Off bottom edge
    /// ```
    pub fn normalize(&self, y: i64, x: i64, size: usize) -> Option<(usize, usize)> {
        let s = size as i64;
        match self {
            Topology::Plane | Topology::Hyperbolic => {
                if (0..s).contains(&y) && (0..s).contains(&x) {
                    Some((y as usize, x as usize))
                } else {
                    None
                }
            }
            Topology::Torus => Some((y.rem_euclid(s) as usize, x.rem_euclid(s) as usize)),
            Topology::CylinderH => {
                // Wraps X, Bounded Y
                if (0..s).contains(&y) {
                    Some((y as usize, x.rem_euclid(s) as usize))
                } else {
                    None
                }
            }
            Topology::CylinderV => {
                // Bounded X, Wraps Y
                if (0..s).contains(&x) {
                    Some((y.rem_euclid(s) as usize, x as usize))
                } else {
                    None
                }
            }
            Topology::Klein => {
                // Wraps X (column) normally.
                // Wraps Y (row) with X-twist.
                // If Y wraps an ODD number of times, we mirror X.

                let y_norm = y.rem_euclid(s);
                let y_wraps = y.div_euclid(s);

                let x_norm = x.rem_euclid(s);

                let x_final = if y_wraps % 2 != 0 {
                    (s - 1) - x_norm // Mirror X: 0 becomes s-1
                } else {
                    x_norm
                };

                Some((y_norm as usize, x_final as usize))
            }
            Topology::Mobius => {
                // Wraps X (column) with Y-twist.
                // Bounded Y (row).

                // First check if Y is in bounds. If not, we fall off the edge.
                if !(0..s).contains(&y) {
                    return None;
                }

                let x_norm = x.rem_euclid(s);
                let x_wraps = x.div_euclid(s);

                let y_final = if x_wraps % 2 != 0 {
                    (s - 1) - y // Mirror Y: 0 becomes s-1
                } else {
                    y
                };

                Some((y_final as usize, x_norm as usize))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec2_ops() {
        let v1 = Vec2::new(1.0, 2.0);
        let v2 = Vec2::new(3.0, 4.0);
        assert_eq!(v1 + v2, Vec2::new(4.0, 6.0));
        assert_eq!(v2 - v1, Vec2::new(2.0, 2.0));
        assert_eq!(v1 * 2.0, Vec2::new(2.0, 4.0));
    }

    #[test]
    fn test_vec2_reflect() {
        let v = Vec2::new(1.0, 0.0);
        let n = Vec2::new(-1.0, 0.0);
        let r = v.reflect(n);
        assert!((r.x - -1.0).abs() < 1e-6);
        assert!((r.y - 0.0).abs() < 1e-6);
    }

    #[test]
    fn test_vec2_reflect_non_normalized() {
        let v = Vec2::new(1.0, 0.0);
        // Normal is (-5, 0), which is parallel to (-1, 0)
        let n = Vec2::new(-5.0, 0.0);
        let r = v.reflect(n);
        // Should be same as normalized case
        assert!((r.x - -1.0).abs() < 1e-6);
        assert!((r.y - 0.0).abs() < 1e-6);
    }

    #[test]
    fn test_vec2_reflect_zero_normal() {
        let v = Vec2::new(1.0, 0.0);
        let n = Vec2::zero();
        let r = v.reflect(n);
        // Should return original vector (no reflection off nothing)
        assert_eq!(r, v);
    }

    #[test]
    fn test_normalize_infinite() {
        // (inf, 0) -> (1, 0)
        let v = Vec2::new(f64::INFINITY, 0.0);
        let n = v.normalize();
        assert!(!n.x.is_nan());
        assert!(!n.y.is_nan());
        assert_eq!(n, Vec2::new(1.0, 0.0));

        // (inf, inf) -> (0.707, 0.707)
        let v = Vec2::new(f64::INFINITY, f64::INFINITY);
        let n = v.normalize();
        assert!(!n.x.is_nan());
        assert!(!n.y.is_nan());
        assert!((n.x - 0.70710678).abs() < 1e-6);
        assert!((n.y - 0.70710678).abs() < 1e-6);

        // (-inf, 500) -> (-1, 0)
        let v = Vec2::new(f64::NEG_INFINITY, 500.0);
        let n = v.normalize();
        assert_eq!(n, Vec2::new(-1.0, 0.0));
    }

    #[test]
    fn test_normalize_nan() {
        let v = Vec2::new(f64::NAN, 0.0);
        let n = v.normalize();
        assert!(n.x.is_nan());
        // n.y could be NaN or something else depending on implementation, but likely NaN
        assert!(n.y.is_nan());
    }

    #[test]
    fn test_normalize_max() {
        // (f64::MAX, 0) -> (1, 0)
        let v = Vec2::new(f64::MAX, 0.0);
        let n = v.normalize();
        assert!((n.x - 1.0).abs() < 1e-6);
        assert!((n.y - 0.0).abs() < 1e-6);

        // (f64::MAX, f64::MAX) -> (0.707, 0.707)
        let v = Vec2::new(f64::MAX, f64::MAX);
        let n = v.normalize();
        assert!((n.x - 0.70710678).abs() < 1e-6);
        assert!((n.y - 0.70710678).abs() < 1e-6);
    }

    #[test]
    fn test_limit_infinite() {
        let v = Vec2::new(f64::INFINITY, 0.0);
        let l = v.limit(5.0);
        // Should be (5.0, 0.0)
        assert!((l.x - 5.0).abs() < 1e-6);
        assert!((l.y - 0.0).abs() < 1e-6);

        let v2 = Vec2::new(f64::INFINITY, f64::INFINITY);
        let l2 = v2.limit(10.0);
        // Magnitude should be 10.0
        assert!((l2.magnitude() - 10.0).abs() < 1e-6);
    }

    #[test]
    fn test_rotate() {
        use std::f64::consts::PI;
        let v = Vec2::new(1.0, 0.0);

        // 90 degrees
        let r90 = v.rotate(PI / 2.0);
        assert!((r90.x - 0.0).abs() < 1e-6);
        assert!((r90.y - 1.0).abs() < 1e-6);

        // 180 degrees
        let r180 = v.rotate(PI);
        assert!((r180.x - -1.0).abs() < 1e-6);
        assert!((r180.y - 0.0).abs() < 1e-6);

        // 270 degrees (-90)
        let r270 = v.rotate(-PI / 2.0);
        assert!((r270.x - 0.0).abs() < 1e-6);
        assert!((r270.y - -1.0).abs() < 1e-6);
    }

    #[test]
    fn test_topology_overflow_klein() {
        let topo = Topology::Klein;
        // y = -16. s = 10.
        // -16 div 10 = -2 (even). rem = 4.
        // Even wrap -> No X flip.
        // Wait, div_euclid(-16, 10) is -2.
        // -16 = -2 * 10 + 4.
        // Wraps -2 times (even).

        let (ny, nx) = topo.normalize(-16, 5, 10).unwrap();
        assert_eq!(ny, 4);
        assert_eq!(nx, 5); // No flip

        // y = -15.
        // -15 div 10 = -2 (even). rem = 5.
        // Wait.
        // -10..-1 is wrap -1 (odd).
        // -20..-11 is wrap -2 (even).

        // Let's test explicit odd wrap
        // y = 10. wrap 1. Odd.
        let (ny2, nx2) = topo.normalize(10, 2, 10).unwrap();
        assert_eq!(ny2, 0);
        assert_eq!(nx2, 7); // Flip: 10 - 1 - 2 = 7

        // Test extreme overflow (regression test for bug)
        // x = i64::MIN.
        // i64::MIN rem 10 = 2 (approx).
        // If y wrap is odd, x should be flipped properly.
        let (ny3, nx3) = topo.normalize(10, i64::MIN, 10).unwrap();
        assert_eq!(ny3, 0);

        let x_norm = i64::MIN.rem_euclid(10) as usize;
        let expected_x = 9 - x_norm;
        assert_eq!(nx3, expected_x);
    }

    #[test]
    fn test_topology_overflow_mobius() {
        let topo = Topology::Mobius;
        // x = 10 (wrap 1, odd). y = 2.
        // Flip Y.
        let (ny, nx) = topo.normalize(2, 10, 10).unwrap();
        assert_eq!(nx, 0);
        assert_eq!(ny, 7); // 10 - 1 - 2 = 7

        // x = i64::MIN. (Even/Odd wrap?)
        // Regression test: should not panic.
        let res = topo.normalize(2, i64::MIN, 10);
        assert!(res.is_some());
    }
}
