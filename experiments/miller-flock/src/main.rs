//! # miller-flock 🕊️💎
//!
//! ## Lineage
//!
//! **Parents**: `crates/miller-lattice` × `crates/flocking`
//!
//! This experiment breeds the discrete hierarchical crystal structures of `miller-lattice` with the continuous swarm intelligence of `flocking`.
//!
//! ### Genetic Traits
//!
//! *   **From `miller-lattice`**: The structural topography of the repository. Directories and files are grown into a deterministic 3D crystal lattice using Miller indices.
//! *   **From `flocking`**: Craig Reynolds' Boids algorithm, simulating emergent swarm behavior (separation, alignment, cohesion).
//!
//! ### Emergent Phenotype
//!
//! Swarm-driven structural navigation. The discrete crystal atoms (representing files/directories) act as cohesion attractors or obstacles for the continuous flocking simulation. The swarm dynamically explores the architecture of the codebase, clustering around dense structural nodes.
//!
use flocking::{compute_force, FlockingParams};
use macroquad::prelude::*;
use miller_lattice::Crystal;
use std::env;
use std::path::Path;

const WIDTH: f64 = 800.0;
const HEIGHT: f64 = 600.0;

fn window_conf() -> Conf {
    Conf {
        window_title: "miller-flock".to_owned(),
        window_width: WIDTH as i32,
        window_height: HEIGHT as i32,
        ..Default::default()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.iter().any(|arg| arg == "--headless") {
        println!("Headless mode: exiting early to prevent X11 panics or execution timeouts.");
        return;
    }
    macroquad::Window::from_config(window_conf(), async_main());
}

async fn async_main() {
    // Generate the Miller lattice crystal from the current directory
    let crystal = Crystal::build_from_path(Path::new(".")).unwrap_or_else(|_| Crystal {
        atoms: vec![],
        bonds: vec![],
        lookup: Default::default(),
    });

    let mut nodes_2d = Vec::new();
    let cx = WIDTH as f32 / 2.0;
    let cy = HEIGHT as f32 / 2.0;

    // Flatten crystal to 2D screen coordinates and find bounds
    let mut min_x = f32::MAX;
    let mut max_x = f32::MIN;
    let mut min_y = f32::MAX;
    let mut max_y = f32::MIN;

    for atom in &crystal.atoms {
        let px = atom.position.x as f32;
        let py = atom.position.y as f32;
        min_x = min_x.min(px);
        max_x = max_x.max(px);
        min_y = min_y.min(py);
        max_y = max_y.max(py);
    }

    let span_x = max_x - min_x;
    let span_y = max_y - min_y;
    let scale = (WIDTH as f32 * 0.8) / span_x.max(span_y).max(1.0);

    for atom in &crystal.atoms {
        let px = (atom.position.x as f32 - min_x - span_x / 2.0) * scale + cx;
        let py = (atom.position.y as f32 - min_y - span_y / 2.0) * scale + cy;
        nodes_2d.push(locus::Vec2::new(px as f64, py as f64));
    }

    let num_boids = 150;
    let mut boid_positions = vec![];
    let mut boid_velocities = vec![];

    for _ in 0..num_boids {
        let px = rand::gen_range(0.0, WIDTH as f64);
        let py = rand::gen_range(0.0, HEIGHT as f64);
        boid_positions.push(locus::Vec2::new(px, py));

        let vx = rand::gen_range(-1.0, 1.0);
        let vy = rand::gen_range(-1.0, 1.0);
        boid_velocities.push(locus::Vec2::new(vx, vy));
    }

    let params = FlockingParams {
        view_radius: 50.0,
        separation_radius: 20.0,
        max_speed: 3.0,
        max_force: 0.1,
        separation_weight: 1.5,
        alignment_weight: 1.0,
        cohesion_weight: 1.0,
    };

    loop {
        clear_background(BLACK);

        // Draw crystal lattice
        for bond in &crystal.bonds {
            let p1 = nodes_2d[bond.0];
            let p2 = nodes_2d[bond.1];
            draw_line(
                p1.x as f32,
                p1.y as f32,
                p2.x as f32,
                p2.y as f32,
                1.0,
                Color::new(0.0, 0.5, 0.8, 0.3),
            );
        }
        for node in &nodes_2d {
            draw_circle(
                node.x as f32,
                node.y as f32,
                2.0,
                Color::new(0.0, 0.8, 1.0, 0.6),
            );
        }

        // Boids logic
        let mut new_velocities = boid_velocities.clone();
        for i in 0..num_boids {
            let flock_force = compute_force(&boid_positions, &boid_velocities, i, &params);

            // Attract towards nearest crystal node
            let mut nearest_dist = f64::MAX;
            let mut node_force = locus::Vec2::new(0.0, 0.0);

            for node in &nodes_2d {
                let dist = boid_positions[i].distance(*node);
                if dist < nearest_dist && dist < 150.0 {
                    nearest_dist = dist;
                    let mut dir = *node - boid_positions[i];
                    let mag = (dir.x * dir.x + dir.y * dir.y).sqrt();
                    if mag > 0.0001 {
                        dir = locus::Vec2::new(dir.x / mag, dir.y / mag);
                    } else {
                        dir = locus::Vec2::new(0.0, 0.0);
                    }
                    // Stronger attraction the closer they are, simulating foraging/reading
                    node_force = dir * (150.0 - dist) * 0.0005;
                }
            }

            new_velocities[i] += flock_force + node_force;
            new_velocities[i] = new_velocities[i].limit(params.max_speed);
        }

        boid_velocities = new_velocities;

        for i in 0..num_boids {
            boid_positions[i] += boid_velocities[i];

            // Wrap around edges
            if boid_positions[i].x < 0.0 {
                boid_positions[i].x = WIDTH;
            }
            if boid_positions[i].x > WIDTH {
                boid_positions[i].x = 0.0;
            }
            if boid_positions[i].y < 0.0 {
                boid_positions[i].y = HEIGHT;
            }
            if boid_positions[i].y > HEIGHT {
                boid_positions[i].y = 0.0;
            }

            // Draw boid
            let mut dir = boid_velocities[i];
            let mag = (dir.x * dir.x + dir.y * dir.y).sqrt();
            if mag > 0.0001 {
                dir = locus::Vec2::new(dir.x / mag, dir.y / mag);
            } else {
                dir = locus::Vec2::new(0.0, 0.0);
            }
            let p1 = boid_positions[i] + dir * 6.0;
            let p2 = boid_positions[i] - dir * 4.0 + locus::Vec2::new(-dir.y, dir.x) * 3.0;
            let p3 = boid_positions[i] - dir * 4.0 - locus::Vec2::new(-dir.y, dir.x) * 3.0;

            draw_triangle(
                vec2(p1.x as f32, p1.y as f32),
                vec2(p2.x as f32, p2.y as f32),
                vec2(p3.x as f32, p3.y as f32),
                Color::new(1.0, 0.4, 0.4, 0.8),
            );
        }

        next_frame().await;
    }
}
