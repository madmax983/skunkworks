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

    // We want to process nodes in topological order or just BFS from roots.
    // metadata.resolve has the graph.
    let resolve = metadata
        .resolve
        .ok_or_else(|| anyhow::anyhow!("No resolve graph found"))?;

    // Center of screen (approximate, we'll adjust in main)
    let center = Vec2::new(600.0, 100.0);

    // First pass: Create nodes for all packages in the resolve graph
    for node in &resolve.nodes {
        let is_root = workspace_members.contains(&node.id);

        // Mass heuristics:
        // We can't easily get code size without file IO.
        // Let's use dependency count as proxy for mass?
        // Or just 1.0.
        // Let's make roots heavy.
        let mass = if is_root { 10.0 } else { 1.0 };

        // Position: Random cloud around center initially
        let offset_x = rng.gen_range(-100.0..100.0);
        let offset_y = rng.gen_range(0.0..200.0);
        let pos = center + Vec2::new(offset_x, offset_y);

        // Fix the workspace roots?
        // Let's fix the first workspace member found, or all of them?
        // If we fix all, they won't swing.
        // Let's fix ONLY the current crate if possible, or just the first one.
        // "chaos-pendulum" should be the anchor?
        // Or "root" of the repo.
        // Let's fix the one that matches the root package of the workspace.
        // Actually, let's fix ANY node that has no parents?
        // In a dependency graph, roots are dependents. Leaves are dependencies.
        // We want the "Top Level" to be fixed, and dependencies to hang from it.
        // Top level depends on A, B. So A and B hang from Top.
        // So Top is the Anchor.

        let fixed = is_root; // For now fix all workspace members

        let idx = sys.add_node(pos, mass, fixed);
        id_to_index.insert(node.id.clone(), idx);
    }

    // Second pass: Create links
    // In `resolve.nodes`, `dependencies` lists the PackageIds this node depends on.
    // Parent -> Child (Parent hangs Child?)
    // If A depends on B, A needs B.
    // Structurally: A holds B.
    // So if A is fixed (Root), B hangs from A.
    // Correct.

    for node in &resolve.nodes {
        if let Some(&parent_idx) = id_to_index.get(&node.id) {
            for dep_id in &node.dependencies {
                if let Some(&child_idx) = id_to_index.get(dep_id) {
                    // Avoid self-loops (shouldn't exist)
                    if parent_idx == child_idx {
                        continue;
                    }

                    // Link length
                    // Maybe vary by "kind"?
                    let length = 20.0 + rng.gen_range(0.0..10.0);

                    sys.add_link(parent_idx, child_idx, length);

                    // Unfix the child if it was tentatively fixed (unless it's a root we really want fixed)
                    // Actually, if a workspace member depends on another workspace member,
                    // should the dependent be fixed?
                    // Let's say only "true roots" (no incoming edges) are fixed.
                    // But here we are iterating outgoing edges.
                    // We don't know incoming edges easily without full graph.
                    // Let's stick to "workspace members are fixed" for now,
                    // BUT "experiments/chaos-pendulum" depends on "macroquad".
                    // Chaos Pendulum is fixed. Macroquad hangs.

                    // What if I change `fixed` logic:
                    // Only the FIRST workspace member is fixed.
                }
            }
        }
    }

    // Post-processing: Unfix everything except one anchor?
    // Let's find "chaos-pendulum" and fix it.
    // Or just fix the node with highest mass?

    // Let's refine fixed status:
    // Unfix all.
    for node in &mut sys.nodes {
        node.fixed = false;
    }

    // Fix the one with name "chaos-pendulum" or just the first workspace member.
    if let Some(first) = workspace_members.first() {
        if let Some(&idx) = id_to_index.get(first) {
            sys.nodes[idx].fixed = true;
            sys.nodes[idx].pos = center; // Reset position
            sys.nodes[idx].prev_pos = center;
        }
    } else {
        // Fallback
        if !sys.nodes.is_empty() {
            sys.nodes[0].fixed = true;
            sys.nodes[0].pos = center;
        }
    }

    // Also, if graph is disconnected, we might have floating islands.
    // That's fine, they will fall under gravity unless fixed.

    Ok(sys)
}
