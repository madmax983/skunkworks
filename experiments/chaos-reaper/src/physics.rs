use macroquad::prelude::Vec2;

const SUB_STEPS: usize = 20;

#[derive(Clone)]
pub struct PendulumNode {
    pub pos: Vec2,
    pub prev_pos: Vec2,
    pub mass: f32,
    pub fixed: bool,
    #[allow(dead_code)]
    pub name: String,
}

#[derive(Clone)]
pub struct Link {
    pub a: usize,
    pub b: usize,
    pub length: f32,
}

#[derive(Clone)]
pub struct PendulumSystem {
    pub nodes: Vec<PendulumNode>,
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

    pub fn add_node(&mut self, pos: Vec2, mass: f32, fixed: bool, name: String) -> usize {
        self.nodes.push(PendulumNode {
            pos,
            prev_pos: pos,
            mass,
            fixed,
            name,
        });
        self.nodes.len() - 1
    }

    pub fn add_link(&mut self, a: usize, b: usize, length: f32) {
        self.links.push(Link { a, b, length });
    }

    pub fn step(&mut self, dt: f32) {
        let dt = dt / SUB_STEPS as f32;

        for _ in 0..SUB_STEPS {
            self.verlet(dt);
            self.solve_constraints();
        }
    }

    fn verlet(&mut self, dt: f32) {
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

    #[allow(dead_code)]
    pub fn total_energy(&self, dt: f32) -> f32 {
        let mut kinetic = 0.0;
        let mut potential = 0.0;

        for node in &self.nodes {
            if node.fixed {
                continue;
            }
            let sub_dt = dt / SUB_STEPS as f32;
            let v = (node.pos - node.prev_pos) / sub_dt;

            kinetic += 0.5 * node.mass * v.length_squared();
            potential -= node.mass * self.gravity.dot(node.pos);
        }
        kinetic + potential
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_energy_conservation() {
        let mut sys = PendulumSystem::new();
        let root = sys.add_node(Vec2::new(0.0, 0.0), 1.0, true, "root".to_string());
        let child = sys.add_node(Vec2::new(1.0, 0.0), 1.0, false, "child".to_string());
        sys.add_link(root, child, 1.0);

        let dt = 0.016;
        sys.step(dt);
        let e1 = sys.total_energy(dt);
        sys.step(dt);

        for _ in 0..10 {
            sys.step(dt);
        }
        let e_final = sys.total_energy(dt);

        assert!(
            (e1 - e_final).abs() < 2.0,
            "Energy diverged significantly: {} vs {}",
            e1,
            e_final
        );
        assert_ne!(sys.nodes[child].pos, Vec2::new(1.0, 0.0));
    }
}
