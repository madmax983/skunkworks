use anyhow::{Context, Result};
use cargo_metadata::{Metadata, MetadataCommand};
use petgraph::Graph;
use petgraph::graph::NodeIndex;
use std::collections::HashMap;

pub type DependencyGraph = Graph<String, ()>;

pub struct WorkspaceGraph {
    pub graph: DependencyGraph,
    pub node_map: HashMap<String, NodeIndex>,
    pub root_index: Option<NodeIndex>,
}

pub fn build_graph() -> Result<WorkspaceGraph> {
    let metadata = MetadataCommand::new()
        .exec()
        .context("Failed to run cargo metadata")?;

    let mut graph = Graph::<String, ()>::new();
    let mut node_map = HashMap::new();

    // Add nodes
    for package in &metadata.packages {
        let name = package.name.to_string();
        if !node_map.contains_key(&name) {
            let idx = graph.add_node(name.clone());
            node_map.insert(name, idx);
        }
    }

    // Add edges
    for package in &metadata.packages {
        let pkg_name = package.name.to_string();
        if let Some(&source_idx) = node_map.get(&pkg_name) {
            for dep in &package.dependencies {
                let dep_name = dep.name.to_string();
                if let Some(&target_idx) = node_map.get(&dep_name) {
                    // Directed edge from Source -> Target (depends on)
                    graph.add_edge(source_idx, target_idx, ());
                }
            }
        }
    }

    // Find root (heuristic: the package in current directory, or first workspace member)
    let root_index = find_root(&metadata, &node_map);

    Ok(WorkspaceGraph {
        graph,
        node_map,
        root_index,
    })
}

fn find_root(metadata: &Metadata, node_map: &HashMap<String, NodeIndex>) -> Option<NodeIndex> {
    // Try to find the package corresponding to the current directory
    // This might be tricky if running from root.
    // Let's pick the first workspace member.
    if let Some(first_member) = metadata.workspace_members.first() {
        // Find the package with this ID
        if let Some(pkg) = metadata.packages.iter().find(|p| &p.id == first_member) {
            return node_map.get(&pkg.name.to_string()).cloned();
        }
    }

    // Fallback: pick the first node in the graph
    if !node_map.is_empty() {
        // Just pick one
        return node_map.values().next().cloned();
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_construction() {
        // This test runs cargo metadata on the actual repo.
        // It might be slow but verifies integration.
        let wg = build_graph().expect("Failed to build graph");
        assert!(wg.graph.node_count() > 0);
        assert!(wg.root_index.is_some());

        let root = wg.root_index.unwrap();
        let name = &wg.graph[root];
        println!("Root node: {}", name);
    }
}
