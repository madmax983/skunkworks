use bevy::prelude::*;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Node {
    pub path: PathBuf,
    pub is_dir: bool,
    pub position: Vec2,
    pub radius: f32,
    pub depth: usize,
    pub parent_idx: Option<usize>,
}

#[derive(Debug, Default)]
pub struct CodeGraph {
    pub nodes: Vec<Node>,
    pub edges: Vec<(usize, usize)>, // (parent, child)
}

impl CodeGraph {
    pub fn scan(root: PathBuf, max_depth: usize) -> Self {
        let mut graph = CodeGraph::default();

        // Add root node
        graph.nodes.push(Node {
            path: root.clone(),
            is_dir: true,
            position: Vec2::ZERO,
            radius: 20.0,
            depth: 0,
            parent_idx: None,
        });

        // Walkdir
        // We need to maintain parent indices.
        // Walkdir is depth-first? Or we can use `into_iter`.
        // To track parents properly, we might need a map of Path -> Index.
        // Or recursive function.

        // Let's use recursive function for better control over structure
        Self::scan_recursive(&mut graph, root, 0, 0, max_depth);

        // Compute Layout
        graph.layout();

        graph
    }

    fn scan_recursive(
        graph: &mut CodeGraph,
        path: PathBuf,
        parent_idx: usize,
        depth: usize,
        max_depth: usize,
    ) {
        if depth >= max_depth {
            return;
        }

        let entries = match std::fs::read_dir(&path) {
            Ok(e) => e,
            Err(_) => return,
        };

        let mut children = Vec::new();

        for entry in entries.flatten() {
            let path = entry.path();
            let is_dir = path.is_dir();
            let file_name = path.file_name().unwrap_or_default().to_string_lossy();

            // Filter hidden
            if file_name.starts_with('.') {
                continue;
            }

            // Create node
            let node_idx = graph.nodes.len();
            graph.nodes.push(Node {
                path: path.clone(),
                is_dir,
                position: Vec2::ZERO, // Layout will fix
                radius: if is_dir { 15.0 } else { 10.0 },
                depth: depth + 1,
                parent_idx: Some(parent_idx),
            });
            graph.edges.push((parent_idx, node_idx));
            children.push((node_idx, path, is_dir));
        }

        // Recurse for dirs
        for (idx, p, is_dir) in children {
            if is_dir {
                Self::scan_recursive(graph, p, idx, depth + 1, max_depth);
            }
        }
    }

    fn layout(&mut self) {
        // Simple Radial Layout
        // We can do this by assigning an angular sector to each node.
        // Root has 0..2PI.
        // Children split the sector.

        // We need to know the number of leaves/descendants to assign sector size properly,
        // or just split evenly (which might cause overlap).
        // Let's try simple even split per level for now.
        // Or better: Force Directed? No, too complex to implement from scratch quickly.
        // Radial Tree:
        // Position = (depth * spacing, angle)

        if self.nodes.is_empty() {
            return;
        }

        // Assign angles.
        // Map parent_idx -> list of children
        let mut children_map: Vec<Vec<usize>> = vec![vec![]; self.nodes.len()];
        for &(p, c) in &self.edges {
            children_map[p].push(c);
        }

        let root_idx = 0;
        self.layout_recursive(root_idx, 0.0, std::f32::consts::TAU, 50.0, &children_map);
    }

    fn layout_recursive(
        &mut self,
        node_idx: usize,
        start_angle: f32,
        end_angle: f32,
        radius_step: f32,
        children_map: &[Vec<usize>],
    ) {
        let children = &children_map[node_idx];
        if children.is_empty() {
            return;
        }

        let angle_span = end_angle - start_angle;
        let angle_step = angle_span / children.len() as f32;

        for (i, &child_idx) in children.iter().enumerate() {
            let my_start = start_angle + i as f32 * angle_step;
            let my_end = my_start + angle_step;
            let my_mid = (my_start + my_end) / 2.0;

            let dist = (self.nodes[child_idx].depth as f32) * radius_step;

            let x = my_mid.cos() * dist;
            let y = my_mid.sin() * dist;

            self.nodes[child_idx].position = Vec2::new(x, y);

            self.layout_recursive(child_idx, my_start, my_end, radius_step, children_map);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_scan_layout() {
        // Create a dummy directory structure
        let temp_dir = std::env::temp_dir().join("syntax_spider_test");
        let _ = std::fs::remove_dir_all(&temp_dir);
        std::fs::create_dir_all(temp_dir.join("a/b")).unwrap();
        std::fs::write(temp_dir.join("a/file.txt"), "content").unwrap();
        std::fs::write(temp_dir.join("root.txt"), "content").unwrap();

        let graph = CodeGraph::scan(temp_dir.clone(), 3);

        // Root + a + a/b + a/file.txt + root.txt = 5 nodes
        // (Assuming temp dir was empty before)
        assert!(graph.nodes.len() >= 5);

        // Check positions
        for node in &graph.nodes {
            if node.depth > 0 {
                assert!(
                    node.position.length() > 0.0,
                    "Node at depth {} should have non-zero position",
                    node.depth
                );
            } else {
                assert_eq!(node.position, Vec2::ZERO);
            }
        }

        // Cleanup
        let _ = std::fs::remove_dir_all(temp_dir);
    }
}
