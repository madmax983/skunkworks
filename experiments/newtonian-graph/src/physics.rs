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
            Self { x: self.x / l, y: self.y / l }
        }
    }
}

impl std::ops::Add for Vec2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self { x: self.x + rhs.x, y: self.y + rhs.y }
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
        Self { x: self.x - rhs.x, y: self.y - rhs.y }
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
        Self { x: self.x * rhs, y: self.y * rhs }
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
        Self { x: self.x / rhs, y: self.y / rhs }
    }
}

impl std::ops::Neg for Vec2 {
    type Output = Self;
    fn neg(self) -> Self {
        Self { x: -self.x, y: -self.y }
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
        }
    }

    pub fn with_velocity(mut self, vx: f32, vy: f32) -> Self {
        self.vel = Vec2::new(vx, vy);
        self
    }
}

pub struct Universe {
    pub bodies: Vec<Body>,
    pub edges: Vec<(usize, usize)>, // Indices into bodies
}

impl Universe {
    pub fn new() -> Self {
        Self {
            bodies: Vec::new(),
            edges: Vec::new(),
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

        // Constants adjusted for TUI resolution (much lower density)
        // But internal physics can be high res.
        let g_edge = 10000.0;
        let g_repulse = 5000.0;
        let center_pull = 0.5;
        let drag = 0.999;

        // 1. Repulsion
        for i in 0..len {
            for j in (i + 1)..len {
                let p1 = self.bodies[i].pos;
                let p2 = self.bodies[j].pos;
                let delta = p2 - p1;
                let dist_sq = delta.length_squared().max(100.0);
                let dist = dist_sq.sqrt();

                let force = (g_repulse * self.bodies[i].mass.sqrt() * self.bodies[j].mass.sqrt()) / dist_sq;
                let dir = delta / dist;

                let f_vec = -dir * force;

                forces[i] += f_vec;
                forces[j] -= f_vec;
            }
        }

        // 2. Attraction
        for &(i, j) in &self.edges {
             let p1 = self.bodies[i].pos;
             let p2 = self.bodies[j].pos;
             let delta = p2 - p1;
             let dist_sq = delta.length_squared().max(100.0);
             let dist = dist_sq.sqrt();

             let force = (g_edge * self.bodies[i].mass * self.bodies[j].mass) / dist_sq;
             let dir = delta / dist;

             let f_vec = dir * force;
             forces[i] += f_vec;
             forces[j] -= f_vec;
        }

        // 3. Central Pull & Integration
        for (i, body) in self.bodies.iter_mut().enumerate() {
            let f_center = -body.pos * center_pull * body.mass; // Center pull scales with mass
            forces[i] += f_center;

            body.acc = forces[i] / body.mass;

            body.vel += body.acc * dt;
            body.vel *= drag;
            body.pos += body.vel * dt;

            if rand::random::<u8>() % 10 == 0 {
                body.trail.push(body.pos);
                if body.trail.len() > 20 { // Shorter trail for TUI
                    body.trail.remove(0);
                }
            }
        }
    }
}
