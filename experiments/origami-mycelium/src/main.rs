//! # Origami Mycelium 🍄📜
//!
//! **Lineage:** `crates/origami` × `experiments/myco-transit`
//!
//! "The paper remembers the paths we walk."
//!
//! ## Concept
//! This experiment combines **deployable origami structures** (Miura-ori) with **slime mold pathfinding** (Physarum polycephalum).
//!
//! The surface of the paper serves as a foraging ground for thousands of slime mold agents. As the agents traverse the paper and deposit pheromones, the chemical concentration physically contracts the paper's creases.
//!
//! ## Novel Trait
//! **Pheromone-Guided Folding:** The geometry of the origami mesh folds dynamically in response to biological intent. The paths the slime mold uses to commute between nodes cause the paper to fold and compress along those exact highways, creating a physical manifestation of the biological network.
//!
//! ## Controls
//! - **Mouse Drag**: Orbit Camera
//! - **Scroll**: Zoom in/out
//!
//! ## Tech Stack
//! - `origami`: Miura-ori mesh generation.
//! - Position Based Dynamics (PBD) for soft-body mesh simulation.
//! - `rayon` for parallel agent updates.
//! - `macroquad` for 3D rendering.
mod mesh_gen;
mod pbd;
mod simulation;

use macroquad::models::{Mesh, Vertex};
use macroquad::prelude::*;
use mesh_gen::generate_miura_ori;
use pbd::Constraint;
use simulation::World;

