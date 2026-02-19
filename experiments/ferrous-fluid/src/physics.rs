use rand::Rng;

#[derive(Clone, Copy, Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    pub fn length_squared(self) -> f32 {
        self.x * self.x + self.y * self.y
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
            }
        } else {
            Self::ZERO
        }
    }
}
impl std::ops::Add for Vec2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl std::ops::Sub for Vec2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl std::ops::Mul<f32> for Vec2 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

pub struct Particle {
    pub pos: Vec2,
    pub vel: Vec2,
    pub acc: Vec2,
}

impl Particle {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            pos: Vec2::new(x, y),
            vel: Vec2::ZERO,
            acc: Vec2::ZERO,
        }
    }
}

pub struct Magnet {
    pub pos: Vec2,
    pub strength: f32,
    pub polarity: bool, // true = North (Pull), false = South (Push/Complex)
}

pub struct Universe {
    pub particles: Vec<Particle>,
    pub magnets: Vec<Magnet>,
    pub width: f32,
    pub height: f32,
    grid: Vec<f32>, // Density grid
    grid_w: usize,
    grid_h: usize,
}

impl Universe {
    pub fn new(width: f32, height: f32) -> Self {
        let mut particles = Vec::new();
        let mut rng = rand::thread_rng();

        // Spawn particles
        for _ in 0..600 {
            particles.push(Particle::new(
                rng.gen_range(width * 0.2..width * 0.8),
                rng.gen_range(height * 0.1..height * 0.5),
            ));
        }

        let grid_w = width as usize + 1;
        let grid_h = height as usize + 1;

        Self {
            particles,
            magnets: Vec::new(),
            width,
            height,
            grid: vec![0.0; grid_w * grid_h],
            grid_w,
            grid_h,
        }
    }

    pub fn add_magnet(&mut self, x: f32, y: f32, polarity: bool) {
        self.magnets.push(Magnet {
            pos: Vec2::new(x, y),
            strength: 2000.0,
            polarity,
        });
    }

    pub fn update(&mut self, dt: f32) {
        let gravity = Vec2::new(0.0, -20.0);
        let damping = 0.96;

        // 1. Clear Grid
        self.grid.fill(0.0);

        // 2. Populate Grid (Density)
        for p in &self.particles {
            let gx = p.pos.x.round() as isize;
            let gy = p.pos.y.round() as isize;
            if gx >= 0 && gx < self.grid_w as isize && gy >= 0 && gy < self.grid_h as isize {
                self.grid[gy as usize * self.grid_w + gx as usize] += 1.0;
                // Smear to neighbors for smoother gradients?
            }
        }

        // 3. Update Particles
        for i in 0..self.particles.len() {
            let mut force = gravity;

            // Magnetism
            for mag in &self.magnets {
                let delta = mag.pos - self.particles[i].pos;
                let dist_sq = delta.length_squared();

                if dist_sq > 1.0 {
                    let dir = delta.normalize_or_zero();
                    let mag_force = (mag.strength / dist_sq).min(200.0);

                    if mag.polarity {
                        force = force + dir * mag_force; // Attract
                    } else {
                        force = force + dir * -mag_force; // Repel
                    }
                }
            }

            // Fluid Pressure (from Grid)
            let p_pos = self.particles[i].pos;
            let gx = p_pos.x.round() as isize;
            let gy = p_pos.y.round() as isize;

            if gx > 0
                && gx < (self.grid_w as isize - 1)
                && gy > 0
                && gy < (self.grid_h as isize - 1)
            {
                let idx = gy as usize * self.grid_w + gx as usize;

                // Gradient
                let dx = self.grid[idx + 1] - self.grid[idx - 1];
                let dy = self.grid[idx + self.grid_w] - self.grid[idx - self.grid_w];

                let pressure_force = Vec2::new(-dx, -dy) * 50.0; // Push away from high density
                force = force + pressure_force;
            }

            self.particles[i].acc = force;
        }

        // Integrate
        for p in &mut self.particles {
            p.vel = p.vel + p.acc * dt;
            p.vel = p.vel * damping;
            p.pos = p.pos + p.vel * dt;

            // Boundaries
            if p.pos.y < 0.0 {
                p.pos.y = 0.0;
                p.vel.y = -p.vel.y * 0.6;
            }
            if p.pos.x < 0.0 {
                p.pos.x = 0.0;
                p.vel.x = -p.vel.x * 0.6;
            }
            if p.pos.x > self.width {
                p.pos.x = self.width;
                p.vel.x = -p.vel.x * 0.6;
            }
            if p.pos.y > self.height {
                p.pos.y = self.height;
                p.vel.y = -p.vel.y * 0.6;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_particle_update() {
        let mut u = Universe::new(100.0, 100.0);

        // Place particle high up
        u.particles[0].pos = Vec2::new(50.0, 90.0);
        u.particles[0].vel = Vec2::ZERO;

        let initial_y = u.particles[0].pos.y;

        u.update(0.1);

        let p = &u.particles[0];
        // Gravity (-20.0) should pull it down
        assert!(p.pos.y < initial_y);
    }
}
