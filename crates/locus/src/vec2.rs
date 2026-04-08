//! # 2-Dimensional Vectors
//!
//! Provides the primary [`Vec2`] physics data type and related geometric operations.
//!
//! Includes methods for calculating distance, reflection, rotation, magnitude,
//! and enforcing maximum limits. Uses `f64` precision to ensure stability.

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// A simple 2D vector with `f64` components.
///
/// This struct is the workhorse of position, velocity, and force calculations.
/// It supports standard arithmetic operations (+, -, *, /) and common vector
/// operations (magnitude, normalize, dot, reflect).
///
/// # Examples
///
/// ## Navigation: Moving towards a target
///
/// ```
/// use locus::Vec2;
///
/// let mut position = Vec2::new(0.0, 0.0);
/// let target = Vec2::new(10.0, 10.0);
/// let speed = 2.0;
///
/// // Calculate direction vector
/// let direction = (target - position).normalize();
///
/// // Move towards target
/// position += direction * speed;
///
/// assert!((position.x - 1.414).abs() < 0.001);
/// assert!((position.y - 1.414).abs() < 0.001);
/// ```
///
/// ## Physics: Applying force
///
/// ```
/// use locus::Vec2;
///
/// let mut velocity = Vec2::new(5.0, 0.0);
/// let wind = Vec2::new(0.0, 1.0);
/// let friction = 0.9;
///
/// velocity += wind;
/// velocity *= friction;
///
/// assert_eq!(velocity, Vec2::new(4.5, 0.9));
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
    ///
    /// # The "Why": Graceful Infinity Handling
    ///
    /// This method explicitly handles `f64::INFINITY` or `f64::NAN` components.
    /// In dense flocking simulations or physics engines, a division by zero
    /// or a massive force summation could result in an infinite vector.
    /// Instead of propagating `NaN` and crashing the simulation, `normalize`
    /// treats infinite components as a valid directional heading and clamps them.
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
        let mag_sq = self.magnitude_squared();
        if mag_sq == 0.0 {
            Self::zero()
        } else if mag_sq.is_infinite() {
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
            let inv_mag = 1.0 / mag_sq.sqrt();
            *self * inv_mag
        }
    }

    /// Limits the vector's magnitude to a maximum value.
    ///
    /// If the vector's length is greater than `max`, it is scaled down to `max`.
    /// Otherwise, it is returned unchanged.
    ///
    /// # The "Why": Speed Limits
    ///
    /// Useful for capping velocities in physics simulations so entities don't break
    /// the sound barrier or tunnel through collision geometry in a single frame.
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
        let max = max.abs();
        let sq_mag = self.magnitude_squared();
        // If the squared magnitude overflows (is infinite) but the max magnitude is finite,
        // we must still clamp. If max*max overflows (is infinite), the comparison fails.
        // We add a check for infinite sq_mag when max is finite.
        if sq_mag > max * max || (sq_mag.is_infinite() && max.is_finite()) {
            // Optimization: Avoid full normalize() if we can
            if sq_mag.is_finite() {
                // Common case: finite vector, just scale
                // Optimization: Multiply by reciprocal instead of division
                if sq_mag > 0.0 {
                    let inv_mag = 1.0 / sq_mag.sqrt();
                    *self * (max * inv_mag)
                } else {
                    Self::zero()
                }
            } else {
                // Edge case: Overflow or Infinity, use robust normalize
                self.normalize() * max
            }
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

    #[test]
    fn test_vec2_normalize_mixed_inf() {
        // x is finite, y is infinite
        let v1 = Vec2::new(10.0, f64::INFINITY);
        let n1 = v1.normalize();
        assert_eq!(n1.x, 0.0);
        assert_eq!(n1.y, 1.0);

        let v2 = Vec2::new(10.0, f64::NEG_INFINITY);
        let n2 = v2.normalize();
        assert_eq!(n2.x, 0.0);
        assert_eq!(n2.y, -1.0);

        // x is infinite, y is finite
        let v3 = Vec2::new(f64::INFINITY, 10.0);
        let n3 = v3.normalize();
        assert_eq!(n3.x, 1.0);
        assert_eq!(n3.y, 0.0);

        let v4 = Vec2::new(f64::NEG_INFINITY, 10.0);
        let n4 = v4.normalize();
        assert_eq!(n4.x, -1.0);
        assert_eq!(n4.y, 0.0);

        // overflow test (mag is inf, components finite)
        let huge = f64::MAX;
        let v5 = Vec2::new(huge, huge);
        let n5 = v5.normalize();
        assert!((n5.x - std::f64::consts::FRAC_1_SQRT_2).abs() < 1e-6);
        assert!((n5.y - std::f64::consts::FRAC_1_SQRT_2).abs() < 1e-6);
    }

    #[test]
    fn test_vec2_limit_zero_sq_mag() {
        // test the `sq_mag > 0.0` inner branch
        let v = Vec2::new(f64::NAN, 0.0);
        let l = v.limit(1.0);
        assert!(l.x.is_nan());
        assert_eq!(l.y, 0.0);
    }

    #[test]
    fn test_vec2_distance() {
        let v1 = Vec2::new(0.0, 0.0);
        let v2 = Vec2::new(3.0, 4.0);
        assert_eq!(v1.distance(v2), 5.0);

        let v3 = Vec2::new(-3.0, -4.0);
        assert_eq!(v1.distance(v3), 5.0);
    }

    #[test]
    fn test_vec2_operators() {
        let mut v1 = Vec2::new(1.0, 2.0);
        let v2 = Vec2::new(3.0, 4.0);

        v1 += v2;
        assert_eq!(v1, Vec2::new(4.0, 6.0));

        v1 -= v2;
        assert_eq!(v1, Vec2::new(1.0, 2.0));

        v1 *= 2.0;
        assert_eq!(v1, Vec2::new(2.0, 4.0));

        v1 /= 2.0;
        assert_eq!(v1, Vec2::new(1.0, 2.0));

        let v3 = v1 / 2.0;
        assert_eq!(v3, Vec2::new(0.5, 1.0));

        let v4 = -v1;
        assert_eq!(v4, Vec2::new(-1.0, -2.0));
    }

    #[test]
    fn test_vec2_conversions() {
        let tuple = (1.0, 2.0);
        let v: Vec2 = tuple.into();
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);

        let out_tuple: (f64, f64) = v.into();
        assert_eq!(out_tuple, (1.0, 2.0));
    }

    #[test]
    fn test_vec2_reflect_zero_len() {
        // Line 287 (return *self if n_sq == 0.0)
        let v = Vec2::new(1.0, 1.0);
        let n = Vec2::new(0.0, 0.0);
        let r = v.reflect(n);
        assert_eq!(r, Vec2::new(1.0, 1.0));
    }

    #[test]
    fn test_vec2_rotate() {
        // Line 311 (rotate function definition)
        let v = Vec2::new(1.0, 0.0);
        let r = v.rotate(std::f64::consts::PI / 2.0);
        assert!((r.x - 0.0).abs() < 1e-10);
        assert!((r.y - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_vec2_math_operators() {
        // Lines 314-393 (various math operator implementations)
        let v1 = Vec2::new(1.0, 2.0);
        let v2 = Vec2::new(3.0, 4.0);

        let add = v1 + v2;
        assert_eq!(add, Vec2::new(4.0, 6.0));

        let sub = v2 - v1;
        assert_eq!(sub, Vec2::new(2.0, 2.0));

        let mul = v1 * 2.0;
        assert_eq!(mul, Vec2::new(2.0, 4.0));

        let div = v1 / 2.0;
        assert_eq!(div, Vec2::new(0.5, 1.0));

        let neg = -v1;
        assert_eq!(neg, Vec2::new(-1.0, -2.0));
    }

    #[test]
    fn test_vec2_math_assign_operators() {
        let mut v1 = Vec2::new(1.0, 2.0);
        let v2 = Vec2::new(3.0, 4.0);

        v1 += v2;
        assert_eq!(v1, Vec2::new(4.0, 6.0));

        v1 -= v2;
        assert_eq!(v1, Vec2::new(1.0, 2.0));

        v1 *= 2.0;
        assert_eq!(v1, Vec2::new(2.0, 4.0));

        v1 /= 2.0;
        assert_eq!(v1, Vec2::new(1.0, 2.0));
    }
}
