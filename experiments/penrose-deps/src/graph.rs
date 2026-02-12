use anyhow::Result;
use cargo_metadata::MetadataCommand;
use petgraph::Graph;
use std::collections::HashMap;

pub type DepGraph = Graph<String, ()>;

pub fn build_graph() -> Result<DepGraph> {
    let metadata = MetadataCommand::new().exec()?;
    let mut graph = Graph::new();
    let mut indices = HashMap::new();

    // Add nodes for workspace members
    for package in metadata.workspace_packages() {
        let idx = graph.add_node(package.name.clone());
        indices.insert(package.id.clone(), idx);
    }

    // Use resolve to add edges between workspace members
    if let Some(resolve) = metadata.resolve {
        for node in resolve.nodes {
            if let Some(&source) = indices.get(&node.id) {
                for dep in node.dependencies {
                    if let Some(&target) = indices.get(&dep) {
                        // Avoid self-loops if any
                        if source != target {
                            graph.update_edge(source, target, ());
                        }
                    }
                }
            }
        }
    }

    Ok(graph)
}
