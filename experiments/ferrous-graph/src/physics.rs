use ferrous_core::Platter;
use ratatui::style::Color;
pub use locus::Vec2;

#[derive(Debug, Clone)]
pub struct Body {
    pub pos: Vec2,
    pub vel: Vec2,
    pub acc: Vec2,
    pub mass: f64,
    pub radius: f64,
    pub color: Color,
    pub trail: Vec<Vec2>,
}

impl Body {
    pub fn new(x: f64, y: f64, mass: f64, radius: f64, color: Color) -> Self {
        Self {
            pos: Vec2::new(x, y),
            vel: Vec2::zero(),
            acc: Vec2::zero(),
            mass,
            radius,
            color,
            trail: Vec::with_capacity(50),
        }
    }

    pub fn with_velocity(mut self, vx: f64, vy: f64) -> Self {
        self.vel = Vec2::new(vx, vy);
        self
    }
}

pub struct Universe {
    pub bodies: Vec<Body>,
    pub edges: Vec<(usize, usize)>, // Indices into bodies
    pub platter: Platter,
}

impl Universe {
    pub fn new() -> Self {
        Self {
            bodies: Vec::new(),
            edges: Vec::new(),
            platter: Platter::new(200, 200), // Fixed grid size covering [-100, 100] with offset
        }
    }

    pub fn add_body(&mut self, body: Body) -> usize {
        let id = self.bodies.len();
        self.bodies.push(body);
        id
    }

    pub fn add_edge(&mut self, source: usize, target: usize) {
        self.edges.push((source, target));
    }

    pub fn step(&mut self, dt: f64) {
        let len = self.bodies.len();
        let mut forces = vec![Vec2::zero(); len];

        // Constants
        let g_edge = 10000.0;
        let g_repulse = 5000.0;
        let center_pull = 0.5;
        let drag = 0.999;

        // Magnetism constants
        let mag_strength = 2000.0;
        let mag_write = 5.0; // How much magnetism to deposit per sec
        let decay_rate = 0.99; // Per step

        // 1. Repulsion
        for i in 0..len {
            for j in (i + 1)..len {
                let p1 = self.bodies[i].pos;
                let p2 = self.bodies[j].pos;
                let delta = p2 - p1;
                let dist_sq = delta.magnitude_squared().max(100.0);

                let force =
                    (g_repulse * self.bodies[i].mass.sqrt() * self.bodies[j].mass.sqrt()) / dist_sq;
                let dir = delta.normalize();

                let f_vec = -dir * force;

                forces[i] += f_vec;
                forces[j] -= f_vec;
            }
        }

        // 2. Attraction
        for &(i, j) in &self.edges {
            let p1 = self.bodies[i].pos;
            let p2 = self.bodies[j].pos;
            let delta = p2 - p1;
            let dist_sq = delta.magnitude_squared().max(100.0);

            let force = (g_edge * self.bodies[i].mass * self.bodies[j].mass) / dist_sq;
            let dir = delta.normalize();

            let f_vec = dir * force;
            forces[i] += f_vec;
            forces[j] -= f_vec;
        }

        // 3. Central Pull, Integration, and Magnetism
        for (i, body) in self.bodies.iter_mut().enumerate() {
            let f_center = -body.pos * center_pull * body.mass;
            forces[i] += f_center;

            // Magnetism Interaction
            // Map pos to grid. [-100, 100] -> [0, 200]
            let gx = (body.pos.x + 100.0) as i32;
            let gy = (body.pos.y + 100.0) as i32;

            if gx >= 0
                && gx < self.platter.width as i32
                && gy >= 0
                && gy < self.platter.height as i32
            {
                let x = gx as usize;
                let y = gy as usize;

                // Write to platter
                self.platter.magnetize(x, y, mag_write * dt);

                // Read from platter (Gradient)
                let left = if x > 0 {
                    self.platter.get_magnetism(x - 1, y)
                } else {
                    0.0
                };
                let right = if x < self.platter.width - 1 {
                    self.platter.get_magnetism(x + 1, y)
                } else {
                    0.0
                };
                let down = if y > 0 {
                    self.platter.get_magnetism(x, y - 1)
                } else {
                    0.0
                }; // smaller y index
                let up = if y < self.platter.height - 1 {
                    self.platter.get_magnetism(x, y + 1)
                } else {
                    0.0
                }; // larger y index

                // Wait, if grid Y matches physics Y (0 is -100, 200 is +100), then:
                // y-1 is physically lower (more negative).
                // y+1 is physically higher (more positive).
                // Gradient Y = (Value at y+1 - Value at y-1) / 2

                let grad_x = (right - left) * 0.5;
                let grad_y = (up - down) * 0.5;

                let mag_force = Vec2::new(grad_x, grad_y) * mag_strength;
                forces[i] += mag_force;
            }

            body.acc = forces[i] / body.mass;

            body.vel += body.acc * dt;
            body.vel *= drag;
            body.pos += body.vel * dt;

            if rand::random::<u8>() % 10 == 0 {
                body.trail.push(body.pos);
                if body.trail.len() > 20 {
                    body.trail.remove(0);
                }
            }
        }

        // 4. Platter Decay
        self.platter.decay(decay_rate);
    }
}
