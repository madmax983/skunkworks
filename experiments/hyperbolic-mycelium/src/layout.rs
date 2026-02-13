use crate::history::{CommitNode, calculate_similarity};
use poincare_disk::Point;
use std::collections::HashMap;
use std::f64::consts::PI;
use num_complex::Complex;

pub fn layout_graph(commits: &[CommitNode]) -> HashMap<String, Point> {
    let mut layout: HashMap<String, Point> = HashMap::new();

    if commits.is_empty() {
        return layout;
    }

    // Message map for similarity
    let message_map: HashMap<String, String> = commits.iter()
        .map(|c| (c.oid.clone(), c.message.clone()))
        .collect();

    // Root is at origin
    layout.insert(commits[0].oid.clone(), Point::new(0.0, 0.0));

    // We need to know children for each node
    let mut children_map: HashMap<String, Vec<String>> = HashMap::new();
    for commit in commits {
        for parent in &commit.parents {
            children_map.entry(parent.clone()).or_default().push(commit.oid.clone());
        }
    }

    // Radial layout
    // Each node has an angular sector (start_angle, end_angle)
    // Root has (0, 2PI)

    let mut sectors: HashMap<String, (f64, f64)> = HashMap::new();
    sectors.insert(commits[0].oid.clone(), (0.0, 2.0 * PI));

    // BFS queue: (oid, depth)
    let mut queue: Vec<(String, usize)> = Vec::new();
    queue.push((commits[0].oid.clone(), 0));

    // To prevent loops if graph is weird (merges), we track visited.
    // Ideally DAG.
    let mut visited: HashMap<String, bool> = HashMap::new();
    visited.insert(commits[0].oid.clone(), true);

    let mut head = 0;
    while head < queue.len() {
        let (parent_oid, depth) = queue[head].clone();
        head += 1;

        if let Some(children) = children_map.get(&parent_oid) {
            let (start_angle, end_angle) = *sectors.get(&parent_oid).unwrap();
            let angle_span = end_angle - start_angle;

            let count = children.len();
            if count == 0 { continue; }

            // Semantic Sort: Sort children by similarity to parent
            // This groups "relevant" children? No, it just orders them.
            // Better: Sort by message similarity to keep similar topics together?
            // For now, just sort by similarity to parent message descending.
            let mut sorted_children = children.clone();
            if let Some(parent_msg) = message_map.get(&parent_oid) {
                sorted_children.sort_by(|a, b| {
                    let msg_a = message_map.get(a).map(|s| s.as_str()).unwrap_or("");
                    let msg_b = message_map.get(b).map(|s| s.as_str()).unwrap_or("");
                    let sim_a = calculate_similarity(parent_msg, msg_a);
                    let sim_b = calculate_similarity(parent_msg, msg_b);
                    // Descending similarity
                    sim_b.partial_cmp(&sim_a).unwrap_or(std::cmp::Ordering::Equal)
                });
            }

            let slice = angle_span / count as f64;

            // Hyperbolic distance increases with depth
            let step = 0.8;
            let h_dist = (depth + 1) as f64 * step;
            let r = (h_dist / 2.0).tanh();

            for (i, child_oid) in sorted_children.iter().enumerate() {
                if visited.contains_key(child_oid) {
                    continue;
                }
                visited.insert(child_oid.clone(), true);

                let child_start = start_angle + i as f64 * slice;
                let child_end = child_start + slice;
                let mid_angle = child_start + slice / 2.0;

                let pos = Complex::from_polar(r, mid_angle);
                layout.insert(child_oid.clone(), pos);
                sectors.insert(child_oid.clone(), (child_start, child_end));

                queue.push((child_oid.clone(), depth + 1));
            }
        }
    }

    layout
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::history::CommitNode;

    #[test]
    fn test_layout_generation() {
        let commits = vec![
            CommitNode {
                oid: "root".to_string(),
                message: "root".to_string(),
                parents: vec![],
                timestamp: 0,
            },
            CommitNode {
                oid: "child1".to_string(),
                message: "child1".to_string(),
                parents: vec!["root".to_string()],
                timestamp: 1,
            },
            CommitNode {
                oid: "child2".to_string(),
                message: "child2".to_string(),
                parents: vec!["root".to_string()],
                timestamp: 2,
            },
        ];

        let layout = layout_graph(&commits);
        assert_eq!(layout.len(), 3);
        assert!(layout.contains_key("root"));
        assert!(layout.contains_key("child1"));
        assert!(layout.contains_key("child2"));

        // Root should be at origin
        let root_pos = layout.get("root").unwrap();
        assert!(root_pos.norm() < 1e-9);
    }
}
