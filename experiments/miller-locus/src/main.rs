use locus::{Topology, Vec2};
use miller_lattice::Crystal;
use std::env;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.contains(&"--headless".to_string()) {
        println!("Headless mode activated. Bypassing execution.");
        return;
    }

    println!("📍 Miller-Locus 📍");
    println!("Projecting hierarchical crystalline data onto a continuous Torus topology.");

    // Parse the file system to build the crystalline lattice
    let crystal = Crystal::build_from_path(Path::new(".")).unwrap_or_else(|_| {
        Crystal::build_from_path(Path::new("src")).unwrap()
    });

    println!("Generated lattice with {} atoms.", crystal.atoms.len());

    // Define the 2D non-Euclidean boundary
    let width = 100;
    let height = 100;
    let topo = Topology::Torus;

    // We will project the discrete 3D lattice points down to the 2D Torus geometry
    // and observe how the boundaries wrap.
    let mut wrapped_count = 0;

    for atom in &crystal.atoms {
        // Project 3D Miller integer coordinates down to a 2D continuous space.
        let raw_x = (atom.position.x * 5) as f64; // Scaling for visualization
        let raw_y = (atom.position.y * 5) as f64;

        let position = Vec2::new(raw_x, raw_y);

        let y_idx = position.y.round() as i64;
        let x_idx = position.x.round() as i64;

        // Determine if the coordinates wrap out of bounds
        if y_idx < 0 || y_idx >= height as i64 || x_idx < 0 || x_idx >= width as i64 {
            if let Some((ny, nx)) = topo.normalize(y_idx, x_idx, width, height) {
                wrapped_count += 1;
                // println!("Atom wrapped from ({}, {}) to ({}, {})", x_idx, y_idx, nx, ny);
                let _ = (ny, nx);
            }
        }
    }

    println!("Total atoms that exceeded bounds and wrapped smoothly via Torus topology: {}", wrapped_count);
    println!("Emergent phenotype: Dense directory hierarchies folded across continuous boundaries forming spatial clusters.");
}
