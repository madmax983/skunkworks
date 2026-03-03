use macroquad::prelude::Vec2;
use ferrous_core::Platter;

const SUB_STEPS: usize = 20;

#[derive(Clone)]
pub struct Node {
    pub pos: Vec2,
    pub prev_pos: Vec2,
    pub mass: f32,
    pub fixed: bool,
    #[allow(dead_code)]
    pub name: String,
    pub magnetism: f32, // 0.0 to 1.0. 0.5 is neutral.
}

#[derive(Clone)]
pub struct Link {
    pub a: usize,
    pub b: usize,
    pub length: f32,
}

#[derive(Clone)]
pub struct PendulumSystem {
    pub nodes: Vec<Node>,
    pub links: Vec<Link>,
    pub gravity: Vec2,
    pub friction: f32,
}

impl PendulumSystem {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            links: Vec::new(),
            gravity: Vec2::new(0.0, 9.81),
            friction: 1.0,
        }
    }

    pub fn add_node(&mut self, pos: Vec2, mass: f32, fixed: bool, name: String, magnetism: f32) -> usize {
        self.nodes.push(Node {
            pos,
            prev_pos: pos,
            mass,
            fixed,
            name,
            magnetism,
        });
        self.nodes.len() - 1
    }

    pub fn add_link(&mut self, a: usize, b: usize, length: f32) {
        self.links.push(Link { a, b, length });
    }

    pub fn step(&mut self, dt: f32, platter: &mut Platter, offset: Vec2, scale: f32) {
        let dt = dt / SUB_STEPS as f32;

        for _ in 0..SUB_STEPS {
            self.apply_magnetic_forces(dt, platter, offset, scale);
            self.verlet(dt);
            self.solve_constraints();
        }
    }

    fn apply_magnetic_forces(&mut self, dt: f32, platter: &Platter, offset: Vec2, scale: f32) {
         // Force = k * m_node * m_field / r^2?
         // Actually, let's just say the field exerts a force proportional to its value.
         // Field 0.0 -> 1.0. 0.5 Neutral.
         // Node 0.0 -> 1.0. 0.5 Neutral.

         let mag_strength = 500.0;

         for node in &mut self.nodes {
            if node.fixed { continue; }

            // Map world pos to grid pos
            // world = grid * scale + offset
            // grid = (world - offset) / scale
            let grid_pos = (node.pos - offset) / scale;
            let gx = grid_pos.x.round() as usize;
            let gy = grid_pos.y.round() as usize;

            let _field_val = platter.get_magnetism(gx, gy);

            // Interaction:
            // Like poles repel.
            // Node: m1 = node.mag - 0.5
            // Field: m2 = field_val - 0.5
            // Force dir?
            // If we just have scalar field, we need gradient to get direction.

            // Let's sample neighbors to get gradient
            let dx = (platter.get_magnetism(gx + 1, gy) - platter.get_magnetism(gx.saturating_sub(1), gy)) * 0.5;
            let dy = (platter.get_magnetism(gx, gy + 1) - platter.get_magnetism(gx, gy.saturating_sub(1))) * 0.5;

            // Gradient points to higher values (South, 1.0).
            // If I am North (0.0), I am attracted to South. So Force = +Gradient.
            // If I am South (1.0), I am repelled by South. Force = -Gradient.

            let m_node = node.magnetism - 0.5; // -0.5 (N) to 0.5 (S)

            // If m_node is negative (N), it seeks Positive (S) values.
            // It moves UP the gradient.
            // Force = Gradient * (-m_node * strength)?
            // N (-0.5): Force = G * 0.5. Moves towards high values. Correct.
            // S (0.5): Force = G * -0.5. Moves away from high values. Correct?
            // Wait, S should be attracted to N (low values).
            // Low values have negative gradient pointing away from them.
            // So if I want to go to low values, I follow -Gradient.
            // So S (0.5) * -0.5 = -0.25. Follows -Gradient. Correct.

            let force = Vec2::new(dx as f32, dy as f32) * (-m_node * mag_strength);

            // Apply to pos (Verlet)
            // F = ma -> a = F/m
            // dx = 0.5 * a * dt^2
            node.pos += force * (dt * dt) / node.mass;
         }
    }

    fn verlet(&mut self, dt: f32) {
        for node in &mut self.nodes {
            if node.fixed {
                continue;
            }
            let velocity = node.pos - node.prev_pos;
            // Apply friction
            let velocity = velocity * self.friction;

            node.prev_pos = node.pos;

            // Verlet integration
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
                let dist = delta.length();
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
