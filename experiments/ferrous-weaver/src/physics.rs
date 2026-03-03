use crate::platter::Platter;
use ratatui::style::Color;

#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };

    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn length(self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn length_squared(self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    pub fn normalize_or_zero(self) -> Self {
        let l = self.length();
        if l == 0.0 {
            Self::ZERO
        } else {
            Self {
                x: self.x / l,
                y: self.y / l,
            }
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

impl std::ops::AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
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

impl std::ops::SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
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

impl std::ops::MulAssign<f32> for Vec2 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl std::ops::Div<f32> for Vec2 {
    type Output = Self;
    fn div(self, rhs: f32) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl std::ops::Neg for Vec2 {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Body {
    pub pos: Vec2,
    pub vel: Vec2,
    pub acc: Vec2,
    pub mass: f32,
    pub radius: f32,
    pub charge: f32, // New: Electric/Magnetic Charge
    pub color: Color,
    pub trail: Vec<Vec2>,
    pub fixed: bool,
}

impl Body {
    pub fn new(x: f32, y: f32, mass: f32, radius: f32, charge: f32, color: Color) -> Self {
        Self {
            pos: Vec2::new(x, y),
            vel: Vec2::ZERO,
            acc: Vec2::ZERO,
            mass,
            radius,
            charge,
            color,
            trail: Vec::with_capacity(50),
            fixed: false,
        }
    }

    pub fn fixed(mut self) -> Self {
        self.fixed = true;
        self
    }
}

pub struct Universe {
    pub bodies: Vec<Body>,
    pub edges: Vec<(usize, usize)>, // Indices into bodies
    pub platter: Platter,
}

impl Universe {
    pub fn new() -> Self {
        Self {
            bodies: Vec::new(),
            edges: Vec::new(),
            platter: Platter::new(200, 200), // Fixed grid size covering [-100, 100] with offset
        }
    }

    pub fn add_body(&mut self, body: Body) -> usize {
        let id = self.bodies.len();
        self.bodies.push(body);
        id
    }

    pub fn add_edge(&mut self, source: usize, target: usize) {
        self.edges.push((source, target));
    }

    pub fn step(&mut self, dt: f32) {
        let len = self.bodies.len();
        let mut forces = vec![Vec2::ZERO; len];

        // Constants
        let g_edge = 2000.0; // Stiffer springs
        let g_repulse = 5000.0; // Nodes repel each other
        let k_coulomb = 8000.0; // Magnetic/Electric constant
        let gravity = Vec2::new(0.0, -30.0); // Downwards gravity
        let drag = 0.98;

        // Magnetism constants (Platter)
        let mag_strength = 2000.0;
        let mag_write = 5.0; // How much magnetism to deposit per sec
        let decay_rate = 0.99; // Per step

        // 1. Repulsion and Coulomb Interaction
        for i in 0..len {
            for j in (i + 1)..len {
                let p1 = self.bodies[i].pos;
                let p2 = self.bodies[j].pos;
                let delta = p2 - p1;
                let dist_sq = delta.length_squared().max(10.0); // Allow closer approach

                // Repulsion (Physical volume)
                let repulse =
                    (g_repulse * self.bodies[i].mass.sqrt() * self.bodies[j].mass.sqrt()) / dist_sq;

                // Coulomb (Magnetic Charge)
                // Force = k * q1 * q2 / dist^2
                // Same signs repel (+), Opposite signs attract (-)
                let coulomb = (k_coulomb * self.bodies[i].charge * self.bodies[j].charge) / dist_sq;

                let force_mag = repulse + coulomb;
                let dir = delta.normalize_or_zero();

                // If force_mag > 0 (Repulsion), pushes i away from j (-dir)
                // If force_mag < 0 (Attraction), pulls i towards j (+dir)
                let f_vec = -dir * force_mag;

                forces[i] += f_vec;
                forces[j] -= f_vec;
            }
        }

        // 2. Attraction (Springs/Edges)
        for &(i, j) in &self.edges {
            let p1 = self.bodies[i].pos;
            let p2 = self.bodies[j].pos;
            let delta = p2 - p1;
            let dist_sq = delta.length_squared().max(10.0);

            let force = (g_edge * self.bodies[i].mass * self.bodies[j].mass) / dist_sq;

            let dir = delta.normalize_or_zero();
            let f_vec = dir * force;
            forces[i] += f_vec;
            forces[j] -= f_vec;
        }

        // 3. Gravity, Integration, and Platter Interaction
        for (i, body) in self.bodies.iter_mut().enumerate() {
            if body.fixed {
                continue;
            }

            forces[i] += gravity * body.mass;

            // Magnetism Interaction (Platter)
            // Only charged bodies interact with the platter? Let's say all do for now,
            // or scale by charge. Let's scale by abs(charge) + 0.1 so even neutral bodies feel a bit.
            let mag_factor = body.charge.abs() + 0.1;

            // Map pos to grid. [-100, 100] -> [0, 200]
            let gx = (body.pos.x + 100.0) as i32;
            let gy = (body.pos.y + 100.0) as i32;

            if gx >= 0
                && gx < self.platter.width() as i32
                && gy >= 0
                && gy < self.platter.height() as i32
            {
                let x = gx as usize;
                let y = gy as usize;

                // Write to platter (Deposit)
                // Write sign of charge? Platter is 0.0 to 1.0 usually (magnitude).
                // Let's assume platter stores "Magnetic Potential" (scalar field).
                self.platter.magnetize(x, y, mag_write * mag_factor * dt);

                // Read from platter (Gradient)
                let left = if x > 0 {
                    self.platter.get_magnetism(x - 1, y)
                } else {
                    0.0
                };
                let right = if x < self.platter.width() - 1 {
                    self.platter.get_magnetism(x + 1, y)
                } else {
                    0.0
                };
                let down = if y > 0 {
                    self.platter.get_magnetism(x, y - 1)
                } else {
                    0.0
                };
                let up = if y < self.platter.height() - 1 {
                    self.platter.get_magnetism(x, y + 1)
                } else {
                    0.0
                };

                let grad_x = (right - left) * 0.5;
                let grad_y = (up - down) * 0.5;

                let mag_force = Vec2::new(grad_x, grad_y) * mag_strength * mag_factor;
                forces[i] += mag_force;
            }

            body.acc = forces[i] / body.mass;

            body.vel += body.acc * dt;
            body.vel *= drag;
            body.pos += body.vel * dt;

            // Trail
            if rand::random::<u8>() % 20 == 0 {
                let p = body.pos;
                body.trail.push(p);
                if body.trail.len() > 10 {
                    body.trail.remove(0);
                }
            }
        }

        // 4. Platter Decay
        self.platter.decay(decay_rate);
    }
}
