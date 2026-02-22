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

    pub fn project_to_3d(&self, camera_w: f32) -> Vec3 {
        let w_dist = camera_w - self.w;
        let scale = 2.0 / w_dist.max(0.1);
        vec3(self.x * scale, self.y * scale, self.z * scale)
    }
}
