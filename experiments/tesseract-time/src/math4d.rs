use macroquad::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0)
    }

    pub fn add(&self, other: Vec4) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }

    pub fn scale(&self, s: f32) -> Self {
        Self {
            x: self.x * s,
            y: self.y * s,
            z: self.z * s,
            w: self.w * s,
        }
    }

    // Rotations involving W axis (The "Impossible" rotations)
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

    // Standard 3D rotations extended to 4D
    pub fn rotate_xy(&self, theta: f32) -> Self {
        let c = theta.cos();
        let s = theta.sin();
        Self {
            x: self.x * c - self.y * s,
            y: self.x * s + self.y * c,
            z: self.z,
            w: self.w,
        }
    }

    pub fn rotate_xz(&self, theta: f32) -> Self {
        let c = theta.cos();
        let s = theta.sin();
        Self {
            x: self.x * c - self.z * s,
            y: self.y,
            z: self.x * s + self.z * c,
            w: self.w,
        }
    }

    pub fn rotate_yz(&self, theta: f32) -> Self {
        let c = theta.cos();
        let s = theta.sin();
        Self {
            x: self.x,
            y: self.y * c - self.z * s,
            z: self.y * s + self.z * c,
            w: self.w,
        }
    }

    // Project 4D point to 3D space using perspective projection
    // 'camera_w' is the distance of the 4D camera along the W axis.
    pub fn project_to_3d(&self, camera_w: f32) -> Vec3 {
        // Perspective factor: 1 / (distance - w)
        // If w gets close to camera_w, it goes to infinity.
        // We assume camera is at w = -camera_w looking towards +w or vice versa.
        // Let's assume camera is at W = -2.0, looking at origin.

        let distance = camera_w - self.w;
        if distance.abs() < 0.001 {
            return vec3(self.x * 1000.0, self.y * 1000.0, self.z * 1000.0);
        }

        // Standard perspective projection formula:
        // x' = x / (d - w)
        // But we want to keep scale reasonable.
        // let scale = 1.0 / (1.0 - self.w / camera_w); // Simplified

        // Let's use standard:
        // x' = x * (focal_length / (w_dist))
        let focal_length = 2.0;
        let w_dist = 3.0 - self.w; // Camera at w=3

        let s = focal_length / w_dist.max(0.1);

        vec3(self.x * s, self.y * s, self.z * s)
    }
}
