use glam::DVec2;

pub const G: f64 = 100.0;

#[derive(Debug, Clone)]
pub struct Body {
    pub pos: DVec2,
    pub vel: DVec2,
    pub force: DVec2,
    pub mass: f64,
    pub radius: f64,
    pub name: String,
    pub is_fixed: bool,
}

impl Body {
    pub fn new(pos: DVec2, vel: DVec2, mass: f64, name: String) -> Self {
        Self {
            pos,
            vel,
            force: DVec2::ZERO,
            mass,
            radius: mass.max(0.0).sqrt(), // Heuristic radius
            name,
            is_fixed: false,
        }
    }
}

pub struct System {
    pub bodies: Vec<Body>,
    first_step: bool,
}

impl Default for System {
    fn default() -> Self {
        Self::new()
    }
}

impl System {
    pub fn new() -> Self {
        Self {
            bodies: Vec::new(),
            first_step: true,
        }
    }

    pub fn add_body(&mut self, body: Body) {
        self.bodies.push(body);
        self.first_step = true; // Reset if we change the system
    }

    fn compute_forces(&mut self) {
        let n = self.bodies.len();
        // Reset forces
        for body in &mut self.bodies {
            body.force = DVec2::ZERO;
        }

        for i in 0..n {
            let (head, tail) = self.bodies.split_at_mut(i + 1);
            let body_i = &mut head[i];

            for body_j in tail {
                let delta = body_j.pos - body_i.pos;
                let dist_sq = delta.length_squared();

                // Avoid singularity and very close interactions
                if dist_sq < 0.1 {
                    continue;
                }

                let dist = dist_sq.sqrt();

                // Softening parameter (Plummer model style)
                let softening = 10.0;
                let force_mag = (G * body_i.mass * body_j.mass) / (dist_sq + softening);

                let force = delta / dist * force_mag;

                if force.is_finite() {
                    if !body_i.is_fixed {
                        body_i.force += force;
                    }
                    if !body_j.is_fixed {
                        body_j.force -= force;
                    }
                }
            }
        }
    }

    pub fn update(&mut self, dt: f64) {
        if self.first_step {
            self.compute_forces();
            self.first_step = false;
        }

        // Velocity Verlet
        // 1. First half-kick: v(t + dt/2) = v(t) + 0.5 * a(t) * dt
        for body in &mut self.bodies {
            if body.is_fixed || body.mass.abs() < 1e-6 {
                continue;
            }
            let accel = body.force / body.mass;
            if accel.is_finite() {
                body.vel += accel * dt * 0.5;
            }
        }

        // 2. Drift: r(t + dt) = r(t) + v(t + dt/2) * dt
        for body in &mut self.bodies {
            if body.is_fixed {
                continue;
            }
            if body.pos.is_finite() && body.vel.is_finite() {
                body.pos += body.vel * dt;
            }
        }

        // 3. Update forces: a(t + dt)
        self.compute_forces();

        // 4. Second half-kick: v(t + dt) = v(t + dt/2) + 0.5 * a(t + dt) * dt
        for body in &mut self.bodies {
            if body.is_fixed || body.mass.abs() < 1e-6 {
                continue;
            }
            let accel = body.force / body.mass;
            if accel.is_finite() {
                body.vel += accel * dt * 0.5;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_orbit_stability() {
        let mut system = System::new();

        let star_mass = 1000.0;
        let star = Body {
            pos: DVec2::ZERO,
            vel: DVec2::ZERO,
            force: DVec2::ZERO,
            mass: star_mass,
            radius: 10.0,
            name: "Star".to_string(),
            is_fixed: true,
        };
        system.add_body(star);

        let orbit_radius = 100.0;
        // v = sqrt(GM/r)
        // With softening in implementation, pure Kepler v might be slightly off, but should be close.
        // F = G M m / (r^2 + s). a = G M / (r^2 + s).
        // v^2/r = a => v = sqrt(r * G * M / (r^2 + s))
        let softening = 10.0;
        let effective_gravity_accel = G * star_mass / (orbit_radius * orbit_radius + softening);
        let orbit_speed = (effective_gravity_accel * orbit_radius).sqrt();

        let planet = Body::new(
            DVec2::new(orbit_radius, 0.0),
            DVec2::new(0.0, orbit_speed),
            10.0,
            "Planet".to_string(),
        );
        system.add_body(planet);

        let dt = 0.05;
        let steps = 2000;

        let initial_pos = system.bodies[1].pos;

        for _ in 0..steps {
            system.update(dt);
        }

        let final_pos = system.bodies[1].pos;
        let final_radius = final_pos.length();

        println!("Initial Radius: {}", orbit_radius);
        println!("Final Radius: {}", final_radius);
        println!("Final Pos: {:?}", final_pos);

        // Check that it moved
        assert!(
            final_pos.distance(initial_pos) > 1.0,
            "Planet did not move!"
        );

        // Check that it stayed in orbit (1% tolerance)
        let drift = (final_radius - orbit_radius).abs();
        assert!(
            drift < orbit_radius * 0.01,
            "Orbit drifted too much! Final radius: {}, Drift: {}",
            final_radius,
            drift
        );
    }
}

#[cfg(test)]
mod havoc_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_system_stability_random_bodies(
            bodies in prop::collection::vec(
                (
                    (any::<f64>(), any::<f64>()), // pos
                    any::<f64>(),                 // mass
                    (any::<f64>(), any::<f64>())  // vel
                ),
                1..10
            )
        ) {
            // Filter out NaNs from inputs, as we want to test if the system *produces* NaNs
            // from valid (even if extreme) inputs.
            prop_assume!(bodies.iter().all(|((px, py), m, (vx, vy))|
                !px.is_nan() && !py.is_nan() && !m.is_nan() && !vx.is_nan() && !vy.is_nan()
            ));
            let mut system = System::new();
            for (i, ((px, py), mass, (vx, vy))) in bodies.into_iter().enumerate() {
                let pos = DVec2::new(px, py);
                let vel = DVec2::new(vx, vy);

                system.add_body(Body::new(pos, vel, mass, format!("Body {}", i)));
            }

            // Run for a few steps
            for _ in 0..10 {
                system.update(0.1);
            }

            // Check for NaNs
            for body in &system.bodies {
                prop_assert!(!body.pos.x.is_nan(), "Pos X is NaN");
                prop_assert!(!body.pos.y.is_nan(), "Pos Y is NaN");
                prop_assert!(!body.vel.x.is_nan(), "Vel X is NaN");
                prop_assert!(!body.vel.y.is_nan(), "Vel Y is NaN");
            }
        }
    }
}
