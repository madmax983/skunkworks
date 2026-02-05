use poincare_disk::{hyperbolic_dist, mobius_add, mobius_sub, Point};
use num_complex::Complex;
use rand::Rng;

#[derive(Clone, Copy)]
pub struct Nutrient {
    pub pos: Point,
    pub active: bool,
}

#[derive(Clone, Copy)]
pub struct RootNode {
    pub pos: Point,
    pub parent: Option<usize>,
    pub thickness: f32,
}

pub struct RootSystem {
    pub nodes: Vec<RootNode>,
    pub nutrients: Vec<Nutrient>,
    pub detection_radius: f64, // Hyperbolic distance
    pub kill_radius: f64,      // Hyperbolic distance
    pub growth_dist: f64,      // Hyperbolic distance
}

impl RootSystem {
    pub fn new() -> Self {
        Self {
            nodes: vec![RootNode {
                pos: Complex::new(0.0, 0.0),
                parent: None,
                thickness: 2.0,
            }],
            nutrients: Vec::new(),
            detection_radius: 1.5, // Hyperbolic distance
            kill_radius: 0.1,      // Hyperbolic distance
            growth_dist: 0.05,     // Hyperbolic distance
        }
    }

    pub fn spawn_nutrients(&mut self, count: usize) {
        let mut rng = rand::thread_rng();
        for _ in 0..count {
            // Uniform in Euclidean disk for visual balance
            // Or maybe slightly biased towards edge?
            let angle = rng.gen_range(0.0..std::f64::consts::TAU);
            let r = rng.gen_range(0.0..0.99f64).sqrt(); // sqrt for uniform disk distribution
            let pos = Complex::from_polar(r, angle);
            self.nutrients.push(Nutrient { pos, active: true });
        }
    }

    pub fn add_nutrient(&mut self, pos: Point) {
        if pos.norm() < 0.99 {
            self.nutrients.push(Nutrient { pos, active: true });
        }
    }

    pub fn grow_step(&mut self) {
        // Map: Node Index -> Sum of Direction Vectors
        let mut attractors: Vec<Complex<f64>> = vec![Complex::new(0.0, 0.0); self.nodes.len()];
        let mut counts: Vec<usize> = vec![0; self.nodes.len()];
        let mut active_nutrients = false;

        // 1. Assign Nutrients to Closest Nodes
        for nutrient in &mut self.nutrients {
            if !nutrient.active {
                continue;
            }
            active_nutrients = true;

            let mut closest_dist = self.detection_radius;
            let mut closest_node_idx = None;

            // Simple search (optimize later if needed)
            for (i, node) in self.nodes.iter().enumerate() {
                let dist = hyperbolic_dist(node.pos, nutrient.pos);
                if dist < closest_dist {
                    closest_dist = dist;
                    closest_node_idx = Some(i);
                }
            }

            // If a node was found
            if let Some(idx) = closest_node_idx {
                // If within kill radius, remove nutrient
                if closest_dist < self.kill_radius {
                    nutrient.active = false;
                } else {
                    // Accumulate growth direction
                    // Direction vector is calculated in the node's local tangent space (mapped to origin)
                    let rel_pos = mobius_sub(nutrient.pos, self.nodes[idx].pos);
                    // Normalize to get direction
                    if rel_pos.norm() > 0.0001 {
                        let dir = rel_pos / rel_pos.norm();
                        attractors[idx] = attractors[idx] + dir;
                        counts[idx] += 1;
                    }
                }
            }
        }

        // Cleanup nutrients
        if active_nutrients {
            self.nutrients.retain(|n| n.active);
        }

        // 2. Grow new nodes
        let mut new_nodes = Vec::new();

        for (i, sum_dir) in attractors.iter().enumerate() {
            if counts[i] > 0 {
                // Average direction
                let dir = sum_dir / (counts[i] as f64);
                let normalized_dir = dir / dir.norm();

                // Calculate new position
                // Move 'growth_dist' along 'normalized_dir' from Origin
                // r = tanh(d/2)
                let move_r = (self.growth_dist / 2.0).tanh();
                let move_vec = normalized_dir * move_r;

                // Map back to global space
                let parent_pos = self.nodes[i].pos;
                let new_pos = mobius_add(move_vec, parent_pos);

                // Add new node
                new_nodes.push(RootNode {
                    pos: new_pos,
                    parent: Some(i),
                    thickness: 1.0, // Should probably decay?
                });
            }
        }

        self.nodes.extend(new_nodes);
    }
}
