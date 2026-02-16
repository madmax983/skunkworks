use macroquad::prelude::*;

#[derive(Debug, Clone)]
pub struct Body {
    pub pos: Vec2,
    pub vel: Vec2,
    pub mass: f32,
    pub radius: f32,
    pub color: Color,
    pub resources: u32,
}

impl Body {
    pub fn new(pos: Vec2, vel: Vec2, mass: f32, radius: f32, color: Color) -> Self {
        Self {
            pos,
            vel,
            mass,
            radius,
            color,
            resources: 0,
        }
    }
}

pub fn calculate_gravitational_force(bodies: &[Body], g: f32) -> Vec<Vec2> {
    let mut forces = vec![Vec2::ZERO; bodies.len()];

    for i in 0..bodies.len() {
        for j in 0..bodies.len() {
            if i == j {
                continue;
            }

            let diff = bodies[j].pos - bodies[i].pos;
            let dist_sq = diff.length_squared();
            let dist = dist_sq.sqrt();

            // F = G * m1 * m2 / r^2
            // F_vec = F * (diff / dist) = G * m1 * m2 * diff / r^3
            if dist > 0.0001 {
                // Avoid division by zero
                let force_mag = g * bodies[i].mass * bodies[j].mass / dist_sq;
                let force_vec = diff.normalize() * force_mag;
                forces[i] += force_vec;
            }
        }
    }
    forces
}

pub fn symplectic_euler(bodies: &mut [Body], dt: f32, g: f32) {
    // 1. Calculate forces based on current positions
    let forces = calculate_gravitational_force(bodies, g);

    // 2. Update velocities and positions
    for (i, body) in bodies.iter_mut().enumerate() {
        if body.mass > 0.0 {
            let accel = forces[i] / body.mass;
            body.vel += accel * dt;
        }
        body.pos += body.vel * dt;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gravity_attraction() {
        let mut bodies = vec![
            Body::new(vec2(0.0, 0.0), vec2(0.0, 0.0), 100.0, 10.0, WHITE),
            Body::new(vec2(10.0, 0.0), vec2(0.0, 0.0), 1.0, 1.0, RED),
        ];

        // Body 1 is at (10, 0). Force should be towards (0, 0).
        // F = G * 100 * 1 / 10^2 = G * 1.
        // Accel = F / 1 = G.
        // If G = 1.0, Accel = 1.0 towards left (-1.0, 0.0).

        let g = 1.0;
        let dt = 1.0;

        symplectic_euler(&mut bodies, dt, g);

        // Check velocity of body 1
        // v_new = v_old + a * dt = 0 + (-1.0, 0) * 1 = (-1.0, 0)
        assert!((bodies[1].vel.x - -1.0).abs() < 0.0001);

        // Check position of body 1
        // x_new = x_old + v_new * dt = 10 + (-1) * 1 = 9.0
        assert!((bodies[1].pos.x - 9.0).abs() < 0.0001);
    }
}
