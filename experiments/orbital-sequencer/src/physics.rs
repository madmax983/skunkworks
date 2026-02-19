use macroquad::prelude::*;

#[derive(Clone, Debug)]
pub struct Body {
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub mass: f32,
    pub radius: f32,
    pub color: Color,
    pub id: usize,
}

impl Body {
    pub fn new(id: usize, position: Vec2, velocity: Vec2, mass: f32, radius: f32, color: Color) -> Self {
        Self {
            id,
            position,
            velocity,
            acceleration: Vec2::ZERO,
            mass,
            radius,
            color,
        }
    }
}

pub struct Universe {
    pub bodies: Vec<Body>,
    pub g_const: f32,
    pub use_newtonian: bool, // If false, use Keplerian (fixed central body, no inter-planet gravity)
}

impl Universe {
    pub fn new() -> Self {
        Self {
            bodies: Vec::new(),
            g_const: 1.0, // Arbitrary units
            use_newtonian: false,
        }
    }

    pub fn add_body(&mut self, body: Body) {
        self.bodies.push(body);
    }

    pub fn step(&mut self, dt: f32) {
        // Velocity Verlet Integration

        // 1. First half-kick for velocity
        // v(t + dt/2) = v(t) + 0.5 * a(t) * dt
        for body in &mut self.bodies {
            body.velocity += 0.5 * body.acceleration * dt;
        }

        // 2. Full step for position
        // r(t + dt) = r(t) + v(t + dt/2) * dt
        for body in &mut self.bodies {
            body.position += body.velocity * dt;
        }

        // 3. Update forces (acceleration) at new position
        let new_accelerations = self.calculate_accelerations();

        // 4. Second half-kick for velocity
        // v(t + dt) = v(t + dt/2) + 0.5 * a(t + dt) * dt
        for (i, body) in self.bodies.iter_mut().enumerate() {
            body.acceleration = new_accelerations[i];
            body.velocity += 0.5 * body.acceleration * dt;
        }
    }

    fn calculate_accelerations(&self) -> Vec<Vec2> {
        let mut accelerations = vec![Vec2::ZERO; self.bodies.len()];

        for i in 0..self.bodies.len() {
            let mut acc = Vec2::ZERO;

            if self.use_newtonian {
                // Full N-body gravity
                for j in 0..self.bodies.len() {
                    if i == j { continue; }
                    let other = &self.bodies[j];
                    let diff = other.position - self.bodies[i].position;
                    let dist_sq = diff.length_squared();
                    if dist_sq < 0.0001 { continue; } // Softening/Avoiding singularity
                    let dist = dist_sq.sqrt();
                    let force_mag = self.g_const * other.mass / dist_sq;
                    acc += diff / dist * force_mag;
                }
            } else {
                // Keplerian: Only affected by Body 0 (The Sun)
                // Assuming Body 0 is the sun and doesn't move due to gravity (fixed)
                // Actually, if Body 0 is massive, we can just let it move or fix it.
                // For "Keplerian Mode" usually we imply planets don't affect each other.
                // But they are affected by the sun.

                if i == 0 {
                    // Sun doesn't accelerate in simple mode (fixed at center or wherever it is)
                    // Or maybe it does if we want binary stars?
                    // Let's assume Body 0 is fixed anchor.
                    acc = Vec2::ZERO;
                } else {
                    let sun = &self.bodies[0];
                    let diff = sun.position - self.bodies[i].position;
                    let dist_sq = diff.length_squared();
                    if dist_sq > 0.0001 {
                        let dist = dist_sq.sqrt();
                        let force_mag = self.g_const * sun.mass / dist_sq;
                        acc += diff / dist * force_mag;
                    }
                }
            }
            accelerations[i] = acc;
        }
        accelerations
    }

    pub fn total_energy(&self) -> f32 {
        let mut kinetic = 0.0;
        let mut potential = 0.0;

        for (i, body) in self.bodies.iter().enumerate() {
            kinetic += 0.5 * body.mass * body.velocity.length_squared();

            if self.use_newtonian {
                for j in (i + 1)..self.bodies.len() {
                    let other = &self.bodies[j];
                    let dist = (body.position - other.position).length();
                    if dist > 0.0001 {
                        potential -= self.g_const * body.mass * other.mass / dist;
                    }
                }
            } else {
                if i != 0 {
                    let sun = &self.bodies[0];
                    let dist = (body.position - sun.position).length();
                    if dist > 0.0001 {
                        potential -= self.g_const * body.mass * sun.mass / dist;
                    }
                }
            }
        }
        kinetic + potential
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use macroquad::prelude::RED;

    #[test]
    fn test_stable_orbit() {
        let mut universe = Universe::new();
        universe.use_newtonian = true; // Test full physics
        universe.g_const = 1.0;

        // Sun
        universe.add_body(Body::new(0, Vec2::ZERO, Vec2::ZERO, 1000.0, 10.0, RED));

        // Earth
        // v = sqrt(GM/r) for circular orbit
        // r = 100, M = 1000, G = 1 => v = sqrt(1000/100) = sqrt(10) ≈ 3.162
        let r = 100.0;
        let v = (universe.g_const * 1000.0 / r).sqrt();

        universe.add_body(Body::new(1, Vec2::new(r, 0.0), Vec2::new(0.0, v), 1.0, 5.0, RED));

        // Initial Energy
        let initial_energy = universe.total_energy();

        // Run for a while
        let dt = 0.1;
        for _ in 0..1000 {
            universe.step(dt);
        }

        // Check if orbit is roughly maintained (position shouldn't drift to infinity or 0)
        let final_pos = universe.bodies[1].position;
        let dist = final_pos.length();

        // Energy should be conserved (Velocity Verlet is symplectic)
        let final_energy = universe.total_energy();

        println!("Initial Energy: {}, Final Energy: {}", initial_energy, final_energy);
        println!("Initial Radius: {}, Final Radius: {}", r, dist);

        // Allow small error
        assert!((initial_energy - final_energy).abs() < 0.1);
        assert!((dist - r).abs() < 5.0); // Should stay close to circular path
    }
}
