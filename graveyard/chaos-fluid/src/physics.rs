use ferrous_core::Platter;
use locus::Vec2;
use rand::Rng;

pub const SUB_STEPS: usize = 20;

#[derive(Clone)]
pub struct Node {
    pub pos: Vec2,
    pub prev_pos: Vec2,
    pub mass: f64,
    pub fixed: bool,
    #[allow(dead_code)]
    pub name: String,
}

#[derive(Clone)]
pub struct Link {
    pub a: usize,
    pub b: usize,
    pub length: f64,
}

#[derive(Clone)]
pub struct PendulumSystem {
    pub nodes: Vec<Node>,
    pub links: Vec<Link>,
    pub gravity: Vec2,
    pub friction: f64,
}

impl PendulumSystem {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            links: Vec::new(),
            // Lineage (chaos-pendulum): Uses Y-up gravity for the physical simulation.
            gravity: Vec2::new(0.0, -9.81),
            friction: 1.0,
        }
    }

    pub fn add_node(&mut self, pos: Vec2, mass: f64, fixed: bool, name: String) -> usize {
        self.nodes.push(Node {
            pos,
            prev_pos: pos,
            mass,
            fixed,
            name,
        });
        self.nodes.len() - 1
    }

    pub fn add_link(&mut self, a: usize, b: usize, length: f64) {
        self.links.push(Link { a, b, length });
    }

    pub fn step(&mut self, dt: f64) {
        let dt = dt / SUB_STEPS as f64;

        for _ in 0..SUB_STEPS {
            self.verlet(dt);
            self.solve_constraints();
        }
    }

    fn verlet(&mut self, dt: f64) {
        for node in &mut self.nodes {
            if node.fixed {
                continue;
            }
            let velocity = node.pos - node.prev_pos;
            let velocity = velocity * self.friction;

            node.prev_pos = node.pos;
            node.pos += velocity + self.gravity * (dt * dt);
        }
    }

    fn solve_constraints(&mut self) {
        for _ in 0..5 {
            for link in &self.links {
                let pos_a = self.nodes[link.a].pos;
                let pos_b = self.nodes[link.b].pos;

                if self.nodes[link.a].fixed && self.nodes[link.b].fixed {
                    continue;
                }

                let delta = pos_b - pos_a;
                let dist = delta.magnitude();
                if dist < 0.000001 {
                    continue;
                }

                let diff = (dist - link.length) / dist;

                let mass_a = self.nodes[link.a].mass;
                let mass_b = self.nodes[link.b].mass;

                let inv_mass_a = if self.nodes[link.a].fixed {
                    0.0
                } else {
                    1.0 / mass_a
                };
                let inv_mass_b = if self.nodes[link.b].fixed {
                    0.0
                } else {
                    1.0 / mass_b
                };

                let total_inv_mass = inv_mass_a + inv_mass_b;
                if total_inv_mass == 0.0 {
                    continue;
                }

                let correction = delta * diff;

                if !self.nodes[link.a].fixed {
                    self.nodes[link.a].pos += correction * (inv_mass_a / total_inv_mass);
                }
                if !self.nodes[link.b].fixed {
                    self.nodes[link.b].pos -= correction * (inv_mass_b / total_inv_mass);
                }
            }
        }
    }
}

pub struct Particle {
    pub pos: Vec2,
    pub vel: Vec2,
    pub acc: Vec2,
}

impl Particle {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            pos: Vec2::new(x, y),
            vel: Vec2::zero(),
            acc: Vec2::zero(),
        }
    }
}

// Lineage (ferrous-fluid): A dense fluid simulation using a `Platter` for fluid density and
// magnetic interactions.
pub struct Universe {
    pub particles: Vec<Particle>,
    pub width: f64,
    pub height: f64,
    pub platter: Platter,
}

impl Universe {
    pub fn new(width: f64, height: f64) -> Self {
        let mut particles = Vec::new();
        let mut rng = rand::thread_rng();

        // Spawn particles
        for _ in 0..1500 {
            particles.push(Particle::new(
                rng.gen_range(width * 0.1..width * 0.9),
                rng.gen_range(height * 0.1..height * 0.9),
            ));
        }

        let grid_w = width as usize + 1;
        let grid_h = height as usize + 1;

        Self {
            particles,
            width,
            height,
            platter: Platter::new(grid_w, grid_h),
        }
    }

    pub fn update(&mut self, dt: f64, pendulum: &PendulumSystem) {
        let gravity = Vec2::new(0.0, -20.0);
        let damping = 0.96;

        self.platter.clear();

        for p in &self.particles {
            let gx = p.pos.x.round() as usize;
            let gy = p.pos.y.round() as usize;
            self.platter.accumulate(gx, gy, 1.0);
        }

        for i in 0..self.particles.len() {
            let mut force = gravity;

            // Pendulum nodes act as moving magnets
            for node in &pendulum.nodes {
                // Ignore fixed nodes or give them less power, or give all nodes power.
                // Let's give all nodes power to act as magnetic poles. Fixed are N, moving are S.
                let mag_pos = Vec2::new(node.pos.x as f64, node.pos.y as f64);
                let polarity = node.fixed; // True for N, False for S
                let strength = 5000.0 * (node.mass as f64);

                let delta = mag_pos - self.particles[i].pos;
                let dist_sq = delta.magnitude_squared();

                if dist_sq > 1.0 {
                    let dir = delta.normalize();
                    let mag_force = (strength / dist_sq).min(300.0);

                    if polarity {
                        force = force + dir * mag_force; // Attract
                    } else {
                        force = force + dir * -mag_force; // Repel
                    }
                }
            }

            let p_pos = self.particles[i].pos;
            let gx = p_pos.x.round() as usize;
            let gy = p_pos.y.round() as usize;

            if gx > 0
                && gx < (self.platter.width() - 1)
                && gy > 0
                && gy < (self.platter.height() - 1)
            {
                let left = self.platter.get_magnetism(gx - 1, gy);
                let right = self.platter.get_magnetism(gx + 1, gy);
                let down = self.platter.get_magnetism(gx, gy - 1);
                let up = self.platter.get_magnetism(gx, gy + 1);

                let dx = right - left;
                let dy = up - down;

                let pressure_force = Vec2::new(-dx, -dy) * 50.0;
                force = force + pressure_force;
            }

            self.particles[i].acc = force;
        }

        for p in &mut self.particles {
            p.vel = p.vel + p.acc * dt;
            p.vel = p.vel * damping;
            p.pos = p.pos + p.vel * dt;

            if p.pos.y < 0.0 {
                p.pos.y = 0.0;
                p.vel.y = -p.vel.y * 0.6;
            }
            if p.pos.x < 0.0 {
                p.pos.x = 0.0;
                p.vel.x = -p.vel.x * 0.6;
            }
            if p.pos.x > self.width {
                p.pos.x = self.width;
                p.vel.x = -p.vel.x * 0.6;
            }
            if p.pos.y > self.height {
                p.pos.y = self.height;
                p.vel.y = -p.vel.y * 0.6;
            }
        }
    }
}
