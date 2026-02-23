use crate::harvester::FunctionSignature;
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
    pub color: Color,
    pub trail: Vec<Vec2>,
    pub signature: Option<FunctionSignature>,
}

impl Body {
    pub fn new(x: f32, y: f32, mass: f32, radius: f32, color: Color) -> Self {
        Self {
            pos: Vec2::new(x, y),
            vel: Vec2::ZERO,
            acc: Vec2::ZERO,
            mass,
            radius,
            color,
            trail: Vec::with_capacity(50),
            signature: None,
        }
    }

    pub fn with_signature(mut self, signature: FunctionSignature) -> Self {
        self.signature = Some(signature);
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

    pub fn step(&mut self, dt: f32) {
        let len = self.bodies.len();
        let mut forces = vec![Vec2::ZERO; len];

        // Constants
        let g_repulse = 2000.0;
        let cladistic_strength = 3000.0;
        let center_pull = 0.5;
        let drag = 0.95;

        // Magnetism constants
        let mag_strength = 1500.0;
        let mag_write = 5.0; // How much magnetism to deposit per sec
        let decay_rate = 0.99; // Per step

        // 1. Repulsion & Cladistic Attraction
        for i in 0..len {
            for j in (i + 1)..len {
                let p1 = self.bodies[i].pos;
                let p2 = self.bodies[j].pos;
                let delta = p2 - p1;
                let dist_sq = delta.length_squared().max(10.0);
                let dir = delta.normalize_or_zero();

                // Repulsion
                let repulse_force =
                    (g_repulse * self.bodies[i].mass.sqrt() * self.bodies[j].mass.sqrt()) / dist_sq;
                let f_repulse = -dir * repulse_force;

                forces[i] += f_repulse;
                forces[j] -= f_repulse;

                // Cladistic Attraction
                if let (Some(sig_a), Some(sig_b)) =
                    (&self.bodies[i].signature, &self.bodies[j].signature)
                {
                    let similarity = calculate_similarity(sig_a, sig_b);
                    if similarity > 0.0 {
                        // Attraction is proportional to similarity and inverse square distance?
                        // Or just linear? Let's use inverse square but weaker falloff
                        let attract_force = (cladistic_strength * similarity) / dist_sq.sqrt();
                        let f_attract = dir * attract_force;
                        forces[i] += f_attract;
                        forces[j] -= f_attract;
                    }
                }
            }
        }

        // 2. Central Pull, Integration, and Magnetism
        for (i, body) in self.bodies.iter_mut().enumerate() {
            let f_center = -body.pos * center_pull * body.mass;
            forces[i] += f_center;

            // Magnetism Interaction
            // Map pos to grid. [-100, 100] -> [0, 200]
            let gx = (body.pos.x + 100.0) as i32;
            let gy = (body.pos.y + 100.0) as i32;

            if gx >= 0
                && gx < self.platter.width as i32
                && gy >= 0
                && gy < self.platter.height as i32
            {
                let x = gx as usize;
                let y = gy as usize;

                // Write to platter
                self.platter.magnetize(x, y, mag_write * dt);

                // Read from platter (Gradient)
                let left = if x > 0 {
                    self.platter.get_magnetism(x - 1, y)
                } else {
                    0.0
                };
                let right = if x < self.platter.width - 1 {
                    self.platter.get_magnetism(x + 1, y)
                } else {
                    0.0
                };
                let down = if y > 0 {
                    self.platter.get_magnetism(x, y - 1)
                } else {
                    0.0
                };
                let up = if y < self.platter.height - 1 {
                    self.platter.get_magnetism(x, y + 1)
                } else {
                    0.0
                };

                let grad_x = (right - left) * 0.5;
                let grad_y = (up - down) * 0.5;

                let mag_force = Vec2::new(grad_x, grad_y) * mag_strength;
                forces[i] += mag_force;
            }

            body.acc = forces[i] / body.mass;

            body.vel += body.acc * dt;
            body.vel *= drag;
            body.pos += body.vel * dt;

            // Keep within bounds roughly
             if body.pos.x.abs() > 100.0 { body.vel.x *= -0.5; body.pos.x = body.pos.x.signum() * 100.0; }
             if body.pos.y.abs() > 100.0 { body.vel.y *= -0.5; body.pos.y = body.pos.y.signum() * 100.0; }

            if rand::random::<u8>() % 10 == 0 {
                body.trail.push(body.pos);
                if body.trail.len() > 10 {
                    body.trail.remove(0);
                }
            }
        }

        // 3. Platter Decay
        self.platter.decay(decay_rate);
    }
}

fn calculate_similarity(a: &FunctionSignature, b: &FunctionSignature) -> f32 {
    let mut score = 0.0;

    // Return type match is a strong signal
    if a.output == b.output && !a.output.is_empty() {
        score += 1.0;
    }

    // Input count match
    if a.inputs.len() == b.inputs.len() {
        score += 0.5;
    }

    // Shared inputs
    // Quick and dirty intersection
    let mut shared = 0;
    for input_a in &a.inputs {
        if b.inputs.contains(input_a) {
            shared += 1;
        }
    }
    score += (shared as f32) * 0.2;

    // Name prefix similarity?
    // "test_" vs "test_"
    if a.name.split('_').next() == b.name.split('_').next() {
        score += 0.3;
    }

    score
}
