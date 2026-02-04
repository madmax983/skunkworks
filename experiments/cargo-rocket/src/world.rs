use crate::physics::{Body, System, Vec2};
use anyhow::{Context, Result};
use cargo_metadata::MetadataCommand;
use rand::Rng;
use ratatui::style::Color;
use std::collections::{HashMap, HashSet, VecDeque};

pub fn load_system() -> Result<System> {
    let metadata = MetadataCommand::new()
        .exec()
        .context("Failed to run cargo metadata")?;

    let resolve = metadata
        .resolve
        .as_ref()
        .context("No resolve graph found")?;
    let root_id = resolve
        .root
        .as_ref()
        .or_else(|| {
            metadata
                .packages
                .iter()
                .find(|p| p.name == "cargo-rocket")
                .map(|p| &p.id)
        })
        .context("No root package found (cargo-rocket)")?;

    let mut system = System::new();
    let mut rng = rand::thread_rng();

    // BFS to determine shells (distance from root)
    let mut depth_map = HashMap::new();
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back((root_id, 0));
    visited.insert(root_id);

    // Also need to look up Package info by ID
    let package_map: HashMap<_, _> = metadata.packages.iter().map(|p| (&p.id, p)).collect();

    while let Some((id, depth)) = queue.pop_front() {
        depth_map.insert(id, depth);

        if let Some(node) = resolve.nodes.iter().find(|n| &n.id == id) {
            for dep in &node.dependencies {
                if visited.insert(dep) {
                    queue.push_back((dep, depth + 1));
                }
            }
        }
    }

    // Now instantiate bodies
    // Count per shell to space them out
    let mut shell_counts = HashMap::new();
    for depth in depth_map.values() {
        *shell_counts.entry(*depth).or_insert(0) += 1;
    }

    let mut shell_indices = HashMap::new();

    for (id, depth) in depth_map {
        let pkg = package_map.get(id).cloned().unwrap();
        let name = pkg.name.clone();

        let count_in_shell = *shell_counts.get(&depth).unwrap_or(&1);
        let index = *shell_indices.get(&depth).unwrap_or(&0);
        shell_indices.insert(depth, index + 1);

        let (pos, vel, mass, radius, color, is_fixed) = if depth == 0 {
            // Sun
            (
                Vec2::zero(),
                Vec2::zero(),
                5000.0,
                15.0,
                Color::Yellow,
                true,
            )
        } else {
            // Planet
            // Radius of orbit based on depth
            let orbit_r = 40.0 * (depth as f64 + 1.0);

            // Angle
            let angle_step = 2.0 * std::f64::consts::PI / (count_in_shell as f64);
            let angle = angle_step * (index as f64) + (depth as f64); // Phase shift per shell

            let pos = Vec2::new(orbit_r * angle.cos(), orbit_r * angle.sin());

            // Circular Orbit Velocity: v = sqrt(G * M / r)
            // But this assumes only Sun gravity. In N-Body, it's approximate.
            // Let's assume Sun dominates.
            let v_mag = (system.g * 5000.0 / orbit_r).sqrt();
            let vel = Vec2::new(-v_mag * angle.sin(), v_mag * angle.cos());

            let mass: f64 = 10.0 + rng.gen_range(0.0..20.0);
            let radius = (mass / 5.0).max(1.0);

            let color = match depth % 6 {
                1 => Color::Cyan,
                2 => Color::Green,
                3 => Color::Magenta,
                4 => Color::Red,
                5 => Color::Blue,
                _ => Color::White,
            };

            (pos, vel, mass, radius, color, false)
        };

        system.bodies.push(Body {
            pos,
            vel,
            acc: Vec2::zero(),
            mass,
            radius,
            name,
            color,
            is_fixed,
        });
    }

    // Place Ship near Sun? Already done in System::new()
    // Let's adjust ship to be at Root (Sun) initially or orbiting close.
    system.ship.pos = Vec2::new(60.0, 0.0); // Orbiting just outside first shell?
    let v_ship = (system.g * 5000.0 / 60.0).sqrt();
    system.ship.vel = Vec2::new(0.0, v_ship);

    Ok(system)
}
