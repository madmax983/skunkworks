//! Mathematical utilities for 4D vector operations.
//!
//! This module provides the [`Vec4`] struct, which represents a 4-dimensional vector
//! (x, y, z, w). It includes methods for arithmetic, normalization, and geometric
//! transformations such as 4D rotation and projection into 3D space.

#[cfg(feature = "macroquad")]
use macroquad::prelude::Vec3 as MacroquadVec3;

/// A simple 3D vector for projection results.
///
/// Used to avoid dependency on external crates for core math types.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

#[cfg(feature = "macroquad")]
impl From<Vec3> for MacroquadVec3 {
    fn from(v: Vec3) -> Self {
        MacroquadVec3::new(v.x, v.y, v.z)
    }
}

#[cfg(feature = "macroquad")]
impl From<MacroquadVec3> for Vec3 {
    fn from(v: MacroquadVec3) -> Self {
        Self::new(v.x, v.y, v.z)
    }
}

/// A 4-dimensional vector with x, y, z, and w components.
///
/// This struct is the fundamental building block for the "Hyper" series experiments,
/// allowing for the representation of points and vectors in 4D space.
///
/// # Examples
///
/// ```
/// use hyper_system::math::Vec4;
///
/// let v = Vec4::new(1.0, 2.0, 3.0, 4.0);
/// assert_eq!(v.w, 4.0);
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
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
    /// use hyper_system::math::Vec4;
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
    /// use hyper_system::math::Vec4;
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
    /// use hyper_system::math::Vec4;
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
    /// use hyper_system::math::Vec4;
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
    /// use hyper_system::math::Vec4;
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
    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }

    /// Calculates the length (magnitude) of the vector.
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    /// Returns a normalized unit vector (length of 1.0).
    ///
    /// If the vector has zero length, it returns the zero vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use hyper_system::math::Vec4;
    /// let v = Vec4::new(0.0, 3.0, 0.0, 4.0); // Length is 5
    /// let n = v.normalize();
    /// assert!((n.length() - 1.0).abs() < 1e-6);
    /// ```
    pub fn normalize(&self) -> Self {
        let len = self.length();
        if len > 0.0 {
            self.scale(1.0 / len)
        } else {
            Self::zero()
        }
    }

    /// Limits the magnitude of the vector to a maximum value.
    ///
    /// If the vector's length is greater than `max`, it is normalized and scaled to `max`.
    pub fn limit(&self, max: f32) -> Self {
        if self.length_squared() > max * max {
            self.normalize().scale(max)
        } else {
            *self
        }
    }

    /// Calculates the squared Euclidean distance to another vector.
    pub fn distance_squared(&self, other: Vec4) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        let dw = self.w - other.w;
        dx * dx + dy * dy + dz * dz + dw * dw
    }

    /// Rotates the vector in the XW plane.
    ///
    /// In 4D space, rotations occur in planes defined by two axes.
    /// Rotating in the XW plane changes the X and W components while leaving Y and Z unchanged.
    ///
    /// # Arguments
    ///
    /// * `theta` - The angle of rotation in radians.
    pub fn rotate_xw(&self, theta: f32) -> Self {
        let c = theta.cos();
        let s = theta.sin();
        Self {
            x: self.x * c - self.w * s,
            y: self.y,
            z: self.z,
            w: self.x * s + self.w * c,
        }
    }

    /// Rotates the vector in the YW plane.
    ///
    /// Changes the Y and W components.
    pub fn rotate_yw(&self, theta: f32) -> Self {
        let c = theta.cos();
        let s = theta.sin();
        Self {
            x: self.x,
            y: self.y * c - self.w * s,
            z: self.z,
            w: self.y * s + self.w * c,
        }
    }

    /// Rotates the vector in the ZW plane.
    ///
    /// Changes the Z and W components.
    pub fn rotate_zw(&self, theta: f32) -> Self {
        let c = theta.cos();
        let s = theta.sin();
        Self {
            x: self.x,
            y: self.y,
            z: self.z * c - self.w * s,
            w: self.z * s + self.w * c,
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
    pub fn project_to_3d(&self, camera_w: f32) -> Vec3 {
        let w_dist = camera_w - self.w;
        // Avoid division by zero
        let scale = 2.0 / w_dist.max(0.1);
        Vec3::new(self.x * scale, self.y * scale, self.z * scale)
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
}
