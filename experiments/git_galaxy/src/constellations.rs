use crate::physics::Graph;
use ratatui::style::Color;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Constellation {
    pub name: String,
    pub description: String,
    pub nodes: Vec<usize>,
    pub color: Color,
}

pub struct ConstellationFinder;

impl ConstellationFinder {
    pub fn find_all(graph: &Graph) -> Vec<Constellation> {
        let mut constellations = Vec::new();
        constellations.extend(Self::find_supernovas(graph));
        constellations.extend(Self::find_chains(graph));
        constellations.extend(Self::find_clusters(graph));
        constellations
    }

    fn find_supernovas(graph: &Graph) -> Vec<Constellation> {
        // Top 3 heaviest nodes
        let mut nodes_with_mass: Vec<(usize, f64)> = graph
            .nodes
            .iter()
            .enumerate()
            .map(|(i, n)| (i, n.mass))
            .collect();
        // Sort descending
        nodes_with_mass.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        nodes_with_mass
            .into_iter()
            .take(3)
            .map(|(i, mass)| Constellation {
                name: format!(
                    "Supernova {}",
                    &graph.nodes[i].id.chars().take(6).collect::<String>()
                ),
                description: format!("Massive commit (churn: {:.0})", mass),
                nodes: vec![i],
                color: Color::Red,
            })
            .collect()
    }

    fn find_chains(graph: &Graph) -> Vec<Constellation> {
        // Build adjacency
        let mut adj = vec![Vec::new(); graph.nodes.len()];
        let mut rev_adj = vec![Vec::new(); graph.nodes.len()];
        for edge in &graph.edges {
            adj[edge.source].push(edge.target);
            rev_adj[edge.target].push(edge.source);
        }

        let mut visited = vec![false; graph.nodes.len()];
        let mut constellations = Vec::new();

        for i in 0..graph.nodes.len() {
            if visited[i] {
                continue;
            }

            // Look for linear chains: nodes with 1 child
            if adj[i].len() == 1 {
                let mut chain = vec![i];
                let mut curr = i;
                visited[i] = true;

                loop {
                    if adj[curr].len() != 1 {
                        break;
                    }
                    let next = adj[curr][0];

                    // To be a clean chain, next node must have only this parent (us)
                    if rev_adj[next].len() != 1 {
                        // Merge point, or something else. We can include it as the end of the chain.
                        chain.push(next);
                        visited[next] = true;
                        break;
                    }

                    if visited[next] {
                        break;
                    } // Loop detected

                    chain.push(next);
                    visited[next] = true;
                    curr = next;
                }

                if chain.len() >= 5 {
                    constellations.push(Constellation {
                        name: "The Chain".to_string(),
                        description: format!("A sequence of {} commits", chain.len()),
                        nodes: chain,
                        color: Color::Cyan,
                    });
                }
            }
        }
        constellations
    }

    fn find_clusters(graph: &Graph) -> Vec<Constellation> {
        // Group by author
        let mut author_nodes: HashMap<&String, Vec<usize>> = HashMap::new();
        for (i, node) in graph.nodes.iter().enumerate() {
            author_nodes.entry(&node.author).or_default().push(i);
        }

        let mut constellations = Vec::new();
        for (author, nodes) in author_nodes {
            if nodes.len() >= 5 {
                constellations.push(Constellation {
                    name: format!("{}'s Nebula", author),
                    description: format!("{} commits by {}", nodes.len(), author),
                    nodes: nodes.clone(),
                    color: Color::Green,
                });
            }
        }
        // Limit to top 3 clusters to avoid noise
        constellations.sort_by_key(|c| std::cmp::Reverse(c.nodes.len()));
        constellations.into_iter().take(3).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::physics::{Edge, Graph, Node};

    fn make_node(id: &str, author: &str, mass: f64) -> Node {
        Node {
            id: id.to_string(),
            author: author.to_string(),
            x: 0.0,
            y: 0.0,
            vx: 0.0,
            vy: 0.0,
            mass,
            color: (0, 0, 0),
        }
    }

    #[test]
    fn test_find_supernovas() {
        let nodes = vec![
            make_node("1", "A", 10.0),
            make_node("2", "A", 100.0),
            make_node("3", "A", 5.0),
        ];
        let graph = Graph {
            nodes,
            edges: vec![],
            node_indices: std::collections::HashMap::new(),
        };

        let constellations = ConstellationFinder::find_supernovas(&graph);
        assert!(!constellations.is_empty());
        assert_eq!(constellations[0].nodes[0], 1); // Index 1 has mass 100.0
    }

    #[test]
    fn test_find_chains() {
        // 0 -> 1 -> 2 -> 3 -> 4
        let nodes = vec![
            make_node("0", "A", 1.0),
            make_node("1", "A", 1.0),
            make_node("2", "A", 1.0),
            make_node("3", "A", 1.0),
            make_node("4", "A", 1.0),
        ];
        let edges = vec![
            Edge {
                source: 0,
                target: 1,
            },
            Edge {
                source: 1,
                target: 2,
            },
            Edge {
                source: 2,
                target: 3,
            },
            Edge {
                source: 3,
                target: 4,
            },
        ];
        let graph = Graph {
            nodes,
            edges,
            node_indices: std::collections::HashMap::new(),
        };

        let constellations = ConstellationFinder::find_chains(&graph);
        assert_eq!(constellations.len(), 1);
        assert_eq!(constellations[0].nodes.len(), 5);
    }
}
