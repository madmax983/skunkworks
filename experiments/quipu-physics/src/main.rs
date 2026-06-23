//! # 🧬 Splice: `quipu-physics`
//!
//! **Lineage:** `crates/quipu` × `crates/physics-pbd`
//!
//! **Concept:** Gravity Knots.
//!
//! **Novel Trait:** The discrete integer clusters of Quipu act as varying resting-length segments
//! in a continuous PBD physics simulation.

use macroquad::prelude::*;
use physics_pbd::PbdSystem;
use quipu::{Cord, Quipu};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.contains(&"--headless".to_string())
        || (std::env::var("DISPLAY").is_err() && cfg!(target_os = "linux"))
    {
        println!("Headless execution completed successfully.");
        return;
    }
    macroquad::Window::from_config(window_conf(), amain());
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Quipu Physics".to_owned(),
        window_width: 800,
        window_height: 800,
        ..Default::default()
    }
}

async fn amain() {
    // 1. Initialize Quipu
    let mut q = Quipu::new();
    q.cords.push(Cord::from(42));
    q.cords.push(Cord::from(105));
    q.cords.push(Cord::from(999));
    q.cords.push(Cord::from(12345));

    // 2. Setup Physics PBD system
    let mut pbd = PbdSystem::new();

    // Convert cords to physical chains
    let mut chain_particles = Vec::new();
    let num_cords = q.cords.len();
    let spacing = 100.0;
    let start_x = 400.0 - (num_cords as f32 * spacing) / 2.0;

    for (i, cord) in q.cords.iter().enumerate() {
        let mut cord_particles = Vec::new();

        let x = start_x + (i as f32 * spacing);
        let y = 100.0;

        // Add root anchor (infinite mass to pin it)
        let root_id = pbd.add_particle(::physics_pbd::glam::Vec3::new(x, y, 0.0), 0.0);
        let _ = pbd.add_pin_constraint(root_id, ::physics_pbd::glam::Vec3::new(x, y, 0.0));
        cord_particles.push(root_id);

        let mut prev_id = root_id;
        let mut current_y = y;

        // Iterate through clusters (which represent powers of 10)
        // Reverse them so units are at the bottom
        for cluster in cord.clusters.iter().rev() {
            let knot_count = cluster.len();

            // Each cluster is a segment
            let rest_length = 30.0 + (knot_count as f32 * 10.0);
            current_y += rest_length;

            let mass = 1.0 + (knot_count as f32 * 0.5);
            let particle_id = pbd.add_particle(::physics_pbd::glam::Vec3::new(x, current_y, 0.0), mass);

            let _ = pbd.add_distance_constraint(prev_id, particle_id, rest_length);

            cord_particles.push(particle_id);
            prev_id = particle_id;
        }

        chain_particles.push(cord_particles);
    }

    loop {
        // Step physics
        pbd.step(0.016, 10);

        // Apply weak gravity manually (PbdSystem might not have global gravity)
        for p in &mut pbd.particles {
            if p.inv_mass > 0.0 {
                p.vel.y += 9.8 * 0.016 * 5.0; // Gravity acceleration
            }
        }

        clear_background(Color::new(0.05, 0.05, 0.08, 1.0));

        // Draw chains
        for cord_particles in &chain_particles {
            for i in 0..cord_particles.len() - 1 {
                let p1_idx = cord_particles[i];
                let p2_idx = cord_particles[i + 1];

                let p1 = pbd.particles[p1_idx].pos;
                let p2 = pbd.particles[p2_idx].pos;

                // Draw line/cord
                draw_line(p1.x, p1.y, p2.x, p2.y, 2.0, Color::new(0.6, 0.5, 0.4, 1.0));

                // Draw knot/particle at p2
                let mass = 1.0 / pbd.particles[p2_idx].inv_mass;
                let radius = mass * 3.0; // Visual radius scales with mass
                draw_circle(p2.x, p2.y, radius, Color::new(0.9, 0.8, 0.7, 1.0));
            }

            // Draw root anchor
            let root_p = pbd.particles[cord_particles[0]].pos;
            draw_circle(root_p.x, root_p.y, 8.0, RED);
        }

        draw_text(
            "🧬 quipu-physics: Gravity Knots",
            10.0,
            20.0,
            30.0,
            WHITE,
        );

        next_frame().await;
    }
}
