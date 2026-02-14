use macroquad::prelude::*;
use ::rand::Rng;

#[derive(Clone, Debug)]
pub struct Body {
    pub pos: Vec2,
    pub vel: Vec2,
    pub mass: f32,
    pub radius: f32,
    pub color: Color,
    pub _id: u64,
}

pub const G: f32 = 1000.0; // Increased G for more dramatic effect in game coordinates

impl Body {
    pub fn get_tidal_force_magnitude(&self, singularity_pos: Vec2, singularity_mass: f32) -> f32 {
        let diff = singularity_pos - self.pos;
        let dist_sq = diff.length_squared();
        if dist_sq < 1.0 {
            return 0.0;
        }
        let dist = dist_sq.sqrt();
        let dist_cb = dist_sq * dist;

        // F_tidal = 2 * G * M * R / r^3
        (2.0 * G * singularity_mass * self.radius) / dist_cb
    }

    pub fn fracture(&self) -> (Body, Body) {
        let mut child1 = self.clone();
        let mut child2 = self.clone();

        child1.mass = self.mass / 2.0;
        child2.mass = self.mass / 2.0;

        // Area conservation: R = sqrt(Area/PI). New Area = Old Area / 2.
        // New R = sqrt(OldArea/2 / PI) = OldR / sqrt(2)
        child1.radius = self.radius / 1.414;
        child2.radius = self.radius / 1.414;

        // Random separation vector
        let mut rng = ::rand::thread_rng();
        let angle = rng.gen_range(0.0..std::f32::consts::PI * 2.0);
        let separation_dir = vec2(angle.cos(), angle.sin());

        let sep_dist = self.radius * 0.5;
        let sep_vel = 1.0;

        child1.pos += separation_dir * sep_dist;
        child2.pos -= separation_dir * sep_dist;

        child1.vel += separation_dir * sep_vel;
        child2.vel -= separation_dir * sep_vel;

        (child1, child2)
    }
}

pub fn integrate(bodies: &mut Vec<Body>, dt: f32, singularity_pos: Vec2, singularity_mass: f32) {
    for body in bodies.iter_mut() {
        // Calculate Gravity
        let diff = singularity_pos - body.pos;
        let dist_sq = diff.length_squared();

        // Avoid division by zero
        if dist_sq > 0.1 {
            let dist = dist_sq.sqrt();
            let force_mag = (G * singularity_mass * body.mass) / dist_sq;
            let force_dir = diff / dist;
            let force = force_dir * force_mag;

            // F = ma -> a = F/m
            let accel = force / body.mass;

            // Symplectic Euler: Update v, then pos
            body.vel += accel * dt;
        }

        body.pos += body.vel * dt;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integration_movement() {
        let mut bodies = vec![Body {
            pos: vec2(0.0, 0.0),
            vel: vec2(10.0, 0.0),
            mass: 1.0,
            radius: 5.0,
            color: WHITE,
            _id: 0,
        }];

        integrate(&mut bodies, 1.0, vec2(100.0, 100.0), 0.0); // No gravity

        // With dt=1.0 and vel=(10,0), pos should be (10,0)
        assert_eq!(bodies[0].pos, vec2(10.0, 0.0));
    }

    #[test]
    fn test_gravity_influence() {
        let mut bodies = vec![Body {
            pos: vec2(10.0, 0.0),
            vel: vec2(0.0, 0.0),
            mass: 1.0,
            radius: 5.0,
            color: WHITE,
            _id: 0,
        }];

        integrate(&mut bodies, 1.0, vec2(0.0, 0.0), 1000.0);

        assert!(bodies[0].vel.x < 0.0);
        assert_eq!(bodies[0].vel.y, 0.0);
    }

    #[test]
    fn test_tidal_magnitude() {
        let body_far = Body {
            pos: vec2(20.0, 0.0),
            vel: Vec2::ZERO,
            mass: 10.0,
            radius: 5.0,
            color: WHITE,
            _id: 1,
        };

        let body_near = Body {
            pos: vec2(10.0, 0.0),
            vel: Vec2::ZERO,
            mass: 10.0,
            radius: 5.0,
            color: WHITE,
            _id: 2,
        };

        let singularity_pos = Vec2::ZERO;
        let singularity_mass = 1000.0;

        let force_far = body_far.get_tidal_force_magnitude(singularity_pos, singularity_mass);
        let force_near = body_near.get_tidal_force_magnitude(singularity_pos, singularity_mass);

        // F ~ 1/r^3. So near force should be roughly 8x far force (since dist is 1/2).
        assert!(force_near > force_far);
        assert!(force_near > 0.0);
    }

    #[test]
    fn test_fracture_conservation() {
        let parent = Body {
            pos: vec2(100.0, 0.0),
            vel: vec2(0.0, 10.0),
            mass: 20.0,
            radius: 10.0,
            color: WHITE,
            _id: 1,
        };

        // Seed RNG for determinism in test? macroquad::rand doesn't allow easy seeding here without window context maybe.
        // We'll assume gen_range works.

        let (child1, child2) = parent.fracture();

        // Mass Conservation
        assert_eq!(child1.mass + child2.mass, parent.mass);

        // Momentum Conservation (approximate sum)
        let p_initial = parent.vel * parent.mass;
        let p_final = (child1.vel * child1.mass) + (child2.vel * child2.mass);

        // Allow floating point error
        assert!((p_initial.x - p_final.x).abs() < 0.01);
        assert!((p_initial.y - p_final.y).abs() < 0.01);

        // Children should be smaller
        assert!(child1.radius < parent.radius);
        assert!(child2.radius < parent.radius);

        // Children should have slightly different velocities/positions
        assert!(child1.vel != child2.vel || child1.pos != child2.pos);
    }
}
