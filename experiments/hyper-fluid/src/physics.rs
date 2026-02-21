use rand::Rng;

#[derive(Clone, Copy, Debug)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec4 {
    pub const ZERO: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        w: 0.0,
    };

    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    pub fn length_squared(self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }

    pub fn length(self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn normalize_or_zero(self) -> Self {
        let len = self.length();
        if len > 0.0 {
            Self {
                x: self.x / len,
                y: self.y / len,
                z: self.z / len,
                w: self.w / len,
            }
        } else {
            Self::ZERO
        }
    }

    pub fn scale_dim(&self, sx: f32, sy: f32, sz: f32, sw: f32) -> Self {
        Self {
            x: self.x * sx,
            y: self.y * sy,
            z: self.z * sz,
            w: self.w * sw,
        }
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
}

impl std::ops::Add for Vec4 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl std::ops::Sub for Vec4 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl std::ops::Mul<f32> for Vec4 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

pub struct Particle {
    pub pos: Vec4,
    pub vel: Vec4,
    pub acc: Vec4,
}

impl Particle {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self {
            pos: Vec4::new(x, y, z, w),
            vel: Vec4::ZERO,
            acc: Vec4::ZERO,
        }
    }
}

pub struct Universe {
    pub particles: Vec<Particle>,
    pub size: f32,
    pub density_grid: Vec<f32>,
    pub grid_res: usize,
}

impl Universe {
    pub fn new(size: f32, num_particles: usize) -> Self {
        let mut particles = Vec::with_capacity(num_particles);
        let mut rng = rand::thread_rng();

        for _ in 0..num_particles {
            particles.push(Particle::new(
                rng.gen_range(size * 0.3..size * 0.7),
                rng.gen_range(size * 0.3..size * 0.7),
                rng.gen_range(size * 0.3..size * 0.7),
                rng.gen_range(size * 0.3..size * 0.7),
            ));
        }

        let grid_res = 16;
        let density_grid = vec![0.0; grid_res * grid_res * grid_res * grid_res];

        Self {
            particles,
            size,
            density_grid,
            grid_res,
        }
    }

    fn get_grid_idx(&self, x: usize, y: usize, z: usize, w: usize) -> usize {
        x + self.grid_res * (y + self.grid_res * (z + self.grid_res * w))
    }

    pub fn update(&mut self, dt: f32, params: PhysicsParams) {
        // 1. Clear Grid
        self.density_grid.fill(0.0);

        // 2. Populate Density Grid
        // Map particle position to grid coordinate
        let cell_size = self.size / self.grid_res as f32;

        for p in &self.particles {
            let gx = (p.pos.x / cell_size).clamp(0.0, (self.grid_res - 1) as f32) as usize;
            let gy = (p.pos.y / cell_size).clamp(0.0, (self.grid_res - 1) as f32) as usize;
            let gz = (p.pos.z / cell_size).clamp(0.0, (self.grid_res - 1) as f32) as usize;
            let gw = (p.pos.w / cell_size).clamp(0.0, (self.grid_res - 1) as f32) as usize;

            let idx = self.get_grid_idx(gx, gy, gz, gw);
            self.density_grid[idx] += 1.0;
        }

        // 3. Update Particles
        for i in 0..self.particles.len() {
            let mut force = Vec4::new(0.0, -params.gravity_strength, 0.0, 0.0);

            // Thermal Agitation (CPU)
            let mut rng = rand::thread_rng();
            let noise = Vec4::new(
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
                rng.gen_range(-1.0..1.0),
            )
            .normalize_or_zero()
                * params.temperature;

            force = force + noise;

            // Pressure (Density Gradient)
            let p_pos = self.particles[i].pos;
            let gx = (p_pos.x / cell_size).clamp(1.0, (self.grid_res - 2) as f32) as usize;
            let gy = (p_pos.y / cell_size).clamp(1.0, (self.grid_res - 2) as f32) as usize;
            let gz = (p_pos.z / cell_size).clamp(1.0, (self.grid_res - 2) as f32) as usize;
            let gw = (p_pos.w / cell_size).clamp(1.0, (self.grid_res - 2) as f32) as usize;

            // Central Difference
            let dx = self.density_grid[self.get_grid_idx(gx + 1, gy, gz, gw)]
                - self.density_grid[self.get_grid_idx(gx - 1, gy, gz, gw)];
            let dy = self.density_grid[self.get_grid_idx(gx, gy + 1, gz, gw)]
                - self.density_grid[self.get_grid_idx(gx, gy - 1, gz, gw)];
            let dz = self.density_grid[self.get_grid_idx(gx, gy, gz + 1, gw)]
                - self.density_grid[self.get_grid_idx(gx, gy, gz - 1, gw)];
            let dw = self.density_grid[self.get_grid_idx(gx, gy, gz, gw + 1)]
                - self.density_grid[self.get_grid_idx(gx, gy, gz, gw - 1)];

            let pressure = Vec4::new(-dx, -dy, -dz, -dw) * 10.0;
            force = force + pressure;

            // Center Attraction (Keep them in bounds lightly)
            let center = self.size * 0.5;
            let to_center = Vec4::new(center, center, center, center) - p_pos;
            force = force + to_center * 0.5;

            self.particles[i].acc = force;
        }

        // Integration
        for p in &mut self.particles {
            p.vel = p.vel + p.acc * dt;
            p.vel = p.vel * params.viscosity; // Damping
            p.pos = p.pos + p.vel * dt;

            // Hard Boundaries
            let bounce = -0.5;
            if p.pos.x < 0.0 {
                p.pos.x = 0.0;
                p.vel.x *= bounce;
            }
            if p.pos.x > self.size {
                p.pos.x = self.size;
                p.vel.x *= bounce;
            }

            if p.pos.y < 0.0 {
                p.pos.y = 0.0;
                p.vel.y *= bounce;
            }
            if p.pos.y > self.size {
                p.pos.y = self.size;
                p.vel.y *= bounce;
            }

            if p.pos.z < 0.0 {
                p.pos.z = 0.0;
                p.vel.z *= bounce;
            }
            if p.pos.z > self.size {
                p.pos.z = self.size;
                p.vel.z *= bounce;
            }

            if p.pos.w < 0.0 {
                p.pos.w = 0.0;
                p.vel.w *= bounce;
            }
            if p.pos.w > self.size {
                p.pos.w = self.size;
                p.vel.w *= bounce;
            }
        }
    }
}

pub struct PhysicsParams {
    pub gravity_strength: f32,
    pub viscosity: f32,
    pub temperature: f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec4_math() {
        let v1 = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let v2 = Vec4::new(4.0, 3.0, 2.0, 1.0);
        let v3 = v1 + v2;
        assert_eq!(v3.x, 5.0);
        assert_eq!(v3.w, 5.0);

        let v4 = v1 * 2.0;
        assert_eq!(v4.x, 2.0);
        assert_eq!(v4.w, 8.0);

        let v5 = Vec4::new(3.0, 0.0, 0.0, 4.0);
        assert_eq!(v5.length(), 5.0);
    }
}
