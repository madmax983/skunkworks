use locus::Topology;
use origami::{generate_miura_mesh, MiuraParams, Orientation};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let headless = args.contains(&"--headless".to_string());

    if !headless {
        println!("Running locus-origami interactively...");
    } else {
        println!("Running locus-origami in headless mode (CI bypass).");
    }

    let params = MiuraParams {
        a: 2.0,
        b: 2.0,
        gamma: 1.4,
        orientation: Orientation::Horizontal,
    };

    let grid_width = 10;
    let grid_height = 10;

    // Topology dimension for boundaries
    let width = 20;
    let height = 20;

    let topo = Topology::Torus;

    println!("Generating Miura-ori mesh and draping it across a Torus topology...");

    let mut mesh = generate_miura_mesh(params, (grid_width, grid_height), 0.5);

    // Output stats
    println!("Generated mesh with {} vertices.", mesh.vertices.len());

    // Simulate mapping mesh vertices into the topology grid
    let mut out_of_bounds = 0;
    let mut wrapped = 0;

    for vertex in mesh.vertices.iter_mut() {
        // Map world space coordinate back to grid coordinate
        let grid_x = vertex.pos[0].round() as i64;
        let grid_y = vertex.pos[1].round() as i64;

        // Offset everything to test boundary conditions
        let shifted_x = grid_x + 15;
        let shifted_y = grid_y + 15;

        // Apply topological normalization. Note locus takes (y, x).
        if let Some((ny, nx)) = topo.normalize(shifted_y, shifted_x, height, width) {
            if nx as i64 != shifted_x || ny as i64 != shifted_y {
                wrapped += 1;
            }
            vertex.pos[0] = nx as f32;
            vertex.pos[1] = ny as f32;
        } else {
            out_of_bounds += 1;
        }
    }

    println!("Simulation complete.");
    println!("Wrapped vertices across topology: {}", wrapped);
    println!(
        "Vertices lost out of bounds (should be 0 for Torus): {}",
        out_of_bounds
    );

    if headless {
        println!("Headless simulation finished successfully.");
    }
}
