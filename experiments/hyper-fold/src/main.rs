use macroquad::prelude::*;
use origami::{MiuraOri, MiuraParams, Orientation};
use hyper_system::math::Vec4;

mod physics;
mod dna;

use physics::PbdSystem4D;
use dna::ChimeraAgent;

#[macroquad::main("Hyper-Fold")]
async fn main() {
    // --- Initialization ---

    // 1. Origami Setup
    let params = MiuraParams {
        a: 1.0,
        b: 1.0,
        gamma: 70.0f32.to_radians(),
        orientation: Orientation::Horizontal,
    };
    let grid_cols = 10;
    let grid_rows = 10;
    let origami = MiuraOri::new(params, (grid_cols, grid_rows));

    // Generate initial 3D flat(tish) mesh
    // expansion 0.5 to allow movement
    let initial_positions_3d = origami.generate_grid(0.8);
    let mesh_structure = origami.generate_mesh(0.8);
    let indices = mesh_structure.indices;

    // 2. Physics Setup
    let mut system = PbdSystem4D::new((grid_cols, grid_rows));

    // Create particles from 3D positions, set W=0
    // Center the mesh
    for pos3 in initial_positions_3d {
        // origami pos is Vec3
        let p = Vec4::new(pos3.x, pos3.y, pos3.z, 0.0);
        system.add_particle(p, 1.0);
    }

    // Add constraints based on grid connectivity
    // Horizontal edges
    let width = grid_cols + 1;
    let height = grid_rows + 1;
    let stiffness = 0.5;

    for y in 0..height {
        for x in 0..width {
            let idx = y * width + x;

            // Horizontal neighbor
            if x + 1 < width {
                let neighbor = idx + 1;
                system.add_distance_constraint(idx, neighbor, stiffness);
            }

            // Vertical neighbor
            if y + 1 < height {
                let neighbor = idx + width;
                system.add_distance_constraint(idx, neighbor, stiffness);
            }

            // Diagonal (shear resistance for Miura)?
            // Actually Miura relies on panel rigidity. PBD distance constraints behave like trusses.
            // Let's add diagonal constraints to simulate rigid panels (Quads)
            if x + 1 < width && y + 1 < height {
                let p00 = idx;
                let p10 = idx + 1;
                let p01 = idx + width;
                let p11 = idx + width + 1;

                // Cross diagonals
                system.add_distance_constraint(p00, p11, stiffness * 0.5);
                system.add_distance_constraint(p10, p01, stiffness * 0.5);
            }
        }
    }

    // Pin corners to prevent flying away?
    // Pin (0,0) and (cols, 0)
    // Actually let it float, or add gravity?
    // Let's pin the center to keep it in view
    let center_idx = (grid_rows / 2) * width + (grid_cols / 2);
    let center_pos = system.particles[center_idx].pos;
    system.add_pin_constraint(center_idx, center_pos);

    // 3. DNA Setup
    let mut agents = Vec::new();
    for i in 0..system.particles.len() {
        agents.push(ChimeraAgent::new(i, i as u64));
    }

    // --- Main Loop ---

    let mut camera_w = 15.0;
    let mut cam_angle_x = 0.0f32;
    let mut cam_angle_y = 0.0f32;

    let mut last_mouse = mouse_position();

    loop {
        let mouse_pos = mouse_position();
        let delta = vec2(mouse_pos.0 - last_mouse.0, mouse_pos.1 - last_mouse.1);
        last_mouse = mouse_pos;

        // Input
        if is_mouse_button_down(MouseButton::Left) {
            cam_angle_y -= delta.x * 0.01;
            cam_angle_x -= delta.y * 0.01;
        }

        let dt = 0.016; // Fixed step

        // Genetics Update (Throttled?)
        // Run every frame for smoothness, or every N frames
        for agent in &mut agents {
            let polarity = agent.update(&system.particles, (grid_cols, grid_rows));
            // Apply to particle
            system.particles[agent.index].magnetic_polarity = polarity;
        }

        // Physics Step
        // Rotate 4D space slightly? Or just simulate
        // Let's add a global "System Load" disturbance?
        // For now, just pure physics
        system.step(dt, 5); // 5 sub-steps

        // Rendering
        clear_background(BLACK);

        // 3D Camera Setup
        set_camera(&Camera3D {
            position: vec3(
                20.0 * cam_angle_y.sin(),
                10.0 + 10.0 * cam_angle_x.sin(),
                20.0 * cam_angle_y.cos()
            ),
            target: vec3(0.0, 0.0, 0.0),
            up: vec3(0.0, 1.0, 0.0),
            ..Default::default()
        });

        // Project particles to 3D
        let mut projected = Vec::with_capacity(system.particles.len());
        for p in &system.particles {
            let proj: Vec3 = p.pos.project_to_3d(camera_w).into();
            projected.push(proj);
        }

        // Draw Mesh
        // Using indices from origami
        for i in (0..indices.len()).step_by(3) {
            let i0 = indices[i] as usize;
            let i1 = indices[i+1] as usize;
            let i2 = indices[i+2] as usize;

            let v0 = projected[i0];
            let v1 = projected[i1];
            let v2 = projected[i2];

            // Color based on W-depth of vertices
            // Avg W
            let w0 = system.particles[i0].pos.w;
            let w1 = system.particles[i1].pos.w;
            let w2 = system.particles[i2].pos.w;
            let avg_w = (w0 + w1 + w2) / 3.0;

            // Map W to Color:
            // W near 0 (Flat) = Green
            // W > 0 (Away) = Blue
            // W < 0 (Close/In) = Red

            let t = (avg_w / 5.0).clamp(-1.0, 1.0);
            let color = if t > 0.0 {
                Color::new(0.0, 1.0 - t, 1.0, 0.8) // Green -> Blue
            } else {
                Color::new(1.0, 1.0 + t, 0.0, 0.8) // Green -> Red
            };

            // Manual backface culling or just draw 3D triangles if macroquad supports it?
            // Macroquad doesn't have a simple draw_triangle_3d in prelude.
            // We need to use draw_mesh or just lines.
            // Let's use lines for wireframe and maybe small spheres for vertices?
            // Actually, draw_line_3d is reliable.
            // Let's stick to wireframe for now to ensure compilation.

            draw_line_3d(v0, v1, color);
            draw_line_3d(v1, v2, color);
            draw_line_3d(v2, v0, color);
        }

        // UI Overlay
        set_default_camera();
        draw_text("HYPER-FOLD", 10.0, 30.0, 30.0, WHITE);
        draw_text(&format!("Vertices: {}", system.particles.len()), 10.0, 50.0, 20.0, LIGHTGRAY);
        draw_text("Drag to Rotate", 10.0, 70.0, 20.0, LIGHTGRAY);

        next_frame().await
    }
}
