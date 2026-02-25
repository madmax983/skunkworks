//! Mathematical utilities for 4D vector operations.
//!
//! This module provides the [`Vec4`] struct (re-exported from `glam`) and the [`HyperVector`] trait
//! for 4D-specific operations.

pub use glam::{Vec3, Vec4};

/// Trait extending `glam::Vec4` with hyper-system specific functionality.
pub trait HyperVector: Sized + Copy {
    fn zero() -> Self;
    fn scale(&self, s: f32) -> Self;
    fn scale_dim(&self, sx: f32, sy: f32, sz: f32, sw: f32) -> Self;
    fn add(&self, other: Self) -> Self;
    fn sub(&self, other: Self) -> Self;
    fn limit(&self, max: f32) -> Self;
    fn distance_squared_to(&self, other: Self) -> f32; // Renamed to avoid collision/ambiguity? No, glam has distance_squared.

    // Custom Hyper methods
    fn rotate_xw(&self, theta: f32) -> Self;
    fn rotate_xw_fast(&self, sin_theta: f32, cos_theta: f32) -> Self;
    fn rotate_yw(&self, theta: f32) -> Self;
    fn rotate_yw_fast(&self, sin_theta: f32, cos_theta: f32) -> Self;
    fn rotate_zw(&self, theta: f32) -> Self;
    fn rotate_zw_fast(&self, sin_theta: f32, cos_theta: f32) -> Self;
    fn project_to_3d(&self, camera_w: f32) -> Vec3;
}

impl HyperVector for Vec4 {
    #[inline]
    fn zero() -> Self {
        Self::ZERO
    }

    #[inline]
    fn scale(&self, s: f32) -> Self {
        *self * s
    }

    #[inline]
    fn scale_dim(&self, sx: f32, sy: f32, sz: f32, sw: f32) -> Self {
        *self * Vec4::new(sx, sy, sz, sw)
    }

    #[inline]
    fn add(&self, other: Self) -> Self {
        *self + other
    }

    #[inline]
    fn sub(&self, other: Self) -> Self {
        *self - other
    }

    #[inline]
    fn limit(&self, max: f32) -> Self {
        self.clamp_length_max(max)
    }

    #[inline]
    fn distance_squared_to(&self, other: Self) -> f32 {
        self.distance_squared(other)
    }

    fn rotate_xw(&self, theta: f32) -> Self {
        self.rotate_xw_fast(theta.sin(), theta.cos())
    }

    fn rotate_xw_fast(&self, sin_theta: f32, cos_theta: f32) -> Self {
        Self::new(
            self.x * cos_theta - self.w * sin_theta,
            self.y,
            self.z,
            self.x * sin_theta + self.w * cos_theta,
        )
    }

    fn rotate_yw(&self, theta: f32) -> Self {
        self.rotate_yw_fast(theta.sin(), theta.cos())
    }

    fn rotate_yw_fast(&self, sin_theta: f32, cos_theta: f32) -> Self {
        Self::new(
            self.x,
            self.y * cos_theta - self.w * sin_theta,
            self.z,
            self.y * sin_theta + self.w * cos_theta,
        )
    }

    fn rotate_zw(&self, theta: f32) -> Self {
        self.rotate_zw_fast(theta.sin(), theta.cos())
    }

    fn rotate_zw_fast(&self, sin_theta: f32, cos_theta: f32) -> Self {
        Self::new(
            self.x,
            self.y,
            self.z * cos_theta - self.w * sin_theta,
            self.z * sin_theta + self.w * cos_theta,
        )
    }

    fn project_to_3d(&self, camera_w: f32) -> Vec3 {
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

// Tests to ensure compatibility and correctness
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec4_ops() {
        let v1 = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let v2 = Vec4::new(4.0, 3.0, 2.0, 1.0);
        let sum = v1.add(v2);
        assert_eq!(sum.x, 5.0);
        assert_eq!(sum.w, 5.0);

        let sub = v1.sub(v2);
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
        let n = v.normalize_or_zero();
        assert_eq!(n.x, 0.0);
        assert_eq!(n.y, 0.0);
        assert_eq!(n.z, 0.0);
        assert_eq!(n.w, 0.0);
        assert_eq!(n.length(), 0.0);
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
}
