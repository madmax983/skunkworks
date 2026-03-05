//! # 4-Dimensional Vectors
//!
//! Provides the [`Vec4`] type for calculating and transforming coordinates in 4D space.
//!
//! Includes utility methods for rotating along different 4D planes (e.g. `rotate_xw`,
//! `rotate_yw`) and projecting the 4D points down to a 3-dimensional [`Vec3`] for
//! rendering logic.

use crate::vec3::Vec3;

/// A 4-dimensional vector with x, y, z, and w components.
///
/// This struct is the fundamental building block for the "Hyper" series experiments,
/// allowing for the representation of points and vectors in 4D space.
///
/// # Examples
///
/// ```
/// use locus::vec4::Vec4;
///
/// let v = Vec4::new(1.0, 2.0, 3.0, 4.0);
/// assert_eq!(v.w, 4.0);
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Vec4 {
    /// The X component.
    pub x: f32,
    /// The Y component.
    pub y: f32,
    /// The Z component.
    pub z: f32,
    /// The W component (the 4th dimension).
    pub w: f32,
}

impl Vec4 {
    /// Creates a new 4D vector.
    ///
    /// # Arguments
    ///
    /// * `x` - The X component.
    /// * `y` - The Y component.
    /// * `z` - The Z component.
    /// * `w` - The W component.
    ///
    /// # Examples
    ///
    /// ```
    /// use locus::vec4::Vec4;
    /// let v = Vec4::new(0.0, 1.0, 0.0, 1.0);
    /// ```
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    /// Creates a zero vector (0, 0, 0, 0).
    ///
    /// # Examples
    ///
    /// ```
    /// use locus::vec4::Vec4;
    /// let v = Vec4::zero();
    /// assert_eq!(v.length(), 0.0);
    /// ```
    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0)
    }

    /// Scales the vector by a scalar value.
    ///
    /// # Examples
    ///
    /// ```
    /// use locus::vec4::Vec4;
    /// let v = Vec4::new(1.0, 1.0, 1.0, 1.0);
    /// let scaled = v.scale(2.0);
    /// assert_eq!(scaled.x, 2.0);
    /// assert_eq!(scaled.w, 2.0);
    /// ```
    pub fn scale(&self, s: f32) -> Self {
        Self::new(self.x * s, self.y * s, self.z * s, self.w * s)
    }

    /// Scales each dimension by a specific factor.
    ///
    /// # Examples
    ///
    /// ```
    /// use locus::vec4::Vec4;
    /// let v = Vec4::new(1.0, 1.0, 1.0, 1.0);
    /// let scaled = v.scale_dim(2.0, 3.0, 4.0, 5.0);
    /// assert_eq!(scaled.x, 2.0);
    /// assert_eq!(scaled.w, 5.0);
    /// ```
    pub fn scale_dim(&self, sx: f32, sy: f32, sz: f32, sw: f32) -> Self {
        Self::new(self.x * sx, self.y * sy, self.z * sz, self.w * sw)
    }

    /// Adds another vector to this one.
    ///
    /// # Examples
    ///
    /// ```
    /// use locus::vec4::Vec4;
    /// let v1 = Vec4::new(1.0, 0.0, 0.0, 0.0);
    /// let v2 = Vec4::new(0.0, 1.0, 0.0, 0.0);
    /// let sum = v1.add(v2);
    /// assert_eq!(sum.x, 1.0);
    /// assert_eq!(sum.y, 1.0);
    /// ```
    pub fn add(&self, other: Vec4) -> Self {
        Self::new(
            self.x + other.x,
            self.y + other.y,
            self.z + other.z,
            self.w + other.w,
        )
    }

    /// Subtracts another vector from this one.
    ///
    /// This is useful for finding the direction and distance from one point to another.
    ///
    /// # Examples
    ///
    /// ```
    /// use locus::vec4::Vec4;
    /// let v1 = Vec4::new(1.0, 2.0, 3.0, 4.0);
    /// let v2 = Vec4::new(1.0, 1.0, 1.0, 1.0);
    /// let diff = v1.sub(v2);
    /// assert_eq!(diff.x, 0.0);
    /// assert_eq!(diff.w, 3.0);
    /// ```
    pub fn sub(&self, other: Vec4) -> Self {
        Self::new(
            self.x - other.x,
            self.y - other.y,
            self.z - other.z,
            self.w - other.w,
        )
    }

    /// Calculates the squared length (magnitude) of the vector.
    ///
    /// This is faster than `length()` as it avoids the square root operation.
    /// Useful for comparisons.
    ///
    /// # Examples
    ///
    /// ```
    /// use locus::vec4::Vec4;
    /// let v = Vec4::new(1.0, 1.0, 1.0, 1.0);
    /// // 1^2 + 1^2 + 1^2 + 1^2 = 4
    /// assert_eq!(v.length_squared(), 4.0);
    /// ```
    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }

    /// Calculates the length (magnitude) of the vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use locus::vec4::Vec4;
    /// let v = Vec4::new(0.0, 3.0, 0.0, 4.0);
    /// // sqrt(3^2 + 4^2) = 5
    /// assert_eq!(v.length(), 5.0);
    /// ```
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    /// Returns a normalized unit vector (length of 1.0).
    ///
    /// If the vector has zero length, it returns the zero vector.
    /// Uses robust normalization to handle very large or small components.
    ///
    /// # Examples
    ///
    /// ```
    /// use locus::vec4::Vec4;
    /// let v = Vec4::new(0.0, 3.0, 0.0, 4.0); // Length is 5
    /// let n = v.normalize();
    /// assert!((n.length() - 1.0).abs() < 1e-6);
    /// ```
    pub fn normalize(&self) -> Self {
        // Robust normalization avoiding overflow/underflow
        let m = self
            .x
            .abs()
            .max(self.y.abs())
            .max(self.z.abs())
            .max(self.w.abs());
        if m > 0.0 && m.is_finite() {
            let s = 1.0 / m;
            let scaled = self.scale(s);
            let len = scaled.length();
            if len > 0.0 {
                scaled.scale(1.0 / len)
            } else {
                Self::zero()
            }
        } else {
            Self::zero()
        }
    }

    /// Limits the magnitude of the vector to a maximum value.
    ///
    /// If the vector's length is greater than `max`, it is normalized and scaled to `max`.
    /// Otherwise, it returns the original vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use locus::vec4::Vec4;
    ///
    /// let v = Vec4::new(10.0, 0.0, 0.0, 0.0);
    /// let limited = v.limit(5.0);
    ///
    /// assert_eq!(limited.x, 5.0);
    /// assert_eq!(limited.length(), 5.0);
    ///
    /// // A vector shorter than the limit is unchanged
    /// let small = Vec4::new(1.0, 0.0, 0.0, 0.0);
    /// let limited_small = small.limit(5.0);
    /// assert_eq!(limited_small.x, 1.0);
    /// ```
    pub fn limit(&self, max: f32) -> Self {
        let max = max.abs();
        let sq_len = self.length_squared();
        // Check for infinite squared length (overflow) with finite max
        if sq_len > max * max || (sq_len.is_infinite() && max.is_finite()) {
            // Optimization: Avoid full normalize() which does extra max/scale logic
            if sq_len.is_finite() {
                // Common case: finite vector, just scale
                let len = sq_len.sqrt();
                if len > 0.0 {
                    self.scale(max / len)
                } else {
                    Self::zero()
                }
            } else {
                // Edge case: Overflow or Infinity, use robust normalize
                self.normalize().scale(max)
            }
        } else {
            *self
        }
    }

    /// Calculates the squared Euclidean distance to another vector.
    ///
    /// This is faster than calculating the exact distance because it avoids the square root.
    /// Useful for distance comparisons (e.g., checking if a point is within a radius).
    ///
    /// # Examples
    ///
    /// ```
    /// use locus::vec4::Vec4;
    ///
    /// let v1 = Vec4::new(0.0, 0.0, 0.0, 0.0);
    /// let v2 = Vec4::new(1.0, 1.0, 1.0, 1.0);
    ///
    /// // 1^2 + 1^2 + 1^2 + 1^2 = 4
    /// assert_eq!(v1.distance_squared(v2), 4.0);
    /// ```
    pub fn distance_squared(&self, other: Vec4) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        let dw = self.w - other.w;
        dx * dx + dy * dy + dz * dz + dw * dw
    }

    /// Rotates the vector in the XY plane.
    ///
    /// In 3D (and 4D), this is a rotation around the Z axis (and W axis).
    /// It affects the X and Y components.
    ///
    /// # Arguments
    ///
    /// * `theta` - The angle of rotation in radians.
    ///
    /// # Examples
    ///
    /// ```
    /// use locus::vec4::Vec4;
    /// use std::f32::consts::PI;
    ///
    /// let v = Vec4::new(1.0, 0.0, 0.0, 0.0);
    /// // Rotate 90 degrees in XY plane
    /// let rotated = v.rotate_xy(PI / 2.0);
    ///
    /// // v moves from X to Y axis
    /// assert!(rotated.x.abs() < 1e-6);
    /// assert!((rotated.y - 1.0).abs() < 1e-6);
    /// ```
    pub fn rotate_xy(&self, theta: f32) -> Self {
        self.rotate_xy_fast(theta.sin(), theta.cos())
    }

    /// Rotates the vector in the XY plane using precomputed sine and cosine values.
    ///
    /// This optimization is critical when rendering a large point cloud or grid,
    /// where computing `sin(theta)` and `cos(theta)` inside a tight loop would severely
    /// impact performance. Precompute the trig functions once, then pass them in.
    ///
    /// # Examples
    ///
    /// ```
    /// use locus::vec4::Vec4;
    /// use std::f32::consts::PI;
    ///
    /// let point = Vec4::new(1.0, 0.0, 0.0, 0.0);
    /// let theta = PI / 2.0;
    /// let sin_t = theta.sin();
    /// let cos_t = theta.cos();
    ///
    /// let rotated = point.rotate_xy_fast(sin_t, cos_t);
    /// assert!((rotated.y - 1.0).abs() < 1e-6);
    /// ```
    pub fn rotate_xy_fast(&self, sin_theta: f32, cos_theta: f32) -> Self {
        Self {
            x: self.x * cos_theta - self.y * sin_theta,
            y: self.x * sin_theta + self.y * cos_theta,
            z: self.z,
            w: self.w,
        }
    }

    /// Rotates the vector in the XW plane.
    ///
    /// In 4D space, rotations occur in planes defined by two axes.
    /// Rotating in the XW plane changes the X and W components while leaving Y and Z unchanged.
    ///
    /// # Arguments
    ///
    /// * `theta` - The angle of rotation in radians.
    ///
    /// # Examples
    ///
    /// ```
    /// use locus::vec4::Vec4;
    /// use std::f32::consts::PI;
    ///
    /// let v = Vec4::new(1.0, 0.0, 0.0, 0.0);
    /// // Rotate 90 degrees in XW plane
    /// let rotated = v.rotate_xw(PI / 2.0);
    /// // v moves from X to W axis
    /// assert!(rotated.x.abs() < 1e-6);
    /// assert!((rotated.w - 1.0).abs() < 1e-6);
    /// ```
    pub fn rotate_xw(&self, theta: f32) -> Self {
        self.rotate_xw_fast(theta.sin(), theta.cos())
    }

    /// Rotates the vector in the XW plane using precomputed sine and cosine values.
    ///
    /// Useful for optimizing loops where the rotation angle is constant across many vectors.
    /// This prevents redundant trigonometry calculations on every iteration.
    ///
    /// # Examples
    ///
    /// ```
    /// use locus::vec4::Vec4;
    /// use std::f32::consts::PI;
    ///
    /// let point = Vec4::new(1.0, 0.0, 0.0, 0.0);
    /// let theta = PI / 2.0;
    /// let (sin_t, cos_t) = theta.sin_cos();
    ///
    /// let rotated = point.rotate_xw_fast(sin_t, cos_t);
    /// assert!((rotated.w - 1.0).abs() < 1e-6);
    /// ```
    pub fn rotate_xw_fast(&self, sin_theta: f32, cos_theta: f32) -> Self {
        Self {
            x: self.x * cos_theta - self.w * sin_theta,
            y: self.y,
            z: self.z,
            w: self.x * sin_theta + self.w * cos_theta,
        }
    }

    /// Rotates the vector in the XZ plane.
    ///
    /// In 3D (and 4D), this is a rotation around the Y axis (and W axis).
    /// It affects the X and Z components.
    ///
    /// # Arguments
    ///
    /// * `theta` - The angle of rotation in radians.
    ///
    /// # Examples
    ///
    /// ```
    /// use locus::vec4::Vec4;
    /// use std::f32::consts::PI;
    ///
    /// let v = Vec4::new(1.0, 0.0, 0.0, 0.0);
    /// // Rotate 90 degrees in XZ plane
    /// let rotated = v.rotate_xz(PI / 2.0);
    ///
    /// // v moves from X to Z axis (Note: typical Y-axis rotation direction might flip sign,
    /// // but here we use standard 2D rotation on (x, z))
    /// assert!(rotated.x.abs() < 1e-6);
    /// assert!((rotated.z - 1.0).abs() < 1e-6);
    /// ```
    pub fn rotate_xz(&self, theta: f32) -> Self {
        self.rotate_xz_fast(theta.sin(), theta.cos())
    }

    /// Rotates the vector in the XZ plane using precomputed sine and cosine values.
    ///
    /// Use this variant inside high-performance loops to apply a uniform 3D/4D
    /// rotation without the overhead of computing `sin` and `cos` for every vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use locus::vec4::Vec4;
    /// use std::f32::consts::PI;
    ///
    /// let point = Vec4::new(1.0, 0.0, 0.0, 0.0);
    /// let theta = PI / 2.0;
    /// let (sin_t, cos_t) = theta.sin_cos();
    ///
    /// let rotated = point.rotate_xz_fast(sin_t, cos_t);
    /// assert!((rotated.z - 1.0).abs() < 1e-6);
    /// ```
    pub fn rotate_xz_fast(&self, sin_theta: f32, cos_theta: f32) -> Self {
        Self {
            x: self.x * cos_theta - self.z * sin_theta,
            y: self.y,
            z: self.x * sin_theta + self.z * cos_theta,
            w: self.w,
        }
    }

    /// Rotates the vector in the YW plane.
    ///
    /// In 4D space, this rotation affects the Y and W components, leaving X and Z unchanged.
    ///
    /// # Arguments
    ///
    /// * `theta` - The angle of rotation in radians.
    ///
    /// # Examples
    ///
    /// ```
    /// use locus::vec4::Vec4;
    /// use std::f32::consts::PI;
    ///
    /// let v = Vec4::new(0.0, 1.0, 0.0, 0.0);
    /// // Rotate 90 degrees in YW plane
    /// let rotated = v.rotate_yw(PI / 2.0);
    ///
    /// // v moves from Y to W axis
    /// assert!(rotated.y.abs() < 1e-6);
    /// assert!((rotated.w - 1.0).abs() < 1e-6);
    /// ```
    pub fn rotate_yw(&self, theta: f32) -> Self {
        self.rotate_yw_fast(theta.sin(), theta.cos())
    }

    /// Rotates the vector in the YW plane using precomputed sine and cosine values.
    ///
    /// Precomputing the sine and cosine saves massive CPU cycles when projecting
    /// thousands of 4D hyper-structures at once.
    ///
    /// # Examples
    ///
    /// ```
    /// use locus::vec4::Vec4;
    /// use std::f32::consts::PI;
    ///
    /// let point = Vec4::new(0.0, 1.0, 0.0, 0.0);
    /// let theta = PI / 2.0;
    /// let (sin_t, cos_t) = theta.sin_cos();
    ///
    /// let rotated = point.rotate_yw_fast(sin_t, cos_t);
    /// assert!((rotated.w - 1.0).abs() < 1e-6);
    /// ```
    pub fn rotate_yw_fast(&self, sin_theta: f32, cos_theta: f32) -> Self {
        Self {
            x: self.x,
            y: self.y * cos_theta - self.w * sin_theta,
            z: self.z,
            w: self.y * sin_theta + self.w * cos_theta,
        }
    }

    /// Rotates the vector in the YZ plane.
    ///
    /// In 3D (and 4D), this is a rotation around the X axis (and W axis).
    /// It affects the Y and Z components.
    ///
    /// # Arguments
    ///
    /// * `theta` - The angle of rotation in radians.
    ///
    /// # Examples
    ///
    /// ```
    /// use locus::vec4::Vec4;
    /// use std::f32::consts::PI;
    ///
    /// let v = Vec4::new(0.0, 1.0, 0.0, 0.0);
    /// // Rotate 90 degrees in YZ plane
    /// let rotated = v.rotate_yz(PI / 2.0);
    ///
    /// // v moves from Y to Z axis
    /// assert!(rotated.y.abs() < 1e-6);
    /// assert!((rotated.z - 1.0).abs() < 1e-6);
    /// ```
    pub fn rotate_yz(&self, theta: f32) -> Self {
        self.rotate_yz_fast(theta.sin(), theta.cos())
    }

    /// Rotates the vector in the YZ plane using precomputed sine and cosine values.
    ///
    /// Bypassing the trigonometric calculation allows the compiler to auto-vectorize
    /// continuous memory layouts of `Vec4` instances much more efficiently.
    ///
    /// # Examples
    ///
    /// ```
    /// use locus::vec4::Vec4;
    /// use std::f32::consts::PI;
    ///
    /// let point = Vec4::new(0.0, 1.0, 0.0, 0.0);
    /// let theta = PI / 2.0;
    /// let (sin_t, cos_t) = theta.sin_cos();
    ///
    /// let rotated = point.rotate_yz_fast(sin_t, cos_t);
    /// assert!((rotated.z - 1.0).abs() < 1e-6);
    /// ```
    pub fn rotate_yz_fast(&self, sin_theta: f32, cos_theta: f32) -> Self {
        Self {
            x: self.x,
            y: self.y * cos_theta - self.z * sin_theta,
            z: self.y * sin_theta + self.z * cos_theta,
            w: self.w,
        }
    }

    /// Rotates the vector in the ZW plane.
    ///
    /// In 4D space, this rotation affects the Z and W components, leaving X and Y unchanged.
    ///
    /// # Arguments
    ///
    /// * `theta` - The angle of rotation in radians.
    ///
    /// # Examples
    ///
    /// ```
    /// use locus::vec4::Vec4;
    /// use std::f32::consts::PI;
    ///
    /// let v = Vec4::new(0.0, 0.0, 1.0, 0.0);
    /// // Rotate 90 degrees in ZW plane
    /// let rotated = v.rotate_zw(PI / 2.0);
    ///
    /// // v moves from Z to W axis
    /// assert!(rotated.z.abs() < 1e-6);
    /// assert!((rotated.w - 1.0).abs() < 1e-6);
    /// ```
    pub fn rotate_zw(&self, theta: f32) -> Self {
        self.rotate_zw_fast(theta.sin(), theta.cos())
    }

    /// Rotates the vector in the ZW plane using precomputed sine and cosine values.
    ///
    /// This fast-path rotation is essential for maintaining frame rates when rendering
    /// dense 4D topologies in software.
    ///
    /// # Examples
    ///
    /// ```
    /// use locus::vec4::Vec4;
    /// use std::f32::consts::PI;
    ///
    /// let point = Vec4::new(0.0, 0.0, 1.0, 0.0);
    /// let theta = PI / 2.0;
    /// let (sin_t, cos_t) = theta.sin_cos();
    ///
    /// let rotated = point.rotate_zw_fast(sin_t, cos_t);
    /// assert!((rotated.w - 1.0).abs() < 1e-6);
    /// ```
    pub fn rotate_zw_fast(&self, sin_theta: f32, cos_theta: f32) -> Self {
        Self {
            x: self.x,
            y: self.y,
            z: self.z * cos_theta - self.w * sin_theta,
            w: self.z * sin_theta + self.w * cos_theta,
        }
    }

    /// Projects the 4D point into 3D space using stereographic projection.
    ///
    /// This simulates a camera looking "down" from the W axis. As points move further
    /// away in the W dimension (relative to `camera_w`), they appear smaller (perspective).
    ///
    /// # Arguments
    ///
    /// * `camera_w` - The W coordinate of the observer/camera.
    ///
    /// # Returns
    ///
    /// A `Vec3` representing the 3D projection.
    ///
    /// # Examples
    ///
    /// ```
    /// use locus::vec4::Vec4;
    ///
    /// // A point "far away" in the W dimension (w=0) relative to camera (w=10)
    /// let far = Vec4::new(1.0, 1.0, 1.0, 0.0);
    /// // A point "close" to the camera (w=8)
    /// let close = Vec4::new(1.0, 1.0, 1.0, 8.0);
    ///
    /// let p_far = far.project_to_3d(10.0);
    /// let p_close = close.project_to_3d(10.0);
    ///
    /// // The closer point appears larger (projected further out)
    /// assert!(p_close.x > p_far.x);
    /// ```
    ///
    /// # Implementation Details
    ///
    /// The projection formula used is:
    /// `scale = 2.0 / (camera_w - w)`
    /// `projected = original * scale`
    ///
    /// - **Scale Factor (2.0)**: Acts as a field-of-view modifier (Zoom).
    /// - **Safety Clamp (0.1)**: The denominator `(camera_w - w)` is clamped to a minimum
    ///   of `0.1` to prevent division by zero or negative projection artifacts when points
    ///   are behind the camera.
    pub fn project_to_3d(&self, camera_w: f32) -> Vec3 {
        let w_dist = camera_w - self.w;
        // Avoid division by zero
        let scale = 2.0 / w_dist.max(0.1);

        let safe_mul = |val: f32, s: f32| {
            let res = val * s;
            if res.is_finite() {
                res
            } else {
                // Clamp overflow or handle NaN
                if res > 0.0 {
                    f32::MAX
                } else if res < 0.0 {
                    f32::MIN
                } else {
                    0.0
                }
            }
        };

        Vec3::new(
            safe_mul(self.x, scale),
            safe_mul(self.y, scale),
            safe_mul(self.z, scale),
        )
    }
}

