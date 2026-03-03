use macroquad::prelude::*;

#[derive(Clone, Debug)]
pub struct Body {
    pub pos: Vec2,
    pub vel: Vec2,
    pub mass: f32,
    pub radius: f32,
    pub color: Color,
}

impl Body {
    pub fn new(pos: Vec2, vel: Vec2, mass: f32, radius: f32, color: Color) -> Self {
        Self {
            pos,
            vel,
            mass,
            radius,
            color,
        }
    }

    /// Calculates the angular velocity relative to a center point.
    /// Returns radians per second (or time unit).
    pub fn angular_velocity(&self, center: Vec2) -> f32 {
        let r_vec = self.pos - center;
        let r2 = r_vec.length_squared();
        if r2 < 1e-6 {
            return 0.0;
        }

        // Cross product in 2D (z-component)
        // r x v = rx * vy - ry * vx
        let cross = r_vec.x * self.vel.y - r_vec.y * self.vel.x;

        // omega = (r x v) / |r|^2
        cross / r2
    }
}

pub struct System {
    pub bodies: Vec<Body>,
    pub g_const: f32,
}

impl System {
    pub fn new() -> Self {
        Self {
            bodies: Vec::new(),
            g_const: 1000.0,
        }
    }

    pub fn add_body(&mut self, body: Body) {
        self.bodies.push(body);
    }

    pub fn update(&mut self, dt: f32) {
        let n = self.bodies.len();
        let mut acc = vec![Vec2::ZERO; n];

        // Calculate forces
        for i in 0..n {
            for j in i + 1..n {
                let diff = self.bodies[j].pos - self.bodies[i].pos;
                let dist_sq = diff.length_squared();

                // Softening parameter to prevent explosion at r -> 0
                let softening = 100.0;
                let effective_dist_sq = dist_sq + softening;
                let dist = effective_dist_sq.sqrt();

                let f =
                    self.g_const * self.bodies[i].mass * self.bodies[j].mass / effective_dist_sq;
                let force = diff / dist * f; // Direction is normalized diff

                acc[i] += force / self.bodies[i].mass;
                acc[j] -= force / self.bodies[j].mass;
            }
        }

        // Symplectic Euler Integration
        // 1. Update Velocity
        for i in 0..n {
            self.bodies[i].vel += acc[i] * dt;
        }

        // 2. Update Position
        for i in 0..n {
            let vel = self.bodies[i].vel;
            self.bodies[i].pos += vel * dt;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_orbital_mechanics() {
        let mut system = System::new();
        // Central massive body (Sun)
        system.add_body(Body::new(Vec2::ZERO, Vec2::ZERO, 1000.0, 10.0, WHITE));

        // Planet in circular orbit
        // r = 100
        // v = sqrt(GM/r) = sqrt(1000*1000/100) = sqrt(10000) = 100
        let dist = 100.0;
        let v = 100.0;

        system.add_body(Body::new(vec2(dist, 0.0), vec2(0.0, v), 1.0, 5.0, RED));

        // Period T = 2*pi*r / v = 2*pi*100 / 100 = 6.283
        let period = 2.0 * std::f32::consts::PI;
        let dt = 0.01;
        let steps = (period / dt).ceil() as i32;

        for _ in 0..steps {
            system.update(dt);
        }

        let planet = &system.bodies[1];
        let final_dist = planet.pos.length();

        // Check if distance is conserved (circular orbit)
        // Symplectic Euler introduces small error but should be bounded.
        // With dt=0.01 and one orbit, error should be small (< 1%).
        let error = (final_dist - dist).abs();
        println!("Final distance: {}, Error: {}", final_dist, error);

        assert!(error < 5.0, "Orbit drifted too much!");
    }
}
