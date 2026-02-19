use petgraph::visit::EdgeRef;
use petgraph::Graph;
use std::collections::{HashMap, HashSet};

pub struct Layout {
    pub positions: HashMap<petgraph::graph::NodeIndex, (i32, i32, i32)>,
}

pub fn calculate_layout(graph: &Graph<String, ()>) -> Layout {
    let mut depths = HashMap::new();

    // Initialize depths
    for node in graph.node_indices() {
        depths.insert(node, 0);
    }

    // Relaxation to find depths (longest path)
    // Limit iterations to avoid infinite loops if cycles exist
    let limit = graph.node_count().max(10) + 10;
    for _ in 0..limit {
        let mut changed = false;
        for edge in graph.edge_references() {
            let src = edge.source();
            let target = edge.target();
            let d_src = *depths.get(&src).unwrap_or(&0);
            let d_target = *depths.get(&target).unwrap_or(&0);

            // If d_target < d_src + 1, update it
            if d_target < d_src + 1 {
                depths.insert(target, d_src + 1);
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }

    // Group by depth
    let mut levels: HashMap<i32, Vec<_>> = HashMap::new();
    for (node, depth) in &depths {
        levels.entry(*depth).or_default().push(*node);
    }

    let mut positions = HashMap::new();
    let mut occupied = HashSet::new();

    // Sort levels by depth
    let mut sorted_levels: Vec<_> = levels.into_iter().collect();
    sorted_levels.sort_by_key(|k| k.0);

    for (depth, nodes) in sorted_levels {
        let count = nodes.len();
        // Use a heuristic radius based on count
        let radius_base = (count as f32).sqrt().max(1.0) * 2.0;

        for (i, &node) in nodes.iter().enumerate() {
            let angle =
                (i as f32 / count as f32) * std::f32::consts::PI * 2.0 + (depth as f32 * 0.5);
            let mut r = radius_base;

            let w = depth;
            let mut u;
            let mut v;

            // Search for free spot
            // Spiral out if occupied
            loop {
                u = (r * angle.cos()).round() as i32;
                v = (r * angle.sin()).round() as i32;

                if !occupied.contains(&(u, v, w)) {
                    occupied.insert((u, v, w));
                    break;
                }
                r += 0.5; // Increment radius
            }
            positions.insert(node, (u, v, w));
        }
    }

    Layout { positions }
}

#[cfg(test)]
mod tests {
    use super::*;
    use petgraph::Graph;

    #[test]
    fn test_simple_layout() {
        let mut graph = Graph::new();
        let a = graph.add_node("A".to_string());
        let b = graph.add_node("B".to_string());
        graph.add_edge(a, b, ());

        let layout = calculate_layout(&graph);

        let pos_a = layout.positions.get(&a).unwrap();
        let pos_b = layout.positions.get(&b).unwrap();

        assert!(pos_b.2 > pos_a.2); // B should be higher than A
    }

    #[test]
    fn test_cycle() {
        let mut graph = Graph::new();
        let a = graph.add_node("A".to_string());
        let b = graph.add_node("B".to_string());
        graph.add_edge(a, b, ());
        graph.add_edge(b, a, ()); // Cycle

        let layout = calculate_layout(&graph);
        // It should not hang
        assert!(layout.positions.contains_key(&a));
        assert!(layout.positions.contains_key(&b));
    }
}