impl std::ops::Add for Vec4 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Vec4::add(&self, other)
    }
}

impl std::ops::AddAssign for Vec4 {
    fn add_assign(&mut self, other: Self) {
        *self = Vec4::add(self, other);
    }
}

impl std::ops::Sub for Vec4 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Vec4::sub(&self, other)
    }
}

impl std::ops::Div<f32> for Vec4 {
    type Output = Self;
    fn div(self, rhs: f32) -> Self {
        self.scale(1.0 / rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec4_ops() {
        let v1 = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let v2 = Vec4::new(4.0, 3.0, 2.0, 1.0);
        let sum = v1 + v2;
        assert_eq!(sum.x, 5.0);
        assert_eq!(sum.w, 5.0);

        let sub = v1 - v2;
        assert_eq!(sub.x, -3.0);
    }

    #[test]
    fn test_scale() {
        let v = Vec4::new(1.0, 1.0, 1.0, 1.0);
        let scaled = v.scale(2.0);
        assert_eq!(scaled.x, 2.0);
        assert_eq!(scaled.w, 2.0);
    }

    #[test]
    fn test_rotation_fast() {
        let v = Vec4::new(1.0, 0.0, 0.0, 0.0);
        let theta = std::f32::consts::PI / 2.0;
        let sin_t = theta.sin();
        let cos_t = theta.cos();

        let rotated = v.rotate_xw(theta);
        let rotated_fast = v.rotate_xw_fast(sin_t, cos_t);

        assert!((rotated.w - 1.0).abs() < 1e-6);
        assert!((rotated_fast.w - 1.0).abs() < 1e-6);
        assert!((rotated.w - rotated_fast.w).abs() < 1e-6);
    }

    #[test]
    fn test_normalize_zero() {
        let v = Vec4::zero();
        let n = v.normalize();
        assert_eq!(n.x, 0.0);
        assert_eq!(n.y, 0.0);
        assert_eq!(n.z, 0.0);
        assert_eq!(n.w, 0.0);
        assert_eq!(n.length(), 0.0);
    }

    #[test]
    fn test_limit() {
        let v = Vec4::new(10.0, 0.0, 0.0, 0.0);
        let limited = v.limit(5.0);
        assert!((limited.length() - 5.0).abs() < 1e-6);
        assert_eq!(limited.x, 5.0);

        let small = Vec4::new(1.0, 0.0, 0.0, 0.0);
        let limited_small = small.limit(5.0);
        assert_eq!(limited_small.x, 1.0);
    }

    #[test]
    fn test_distance_squared() {
        let v1 = Vec4::new(0.0, 0.0, 0.0, 0.0);
        let v2 = Vec4::new(1.0, 1.0, 1.0, 1.0);
        assert_eq!(v1.distance_squared(v2), 4.0);
    }

    #[test]
    fn test_scale_dim() {
        let v = Vec4::new(1.0, 1.0, 1.0, 1.0);
        let s = v.scale_dim(2.0, 3.0, 4.0, 5.0);
        assert_eq!(s.x, 2.0);
        assert_eq!(s.y, 3.0);
        assert_eq!(s.z, 4.0);
        assert_eq!(s.w, 5.0);
    }

    #[test]
    fn test_rotations_all_planes() {
        let theta = std::f32::consts::PI / 2.0;

        // Rotate XY: changes X and Y
        let v_xy = Vec4::new(1.0, 0.0, 0.0, 0.0);
        let rot_xy = v_xy.rotate_xy(theta);
        // x' = x cos - y sin = 0 - 0 = 0
        // y' = x sin + y cos = 1 + 0 = 1
        assert!(rot_xy.x.abs() < 1e-6);
        assert!((rot_xy.y - 1.0).abs() < 1e-6);
        assert_eq!(rot_xy.z, 0.0);
        assert_eq!(rot_xy.w, 0.0);

        // Rotate XZ: changes X and Z
        let v_xz = Vec4::new(1.0, 0.0, 0.0, 0.0);
        let rot_xz = v_xz.rotate_xz(theta);
        // x' = x cos - z sin = 0 - 0 = 0
        // z' = x sin + z cos = 1 + 0 = 1
        assert!(rot_xz.x.abs() < 1e-6);
        assert!((rot_xz.z - 1.0).abs() < 1e-6);
        assert_eq!(rot_xz.y, 0.0);
        assert_eq!(rot_xz.w, 0.0);

        // Rotate YZ: changes Y and Z
        let v_yz = Vec4::new(0.0, 1.0, 0.0, 0.0);
        let rot_yz = v_yz.rotate_yz(theta);
        // y' = y cos - z sin = 0 - 0 = 0
        // z' = y sin + z cos = 1 + 0 = 1
        assert!(rot_yz.y.abs() < 1e-6);
        assert!((rot_yz.z - 1.0).abs() < 1e-6);
        assert_eq!(rot_yz.x, 0.0);
        assert_eq!(rot_yz.w, 0.0);

        // Rotate YW: changes Y and W
        let v_yw = Vec4::new(0.0, 1.0, 0.0, 0.0);
        let rot_yw = v_yw.rotate_yw(theta);
        // y' = y cos - w sin = 0 - 0 = 0
        // w' = y sin + w cos = 1 + 0 = 1
        assert!(rot_yw.y.abs() < 1e-6);
        assert!((rot_yw.w - 1.0).abs() < 1e-6);
        assert_eq!(rot_yw.x, 0.0);
        assert_eq!(rot_yw.z, 0.0);

        // Rotate ZW: changes Z and W
        let v_zw = Vec4::new(0.0, 0.0, 1.0, 0.0);
        let rot_zw = v_zw.rotate_zw(theta);
        // z' = z cos - w sin = 0 - 0 = 0
        // w' = z sin + w cos = 1 + 0 = 1
        assert!(rot_zw.z.abs() < 1e-6);
        assert!((rot_zw.w - 1.0).abs() < 1e-6);
        assert_eq!(rot_zw.x, 0.0);
        assert_eq!(rot_zw.y, 0.0);
    }

    #[test]
    fn test_project_to_3d_basic() {
        let v = Vec4::new(1.0, 2.0, 3.0, 0.0);
        let camera_w = 10.0;
        // w_dist = 10.0 - 0.0 = 10.0
        // scale = 2.0 / 10.0 = 0.2
        let proj = v.project_to_3d(camera_w);
        assert!((proj.x - 0.2).abs() < 1e-6);
        assert!((proj.y - 0.4).abs() < 1e-6);
        assert!((proj.z - 0.6).abs() < 1e-6);
    }

    #[test]
    fn test_project_to_3d_clamping() {
        // Point exactly at camera
        let v_at_cam = Vec4::new(1.0, 1.0, 1.0, 10.0);
        let camera_w = 10.0;
        // w_dist = 0.0 -> max(0.1) -> 0.1
        // scale = 2.0 / 0.1 = 20.0
        let proj = v_at_cam.project_to_3d(camera_w);
        assert!((proj.x - 20.0).abs() < 1e-6);

        // Point behind camera
        let v_behind = Vec4::new(1.0, 1.0, 1.0, 15.0);
        // w_dist = -5.0 -> max(0.1) -> 0.1
        // scale = 2.0 / 0.1 = 20.0
        let proj_behind = v_behind.project_to_3d(camera_w);
        assert!((proj_behind.x - 20.0).abs() < 1e-6);
    }
}
