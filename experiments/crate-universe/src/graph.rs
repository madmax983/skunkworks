use crate::physics::{Body, System, G};
use anyhow::Result;
use cargo_metadata::{MetadataCommand, PackageId};
use glam::DVec2;
use std::collections::HashMap;

pub fn load_workspace() -> Result<System> {
    let metadata = MetadataCommand::new().exec()?;

    let mut system = System::new();
    let mut package_masses: HashMap<PackageId, f64> = HashMap::new();
    let mut reverse_deps: HashMap<PackageId, usize> = HashMap::new();

    // Better approach: Use the resolve graph
    let resolve = metadata
        .resolve
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("No resolve graph found"))?;

    // Count incoming edges
    for node in &resolve.nodes {
        reverse_deps.entry(node.id.clone()).or_insert(0); // Ensure entry
        for dep in &node.dependencies {
            *reverse_deps.entry(dep.clone()).or_insert(0) += 1;
        }
    }

    // Assign masses
    for node in &resolve.nodes {
        let count = reverse_deps.get(&node.id).unwrap_or(&0);
        // Mass = 10.0 for existence + 5.0 per reverse dep
        let mass = 10.0 + (*count as f64) * 20.0;
        package_masses.insert(node.id.clone(), mass);
    }

    // 2. Create Bodies
    // Sort by mass descending to place heavy stars first
    let mut sorted_nodes = resolve.nodes.clone();
    sorted_nodes.sort_by(|a, b| {
        let mass_a = package_masses.get(&a.id).unwrap();
        let mass_b = package_masses.get(&b.id).unwrap();
        mass_b.partial_cmp(mass_a).unwrap()
    });

    let mut bodies = Vec::new();
    let mut id_to_index: HashMap<PackageId, usize> = HashMap::new();

    // Place the heaviest one at center (approx)
    // Actually, let's just use the spiral for everything, but massive ones are closer.

    for (i, node) in sorted_nodes.iter().enumerate() {
        let pkg = metadata.packages.iter().find(|p| p.id == node.id).unwrap();
        let mass = *package_masses.get(&node.id).unwrap();

        let pos;
        let vel;

        if i == 0 {
            // Supermassive black hole / Central Star
            pos = DVec2::ZERO;
            vel = DVec2::ZERO; // Fixed? Or just heavy?
                               // Let's make it fixed if it's really huge, or just heavy.
                               // Let's make the center fixed to anchor the simulation?
                               // "Forbidden: Euler integration (energy will drift!)" -> implies we want a stable system.
                               // A fixed center helps stability.
        } else {
            // Spiral distribution
            // Angle
            let angle = i as f64 * 0.5; // Radians
                                        // Radius: spread out based on index (rank)
                                        // r = c * sqrt(i) is good for uniform density
            let r = 50.0 + (i as f64).sqrt() * 30.0;

            pos = DVec2::new(r * angle.cos(), r * angle.sin());

            // Tangential velocity for circular orbit around (0,0)
            // Assuming center mass dominates?
            // v = sqrt(G * M_enclosed / r)
            // Approximate M_enclosed as the central mass (if it's huge)
            let center_mass = *package_masses.get(&sorted_nodes[0].id).unwrap();

            // Softening
            let accel_mag = G * center_mass / (r * r + 100.0); // Softening 100
            let v_mag = (accel_mag * r).sqrt();

            // Perpendicular direction (-sin, cos)
            let v_dir = DVec2::new(-angle.sin(), angle.cos());
            vel = v_dir * v_mag;
        }

        let is_fixed = i == 0; // Fix the heaviest crate

        let body = Body {
            pos,
            vel,
            force: DVec2::ZERO,
            mass,
            radius: (mass.sqrt()).max(1.0),
            name: pkg.name.clone(),
            is_fixed,
        };

        bodies.push(body);
        id_to_index.insert(node.id.clone(), i);
    }

    system.bodies = bodies;

    // Total bodies
    println!("Loaded {} crates as bodies.", system.bodies.len());

    Ok(system)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_workspace() {
        // This might fail if run in a restricted environment without cargo access?
        // But we are in the repo, so it should work.
        let system = load_workspace();
        match system {
            Ok(sys) => {
                println!("Loaded system with {} bodies", sys.bodies.len());
                assert!(!sys.bodies.is_empty());
            }
            Err(e) => {
                println!("Failed to load workspace: {:?}", e);
                // In CI, cargo metadata might fail if no Cargo.lock or internet?
                // But we are in a monorepo with Cargo.lock.
                panic!("Failed to load workspace: {:?}", e);
            }
        }
    }
}
