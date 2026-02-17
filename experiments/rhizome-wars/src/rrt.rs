use macroquad::prelude::*;
use ::rand::Rng;

#[derive(Clone, Copy, Debug)]
pub struct Node {
    pub pos: Vec2,
    pub parent_index: Option<usize>,
    pub cost: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct Obstacle {
    pub pos: Vec2,
    pub radius: f32,
}

pub struct Tree {
    pub nodes: Vec<Node>,
    pub color: Color,
    pub step_size: f32,
    pub rewire_radius: f32,
}

impl Tree {
    pub fn new(start_pos: Vec2, color: Color) -> Self {
        Self {
            nodes: vec![Node {
                pos: start_pos,
                parent_index: None,
                cost: 0.0,
            }],
            color,
            step_size: 5.0,
            rewire_radius: 15.0,
        }
    }

    pub fn grow_step(&mut self, target: Option<Vec2>, obstacles: &[Obstacle], other_trees: &[&Tree], bounds: Rect) {
        let mut rng = ::rand::thread_rng();

        // 1. Sample
        // 10% chance to steer towards target if it exists
        let sample = if target.is_some() && rng.gen_bool(0.1) {
            target.unwrap()
        } else {
            Vec2::new(
                rng.gen_range(bounds.x..bounds.x + bounds.w),
                rng.gen_range(bounds.y..bounds.y + bounds.h),
            )
        };

        // 2. Nearest
        let nearest_idx = self.nearest_node_index(sample);
        let nearest_node = self.nodes[nearest_idx];

        // 3. Steer
        let diff = sample - nearest_node.pos;
        let dist = diff.length();

        if dist < 0.1 { return; } // Too close

        let step = diff.normalize() * self.step_size.min(dist);
        let new_pos = nearest_node.pos + step;

        // 4. Collision Check
        // Check obstacles
        for obs in obstacles {
            if new_pos.distance(obs.pos) < obs.radius {
                return;
            }
        }

        // Check bounds
        if !bounds.contains(new_pos) {
            return;
        }

        // Check other trees (Competition)
        for tree in other_trees {
            for node in &tree.nodes {
                if new_pos.distance(node.pos) < self.step_size * 0.8 {
                    // Too close to enemy root
                    return;
                }
            }
        }

        // Check self collision (don't grow back into self too densely)
        // Optimization: check only local nodes? For now, brute force (O(N)).
        // RRT doesn't usually check self-collision, but for "roots" we don't want overlap.
        // Let's allow self-overlap for now as branches can cross, but maybe not too close.
        // Actually, let's skip self-check to allow density.

        // 5. Add Node
        let new_cost = nearest_node.cost + step.length();
        let new_node = Node {
            pos: new_pos,
            parent_index: Some(nearest_idx),
            cost: new_cost,
        };

        // 6. RRT* Rewire
        // Find neighbors within rewire radius
        let mut min_cost = new_cost;
        let mut best_parent_idx = nearest_idx;

        let neighbors = self.find_neighbors(new_pos, self.rewire_radius);

        // Check if we can connect to a better parent
        for &idx in &neighbors {
            let neighbor = self.nodes[idx];
            let dist = neighbor.pos.distance(new_pos);
            if neighbor.cost + dist < min_cost {
                // Check collision for this new connection? Assuming clear if within radius.
                min_cost = neighbor.cost + dist;
                best_parent_idx = idx;
            }
        }

        let mut new_node_final = new_node;
        new_node_final.parent_index = Some(best_parent_idx);
        new_node_final.cost = min_cost;

        let new_idx = self.nodes.len();
        self.nodes.push(new_node_final);

        // Rewire neighbors to use new node as parent if it's shorter
        for &idx in &neighbors {
            let neighbor = self.nodes[idx];
            let dist = neighbor.pos.distance(new_pos);
            if min_cost + dist < neighbor.cost {
                // Rewire!
                // We need to update neighbor's parent and cost.
                // And recursively update its children costs?
                // Updating children costs is O(N) in worst case.
                // For simplicity, we just change the parent and update cost.
                // Correct RRT* updates children.
                // Let's just update parent for visual effect, exact cost propagation is expensive.
                self.nodes[idx].parent_index = Some(new_idx);
                self.nodes[idx].cost = min_cost + dist;
            }
        }
    }

    fn nearest_node_index(&self, pos: Vec2) -> usize {
        let mut min_dist = f32::MAX;
        let mut idx = 0;
        for (i, node) in self.nodes.iter().enumerate() {
            let d = node.pos.distance_squared(pos);
            if d < min_dist {
                min_dist = d;
                idx = i;
            }
        }
        idx
    }

    fn find_neighbors(&self, pos: Vec2, radius: f32) -> Vec<usize> {
        let mut neighbors = Vec::new();
        let r2 = radius * radius;
        for (i, node) in self.nodes.iter().enumerate() {
            if node.pos.distance_squared(pos) < r2 {
                neighbors.push(i);
            }
        }
        neighbors
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_initialization() {
        let tree = Tree::new(Vec2::new(0.0, 0.0), RED);
        assert_eq!(tree.nodes.len(), 1);
        assert_eq!(tree.nodes[0].pos, Vec2::new(0.0, 0.0));
    }

    #[test]
    fn test_nearest_neighbor() {
        let mut tree = Tree::new(Vec2::new(0.0, 0.0), RED);
        tree.nodes.push(Node { pos: Vec2::new(10.0, 10.0), parent_index: Some(0), cost: 14.14 });

        let idx = tree.nearest_node_index(Vec2::new(11.0, 11.0));
        assert_eq!(idx, 1);

        let idx = tree.nearest_node_index(Vec2::new(1.0, 1.0));
        assert_eq!(idx, 0);
    }
}
