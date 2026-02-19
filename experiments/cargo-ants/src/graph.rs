use petgraph::graph::{DiGraph, NodeIndex};
use rand::Rng;
use glam::Vec2;

#[derive(Clone, Debug)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub position: Vec2,
    pub is_root: bool,
    pub is_conflict: bool, // If true, this package is broken/incompatible
}

#[derive(Clone, Debug)]
pub struct Dependency {
    pub req: String,
}

pub struct DependencyGraph {
    pub graph: DiGraph<Package, Dependency>,
    pub root_index: NodeIndex,
}

impl DependencyGraph {
    pub fn new() -> Self {
        let mut graph = DiGraph::new();

        let root = graph.add_node(Package {
            name: "root".to_string(),
            version: "0.0.0".to_string(),
            position: Vec2::new(50.0, 300.0),
            is_root: true,
            is_conflict: false,
        });

        DependencyGraph {
            graph,
            root_index: root,
        }
    }

    pub fn generate_random(&mut self, width: f32, height: f32) {
        let mut rng = rand::thread_rng();
        let layer_count = 8;
        let layer_dist = (width - 100.0) / layer_count as f32;

        let mut previous_layer: Vec<NodeIndex> = vec![self.root_index];

        for l in 1..=layer_count {
            let x = 50.0 + l as f32 * layer_dist;
            let nodes_in_layer = rng.gen_range(2..6);
            let mut current_layer = Vec::new();

            for i in 0..nodes_in_layer {
                let y = rng.gen_range(50.0..height - 50.0);
                let is_conflict = rng.gen_bool(0.15); // 15% chance of conflict

                let node = self.graph.add_node(Package {
                    name: format!("pkg_{}_{}", l, i),
                    version: format!("1.{}.{}", l, i),
                    position: Vec2::new(x, y),
                    is_root: false,
                    is_conflict,
                });
                current_layer.push(node);
            }

            // Connect previous layer to current layer
            for &src in &previous_layer {
                let num_edges = rng.gen_range(1..=3);
                for _ in 0..num_edges {
                    if current_layer.is_empty() { break; }
                    let dest = current_layer[rng.gen_range(0..current_layer.len())];
                    // Avoid duplicate edges if possible, but petgraph allows parallel edges.
                    // For simplicity, just add.
                    self.graph.add_edge(src, dest, Dependency {
                        req: "^1.0".to_string(),
                    });
                }
            }
            previous_layer = current_layer;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_generation() {
        let mut graph = DependencyGraph::new();
        graph.generate_random(1000.0, 1000.0);
        assert!(graph.graph.node_count() > 10);
        assert!(graph.graph.edge_count() > 10);
    }
}
