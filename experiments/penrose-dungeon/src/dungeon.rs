use crate::tiling::{Triangle, TriangleType};
use macroquad::prelude::*;
use petgraph::graph::{Graph, NodeIndex};
use petgraph::Undirected;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RoomType {
    AcuteRoom,
    ObtuseRoom,
}

pub struct Dungeon {
    pub graph: Graph<RoomType, (), Undirected>,
    pub room_centers: HashMap<NodeIndex, Vec2>,
    pub room_triangles: HashMap<NodeIndex, Triangle>,
}

impl Dungeon {
    pub fn new(triangles: Vec<Triangle>) -> Self {
        let mut graph = Graph::new_undirected();
        let mut room_centers = HashMap::new();
        let mut room_triangles = HashMap::new();

        // Quantization for vertex welding
        let mut vertex_map: HashMap<(i64, i64), usize> = HashMap::new();
        let mut vertices: Vec<Vec2> = Vec::new();
        // Use a large multiplier to preserve precision but allow integer keys
        // Penrose coords are related to PHI, so they are irrational.
        // But if we generated them locally, they are close.
        let quantization = 1000.0;

        let mut get_vertex_idx = |v: Vec2| -> usize {
            let x = (v.x * quantization).round() as i64;
            let y = (v.y * quantization).round() as i64;
            if let Some(&idx) = vertex_map.get(&(x, y)) {
                idx
            } else {
                let idx = vertices.len();
                vertices.push(v);
                vertex_map.insert((x, y), idx);
                idx
            }
        };

        struct IndexedTriangle {
            indices: [usize; 3],
            node_idx: NodeIndex,
        }

        let mut indexed_triangles = Vec::new();

        // Create nodes
        for t in triangles {
            let i0 = get_vertex_idx(t.a);
            let i1 = get_vertex_idx(t.b);
            let i2 = get_vertex_idx(t.c);

            let room_type = match t.kind {
                TriangleType::Acute => RoomType::AcuteRoom,
                TriangleType::Obtuse => RoomType::ObtuseRoom,
            };

            let node_idx = graph.add_node(room_type);
            room_centers.insert(node_idx, (t.a + t.b + t.c) / 3.0);
            room_triangles.insert(node_idx, t.clone());

            indexed_triangles.push(IndexedTriangle {
                indices: [i0, i1, i2],
                node_idx,
            });
        }

        // Build edges
        let mut edge_map: HashMap<(usize, usize), Vec<NodeIndex>> = HashMap::new();

        for t in &indexed_triangles {
            let edges = [
                (t.indices[0], t.indices[1]),
                (t.indices[1], t.indices[2]),
                (t.indices[2], t.indices[0]),
            ];

            for (u, v) in edges {
                let key = if u < v { (u, v) } else { (v, u) };
                edge_map.entry(key).or_default().push(t.node_idx);
            }
        }

        for (_key, nodes) in edge_map {
            if nodes.len() == 2 {
                graph.add_edge(nodes[0], nodes[1], ());
            }
        }

        Self {
            graph,
            room_centers,
            room_triangles,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tiling::{generate_initial_tiling, subdivide};
    // use petgraph::visit::Bfs; // Can't import trait impl directly? Just use full path.

    #[test]
    fn test_dungeon_connectivity() {
        // Initial wheel (10 triangles)
        let mut triangles = generate_initial_tiling();

        // Subdivide once to get more internal connections
        triangles = subdivide(&triangles);

        let dungeon = Dungeon::new(triangles);

        assert!(dungeon.graph.node_count() > 0);

        let mut edges_count = 0;
        for _ in dungeon.graph.edge_references() {
            edges_count += 1;
        }
        println!("Nodes: {}, Edges: {}", dungeon.graph.node_count(), edges_count);
        assert!(edges_count > 0);

        // Traverse from node 0
        let start = NodeIndex::new(0);
        let mut count = 0;
        let mut bfs = petgraph::visit::Bfs::new(&dungeon.graph, start);
        while let Some(_) = bfs.next(&dungeon.graph) {
            count += 1;
        }

        assert_eq!(count, dungeon.graph.node_count());
    }
}
