use macroquad::prelude::Vec2;

#[derive(Clone, Copy, Debug)]
pub struct Nutrient {
    pub pos: Vec2,
    pub strength: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct Node {
    pub pos: Vec2,
    pub parent: Option<usize>,
    pub thickness: f32,
    pub growth_vec: Vec2, // Accumulator for attraction vectors
    pub count: i32,       // Number of nutrients influencing this node
}

pub struct RootSystem {
    pub nodes: Vec<Node>,
    pub nutrients: Vec<Nutrient>,
    pub detection_radius: f32,
    pub kill_radius: f32,
    pub segment_length: f32,
}

impl RootSystem {
    pub fn new(start_pos: Vec2, nutrients: Vec<Nutrient>) -> Self {
        Self {
            nodes: vec![Node {
                pos: start_pos,
                parent: None,
                thickness: 1.0,
                growth_vec: Vec2::ZERO,
                count: 0,
            }],
            nutrients,
            detection_radius: 50.0,
            kill_radius: 10.0,
            segment_length: 5.0,
        }
    }

    pub fn grow_step(&mut self) {
        // 1. Reset nodes
        for node in &mut self.nodes {
            node.growth_vec = Vec2::ZERO;
            node.count = 0;
        }

        // 2. Nutrients attract nodes
        let mut nutrients_to_remove = Vec::new();

        for (n_idx, nutrient) in self.nutrients.iter().enumerate() {
            let mut closest_dist = self.detection_radius;
            let mut closest_node_idx = None;

            for (node_idx, node) in self.nodes.iter().enumerate() {
                let dist = node.pos.distance(nutrient.pos);
                if dist < closest_dist {
                    closest_dist = dist;
                    closest_node_idx = Some(node_idx);
                }
            }

            if let Some(idx) = closest_node_idx {
                let node = &mut self.nodes[idx];
                let dir = (nutrient.pos - node.pos).normalize_or_zero();
                node.growth_vec += dir;
                node.count += 1;

                if closest_dist < self.kill_radius {
                    nutrients_to_remove.push(n_idx);
                }
            }
        }

        // 3. Remove eaten nutrients (sort descending to remove by index correctly)
        nutrients_to_remove.sort_unstable_by(|a, b| b.cmp(a));
        nutrients_to_remove.dedup(); // Just in case
        for idx in nutrients_to_remove {
            self.nutrients.swap_remove(idx);
        }

        // 4. Grow new nodes
        // We can't mutate self.nodes while iterating it.
        // We'll collect new nodes first.
        let mut new_nodes = Vec::new();

        for (i, node) in self.nodes.iter().enumerate() {
            if node.count > 0 {
                let avg_dir = (node.growth_vec / node.count as f32).normalize_or_zero();
                if avg_dir != Vec2::ZERO {
                    let new_pos = node.pos + avg_dir * self.segment_length;
                    new_nodes.push(Node {
                        pos: new_pos,
                        parent: Some(i),
                        thickness: 1.0,
                        growth_vec: Vec2::ZERO,
                        count: 0,
                    });
                }
            }
        }

        self.nodes.extend(new_nodes);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_growth_step() {
        // Arrange
        let start = Vec2::new(0.0, 0.0);
        let nutrient_pos = Vec2::new(0.0, 10.0);
        let nutrients = vec![Nutrient {
            pos: nutrient_pos,
            strength: 1.0,
        }];

        let mut system = RootSystem::new(start, nutrients);
        system.detection_radius = 20.0;
        system.segment_length = 5.0;
        system.kill_radius = 2.0; // Don't kill it immediately

        // Act
        system.grow_step();

        // Assert
        assert_eq!(system.nodes.len(), 2, "Expected 1 new node to be added");

        let new_node = &system.nodes[1];
        // The new node should be at (0, 5) roughly.
        println!("New node pos: {:?}", new_node.pos);
        assert!((new_node.pos.x - 0.0).abs() < 0.1);
        assert!((new_node.pos.y - 5.0).abs() < 0.1);
        assert_eq!(new_node.parent, Some(0));
    }
}
