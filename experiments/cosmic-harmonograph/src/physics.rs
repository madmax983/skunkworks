use glam::Vec3;

#[derive(Debug, Clone)]
pub struct StringNode {
    pub pos: Vec3,
    pub vel: Vec3,
    pub force: Vec3,
    pub fixed: bool,
    pub mass: f32,
}

impl StringNode {
    pub fn new(pos: Vec3, fixed: bool, mass: f32) -> Self {
        Self {
            pos,
            vel: Vec3::ZERO,
            force: Vec3::ZERO,
            fixed,
            mass,
        }
    }
}

#[derive(Debug)]
pub struct CosmicString {
    pub nodes: Vec<StringNode>,
    pub rest_length: f32, // Distance between adjacent nodes at rest
    pub tension: f32,     // Spring constant k
    pub damping: f32,
}

impl CosmicString {
    pub fn new(start: Vec3, end: Vec3, segments: usize, tension: f32, damping: f32) -> Self {
        let mut nodes = Vec::with_capacity(segments + 1);
        let step = (end - start) / segments as f32;
        let rest_length = step.length();

        for i in 0..=segments {
            let pos = start + step * i as f32;
            let fixed = i == 0 || i == segments;
            nodes.push(StringNode::new(pos, fixed, 1.0));
        }

        Self {
            nodes,
            rest_length,
            tension,
            damping,
        }
    }

    pub fn update(&mut self, dt: f32) {
        // Calculate internal spring forces
        let mut forces = vec![Vec3::ZERO; self.nodes.len()];

        for i in 0..self.nodes.len() - 1 {
            let p1 = self.nodes[i].pos;
            let p2 = self.nodes[i + 1].pos;

            let delta = p2 - p1;
            let dist = delta.length();

            if dist > 1e-6 {
                let dir = delta / dist;
                let stretch = dist - self.rest_length;

                // Hooke's Law: F = k * x (pulling p1 towards p2)
                let f = dir * (self.tension * stretch);

                forces[i] += f;
                forces[i + 1] -= f;
            }
        }

        // Apply forces and integrate
        for (i, node) in self.nodes.iter_mut().enumerate() {
            if node.fixed {
                node.vel = Vec3::ZERO;
                node.force = Vec3::ZERO;
                continue;
            }

            // External forces (none for now, maybe gravity later)
            // node.force = Vec3::new(0.0, -9.8 * node.mass, 0.0);
            node.force = Vec3::ZERO;

            // Add spring forces
            node.force += forces[i];

            // Add damping: F_d = -b * v
            node.force -= node.vel * self.damping;

            // F = ma => a = F/m
            let acc = node.force / node.mass;

            // Semi-Implicit Euler
            node.vel += acc * dt;
            node.pos += node.vel * dt;
        }
    }

    pub fn pluck(&mut self, index: usize, force: Vec3) {
        if index < self.nodes.len() && !self.nodes[index].fixed {
            // Apply impulse: J = F * dt -> delta_v = J / m
            // Here 'force' is treated as impulse for instantaneous pluck
            let mass = self.nodes[index].mass;
            self.nodes[index].vel += force / mass;
        }
    }

    #[allow(dead_code)]
    pub fn total_kinetic_energy(&self) -> f32 {
        self.nodes
            .iter()
            .map(|n| 0.5 * n.mass * n.vel.length_squared())
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cosmic_string_creation() {
        let start = Vec3::new(0.0, 0.0, 0.0);
        let end = Vec3::new(10.0, 0.0, 0.0);
        let string = CosmicString::new(start, end, 10, 10.0, 0.1);

        assert_eq!(string.nodes.len(), 11);
        assert_eq!(string.nodes[0].fixed, true);
        assert_eq!(string.nodes[10].fixed, true);
        assert_eq!(string.rest_length, 1.0);
    }

    #[test]
    fn test_pluck_updates_velocity() {
        let start = Vec3::new(0.0, 0.0, 0.0);
        let end = Vec3::new(10.0, 0.0, 0.0);
        let mut string = CosmicString::new(start, end, 10, 10.0, 0.1);

        let pluck_force = Vec3::new(0.0, 1.0, 0.0);
        string.pluck(5, pluck_force);

        assert!(string.nodes[5].vel.y > 0.0);
    }

    #[test]
    fn test_physics_update_moves_nodes() {
        let start = Vec3::new(0.0, 0.0, 0.0);
        let end = Vec3::new(10.0, 0.0, 0.0);
        let mut string = CosmicString::new(start, end, 10, 10.0, 0.0); // No damping

        // Displace middle node
        string.nodes[5].pos.y = 1.0;

        // Run one step
        string.update(0.1);

        // Neighbor nodes should be pulled up by spring force
        assert!(string.nodes[4].vel.y > 0.0);
        assert!(string.nodes[6].vel.y > 0.0);

        // Middle node should be pulled down
        assert!(string.nodes[5].vel.y < 0.0);
    }
}
