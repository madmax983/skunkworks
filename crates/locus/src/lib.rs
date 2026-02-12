//! # Locus 📍
//!
//! A lightweight 2D geometry library for TUI applications and grid-based simulations.
//!
//! `locus` provides the fundamental primitives for moving, measuring, and mapping coordinates
//! in discrete or continuous space.
//!
//! ## Features
//!
//! - **`Vec2`**: A robust 2D vector struct for physics and movement.
//! - **`Topology`**: A system for defining how your world wraps (Plane, Torus, Klein Bottle, etc.).
//! - **`serde`**: (Optional) Enables `Serialize` and `Deserialize`.
//!
//! ## Example: The Hero's Journey (Moving on a Torus)
//!
//! ```
//! use locus::{Vec2, Topology};
//!
//! # fn main() {
//!     let width = 20;
//!     let height = 10;
//!     let topo = Topology::Torus;
//!
//!     // Start at position (x=19.0, y=5.0) - at the right edge
//!     let mut position = Vec2::new(19.0, 5.0);
//!     let velocity = Vec2::new(1.0, 0.0); // Moving right
//!
//!     // Move
//!     position += velocity;
//!
//!     // Normalize using Topology to find the grid cell
//!     // Note: Topology expects (row, col) i.e. (y, x) integers
//!     let y_idx = position.y.round() as i64;
//!     let x_idx = position.x.round() as i64;
//!
//!     if let Some((ny, nx)) = topo.normalize(y_idx, x_idx, width, height) {
//!         // Should wrap to left side (x=0)
//!         assert_eq!(nx, 0);
//!         assert_eq!(ny, 5);
//!     }
//! # }
//! ```

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
#[doc(alias = "Point")]
#[doc(alias = "Vector")]
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
    /// # Performance
    ///
    /// This method is significantly faster than [`magnitude`](Self::magnitude) because it avoids
    /// the expensive square root operation. Prefer this for distance comparisons
    /// (e.g., `dist_sq < radius * radius`).
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
        assert!((n.x - std::f64::consts::FRAC_1_SQRT_2).abs() < 1e-6);
        assert!((n.y - std::f64::consts::FRAC_1_SQRT_2).abs() < 1e-6);

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
        assert!((n.x - std::f64::consts::FRAC_1_SQRT_2).abs() < 1e-6);
        assert!((n.y - std::f64::consts::FRAC_1_SQRT_2).abs() < 1e-6);
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
/// Determines how coordinates wrap or bound at the edges. This allows for simulating
/// different geometric surfaces (like a donut-shaped world or a Klein bottle)
/// using a simple 2D grid.
///
/// # Bridging the Gap: Vectors vs. Grids
///
/// Be mindful when converting between continuous space (like [`Vec2`]) and discrete grids.
/// - [`Vec2`] uses `(x, y)` (Cartesian coordinates).
/// - [`Topology::normalize`] uses `(y, x)` (Row-Major / Matrix indexing).
///
/// When using them together, ensure you swap the components correctly.
///
/// ```rust
/// use locus::{Vec2, Topology};
/// let pos = Vec2::new(10.5, 5.2);
/// let (y, x) = (pos.y as i64, pos.x as i64);
/// // Topology::normalize(y, x, ...);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[doc(alias = "wrapping")]
#[doc(alias = "boundary")]
pub enum Topology {
    /// **Plane**: A standard bounded grid.
    ///
    /// Edges are hard walls. Coordinates outside `[0, width)` or `[0, height)` are invalid.
    Plane,

    /// **Torus**: Wraps both X and Y.
    ///
    /// * `x` wraps to `x % width`
    /// * `y` wraps to `y % height`
    ///
    /// This simulates a world where walking off the right edge brings you to the left,
    /// and walking off the bottom brings you to the top.
    Torus,

    /// **Horizontal Cylinder**: Wraps X (Horizontal), Bounded Y (Vertical).
    ///
    /// The grid forms a tube running horizontally.
    /// * `x` wraps around.
    /// * `y` is bounded (hard walls at top/bottom).
    CylinderH,

    /// **Vertical Cylinder**: Bounded X (Horizontal), Wraps Y (Vertical).
    ///
    /// The grid forms a tube running vertically.
    /// * `x` is bounded (hard walls at left/right).
    /// * `y` wraps around.
    CylinderV,

