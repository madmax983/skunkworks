use ratatui::style::Color;
use std::collections::VecDeque;

pub const G: f64 = 100.0; // Gravitational constant (arbitrary units for visual pleasing)
const TRAIL_LENGTH: usize = 50;

#[derive(Clone, Debug)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

impl Vector2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    pub fn distance_sq(&self, other: &Vector2) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        dx * dx + dy * dy
    }

    pub fn distance(&self, other: &Vector2) -> f64 {
        self.distance_sq(other).sqrt()
    }
}

#[derive(Clone, Debug)]
pub struct Body {
    pub pos: Vector2,
    pub vel: Vector2,
    pub mass: f64,
    pub radius: f64,
    pub color: Color,
    pub trail: VecDeque<(f64, f64)>,
    pub name: String,
    // For audio events
    pub last_angle: f64,
}

impl Body {
    pub fn new(x: f64, y: f64, mass: f64, radius: f64, color: Color, name: String) -> Self {
        Self {
            pos: Vector2::new(x, y),
            vel: Vector2::zero(),
            mass,
            radius,
            color,
            trail: VecDeque::with_capacity(TRAIL_LENGTH),
            name,
            last_angle: y.atan2(x),
        }
    }

    pub fn with_velocity(mut self, vx: f64, vy: f64) -> Self {
        self.vel = Vector2::new(vx, vy);
        self
    }
}

pub struct Universe {
    pub bodies: Vec<Body>,
    pub events: Vec<AudioEvent>,
}

#[derive(Debug, Clone)]
pub enum AudioEvent {
    OrbitComplete { body_index: usize, radius: f64 },
    Conjunction { body1: usize, body2: usize },
}

impl Universe {
    pub fn new() -> Self {
        Self {
            bodies: Vec::new(),
            events: Vec::new(),
        }
    }

    pub fn add_body(&mut self, body: Body) {
        self.bodies.push(body);
    }

    pub fn tick(&mut self, dt: f64) {
        self.events.clear();
        let n = self.bodies.len();

        // Calculate forces/accelerations
        // We use Symplectic Euler:
        // v(t+1) = v(t) + a(r(t)) * dt
        // r(t+1) = r(t) + v(t+1) * dt

        // We need to compute acceleration for each body
        // a_i = sum( G * m_j * (r_j - r_i) / |r_j - r_i|^3 )

        let mut accelerations = vec![Vector2::zero(); n];

        for i in 0..n {
            for j in 0..n {
                if i == j {
                    continue;
                }

                let dx = self.bodies[j].pos.x - self.bodies[i].pos.x;
                let dy = self.bodies[j].pos.y - self.bodies[i].pos.y;
                let dist_sq = dx*dx + dy*dy;
                // Softening parameter to prevent explosion at r=0
                let softening = 5.0;
                let dist_cubed = (dist_sq + softening).powf(1.5);

                let f = G * self.bodies[j].mass / dist_cubed;

                accelerations[i].x += f * dx;
                accelerations[i].y += f * dy;
            }
        }

        // Update positions and velocities
        for i in 0..n {
            // Update Velocity
            self.bodies[i].vel.x += accelerations[i].x * dt;
            self.bodies[i].vel.y += accelerations[i].y * dt;

            // Update Position
            self.bodies[i].pos.x += self.bodies[i].vel.x * dt;
            self.bodies[i].pos.y += self.bodies[i].vel.y * dt;

            // Update Trail
            if self.bodies[i].trail.len() >= TRAIL_LENGTH {
                self.bodies[i].trail.pop_front();
            }
            let px = self.bodies[i].pos.x;
            let py = self.bodies[i].pos.y;
            self.bodies[i].trail.push_back((px, py));

            // Check for orbit completion (crossing positive X axis)
            // Current angle
            let current_angle = py.atan2(px);

            if self.bodies[i].trail.len() >= 2 {
                let (_prev_x, prev_y) = self.bodies[i].trail[self.bodies[i].trail.len() - 2];
                let (curr_x, curr_y) = (px, py);

                if prev_y < 0.0 && curr_y >= 0.0 && curr_x > 0.0 {
                    // Orbit complete!
                    let radius = (curr_x*curr_x + curr_y*curr_y).sqrt();
                    self.events.push(AudioEvent::OrbitComplete { body_index: i, radius });
                }
            }

            self.bodies[i].last_angle = current_angle;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_orbit_stability() {
        // Earth-Sun system approximation
        // Sun at 0,0, Mass 1000
        // Earth at 100,0, Velocity needs to be sqrt(G*M/R) for circular orbit
        // V = sqrt(100 * 1000 / 100) = sqrt(1000) approx 31.62

        let mut universe = Universe::new();
        universe.add_body(Body::new(0.0, 0.0, 1000.0, 10.0, Color::Yellow, "Sun".to_string()));

        let v_circ = (100.0f64 * 1000.0 / 100.0).sqrt();
        universe.add_body(Body::new(100.0, 0.0, 10.0, 2.0, Color::Blue, "Earth".to_string())
            .with_velocity(0.0, v_circ)); // Perpendicular velocity

        // Simulate for a bit
        let dt = 0.1;
        for _ in 0..100 {
            universe.tick(dt);
        }

        let earth = &universe.bodies[1];
        let r = earth.pos.distance(&Vector2::zero());

        // Should remain roughly 100.0
        assert!((r - 100.0).abs() < 5.0, "Orbit drifted too much: radius {}", r);
    }
}
