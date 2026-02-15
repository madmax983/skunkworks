use macroquad::prelude::*;

#[derive(Clone, Debug)]
pub struct Node {
    pub id: usize,
    pub name: String,
    pub pos: Vec2,
    pub depth: usize,
}

#[derive(Clone, Debug)]
pub struct Edge {
    pub source: usize,
    pub target: usize,
    pub weight: f32, // Call count / frequency
}

#[derive(Clone, Debug)]
pub struct CallGraph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

impl CallGraph {
    pub fn new() -> Self {
        Self { nodes: Vec::new(), edges: Vec::new() }
    }

    pub fn generate_random() -> Self {
        let mut graph = Self::new();
        // macroquad::rand uses global state, no thread_rng needed

        // Root
        graph.nodes.push(Node {
            id: 0,
            name: "main".to_string(),
            pos: vec2(0.0, 0.0),
            depth: 0,
        });

        let mut next_id = 1;
        let layers = 4;
        let nodes_per_layer = 8;
        let mut previous_layer_nodes = vec![0];

        // Create Layers
        for layer in 1..=layers {
            let mut current_layer_nodes = Vec::new();
            let radius = layer as f32 * 80.0; // Distance from center

            for i in 0..nodes_per_layer {
                let angle = (i as f32 / nodes_per_layer as f32) * std::f32::consts::PI * 2.0;
                let jitter_angle = rand::gen_range(-0.2, 0.2);
                let pos = vec2(
                    radius * (angle + jitter_angle).cos(),
                    radius * (angle + jitter_angle).sin()
                );

                let id = next_id;
                next_id += 1;

                graph.nodes.push(Node {
                    id,
                    name: format!("fn_{}", id),
                    pos,
                    depth: layer,
                });
                current_layer_nodes.push(id);

                // Connect to 1 or 2 random parents from previous layer
                let parent_count = rand::gen_range(1, 3); // 1 to 2 (upper bound exclusive in macroquad rand?)
                // macroquad::rand::gen_range(low, high) is [low, high) for integers?
                // Documentation says: "Generates a random value in the range [low, high)."

                for _ in 0..parent_count {
                    if !previous_layer_nodes.is_empty() {
                         let p_idx = rand::gen_range(0, previous_layer_nodes.len());
                         let parent = previous_layer_nodes[p_idx];

                         // Weights distribution: mostly small, some huge (hot paths)
                         let roll = rand::gen_range(0.0, 1.0);
                         let weight = if roll > 0.8 {
                             rand::gen_range(500.0, 1000.0)
                         } else {
                             rand::gen_range(10.0, 100.0)
                         };

                         graph.edges.push(Edge {
                             source: parent,
                             target: id,
                             weight,
                         });
                    }
                }
            }
            previous_layer_nodes = current_layer_nodes;
        }

        // Add random cross links
        for _ in 0..20 {
            let source = rand::gen_range(0, graph.nodes.len());
            let target = rand::gen_range(0, graph.nodes.len());

            if graph.nodes[source].depth < graph.nodes[target].depth {
                graph.edges.push(Edge {
                    source,
                    target,
                    weight: rand::gen_range(10.0, 200.0),
                });
            }
        }

        graph
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_generation() {
        // macroquad::rand needs to be seeded? Or it works?
        // In tests, macroquad context might not exist.
        // If rand::gen_range relies on miniquad context, it might panic.
        // We might need to use ::rand for tests if macroquad::rand fails outside window.
        // Let's see.
        // Usually macroquad functions panic outside of `macroquad::main`.
        // But rand might be pure rust implementation inside macroquad.
        // If it fails, I will switch back to ::rand with fully qualified path.
        let graph = CallGraph::generate_random();
        assert!(!graph.nodes.is_empty());
        assert!(!graph.edges.is_empty());
        assert_eq!(graph.nodes[0].name, "main");
    }
}
