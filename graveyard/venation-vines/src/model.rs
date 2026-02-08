use glam::Vec2;
use rand::Rng;
use std::collections::HashMap;

#[derive(Clone, Copy)]
pub struct Attractor {
    pub pos: Vec2,
    pub active: bool,
}

#[derive(Clone)]
pub struct VeinNode {
    pub pos: Vec2,
    pub parent_idx: Option<usize>,
    pub thickness: f32,
}

pub struct Leaf {
    pub attractors: Vec<Attractor>,
    pub veins: Vec<VeinNode>,
    pub detection_radius: f32,
    pub kill_radius: f32,
    pub growth_distance: f32,
}

impl Leaf {
    pub fn new() -> Self {
        Self {
            attractors: Vec::new(),
            veins: Vec::new(),
            detection_radius: 10.0,
            kill_radius: 2.0,
            growth_distance: 1.0,
        }
    }

    pub fn seed_random(&mut self, width: f32, height: f32, count: usize) {
        let mut rng = rand::thread_rng();
        for _ in 0..count {
            let x = rng.gen_range(0.0..width);
            let y = rng.gen_range(0.0..height);
            // Simple circle constraint
            let center = Vec2::new(width / 2.0, height / 2.0);
            let pos = Vec2::new(x, y);
            if pos.distance(center) < width.min(height) / 2.0 {
                self.attractors.push(Attractor { pos, active: true });
            }
        }
    }

    pub fn init_vein(&mut self, x: f32, y: f32) {
        self.veins.push(VeinNode {
            pos: Vec2::new(x, y),
            parent_idx: None,
            thickness: 1.0,
        });
    }

    pub fn grow(&mut self) {
        if self.veins.is_empty() {
            return;
        }

        let mut forces: HashMap<usize, Vec2> = HashMap::new();
        let mut attractors_to_kill: Vec<usize> = Vec::new();

        // 1. Associate Attractors to Nodes
        for (attr_idx, attractor) in self.attractors.iter().enumerate() {
            if !attractor.active {
                continue;
            }

            let mut closest_dist = f32::MAX;
            let mut closest_node_idx = None;

            // Simple brute force search
            // Optimization: check only active nodes? No, veins don't deactivate.
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

                    forces.entry(idx).and_modify(|f| *f += dir).or_insert(dir);
                }
            }
        }

        // 2. Kill Attractors
        for idx in attractors_to_kill {
            self.attractors[idx].active = false;
        }

        // 3. Grow New Nodes
        let mut new_nodes = Vec::new();
        for (parent_idx, force) in forces {
            let parent = &self.veins[parent_idx];
            // Normalize the average direction
            let dir = force.normalize();
            let new_pos = parent.pos + dir * self.growth_distance;

            new_nodes.push(VeinNode {
                pos: new_pos,
                parent_idx: Some(parent_idx),
                thickness: 0.5,
            });
        }

        let start_len = self.veins.len();
        self.veins.extend(new_nodes);

        // 4. Backpropagate thickness
        for i in start_len..self.veins.len() {
            let mut curr = self.veins[i].parent_idx;
            while let Some(idx) = curr {
                // Simple heuristic: accumulate thickness
                self.veins[idx].thickness = (self.veins[idx].thickness + 0.05).min(5.0);
                curr = self.veins[idx].parent_idx;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vein_growth() {
        let mut leaf = Leaf::new();
        // Setup attractor nearby (distance 5, detection 10, kill 2)
        leaf.attractors.push(Attractor {
            pos: Vec2::new(0.0, 5.0),
            active: true,
        });

        // Setup initial vein
        leaf.init_vein(0.0, 0.0);

        let initial_count = leaf.veins.len();

        // Grow
        leaf.grow();

        assert!(
            leaf.veins.len() > initial_count,
            "Veins should have grown towards attractor"
        );

        // Check position of new vein
        let new_vein = leaf.veins.last().unwrap();
        assert!(
            new_vein.pos.y > 0.0,
            "New vein should have moved in Y direction"
        );
    }
}
