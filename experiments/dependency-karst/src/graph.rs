use anyhow::Result;
use cargo_metadata::MetadataCommand;
use petgraph::graph::Graph;
use petgraph::Directed;
use std::collections::HashMap;

pub type CrateGraph = Graph<String, (), Directed>;

pub fn load_graph() -> Result<CrateGraph> {
    let metadata = MetadataCommand::new().exec()?;
    let mut graph = Graph::new();
    let mut indices = HashMap::new();

    // Add nodes
    for package in &metadata.packages {
        if !indices.contains_key(&package.name) {
            let idx = graph.add_node(package.name.clone());
            indices.insert(package.name.clone(), idx);
        }
    }

    // Add edges
    for package in &metadata.packages {
        if let Some(&src_idx) = indices.get(&package.name) {
            for dep in &package.dependencies {
                if let Some(&dest_idx) = indices.get(&dep.name) {
                    // Avoid self-loops and duplicates if possible
                    if src_idx != dest_idx {
                        graph.update_edge(src_idx, dest_idx, ());
                    }
                }
            }
        }
    }

    Ok(graph)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_graph() {
        let graph = load_graph().expect("Failed to load graph");
        assert!(graph.node_count() > 0);
        assert!(graph.edge_count() > 0);
    }
}
