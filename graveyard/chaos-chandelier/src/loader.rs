use anyhow::Result;
use cargo_metadata::MetadataCommand;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct PendulumConfig {
    #[allow(dead_code)]
    pub name: String,
    pub m1: f32,
    pub l1: f32,
    pub m2: f32,
    pub l2: f32,
    // Store the path for display: "Grandparent -> Parent -> Child"
    #[allow(dead_code)]
    pub path: String,
}

pub fn load_dependency_chains() -> Result<Vec<PendulumConfig>> {
    let metadata = MetadataCommand::new().exec()?;
    let resolve = metadata
        .resolve
        .ok_or_else(|| anyhow::anyhow!("No resolve graph found"))?;

    let package_map: HashMap<_, _> = metadata
        .packages
        .iter()
        .map(|p| (p.id.clone(), p))
        .collect();

    let mut configs = Vec::new();

    // Map NodeId -> Node
    let node_map: HashMap<_, _> = resolve.nodes.iter().map(|n| (n.id.clone(), n)).collect();

    // Find all A -> B -> C chains
    // Iterate over all nodes as "Grandparent" (A)
    for node_a in &resolve.nodes {
        let pkg_a = package_map.get(&node_a.id);
        let name_a = pkg_a
            .map(|p| p.name.clone())
            .unwrap_or_else(|| "Unknown".to_string());

        // Iterate over children (B)
        for dep_id_b in &node_a.dependencies {
            if let Some(node_b) = node_map.get(dep_id_b) {
                let pkg_b = package_map.get(&node_b.id);
                let name_b = pkg_b
                    .map(|p| p.name.clone())
                    .unwrap_or_else(|| "Unknown".to_string());

                // Iterate over grandchildren (C)
                let mut has_grandchildren = false;
                for dep_id_c in &node_b.dependencies {
                    if let Some(node_c) = node_map.get(dep_id_c) {
                        has_grandchildren = true;
                        let pkg_c = package_map.get(&node_c.id);
                        let name_c = pkg_c
                            .map(|p| p.name.clone())
                            .unwrap_or_else(|| "Unknown".to_string());

                        // Chain Found: A -> B -> C

                        // Mass 1 (B): Based on name length + dep count
                        let m1 = (name_b.len() as f32).max(1.0)
                            + (node_b.dependencies.len() as f32 * 0.5);
                        // Length 1 (A->B): Fixed or based on something else? Let's say random or fixed.
                        let l1 = 1.0;

                        // Mass 2 (C):
                        let m2 = (name_c.len() as f32).max(1.0)
                            + (node_c.dependencies.len() as f32 * 0.5);
                        // Length 2 (B->C):
                        let l2 = 1.0;

                        configs.push(PendulumConfig {
                            name: format!("{} -> {} -> {}", name_a, name_b, name_c),
                            m1,
                            l1,
                            m2,
                            l2,
                            path: format!("{} -> {} -> {}", name_a, name_b, name_c),
                        });
                    }
                }

                if !has_grandchildren {
                    // Leaf case: A -> B -> (Ghost)
                    // We treat B as the first mass, and a ghost as the second.
                    let m1 =
                        (name_b.len() as f32).max(1.0) + (node_b.dependencies.len() as f32 * 0.5);
                    let l1 = 1.0;
                    let m2 = 1.0; // Ghost mass
                    let l2 = 0.5; // Ghost length

                    configs.push(PendulumConfig {
                        name: format!("{} -> {} -> (Ghost)", name_a, name_b),
                        m1,
                        l1,
                        m2,
                        l2,
                        path: format!("{} -> {}", name_a, name_b),
                    });
                }
            }
        }
    }

    // Sort or shuffle? Shuffle to mix them up visually.
    // We'll leave them as is, the renderer can shuffle.
    println!("Loaded {} chains", configs.len());

    Ok(configs)
}
