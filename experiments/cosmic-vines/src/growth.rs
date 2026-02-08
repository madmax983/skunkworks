use glam::Vec3;
use rand::Rng;
use std::collections::HashMap;
use crate::physics::CosmicString;

#[derive(Clone, Copy)]
pub struct Attractor {
    pub pos: Vec3,      // Current world position
    pub offset: Vec3,   // Offset from the string node
    pub node_idx: usize, // Index of the string node it belongs to
    pub active: bool,
}

#[derive(Clone)]
pub struct VeinNode {
    pub pos: Vec3,
    pub parent_idx: Option<usize>,
    pub thickness: f32,
}

pub struct Vine {
    pub attractors: Vec<Attractor>,
    pub veins: Vec<VeinNode>,
    pub detection_radius: f32,
    pub kill_radius: f32,
    pub growth_distance: f32,
}

impl Vine {
    pub fn new() -> Self {
        Self {
            attractors: Vec::new(),
            veins: Vec::new(),
            detection_radius: 15.0, // Adjusted for 3D scale
            kill_radius: 2.0,
            growth_distance: 1.0,
        }
    }

    pub fn seed_around_string(&mut self, string: &CosmicString, count_per_node: usize, radius: f32) {
        let mut rng = rand::thread_rng();
        for (idx, node) in string.nodes.iter().enumerate() {
            // Skip fixed nodes? Or maybe vines anchor there?
            // Let's seed everywhere.
            for _ in 0..count_per_node {
                // Random point in sphere around node
                // Uniform distribution in sphere
                let u: f32 = rng.gen_range(0.0..1.0);
                let v: f32 = rng.gen_range(0.0..1.0);
                let theta = 2.0 * std::f32::consts::PI * u;
                let phi = (2.0 * v - 1.0).acos();
                let r = radius * rng.gen::<f32>().cbrt();

                let x = r * phi.sin() * theta.cos();
                let y = r * phi.sin() * theta.sin();
                let z = r * phi.cos();

                let offset = Vec3::new(x, y, z);
                let pos = node.pos + offset;

                self.attractors.push(Attractor {
                    pos,
                    offset,
                    node_idx: idx,
                    active: true,
                });
            }
        }
    }

    pub fn update_attractors(&mut self, string: &CosmicString) {
        for att in &mut self.attractors {
            if att.active {
                if let Some(node) = string.nodes.get(att.node_idx) {
                    att.pos = node.pos + att.offset;
                }
            }
        }
    }

    pub fn add_root(&mut self, pos: Vec3) {
        self.veins.push(VeinNode {
            pos,
            parent_idx: None,
            thickness: 2.0,
        });
    }

    pub fn grow(&mut self) {
        if self.veins.is_empty() {
            return;
        }

        let mut forces: HashMap<usize, Vec3> = HashMap::new();
        let mut attractors_to_kill: Vec<usize> = Vec::new();

        // 1. Associate Attractors to Nodes
        for (attr_idx, attractor) in self.attractors.iter().enumerate() {
            if !attractor.active {
                continue;
            }

            let mut closest_dist = f32::MAX;
            let mut closest_node_idx = None;

            for (node_idx, node) in self.veins.iter().enumerate() {
                let d = attractor.pos.distance(node.pos);
                if d < closest_dist {
                    closest_dist = d;
                    closest_node_idx = Some(node_idx);
                }
            }

            if let Some(idx) = closest_node_idx {
                if closest_dist < self.kill_radius {
                    attractors_to_kill.push(attr_idx);
                } else if closest_dist < self.detection_radius {
                    let node = &self.veins[idx];
                    let dir = (attractor.pos - node.pos).normalize();

                    // Accumulate forces
                    forces.entry(idx)
                        .and_modify(|f| *f += dir)
                        .or_insert(dir);
                }
            }
        }

        // 2. Kill Attractors
        // Iterate in reverse to keep indices valid? No, just mark them inactive.
        for idx in attractors_to_kill {
            self.attractors[idx].active = false;
        }

        // 3. Grow New Nodes
        let mut new_nodes = Vec::new();
        for (parent_idx, force) in forces {
            let parent = &self.veins[parent_idx];
            let dir = force.normalize(); // Average direction
            let new_pos = parent.pos + dir * self.growth_distance;

            // Check if new position is too close to existing nodes?
            // Usually not strictly required for basic SCA, but helps density.
            // We'll skip for simplicity/performance.

            new_nodes.push(VeinNode {
                pos: new_pos,
                parent_idx: Some(parent_idx),
                thickness: 0.5, // Start thin
            });
        }

        let start_len = self.veins.len();
        self.veins.extend(new_nodes);

        // 4. Backpropagate thickness (optional visual flair)
        // Only update thickness for parents of NEW nodes to save perf
        for i in start_len..self.veins.len() {
            let mut curr = self.veins[i].parent_idx;
            let mut steps = 0;
            while let Some(idx) = curr {
                if steps > 100 { break; } // prevent infinite loop if cyclic graph (shouldn't happen in tree)
                self.veins[idx].thickness = (self.veins[idx].thickness + 0.02).min(5.0);
                curr = self.veins[idx].parent_idx;
                steps += 1;
            }
        }
    }
}
