use macroquad::prelude::*;

pub const G: f32 = 1.0;
const SOFTENING: f32 = 0.0001;

#[derive(Clone, Copy, Debug)]
pub struct Body {
    pub pos: Vec2,
    pub vel: Vec2,
    pub mass: f32,
    pub radius: f32,
    pub color: Color,
}

pub fn integrate(bodies: &mut [Body], dt: f32) {
    let n = bodies.len();
    let mut accelerations = vec![Vec2::ZERO; n];

    // Compute forces
    for i in 0..n {
        for j in 0..n {
            if i == j { continue; }

            let r_vec = bodies[j].pos - bodies[i].pos;
            let dist_sq = r_vec.length_squared();
            // let dist = dist_sq.sqrt();

            // F = G * m1 * m2 / (r^2 + eps)
            // a1 = F / m1 = G * m2 / (r^2 + eps)
            // Vector: a1 = (G * m2 / (r^2 + eps)) * (r_vec / r)
            // But usually softening is applied as (r^2 + eps^2)^1.5 in denominator

            let dist_soft = (dist_sq + SOFTENING * SOFTENING).sqrt();
            let dist_cubed = dist_soft * dist_soft * dist_soft;

            let f = G * bodies[j].mass / dist_cubed;
            accelerations[i] += r_vec * f;
        }
    }

    // Update (Symplectic Euler)
    for i in 0..n {
        bodies[i].vel += accelerations[i] * dt;
        bodies[i].pos += bodies[i].vel * dt;
    }
}
