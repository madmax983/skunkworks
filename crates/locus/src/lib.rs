//! # Locus
//!
//! A lightweight 2D geometry library for TUI applications.
//!
//! This crate provides a simple `Vec2` struct for position, velocity, and force calculations.
//! It supports standard arithmetic operations (+, -, *, /) and common vector
//! operations (magnitude, normalize, dot, reflect).
//!
//! ## Features
//!
//! - `serde`: Enables `Serialize` and `Deserialize` implementation for `Vec2`.

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// A simple 2D vector with `f64` components.
///
/// This struct is the workhorse of position, velocity, and force calculations.
/// It supports standard arithmetic operations (+, -, *, /) and common vector
/// operations (magnitude, normalize, dot, reflect).
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
    /// If the vector has infinite components, it attempts to handle them gracefully.
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
    ///
    /// let v2 = Vec2::new(3.0, 0.0);
    /// assert_eq!(v2.limit(5.0), Vec2::new(3.0, 0.0));
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
    /// The normal vector does not need to be normalized.
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
        // r = d - 2 * (d . n) / (n . n) * n
        let factor = 2.0 * dot / n_sq;
        Self {
            x: self.x - factor * normal.x,
            y: self.y - factor * normal.y,
        }
    }

    /// Rotates the vector by the given angle (in radians).
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
}

/// Represents the topology of a grid or space.
///
/// Determines how coordinates wrap or bound at the edges.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Topology {
    Plane,      // 0: Bounded. Edges are walls.
    Torus,      // 1: Wraps X and Y.
    CylinderH,  // 2: Wraps X, Bounded Y.
    CylinderV,  // 3: Bounded X, Wraps Y.
    Klein,      // 4: Wraps X, Wraps Y with twist (x' = W-1-x).
    Mobius,     // 5: Wraps X with twist, Bounded Y.
    Hyperbolic, // 6: Poincaré Disk model mapping (handled externally or treated as bounded).
}

impl Topology {
    /// Normalizes coordinates based on the topology and grid size.
    ///
    /// Returns `Some((y, x))` if the coordinates are valid or wrapped.
    /// Returns `None` if the coordinates are out of bounds (for bounded topologies).
    ///
    /// # Arguments
    ///
    /// * `y` - The Y coordinate (row).
    /// * `x` - The X coordinate (column).
    /// * `width` - The width of the grid.
    /// * `height` - The height of the grid.
    pub fn normalize(
        &self,
        y: i64,
        x: i64,
        width: usize,
        height: usize,
    ) -> Option<(usize, usize)> {
        if width == 0 || height == 0 {
            return None;
        }
        let w = width as i64;
        let h = height as i64;
        match self {
            Topology::Plane | Topology::Hyperbolic => {
                if (0..h).contains(&y) && (0..w).contains(&x) {
                    Some((y as usize, x as usize))
                } else {
                    None
                }
            }
            Topology::Torus => Some((
                y.rem_euclid(h) as usize,
                x.rem_euclid(w) as usize,
            )),
            Topology::CylinderH => {
                // Wraps X, Bounded Y
                if (0..h).contains(&y) {
                    Some((y as usize, x.rem_euclid(w) as usize))
                } else {
                    None
                }
            }
            Topology::CylinderV => {
                // Bounded X, Wraps Y
                if (0..w).contains(&x) {
                    Some((y.rem_euclid(h) as usize, x as usize))
                } else {
                    None
                }
            }
            Topology::Klein => {
                // Wraps X normal, Y wraps with X-twist
                let mut nx = x;
                let mut ny = y;

                if !(0..h).contains(&ny) {
                    let wrap_count = ny.div_euclid(h);
                    // Normalize X to [0, w-1] before flipping
                    nx = nx.rem_euclid(w);
                    if wrap_count % 2 != 0 {
                        nx = (w - 1) - nx; // Twist X
                    }
                    ny = ny.rem_euclid(h);
                } else {
                    nx = nx.rem_euclid(w);
                }

                Some((ny as usize, nx as usize))
            }
            Topology::Mobius => {
                // Wraps X with twist, Bounded Y
                let mut nx = x;
                let mut ny = y;

                if !(0..w).contains(&nx) {
                    let wrap_count = nx.div_euclid(w);
                    nx = nx.rem_euclid(w);
                    if wrap_count % 2 != 0 {
                        // Use wrapping_sub to avoid panic on overflow if ny is out of bounds
                        ny = (h - 1).wrapping_sub(ny); // Twist Y
                    }
                } else {
                    nx = nx.rem_euclid(w);
                }

                if (0..h).contains(&ny) {
                    Some((ny as usize, nx as usize))
                } else {
                    None
                }
            }
        }
    }
}

#[cfg(test)]
mod topology_tests {
    use super::*;

    #[test]
    fn test_topology_overflow_klein() {
        let topo = Topology::Klein;
        // y = -16 (wraps once, odd), x = i64::MIN
        // Should not panic and return valid coordinate
        let res = topo.normalize(-16, i64::MIN, 16, 16);
        assert!(res.is_some());
        let (y, x) = res.unwrap();
        assert!(y < 16);
        assert!(x < 16);
    }

    #[test]
    fn test_topology_overflow_mobius() {
        let topo = Topology::Mobius;
        // x = -16 (wraps once, odd), y = i64::MIN
        // Should not panic.
        let _ = topo.normalize(i64::MIN, -16, 16, 16);
    }

    #[test]
    fn test_klein_wrapping() {
        let topo = Topology::Klein;
        let width = 10;
        let height = 10;
        // Normal wrapping (even wrap)
        // y = 20 -> wraps to 0. No twist. x=5 -> 5.
        assert_eq!(topo.normalize(20, 5, width, height), Some((0, 5)));

        // Twisted wrapping (odd wrap)
        // y = 10 -> wraps to 0. 1 wrap (odd). Twist x.
        // x = 2 -> 9 - 2 = 7.
        assert_eq!(topo.normalize(10, 2, width, height), Some((0, 7)));

        // Twisted wrapping (odd wrap) with negative y
        // y = -1 -> wraps to 9. -1 div 10 = -1 (odd). Twist x.
        // x = 2 -> 9 - 2 = 7.
        assert_eq!(topo.normalize(-1, 2, width, height), Some((9, 7)));

        // X out of bounds + Twist
        // y = 10 (twist). x = 12.
        // x normalized: 12 % 10 = 2.
        // twist: 9 - 2 = 7.
        assert_eq!(topo.normalize(10, 12, width, height), Some((0, 7)));
    }

    #[test]
    fn test_rectangular_klein() {
        let topo = Topology::Klein;
        let width = 10;
        let height = 20;

        // y = -1 -> y wraps to 19. Twist x.
        // x = 2 -> x = 10-1-2 = 7.
        assert_eq!(topo.normalize(-1, 2, width, height), Some((19, 7)));
    }

    #[test]
    fn test_mobius_wrapping() {
        let topo = Topology::Mobius;
        let width = 10;
        let height = 10;

        // Normal wrapping (even wrap)
        // x = 20 -> 0. y=5 -> 5.
        assert_eq!(topo.normalize(5, 20, width, height), Some((5, 0)));

        // Twisted wrapping (odd wrap)
        // x = 10 -> 0. 1 wrap. Twist y.
        // y = 2 -> 9 - 2 = 7.
        assert_eq!(topo.normalize(2, 10, width, height), Some((7, 0)));

        // Twisted wrapping + Y out of bounds
        // x = 10 (twist). y = 12.
        // twist y: 9 - 12 = -3.
        // -3 out of bounds. -> None.
        assert_eq!(topo.normalize(12, 10, width, height), None);
    }
}
