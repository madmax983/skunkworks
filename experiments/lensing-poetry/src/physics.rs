use macroquad::prelude::*;

pub const G: f32 = 1.0;
const SOFTENING: f32 = 0.0001;

/// ⚡ Bolt Optimization:
/// Moving the `acc` buffer onto the struct itself allows us to
/// avoid a per-frame `vec![Vec2::ZERO; n]` allocation in the hot path.
/// This zero-cost abstraction eliminates O(N) heap allocations.
#[derive(Clone, Copy, Debug)]
pub struct Body {
    pub pos: Vec2,
    pub vel: Vec2,
    pub acc: Vec2,
    pub mass: f32,
    pub radius: f32,
    pub color: Color,
}

pub fn integrate(bodies: &mut [Body], dt: f32) {
    let n = bodies.len();

    // Compute forces
    for i in 0..n {
        let mut acc = Vec2::ZERO;
        let pos_i = bodies[i].pos;

        for (j, body_j) in bodies.iter().enumerate() {
            if i == j {
                continue;
            }

            let r_vec = body_j.pos - pos_i;
            let dist_sq = r_vec.length_squared();

            let dist_soft = (dist_sq + SOFTENING * SOFTENING).sqrt();
            let dist_cubed = dist_soft * dist_soft * dist_soft;

            let f = G * body_j.mass / dist_cubed;
            acc += r_vec * f;
        }
        bodies[i].acc = acc;
    }

    // Update (Symplectic Euler)
    for body in bodies.iter_mut() {
        body.vel += body.acc * dt;
        body.pos += body.vel * dt;
    }
}
