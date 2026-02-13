use crate::physics::PendulumSystem;
use cargo_metadata::MetadataCommand;
use macroquad::prelude::Vec2;
use rand::Rng;
use std::collections::HashMap;

pub fn load_dependencies() -> anyhow::Result<PendulumSystem> {
    let metadata = MetadataCommand::new().exec()?;

    let mut sys = PendulumSystem::new();
    let mut id_to_index = HashMap::new();
    let mut rng = rand::thread_rng();

    // Identify workspace root(s)
    let workspace_members = &metadata.workspace_members;

    // Create a map of PackageId -> Package for name lookup
    let package_map: HashMap<_, _> = metadata
        .packages
        .iter()
        .map(|p| (p.id.clone(), p))
        .collect();

    // metadata.resolve has the graph.
    let resolve = metadata
        .resolve
        .ok_or_else(|| anyhow::anyhow!("No resolve graph found"))?;

    // Center of screen
    let center = Vec2::new(600.0, 100.0);

    // First pass: Create nodes for all packages in the resolve graph
    for node in &resolve.nodes {
        // Name lookup
        let name = package_map
            .get(&node.id)
            .map(|p| p.name.clone())
            .unwrap_or_else(|| node.id.repr.clone());

        // Mass based on dependency count
        // More dependencies = heavier node
        let deps_count = node.dependencies.len();
        let mass = 1.0 + (deps_count as f32) * 0.2;

        // Position: Random cloud around center initially
        let offset_x = rng.gen_range(-200.0..200.0);
        let offset_y = rng.gen_range(0.0..300.0);
        let pos = center + Vec2::new(offset_x, offset_y);

        // Initially fix nothing, we'll fix the anchor later
        let fixed = false;

        let idx = sys.add_node(pos, mass, fixed, name);
        id_to_index.insert(node.id.clone(), idx);
    }

    // Second pass: Create links
    for node in &resolve.nodes {
        if let Some(&parent_idx) = id_to_index.get(&node.id) {
            for dep_id in &node.dependencies {
                if let Some(&child_idx) = id_to_index.get(dep_id) {
                    if parent_idx == child_idx {
                        continue;
                    }

                    // Link length
                    let length = 30.0 + rng.gen_range(0.0..10.0);
                    sys.add_link(parent_idx, child_idx, length);
                }
            }
        }
    }

    // Fix the anchor (the main workspace member)
    // We try to find "chaos-pendulum" or the first workspace member.
    let anchor_id = workspace_members
        .iter()
        .find(|id| {
            package_map
                .get(id)
                .map(|p| p.name == "chaos-pendulum")
                .unwrap_or(false)
        })
        .or_else(|| workspace_members.first());

    if let Some(anchor) = anchor_id {
        if let Some(&idx) = id_to_index.get(anchor) {
            sys.nodes[idx].fixed = true;
            sys.nodes[idx].pos = center;
            sys.nodes[idx].prev_pos = center;
            println!("Anchored to: {}", sys.nodes[idx].name);
        }
    } else if !sys.nodes.is_empty() {
        // Fallback
        sys.nodes[0].fixed = true;
        sys.nodes[0].pos = center;
    }

    Ok(sys)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_dependencies_structure() {
        let result = load_dependencies();
        match result {
            Ok(sys) => {
                assert!(sys.nodes.len() > 0, "Should load nodes");
                // Check that we have an anchor
                let anchor_exists = sys.nodes.iter().any(|n| n.fixed);
                assert!(anchor_exists, "Should have at least one fixed anchor node");
            }
            Err(e) => {
                panic!("Failed to load dependencies: {:?}", e);
            }
        }
    }
}
