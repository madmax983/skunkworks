use crate::platter::Platter;
use crate::roman::Roman;
use ratatui::style::Color;
use std::collections::HashSet;
use num_traits::ToPrimitive;

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

    pub fn dot(self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y
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
    pub value: Roman,
}

impl Body {
    pub fn new(x: f32, y: f32, value: Roman, color: Color) -> Self {
        let mass = (value.value().to_u64().unwrap_or(1) as f32).max(1.0);
        let radius = mass.sqrt() * 0.8 + 2.0; // Ensure visible size
        Self {
            pos: Vec2::new(x, y),
            vel: Vec2::ZERO,
            acc: Vec2::ZERO,
            mass,
            radius,
            color,
            trail: Vec::with_capacity(50),
            value,
        }
    }

    pub fn with_velocity(mut self, vx: f32, vy: f32) -> Self {
        self.vel = Vec2::new(vx, vy);
        self
    }
}

pub struct Universe {
    pub bodies: Vec<Body>,
    pub edges: Vec<(usize, usize)>, // Indices into bodies (Legacy support, maybe unused)
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
        // Resolve collisions first (modify bodies list)
        self.resolve_collisions();

        let len = self.bodies.len();
        let mut forces = vec![Vec2::ZERO; len];

        // Constants
        let g_attract = 500.0;
        let center_pull = 0.2;
        let drag = 0.995;

        // Magnetism constants
        let mag_strength = 1000.0;
        let mag_write = 10.0;
        let decay_rate = 0.99;

        // 1. Repulsion (Gravity)
        for i in 0..len {
            for j in (i + 1)..len {
                let p1 = self.bodies[i].pos;
                let p2 = self.bodies[j].pos;
                let delta = p2 - p1;
                let dist_sq = delta.length_squared().max(25.0); // Avoid singularity

                // Standard gravity (Attraction) would be G * m1 * m2 / r^2
                // We want them to clump? Or repel?
                // Let's make them attract slightly like gravity.
                // But `ferrous-graph` had Repulsion.
                // For "Legion", let's have Attraction (Gravity).
                let force_mag = (g_attract * self.bodies[i].mass * self.bodies[j].mass) / dist_sq;
                let dir = delta.normalize_or_zero();

                let f_vec = dir * force_mag;
                forces[i] += f_vec;
                forces[j] -= f_vec;
            }
        }

        // 2. Central Pull, Integration, and Magnetism
        for (i, body) in self.bodies.iter_mut().enumerate() {
            // Keep them on screen
            let f_center = -body.pos * center_pull * body.mass;
            forces[i] += f_center;

            // Magnetism Interaction
            let gx = (body.pos.x + 100.0) as i32;
            let gy = (body.pos.y + 100.0) as i32;

            if gx >= 0 && gx < self.platter.width as i32 && gy >= 0 && gy < self.platter.height as i32 {
                let x = gx as usize;
                let y = gy as usize;

                // Write to platter (Trail)
                self.platter.magnetize(x, y, mag_write * dt);

                // Read from platter (Gradient) -> Follow trails
                let left = if x > 0 { self.platter.get_magnetism(x-1, y) } else { 0.0 };
                let right = if x < self.platter.width-1 { self.platter.get_magnetism(x+1, y) } else { 0.0 };
                let down = if y > 0 { self.platter.get_magnetism(x, y-1) } else { 0.0 };
                let up = if y < self.platter.height-1 { self.platter.get_magnetism(x, y+1) } else { 0.0 };

                let grad_x = (right - left) * 0.5;
                let grad_y = (up - down) * 0.5;

                let mag_force = Vec2::new(grad_x, grad_y) * mag_strength;
                forces[i] += mag_force;
            }

            body.acc = forces[i] / body.mass;

            body.vel += body.acc * dt;
            body.vel *= drag;
            body.pos += body.vel * dt;

            // Boundary hard limit
            if body.pos.x < -100.0 { body.pos.x = -100.0; body.vel.x *= -1.0; }
            if body.pos.x > 100.0 { body.pos.x = 100.0; body.vel.x *= -1.0; }
            if body.pos.y < -100.0 { body.pos.y = -100.0; body.vel.y *= -1.0; }
            if body.pos.y > 100.0 { body.pos.y = 100.0; body.vel.y *= -1.0; }

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

    fn resolve_collisions(&mut self) {
        let mut dead_indices = HashSet::new();
        let mut new_bodies = Vec::new();
        let len = self.bodies.len();

        for i in 0..len {
            if dead_indices.contains(&i) { continue; }
            for j in (i + 1)..len {
                if dead_indices.contains(&j) { continue; }

                let p1 = self.bodies[i].pos;
                let p2 = self.bodies[j].pos;
                let dist_sq = (p1 - p2).length_squared();
                let r_sum = self.bodies[i].radius + self.bodies[j].radius;

                if dist_sq < r_sum * r_sum {
                    // Collision
                    let m1 = self.bodies[i].mass;
                    let m2 = self.bodies[j].mass;
                    let v1 = self.bodies[i].value.value().to_u64().unwrap_or(0);
                    let v2 = self.bodies[j].value.value().to_u64().unwrap_or(0);
                    let sum_val = v1 + v2;

                    if sum_val <= 1000 {
                        // Merge
                        dead_indices.insert(i);
                        dead_indices.insert(j);

                        let new_pos = (p1 * m1 + p2 * m2) / (m1 + m2);
                        let new_vel = (self.bodies[i].vel * m1 + self.bodies[j].vel * m2) / (m1 + m2);
                        let new_roman = Roman::from_u64(sum_val);

                        // Color based on magnitude
                        let color = if sum_val >= 1000 { Color::Magenta }
                        else if sum_val >= 500 { Color::Red }
                        else if sum_val >= 100 { Color::Yellow }
                        else if sum_val >= 50 { Color::Green }
                        else if sum_val >= 10 { Color::Blue }
                        else { Color::Cyan };

                        let mut body = Body::new(new_pos.x, new_pos.y, new_roman, color);
                        body.vel = new_vel;
                        new_bodies.push(body);
                        break; // Stop processing i
                    } else {
                        // Bounce (Elastic)
                        let n = (p1 - p2).normalize_or_zero();
                        let v_rel = self.bodies[i].vel - self.bodies[j].vel;
                        let vel_along_normal = v_rel.dot(n);

                        if vel_along_normal > 0.0 { continue; } // Separating

                        let j_impulse = -(1.5) * vel_along_normal / (1.0/m1 + 1.0/m2);
                        let impulse = n * j_impulse;

                        self.bodies[i].vel += impulse / m1;
                        self.bodies[j].vel -= impulse / m2;

                        // Push apart to prevent sticking
                        let overlap = r_sum - dist_sq.sqrt();
                        if overlap > 0.0 {
                            let separation = n * (overlap * 0.5);
                            self.bodies[i].pos += separation;
                            self.bodies[j].pos -= separation;
                        }
                    }
                }
            }
        }

        // Apply removals
        if !dead_indices.is_empty() {
            let mut surviving_bodies = Vec::new();
            for (i, body) in self.bodies.iter().enumerate() {
                if !dead_indices.contains(&i) {
                    surviving_bodies.push(body.clone());
                }
            }
            self.bodies = surviving_bodies;
            self.bodies.extend(new_bodies);
        }
    }
}