#[macroquad::main("Origami Mycelium")]
async fn main() {
    let rows = 15;
    let cols = 15;
    let mesh_data = generate_miura_ori(rows, cols);
    let mut system = mesh_data.system;
    let indices = mesh_data.indices;

    // Simulation grid sizes
    let grid_width = cols * 10;
    let grid_height = rows * 10;

    // Create World (Pheromones) and Agents
    // We only need a single target/home logic for agents, or we can just let them wander
    // Let's create multiple "cities" as foraging points to encourage trails
    let (mut world, mut agents) = World::with_cities_and_agents(grid_width, grid_height, 4);

    // Camera state
    let mut cam_yaw: f32 = 0.0;
    let mut cam_pitch: f32 = 0.5;
    let mut cam_dist: f32 = 35.0;

    let mut last_mouse_pos = mouse_position();

    loop {
        let dt = 0.016; // Fixed step

        // Input
        let mouse_pos = mouse_position();
        let delta = vec2(
            mouse_pos.0 - last_mouse_pos.0,
            mouse_pos.1 - last_mouse_pos.1,
        );
        last_mouse_pos = mouse_pos;

        if is_mouse_button_down(MouseButton::Left) {
            cam_yaw -= delta.x * 0.01;
            cam_pitch += delta.y * 0.01;
            cam_pitch = cam_pitch.clamp(-1.5, 1.5);
        }

        let wheel = mouse_wheel().1;
        cam_dist -= wheel * 0.1 * cam_dist;
        cam_dist = cam_dist.clamp(5.0, 150.0);

        // Update Slime Mold
        world.update_agents_parallel(&mut agents);
        world.diffuse_and_decay();

        // Map pheromone trail to mesh vertices/actuators
        // We evaluate pheromone concentration at each vertex by projecting its (u, v) into the grid
        let mut vertex_pheromones = vec![0.0; system.particles.len()];

        for (idx, _particle) in system.particles.iter().enumerate() {
            // Reconstruct approximate (i, j) based on how mesh is built in mesh_gen
            // Width = cols + 1
            let width = cols + 1;
            let j = idx / width;
            let i = idx % width;

            // Map (i,j) to grid (x,y)
            let x = (i as f32 / cols as f32 * grid_width as f32) as usize;
            let y = (j as f32 / rows as f32 * grid_height as f32) as usize;

            let x = x.clamp(0, grid_width - 1);
            let y = y.clamp(0, grid_height - 1);

            let pheromone = world.get_trail(x, y);
            vertex_pheromones[idx] = pheromone as f32; // 0.0..255.0
        }

        // Apply Fold / Constrain actuators based on pheromones
        for constraint in &mut system.constraints {
            if let Constraint::Actuator {
                p1,
                p2,
                ref mut factor,
                ..
            } = constraint
            {
                let ph1 = vertex_pheromones[*p1];
                let ph2 = vertex_pheromones[*p2];

                // High pheromone -> contract (factor -> 0)
                // Low pheromone -> relax (factor -> 1)
                let avg_ph = (ph1 + ph2) * 0.5;
                let normalized_ph = (avg_ph / 255.0).clamp(0.0, 1.0);

                // fold target 0.0 = fully folded, 1.0 = flat
                let target_fold = 1.0 - normalized_ph * 0.8;

                *factor = *factor * 0.95 + target_fold * 0.05; // smooth transition
            }
        }

        // Physics Step
        system.step(dt, 10);

        // Rendering
        clear_background(BLACK);

        let cam_pos = vec3(
            cam_yaw.cos() * cam_pitch.cos() * cam_dist,
            cam_pitch.sin() * cam_dist,
            cam_yaw.sin() * cam_pitch.cos() * cam_dist,
        );

        set_camera(&Camera3D {
            position: cam_pos,
            target: vec3(0.0, 0.0, 0.0),
            up: vec3(0.0, 1.0, 0.0),
            ..Default::default()
        });

        // Draw Mesh
        let mut mesh = Mesh {
            vertices: Vec::new(),
            indices: Vec::new(),
            texture: None,
        };

        for i in (0..indices.len()).step_by(3) {
            let idx0 = indices[i] as usize;
            let idx1 = indices[i + 1] as usize;
            let idx2 = indices[i + 2] as usize;

            let v0 = system.particles[idx0].pos;
            let v1 = system.particles[idx1].pos;
            let v2 = system.particles[idx2].pos;

            let normal = (v1 - v0).cross(v2 - v0).normalize_or_zero();
            let light_dir = vec3(0.5, 1.0, 0.5).normalize();
            let intensity = normal.dot(light_dir).abs() * 0.7 + 0.3;

            let ph0 = vertex_pheromones[idx0] / 255.0;
            let ph1 = vertex_pheromones[idx1] / 255.0;
            let ph2 = vertex_pheromones[idx2] / 255.0;
            let avg_ph = (ph0 + ph1 + ph2) / 3.0;

            // Color based on pheromone (Slime mold is yellowish/greenish)
            let r = 0.8 + avg_ph * 0.2;
            let g = 0.8 + avg_ph * 0.2;
            let b = 0.8 - avg_ph * 0.8;

            let color = Color::new(r * intensity, g * intensity, b * intensity, 1.0);
            let color_bytes: [u8; 4] = color.into();

            let start_idx = mesh.vertices.len() as u16;
            let normal_v4 = vec4(normal.x, normal.y, normal.z, 1.0);

            mesh.vertices.push(Vertex {
                position: v0,
                uv: vec2(0., 0.),
                color: color_bytes,
                normal: normal_v4,
            });
            mesh.vertices.push(Vertex {
                position: v1,
                uv: vec2(0., 0.),
                color: color_bytes,
                normal: normal_v4,
            });
            mesh.vertices.push(Vertex {
                position: v2,
                uv: vec2(0., 0.),
                color: color_bytes,
                normal: normal_v4,
            });

            mesh.indices.push(start_idx);
            mesh.indices.push(start_idx + 1);
            mesh.indices.push(start_idx + 2);
        }

        draw_mesh(&mesh);

        set_default_camera();
        draw_text("Origami Mycelium", 10.0, 30.0, 30.0, WHITE);
        draw_text(
            "Slime Mold Pheromones guide Miura-ori creasing.",
            10.0,
            60.0,
            20.0,
            GRAY,
        );

        next_frame().await
    }
}
