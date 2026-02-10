mod pbd;
mod neuron;
mod network;
mod boid;

use macroquad::prelude::*;
use boid::OrigamiBoid;

#[macroquad::main("Origami Swarm")]
async fn main() {
    let mut boids: Vec<OrigamiBoid> = Vec::new();
    let num_boids = 50;

    // Spawn in a cluster
    for i in 0..num_boids {
        let pos = vec3(
            rand::gen_range(-20.0, 20.0),
            rand::gen_range(-20.0, 20.0),
            rand::gen_range(-20.0, 20.0),
        );
        boids.push(OrigamiBoid::new(i, pos));
    }

    loop {
        clear_background(BLACK);

        // Camera
        set_camera(&Camera3D {
            position: vec3(0.0, -80.0, 40.0),
            target: vec3(0.0, 0.0, 0.0),
            up: vec3(0.0, 0.0, 1.0),
            ..Default::default()
        });

        draw_grid(20, 5.0, Color::new(0.2, 0.2, 0.2, 1.0), GRAY);

        // Update Flocking Logic (O(N^2))
        // We need to clone state for safe read-write separation
        // Or calculate forces first, then apply.

        let positions: Vec<Vec3> = boids.iter().map(|b| b.center_of_mass).collect();
        let velocities: Vec<Vec3> = boids.iter().map(|b| b.velocity).collect();
        let phases: Vec<f32> = boids.iter().map(|b| b.phase).collect();

        // Bounds
        let bounds_min = vec3(-50.0, -50.0, -50.0);
        let bounds_max = vec3(50.0, 50.0, 50.0);

        // Parallel update if rayon is used, or sequential for simplicity.
        // Let's do sequential first to ensure correctness.
        for i in 0..boids.len() {
            boids[i].update_flocking(&positions, &velocities, &phases, bounds_min, bounds_max);
        }

        // Update Internal Logic (Brain, Physics)
        for boid in &mut boids {
            boid.update_internal(0.016);
        }

        // Draw
        for boid in &boids {
            // Visualize boid
            let pos = boid.center_of_mass;

            // Draw physics mesh
            // Need to reconstruct mesh from particles
            // boid.draw() handles this inside

            // We need to draw mesh in world space
            // boid.draw() uses draw_mesh which uses global transform?
            // No, macroquad meshes use vertex positions.
            // boid.system.particles store world positions.

            // So calling draw_mesh inside boid.draw() is correct.
            // Wait, macroquad `draw_mesh`?
            // `draw_mesh` takes a reference to a Mesh.

            // Let's check boid.rs implementation of draw().
            // It creates a new mesh every frame. This is slow but fine for 50 boids.

            // We need to call boid.draw() here.
            // But boid.draw() is defined in boid.rs.
            // However, boid.rs uses `draw_mesh(&mesh)`.
            // Does macroquad handle multiple meshes per frame? Yes.

            // One issue: boid.rs uses `draw_mesh`.
            // main.rs needs to call it.

            // But `boid.rs` struct definition:
            /*
            pub fn draw(&self) {
                let mut mesh = Mesh { ... };
                ...
                draw_mesh(&mesh);
            }
            */
            // So we just call it.
        }

        // Since we can't call boid.draw() if we are borrowing boids mutably?
        // Wait, loop `for boid in &mut boids` ended.
        // Now `for boid in &boids`.

        for boid in &boids {
            // Manual draw implementation here because we can't easily export `draw` if it depends on macroquad
            // Actually `boid.rs` has `use macroquad::prelude::*`.
            // So `boid.draw()` should work if `draw_mesh` is available.

            // However, creating a Mesh every frame involves allocation.
            // For 50 boids * 4 vertices, it's negligible.

            // Wait, I need to make sure `boid.draw()` is public. It is.

            // But I cannot see `boid.draw` in the file `boid.rs` I wrote?
            // Let me check what I wrote to `boid.rs`.
            /*
            pub fn draw(&self) {
               let mut mesh = Mesh { ... };
               ...
               draw_mesh(&mesh);
            }
            */
            // Yes, I wrote it.
             boid.draw();
        }

        set_default_camera();

        draw_text("ORIGAMI SWARM", 10.0, 30.0, 30.0, WHITE);
        draw_text(format!("Boids: {}", num_boids).as_str(), 10.0, 50.0, 20.0, GRAY);

        // Sync Visualizer
        let mut sum_sin = 0.0;
        let mut sum_cos = 0.0;
        for p in &phases {
            sum_sin += (p * std::f32::consts::TAU).sin();
            sum_cos += (p * std::f32::consts::TAU).cos();
        }
        let r = ((sum_sin / num_boids as f32).powi(2) + (sum_cos / num_boids as f32).powi(2)).sqrt();
        draw_text(format!("Synchronization: {:.3}", r).as_str(), 10.0, 70.0, 20.0, if r > 0.8 { GREEN } else { RED });

        next_frame().await
    }
}
