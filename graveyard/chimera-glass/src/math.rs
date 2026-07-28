use macroquad::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec4D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec4D {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0)
    }

    pub fn scale(&self, s: f32) -> Self {
        Self::new(self.x * s, self.y * s, self.z * s, self.w * s)
    }

    pub fn scale_dim(&self, sx: f32, sy: f32, sz: f32, sw: f32) -> Self {
        Self::new(self.x * sx, self.y * sy, self.z * sz, self.w * sw)
    }

    pub fn add(&self, other: Vec4D) -> Self {
        Self::new(
            self.x + other.x,
            self.y + other.y,
            self.z + other.z,
            self.w + other.w,
        )
    }

    pub fn sub(&self, other: Vec4D) -> Self {
        Self::new(
            self.x - other.x,
            self.y - other.y,
            self.z - other.z,
            self.w - other.w,
        )
    }

    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn normalize(&self) -> Self {
        let len = self.length();
        if len > 0.0 {
            self.scale(1.0 / len)
        } else {
            Self::zero()
        }
    }

    pub fn limit(&self, max: f32) -> Self {
        if self.length_squared() > max * max {
            self.normalize().scale(max)
        } else {
            *self
        }
    }

    pub fn distance_squared(&self, other: Vec4D) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        let dw = self.w - other.w;
        dx * dx + dy * dy + dz * dz + dw * dw
    }

    // Rotations involving W axis
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

    // Project 4D point to 3D space
    pub fn project_to_3d(&self, camera_w: f32) -> Vec3 {
        let w_dist = camera_w - self.w;
        // Avoid division by zero
        let scale = 2.0 / w_dist.max(0.1);
        vec3(self.x * scale, self.y * scale, self.z * scale)
    }
}

impl std::ops::Add for Vec4D {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Vec4D::add(&self, other)
    }
}

impl std::ops::AddAssign for Vec4D {
    fn add_assign(&mut self, other: Self) {
        *self = Vec4D::add(self, other);
    }
}

impl std::ops::Sub for Vec4D {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Vec4D::sub(&self, other)
    }
}

impl std::ops::Div<f32> for Vec4D {
    type Output = Self;
    fn div(self, rhs: f32) -> Self {
        self.scale(1.0 / rhs)
    }
}
