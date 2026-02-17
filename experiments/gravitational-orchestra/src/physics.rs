use macroquad::prelude::*;

pub const G: f32 = 1000.0; // Gravitational constant for the simulation
pub const SOFTENING: f32 = 10.0; // To prevent singularities
pub const MAX_BODIES: usize = 8;

#[derive(Clone, Copy, Debug)]
pub struct Body {
    pub pos: Vec2,
    pub vel: Vec2,
    pub mass: f32,
    pub color: Color,
    pub radius: f32,
}

impl Body {
    pub fn new(pos: Vec2, vel: Vec2, mass: f32, color: Color) -> Self {
        Self {
            pos,
            vel,
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
        let _dist = dist_sq.sqrt();

        let f = (G * body.mass) / dist_sq;
        acc += delta.normalize() * f;
    }
    acc
}

pub fn verlet_step(bodies: &mut [Body], dt: f32) {
    // 1. Calculate acceleration for current positions
    let mut accelerations = Vec::with_capacity(bodies.len());
    for i in 0..bodies.len() {
        accelerations.push(calculate_acceleration(bodies, i));
    }

    // 2. Update positions: r(t+dt) = r(t) + v(t)dt + 0.5*a(t)dt^2
    for (i, body) in bodies.iter_mut().enumerate() {
        let a = accelerations[i];
        body.pos += body.vel * dt + a * 0.5 * dt * dt;
    }

    // 3. Calculate new accelerations for new positions
    let mut new_accelerations = Vec::with_capacity(bodies.len());
    for i in 0..bodies.len() {
        new_accelerations.push(calculate_acceleration(bodies, i));
    }

    // 4. Update velocities: v(t+dt) = v(t) + 0.5*(a(t) + a(t+dt))*dt
    for (i, body) in bodies.iter_mut().enumerate() {
        let a = accelerations[i];
        let new_a = new_accelerations[i];
        body.vel += (a + new_a) * 0.5 * dt;
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
