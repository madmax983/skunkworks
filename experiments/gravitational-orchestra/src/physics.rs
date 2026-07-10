use macroquad::prelude::*;

pub const G: f32 = 1000.0; // Gravitational constant for the simulation
pub const SOFTENING: f32 = 10.0; // To prevent singularities
pub const MAX_BODIES: usize = 8;

#[derive(Clone, Copy, Debug)]
pub struct Body {
    pub pos: Vec2,
    pub vel: Vec2,
    pub acc: Vec2,
    pub mass: f32,
    pub color: Color,
    pub radius: f32,
}

impl Body {
    pub fn new(pos: Vec2, vel: Vec2, mass: f32, color: Color) -> Self {
        Self {
            pos,
            vel,
            acc: Vec2::ZERO,
            mass,
            color,
            radius: (mass.sqrt() / 2.0).clamp(2.0, 50.0),
        }
    }
}

pub fn calculate_acceleration(bodies: &[Body], index: usize) -> Vec2 {
    let mut acc = Vec2::ZERO;
    let target = bodies[index];

    for (i, body) in bodies.iter().enumerate() {
        if i == index {
            continue;
        }

        let delta = body.pos - target.pos;
        let dist_sq = delta.length_squared() + SOFTENING * SOFTENING;
        let dist = dist_sq.sqrt();

        // ⚡ Bolt: Removed `delta.normalize()` which hides a redundant `sqrt`.
        // `delta / dist` accurately handles softening while avoiding a hidden `dist = 0` panic
        // and redundant calculations since we already computed `dist` above.
        let f = (G * body.mass) / dist_sq;
        acc += (delta / dist) * f;
    }
    acc
}

pub fn verlet_step(bodies: &mut [Body], dt: f32) {
    // 1. Calculate acceleration for current positions
    for i in 0..bodies.len() {
        bodies[i].acc = calculate_acceleration(bodies, i);
    }

    // 2. Update positions: r(t+dt) = r(t) + v(t)dt + 0.5*a(t)dt^2
    for body in bodies.iter_mut() {
        body.pos += body.vel * dt + body.acc * 0.5 * dt * dt;
    }

    // 3. Calculate new accelerations for new positions & update velocities
    // v(t+dt) = v(t) + 0.5*(a(t) + a(t+dt))*dt
    for i in 0..bodies.len() {
        let new_a = calculate_acceleration(bodies, i);
        let a = bodies[i].acc;
        bodies[i].vel += (a + new_a) * 0.5 * dt;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verlet_integration() {
        let mut bodies = vec![
            Body::new(Vec2::new(0.0, 0.0), Vec2::ZERO, 1000.0, WHITE),
            Body::new(Vec2::new(100.0, 0.0), Vec2::new(0.0, 10.0), 10.0, RED),
        ];

        // Run for a few steps
        for _ in 0..100 {
            verlet_step(&mut bodies, 0.016);
        }

        // The second body should have moved
        assert_ne!(bodies[1].pos, Vec2::new(100.0, 0.0));
        // The first body (heavy) should have moved slightly (conservation of momentum)
        assert_ne!(bodies[0].pos, Vec2::new(0.0, 0.0));
    }
}

#[test]
fn test_bolt_zero_alloc() {
    let b = Body::new(Vec2::ZERO, Vec2::ZERO, 10.0, WHITE);
    assert_eq!(b.acc, Vec2::ZERO);
}