    /// **Klein Bottle**: Wraps X normally. Wraps Y with a twist in X.
    ///
    /// A non-orientable surface.
    /// * `x` wraps normally (`x % width`).
    /// * `y` wraps (`y % height`), but if it wraps, `x` is mirrored: `x' = (width - 1) - x`.
    Klein,

    /// **Möbius Strip**: Wraps X with a twist, Bounded Y.
    ///
    /// A non-orientable surface with a boundary.
    /// * If `x` wraps (off left/right), `y` is mirrored: `y' = (height - 1) - y`.
    /// * `y` is bounded (cannot wrap).
    Mobius,

    /// **Hyperbolic**: Poincaré Disk model mapping.
    ///
    /// Typically handled externally or treated as bounded.
    Hyperbolic,
}

impl Topology {
    /// Normalizes coordinates based on the topology and grid size.
    ///
    /// This function takes arbitrary signed coordinates (which may be negative or
    /// larger than the grid dimensions) and maps them to a valid `(row, col)` index
    /// within the grid, if possible.
    ///
    /// # Returns
    ///
    /// * `Some((row, col))` (i.e., `(y, x)`) if the coordinates are valid or successfully wrapped.
    /// * `None` if the coordinates are out of bounds (for bounded topologies).
    ///
    /// # Arguments
    ///
    /// * `y` - The Y coordinate (row/vertical).
    /// * `x` - The X coordinate (column/horizontal).
    /// * `width` - The width of the grid (number of columns).
    /// * `height` - The height of the grid (number of rows).
    ///
    /// # Examples
    ///
    /// ```
    /// use locus::Topology;
    ///
    /// // ---------------------------------------------------------
    /// // Torus: The Classic Video Game World (Wraps Both Ways)
    /// // ---------------------------------------------------------
    /// let topo = Topology::Torus;
    /// // Wrapping off top-left corner (-1, -1) -> Bottom-Right (9, 9)
    /// assert_eq!(topo.normalize(-1, -1, 10, 10), Some((9, 9)));
    ///
    /// // ---------------------------------------------------------
    /// // Plane: Hard Boundaries (No Wrapping)
    /// // ---------------------------------------------------------
    /// let plane = Topology::Plane;
    /// assert_eq!(plane.normalize(-1, 0, 10, 10), None);
    ///
    /// // ---------------------------------------------------------
    /// // Klein Bottle: A Twist in the Fabric
    /// // ---------------------------------------------------------
    /// // Wraps X normally, but twists X when wrapping Y.
    /// let klein = Topology::Klein;
    ///
    /// // Moving off the top edge (y=-1) wraps to bottom (y=9)
    /// // BUT flips the X coordinate (x=2 becomes width-1-2 = 7)
    /// assert_eq!(klein.normalize(-1, 2, 10, 10), Some((9, 7)));
    /// ```
    pub fn normalize(&self, y: i64, x: i64, width: usize, height: usize) -> Option<(usize, usize)> {
        if width == 0 || height == 0 {
            return None;
        }

        // Helper to safely normalize a coordinate and return the wrapped value + wrap count.
        // Handles cases where `size` exceeds `i64::MAX`.
        fn normalize_coord(coord: i64, size: usize) -> (usize, i64) {
            // If size fits in i64, use standard Euclidean division
            if size <= i64::MAX as usize {
                let s = size as i64;
                let wrapped = coord.rem_euclid(s);
                let count = coord.div_euclid(s);
                (wrapped as usize, count)
            } else {
                // Size is huge (larger than any positive i64).
                // If coord >= 0, it fits within [0, size) trivially.
                if coord >= 0 {
                    (coord as usize, 0)
                } else {
                    // Coord is negative. Since coord >= i64::MIN, and size > i64::MAX,
                    // abs(coord) < size (approximately).
                    // So it wraps exactly once backwards.
                    // result = size - abs(coord)
                    // wrap count = -1
                    (size.wrapping_sub(coord.unsigned_abs() as usize), -1)
                }
            }
        }

        match self {
            Topology::Plane | Topology::Hyperbolic => {
                // Check bounds using normalize logic logic would be overkill, simple range check is enough.
                // But we must be careful casting width/height to i64.
                // If width > i64::MAX, then any valid positive i64 x is within bounds.
                // Any negative x is out of bounds.

                let in_bounds_y = if height > i64::MAX as usize {
                    y >= 0
                } else {
                    (0..height as i64).contains(&y)
                };

                let in_bounds_x = if width > i64::MAX as usize {
                    x >= 0
                } else {
                    (0..width as i64).contains(&x)
                };

                if in_bounds_y && in_bounds_x {
                    Some((y as usize, x as usize))
                } else {
                    None
                }
            }
            Topology::Torus => {
                let (ny, _) = normalize_coord(y, height);
                let (nx, _) = normalize_coord(x, width);
                Some((ny, nx))
            }
            Topology::CylinderH => {
                // Wraps X, Bounded Y
                let in_bounds_y = if height > i64::MAX as usize {
                    y >= 0
                } else {
                    (0..height as i64).contains(&y)
                };

                if in_bounds_y {
                    let (nx, _) = normalize_coord(x, width);
                    Some((y as usize, nx))
                } else {
                    None
                }
            }
            Topology::CylinderV => {
                // Bounded X, Wraps Y
                let in_bounds_x = if width > i64::MAX as usize {
                    x >= 0
                } else {
                    (0..width as i64).contains(&x)
                };

                if in_bounds_x {
                    let (ny, _) = normalize_coord(y, height);
                    Some((ny, x as usize))
                } else {
                    None
                }
            }
            Topology::Klein => {
                // Wraps X normal, Y wraps with X-twist
                let (ny, wrap_count_y) = normalize_coord(y, height);
                let (mut nx, _) = normalize_coord(x, width);

                if wrap_count_y % 2 != 0 {
                    // Twist X: x' = (width - 1) - x
                    nx = (width - 1) - nx;
                }

                Some((ny, nx))
            }
            Topology::Mobius => {
                // Wraps X with twist, Bounded Y
                let (nx, wrap_count_x) = normalize_coord(x, width);

                if wrap_count_x % 2 != 0 {
                    // Twist Y
                    // Check bounds and flip if valid

                    // Case 1: Small Height
                    if height <= i64::MAX as usize {
                        let h = height as i64;
                        let twisted_y = (h - 1).wrapping_sub(y);
                        if (0..h).contains(&twisted_y) {
                            Some((twisted_y as usize, nx))
                        } else {
                            None
                        }
                    } else {
                        // Case 2: Huge Height
                        // y must be >= 0 to map to valid range [height - 1 - y, height - 1]
                        if y >= 0 {
                            let twisted_y = (height - 1) - (y as usize);
                            Some((twisted_y, nx))
                        } else {
                            None
                        }
                    }
                } else {
                    // No Twist
                    // Check bounds
                    if height <= i64::MAX as usize {
                        let h = height as i64;
                        if (0..h).contains(&y) {
                            Some((y as usize, nx))
                        } else {
                            None
                        }
                    } else {
                        // Huge height
                        if y >= 0 {
                            Some((y as usize, nx))
                        } else {
                            None
                        }
                    }
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

    #[test]
    fn test_plane_bounds_simple() {
        let topo = Topology::Plane;
        let w = 10;
        let h = 10;

        assert_eq!(topo.normalize(0, 0, w, h), Some((0, 0)));
        assert_eq!(topo.normalize(9, 9, w, h), Some((9, 9)));
        assert_eq!(topo.normalize(-1, 0, w, h), None);
        assert_eq!(topo.normalize(0, -1, w, h), None);
        assert_eq!(topo.normalize(10, 0, w, h), None);
        assert_eq!(topo.normalize(0, 10, w, h), None);
    }

    #[test]
    fn test_torus_wrapping_simple() {
        let topo = Topology::Torus;
        let w = 10;
        let h = 10;

        assert_eq!(topo.normalize(0, 0, w, h), Some((0, 0)));
        assert_eq!(topo.normalize(-1, -1, w, h), Some((9, 9)));
        assert_eq!(topo.normalize(10, 10, w, h), Some((0, 0)));
        assert_eq!(topo.normalize(25, 15, w, h), Some((5, 5))); // 25%10=5, 15%10=5
        assert_eq!(topo.normalize(-11, -21, w, h), Some((9, 9))); // -11 -> 9, -21 -> 9
    }

    #[test]
    fn test_cylinder_wrapping() {
        let h_cyl = Topology::CylinderH; // Wraps X, Bounded Y
        let v_cyl = Topology::CylinderV; // Bounded X, Wraps Y
        let w = 10;
        let h = 10;

        // Cylinder H
        assert_eq!(h_cyl.normalize(5, 15, w, h), Some((5, 5))); // Wraps X
        assert_eq!(h_cyl.normalize(5, -5, w, h), Some((5, 5))); // Wraps X
        assert_eq!(h_cyl.normalize(-1, 5, w, h), None); // Bounded Y
        assert_eq!(h_cyl.normalize(10, 5, w, h), None); // Bounded Y

        // Cylinder V
        assert_eq!(v_cyl.normalize(15, 5, w, h), Some((5, 5))); // Wraps Y
        assert_eq!(v_cyl.normalize(-5, 5, w, h), Some((5, 5))); // Wraps Y
        assert_eq!(v_cyl.normalize(5, -1, w, h), None); // Bounded X
        assert_eq!(v_cyl.normalize(5, 10, w, h), None); // Bounded X
    }
}

#[cfg(test)]
mod prop_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_plane_bounds(
            y in any::<i64>(),
            x in any::<i64>(),
            width in 1..=usize::MAX,
            height in 1..=usize::MAX
        ) {
            let res = Topology::Plane.normalize(y, x, width, height);

            let in_y = if height > i64::MAX as usize { y >= 0 } else { y >= 0 && y < height as i64 };
            let in_x = if width > i64::MAX as usize { x >= 0 } else { x >= 0 && x < width as i64 };

            if in_y && in_x {
                prop_assert_eq!(res, Some((y as usize, x as usize)));
            } else {
                prop_assert_eq!(res, None);
            }
        }

        #[test]
        fn test_torus_wrapping(
            y in any::<i64>(),
            x in any::<i64>(),
            width in 1..=usize::MAX,
            height in 1..=usize::MAX
        ) {
            let res = Topology::Torus.normalize(y, x, width, height);
            prop_assert!(res.is_some());
            let (ny, nx) = res.unwrap();

            prop_assert!(ny < height);
            prop_assert!(nx < width);
        }

        #[test]
        fn test_cylinder_h_wrapping(
            y in any::<i64>(),
            x in any::<i64>(),
            width in 1..=usize::MAX,
            height in 1..=usize::MAX
        ) {
             let res = Topology::CylinderH.normalize(y, x, width, height);
             let in_y = if height > i64::MAX as usize { y >= 0 } else { y >= 0 && y < height as i64 };

             if in_y {
                 prop_assert!(res.is_some());
                 let (ny, nx) = res.unwrap();
                 prop_assert_eq!(ny, y as usize);
                 prop_assert!(nx < width);
             } else {
                 prop_assert_eq!(res, None);
             }
        }

        #[test]
        fn test_cylinder_v_wrapping(
            y in any::<i64>(),
            x in any::<i64>(),
            width in 1..=usize::MAX,
            height in 1..=usize::MAX
        ) {
             let res = Topology::CylinderV.normalize(y, x, width, height);
             let in_x = if width > i64::MAX as usize { x >= 0 } else { x >= 0 && x < width as i64 };

             if in_x {
                 prop_assert!(res.is_some());
                 let (ny, nx) = res.unwrap();
                 prop_assert!(ny < height);
                 prop_assert_eq!(nx, x as usize);
             } else {
                 prop_assert_eq!(res, None);
             }
        }

        #[test]
        fn test_klein_bounds(
            y in any::<i64>(),
            x in any::<i64>(),
            width in 1..=usize::MAX,
            height in 1..=usize::MAX
        ) {
            let res = Topology::Klein.normalize(y, x, width, height);
            prop_assert!(res.is_some());
            let (ny, nx) = res.unwrap();
            prop_assert!(ny < height);
            prop_assert!(nx < width);
        }

        #[test]
        fn test_mobius_bounds(
            y in any::<i64>(),
            x in any::<i64>(),
            width in 1..=usize::MAX,
            height in 1..=usize::MAX
        ) {
            let res = Topology::Mobius.normalize(y, x, width, height);
            // Mobius returns None if twisted y is out of bounds
            if let Some((ny, nx)) = res {
                prop_assert!(ny < height);
                prop_assert!(nx < width);
            }
        }

        #[test]
        fn test_normalize_zero_dimensions_safe(
            y in any::<i64>(),
            x in any::<i64>(),
        ) {
             // 0 dimensions should always return None
             prop_assert_eq!(Topology::Plane.normalize(y, x, 0, 10), None);
             prop_assert_eq!(Topology::Plane.normalize(y, x, 10, 0), None);
             prop_assert_eq!(Topology::Torus.normalize(y, x, 0, 0), None);
        }
    }
}
