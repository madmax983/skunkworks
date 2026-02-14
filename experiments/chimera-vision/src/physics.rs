use glam::Vec2;

const SUB_STEPS: usize = 8; // Reduced for TUI performance

#[derive(Clone)]
pub struct Node {
    pub pos: Vec2,
    pub prev_pos: Vec2,
    pub mass: f32,
    pub fixed: bool,
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
            friction: 0.999,
        }
    }

    pub fn add_node(&mut self, pos: Vec2, mass: f32, fixed: bool, name: String) -> usize {
        self.nodes.push(Node {
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
            // Apply friction
            let velocity = velocity * self.friction;

            node.prev_pos = node.pos;

            // Verlet integration
            node.pos += velocity + self.gravity * (dt * dt);

            // Bounds check (keep inside some reasonable box so it doesn't fly off to infinity if unstable)
            // But TUI canvas is virtual, so maybe not strictly needed.
            // However, to prevent NaN explosions:
            if node.pos.length_squared() > 1000000.0 {
                node.pos = node.pos.normalize() * 1000.0;
                node.prev_pos = node.pos; // Kill velocity
            }
        }
    }

    fn solve_constraints(&mut self) {
        for _ in 0..5 {
            for link in &self.links {
                let pos_a = self.nodes[link.a].pos;
                let pos_b = self.nodes[link.b].pos;

                // Optimization: if both fixed, skip
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

    /// Returns the "Chaos Level" (Kinetic Energy) at a given position by finding the nearest node.
    pub fn get_chaos_level(&self, pos: Vec2) -> f32 {
        let mut min_dist_sq = f32::MAX;
        let mut nearest_node_idx = 0;

        if self.nodes.is_empty() {
            return 0.0;
        }

        for (i, node) in self.nodes.iter().enumerate() {
            let d2 = node.pos.distance_squared(pos);
            if d2 < min_dist_sq {
                min_dist_sq = d2;
                nearest_node_idx = i;
            }
        }

        if min_dist_sq > 10000.0 {
            // Too far from any node
            return 0.0;
        }

        let node = &self.nodes[nearest_node_idx];
        let velocity = node.pos - node.prev_pos;
        velocity.length_squared() * 1000.0 // Kinetic energy scaling
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_physics_step() {
        let mut sys = PendulumSystem::new();
        sys.add_node(Vec2::new(0.0, 0.0), 1.0, true, "root".to_string());
        let child = sys.add_node(Vec2::new(10.0, 0.0), 1.0, false, "child".to_string());

        // Run a few steps
        for _ in 0..10 {
            sys.step(0.16);
        }

        let pos = sys.nodes[child].pos;
        assert!(!pos.x.is_nan());
        assert!(!pos.y.is_nan());
        // Should have fallen due to gravity
        assert!(pos.y > 0.0);
    }

    #[test]
    fn test_chaos_level() {
        let mut sys = PendulumSystem::new();
        sys.add_node(Vec2::new(0.0, 0.0), 1.0, true, "root".to_string());
        let idx = sys.add_node(Vec2::new(10.0, 0.0), 1.0, false, "moving".to_string());

        // Move the node manually to create velocity
        sys.nodes[idx].pos = Vec2::new(11.0, 0.0);

        let chaos = sys.get_chaos_level(Vec2::new(11.0, 0.0));
        assert!(chaos > 0.0);
    }
}
