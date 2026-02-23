use crate::platter::Platter;
use chimera_lang::prelude::*;
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
    pub magnetism: f32,
    pub vm: ChimeraVM,
    pub trail: Vec<Vec2>,
}

impl Body {
    pub fn new(x: f32, y: f32, mass: f32, radius: f32, color: Color, dna: Dna) -> Self {
        Self {
            pos: Vec2::new(x, y),
            vel: Vec2::ZERO,
            acc: Vec2::ZERO,
            mass,
            radius,
            color,
            magnetism: 0.5, // Start neutral
            vm: ChimeraVM::new(dna),
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
    pub platter: Platter,
}

impl Universe {
    pub fn new() -> Self {
        Self {
            bodies: Vec::new(),
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
        let mut next_magnetism = vec![0.0; len];

        // Constants
        let g_repulse = 5000.0;
        let center_pull = 0.5;
        let drag = 0.99;

        let mag_strength = 2000.0;
        let mag_write = 5.0;
        let decay_rate = 0.98;

        // 1. Physics: Repulsion and Attraction (Magnetism)
        for i in 0..len {
            // Self-center pull
            forces[i] += -self.bodies[i].pos * center_pull * self.bodies[i].mass;

            // Interactions
            for j in (i + 1)..len {
                let p1 = self.bodies[i].pos;
                let p2 = self.bodies[j].pos;
                let delta = p2 - p1;
                let dist_sq = delta.length_squared().max(25.0); // Minimal distance to avoid singularities
                let dir = delta.normalize_or_zero();

                // Repulsion (Pauli exclusion principle-ish)
                let repulse =
                    (g_repulse * self.bodies[i].mass.sqrt() * self.bodies[j].mass.sqrt()) / dist_sq;
                let f_repulse = -dir * repulse;

                forces[i] += f_repulse;
                forces[j] -= f_repulse;

                // Magnetic Attraction/Repulsion
                // Like poles repel, opposite attract?
                // Let's say Magnetism is 0.0 (North) to 1.0 (South). 0.5 is Neutral.
                // Force = (M1 - 0.5) * (M2 - 0.5) * Strength / dist_sq
                // If both > 0.5 (South), result is positive -> Repulsion?
                // If one > 0.5, one < 0.5 (South-North), result is negative -> Attraction?
                // Yes, that works.

                let m1 = self.bodies[i].magnetism - 0.5;
                let m2 = self.bodies[j].magnetism - 0.5;
                let mag_force_val = (m1 * m2 * mag_strength * 100.0) / dist_sq;
                let f_mag = dir * mag_force_val; // Positive = Repulsion (push away)

                forces[i] += f_mag;
                forces[j] -= f_mag;
            }
        }

        // 2. Sensing & VM Step
        for i in 0..len {
            let body = &mut self.bodies[i];

            // Collect Inputs
            // Input 1: Neighbor Count (within radius 20.0)
            let mut neighbor_count = 0;
            // Input 2: Local Magnetism Field Strength (from Platter)
            // Map pos to grid
            let gx = (body.pos.x + 100.0) as i32;
            let gy = (body.pos.y + 100.0) as i32;
            let local_mag = if gx >= 0
                && gx < self.platter.width as i32
                && gy >= 0
                && gy < self.platter.height as i32
            {
                self.platter.get_magnetism(gx as usize, gy as usize)
            } else {
                0.0
            };

            // We can't iterate bodies here efficiently due to borrow checker if we used nested loops
            // But we can just use the platter for "environment" sensing

            // Let's convert inputs to Integers for VM
            let input_mag = (local_mag * 100.0) as i64;
            let input_self = (body.magnetism * 100.0) as i64;

            // Push Inputs to Stack: [LocalMag, SelfMag]
            // We want the VM to decide new SelfMag.
            body.vm.stack.push(Value::Int(input_mag));
            body.vm.stack.push(Value::Int(input_self));

            // Run VM
            for _ in 0..50 {
                body.vm.step();
            }

            // Read Output (Top of stack)
            if let Some(Value::Int(val)) = body.vm.stack.pop() {
                // Clamp 0-100 -> 0.0-1.0
                let new_mag = (val.clamp(0, 100) as f32) / 100.0;
                next_magnetism[i] = new_mag;
            } else {
                next_magnetism[i] = body.magnetism; // No change
            }

            // Clear Stack and Reset for next tick
            body.vm.stack.clear();
            body.vm.energy = 1000;
            body.vm.ip = (0, 0);

            // Update Color based on Magnetism
            // 0.0 (Blue/North) -> 0.5 (Green/Neutral) -> 1.0 (Red/South)
            body.color = if body.magnetism < 0.4 {
                Color::Blue
            } else if body.magnetism > 0.6 {
                Color::Red
            } else {
                Color::Green
            };
        }

        // 3. Actuation (Apply Forces and Update Pos/Mag)
        for (i, body) in self.bodies.iter_mut().enumerate() {
            body.magnetism = body.magnetism * 0.9 + next_magnetism[i] * 0.1; // Smooth transition

            body.acc = forces[i] / body.mass;
            body.vel += body.acc * dt;
            body.vel *= drag;
            body.pos += body.vel * dt;

            // Trail
            if rand::random::<u8>() % 10 == 0 {
                body.trail.push(body.pos);
                if body.trail.len() > 20 {
                    body.trail.remove(0);
                }
            }

            // Write to Platter
            let gx = (body.pos.x + 100.0) as i32;
            let gy = (body.pos.y + 100.0) as i32;
            if gx >= 0
                && gx < self.platter.width as i32
                && gy >= 0
                && gy < self.platter.height as i32
            {
                // Write amount based on deviation from neutral
                let intensity = (body.magnetism - 0.5).abs() * mag_write * dt;
                self.platter.magnetize(gx as usize, gy as usize, intensity);
            }
        }

        // 4. Platter Decay
        self.platter.decay(decay_rate);
    }
}
