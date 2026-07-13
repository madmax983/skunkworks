use std::collections::VecDeque;

#[derive(Clone, Default)]
pub struct Vec2 { x: f32, y: f32 }
impl Vec2 {
    pub const ZERO: Vec2 = Vec2 { x: 0.0, y: 0.0 };
    pub fn length_squared(&self) -> f32 { self.x*self.x + self.y*self.y }
}
impl std::ops::Sub for Vec2 {
    type Output = Vec2;
    fn sub(self, rhs: Vec2) -> Vec2 { Vec2 { x: self.x - rhs.x, y: self.y - rhs.y } }
}
impl std::ops::AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Vec2) { self.x += rhs.x; self.y += rhs.y; }
}
impl std::ops::Mul<f32> for Vec2 {
    type Output = Vec2;
    fn mul(self, rhs: f32) -> Vec2 { Vec2 { x: self.x * rhs, y: self.y * rhs } }
}
impl std::ops::Div<f32> for Vec2 {
    type Output = Vec2;
    fn div(self, rhs: f32) -> Vec2 { Vec2 { x: self.x / rhs, y: self.y / rhs } }
}

pub const G: f32 = 5000.0;
const TRAIL_LENGTH: usize = 200;

#[derive(Clone)]
pub struct Body {
    pub pos: Vec2,
    pub vel: Vec2,
    pub acc: Vec2,
    pub mass: f32,
    pub radius: f32,
    pub trail: VecDeque<Vec2>,
}

impl Body {
    pub fn new(pos: Vec2, vel: Vec2, mass: f32, radius: f32) -> Self {
        Self {
            pos,
            vel,
            acc: Vec2::ZERO,
            mass,
            radius,
            trail: VecDeque::with_capacity(TRAIL_LENGTH),
        }
    }
}

pub fn update(bodies: &mut [Body], dt: f32) {
    for i in 1..bodies.len() { // Start at 1 to pin the central star
        let mut acc = Vec2::ZERO;
        let pos_i = bodies[i].pos;
        let radius_i = bodies[i].radius;
        for j in 0..bodies.len() {
            if i == j {
                continue;
            }

            let r = bodies[j].pos - pos_i;
            let dist_sq = r.length_squared();
            let dist = dist_sq.sqrt();

            // Softening
            if dist < radius_i + bodies[j].radius {
                continue;
            }

            let f = (G * bodies[j].mass) / dist_sq;
            let dir = r / dist;

            acc += dir * f;
        }
        bodies[i].acc = acc;
    }

    for (i, body) in bodies.iter_mut().enumerate() {
        if i == 0 {
            continue;
        } // Pin the central star

        body.vel += body.acc * dt;
        body.pos += body.vel * dt;

        if body.trail.len() >= TRAIL_LENGTH {
            body.trail.pop_front();
        }
        body.trail.push_back(body.pos.clone());
    }
}

fn main() {
    let mut bodies = vec![Body::new(Vec2::ZERO, Vec2::ZERO, 1000.0, 10.0)];
    update(&mut bodies, 0.1);
    println!("Works");
}
