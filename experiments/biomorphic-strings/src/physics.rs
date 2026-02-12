use glam::Vec3;

#[derive(Debug, Clone)]
pub struct BioStringNode {
    pub pos: Vec3,
    pub vel: Vec3,
    pub force: Vec3,
    pub fixed: bool,
    pub mass: f32,
    // Chemical concentrations
    pub u: f32,
    pub v: f32,
}

impl BioStringNode {
    pub fn new(pos: Vec3, fixed: bool, mass: f32) -> Self {
        Self {
            pos,
            vel: Vec3::ZERO,
            force: Vec3::ZERO,
            fixed,
            mass,
            u: 1.0,
            v: 0.0,
        }
    }
}

#[derive(Debug)]
pub struct BioString {
    pub nodes: Vec<BioStringNode>,
    pub rest_length: f32,
    pub tension: f32,
    pub damping: f32,
    // Gray-Scott parameters
    pub feed: f32,
    pub kill: f32,
    pub diff_u: f32,
    pub diff_v: f32,
}

impl BioString {
    pub fn new(start: Vec3, end: Vec3, segments: usize, tension: f32, damping: f32) -> Self {
        let mut nodes = Vec::with_capacity(segments + 1);
        let step = (end - start) / segments as f32;
        let rest_length = step.length();

        for i in 0..=segments {
            let pos = start + step * i as f32;
            let fixed = i == 0 || i == segments;
            let mut node = BioStringNode::new(pos, fixed, 1.0);

            // Seed the reaction in the middle
            if i > segments / 2 - 2 && i < segments / 2 + 2 {
                node.v = 1.0;
            }

            nodes.push(node);
        }

        Self {
            nodes,
            rest_length,
            tension,
            damping,
            // Standard Gray-Scott "Spots" parameters
            feed: 0.055,
            kill: 0.062,
            diff_u: 1.0,
            diff_v: 0.5,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.update_chemistry(dt);
        self.update_physics(dt);
    }

    fn update_chemistry(&mut self, dt: f32) {
        let len = self.nodes.len();

        // Snapshot current state to avoid borrow checker issues
        let current_state: Vec<(f32, f32)> = self.nodes.iter().map(|n| (n.u, n.v)).collect();
        let mut next_state: Vec<(f32, f32)> = Vec::with_capacity(len);

        for i in 0..len {
            let (left_u, left_v) = if i == 0 { current_state[i] } else { current_state[i-1] };
            let (right_u, right_v) = if i == len - 1 { current_state[i] } else { current_state[i+1] };
            let (center_u, center_v) = current_state[i];

            // 1D Laplacian: (left + right - 2*center)
            let lap_u = left_u + right_u - 2.0 * center_u;
            let lap_v = left_v + right_v - 2.0 * center_v;

            // Reaction
            let uvv = center_u * center_v * center_v;

            // Feedback from Physics: Tension affects Reaction Rate
            // Calculate local stretch
            let mut stretch_factor = 1.0;
            if i < len - 1 {
                 let dist = self.nodes[i].pos.distance(self.nodes[i+1].pos);
                 stretch_factor = dist / self.rest_length;
            }

            // Modulate feed rate by stretch (high tension = faster metabolism)
            let local_feed = self.feed * stretch_factor.clamp(0.5, 2.0);
            let local_kill = self.kill; // Could also modulate kill rate

            // Update with RD equation
            // du/dt = diff_u * lap_u - uv^2 + F * (1 - u)
            // dv/dt = diff_v * lap_v + uv^2 - (F + k) * v
            let du = self.diff_u * lap_u - uvv + local_feed * (1.0 - center_u);
            let dv = self.diff_v * lap_v + uvv - (local_feed + local_kill) * center_v;

            let next_u = (center_u + du * dt).clamp(0.0, 1.0);
            let next_v = (center_v + dv * dt).clamp(0.0, 1.0);

            next_state.push((next_u, next_v));
        }

        // Commit updates
        for (i, node) in self.nodes.iter_mut().enumerate() {
            node.u = next_state[i].0;
            node.v = next_state[i].1;
        }
    }

    fn update_physics(&mut self, dt: f32) {
        let len = self.nodes.len();
        let mut forces = vec![Vec3::ZERO; len];

        // Calculate spring forces
        for i in 0..len - 1 {
            let p1 = self.nodes[i].pos;
            let p2 = self.nodes[i + 1].pos;

            let delta = p2 - p1;
            let dist = delta.length();

            if dist > 1e-6 {
                let dir = delta / dist;
                let stretch = dist - self.rest_length;

                // Hooke's Law
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

            // Feedback from Chemistry: Concentration 'v' increases mass
            // "Heavy Metal" effect: high reaction product = heavy node
            let base_mass = 1.0;
            node.mass = base_mass * (1.0 + node.v * 5.0); // Mass can range from 1.0 to 6.0

            node.force = forces[i];

            // Damping
            node.force -= node.vel * self.damping;

            // F = ma
            let acc = node.force / node.mass;

            // Semi-Implicit Euler
            node.vel += acc * dt;
            node.pos += node.vel * dt;
        }
    }

    pub fn pluck(&mut self, index: usize, force: Vec3) {
        if index < self.nodes.len() && !self.nodes[index].fixed {
            let mass = self.nodes[index].mass;
            self.nodes[index].vel += force / mass;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_biostring_creation() {
        let start = Vec3::new(0.0, 0.0, 0.0);
        let end = Vec3::new(10.0, 0.0, 0.0);
        let string = BioString::new(start, end, 10, 10.0, 0.1);

        assert_eq!(string.nodes.len(), 11);
        assert_eq!(string.nodes[0].fixed, true);
        assert_eq!(string.nodes[10].fixed, true);
        // Middle should have v=1.0 due to seeding
        assert_eq!(string.nodes[5].v, 1.0);
    }

    #[test]
    fn test_chemistry_update() {
        let start = Vec3::new(0.0, 0.0, 0.0);
        let end = Vec3::new(10.0, 0.0, 0.0);
        let mut string = BioString::new(start, end, 10, 10.0, 0.1);

        // Run chemistry
        string.update_chemistry(0.1);

        // Diffusion should happen
        // Node 4 is neighbor of 5 (which has v=1.0)
        // v diffuses, so node 4 should get some v
        assert!(string.nodes[4].v > 0.0);
    }

    #[test]
    fn test_physics_feedback() {
        let start = Vec3::new(0.0, 0.0, 0.0);
        let end = Vec3::new(10.0, 0.0, 0.0);
        let mut string = BioString::new(start, end, 10, 10.0, 0.1);

        // Check mass of seeded node (index 5) vs unseeded (index 0)
        // update_physics needs to run to update mass based on v
        string.update_physics(0.1);

        let mass_seeded = string.nodes[5].mass;
        let mass_unseeded = string.nodes[0].mass;

        assert!(mass_seeded > mass_unseeded);
    }
}
