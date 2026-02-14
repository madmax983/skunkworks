mod grid;
mod ant;

use macroquad::prelude::*;
use crate::grid::MiuraGrid;
use crate::ant::{Colony, AntState};

#[macroquad::main("Folded Colony")]
async fn main() {
    let grid = MiuraGrid::new(20, 20);
    let mut colony = Colony::new(20, 20, 200);

    // Camera
    let mut cam_yaw: f32 = 0.0;
    let mut cam_pitch: f32 = 0.5;
    let mut cam_dist: f32 = 40.0;
    let mut last_mouse_pos = mouse_position();

    // Expansion
    let mut expansion = 0.5;

    loop {
        let mut expansion_speed = 0.0;
        let dt = get_frame_time();

        // Input: Camera
        if is_mouse_button_down(MouseButton::Left) {
            let mouse_pos = mouse_position();
            let delta = vec2(
                mouse_pos.0 - last_mouse_pos.0,
                mouse_pos.1 - last_mouse_pos.1,
            );
            cam_yaw -= delta.x * 0.01;
            cam_pitch += delta.y * 0.01;
            cam_pitch = cam_pitch.clamp(-1.5, 1.5);
        }
        last_mouse_pos = mouse_position();

        let wheel = mouse_wheel().1;
        cam_dist -= wheel * 0.1 * cam_dist;
        cam_dist = cam_dist.clamp(5.0, 150.0);

        // Input: Expansion
        if is_key_down(KeyCode::Right) {
            expansion_speed = 0.5;
        } else if is_key_down(KeyCode::Left) {
            expansion_speed = -0.5;
        } else {
            expansion_speed = 0.0;
        }

        // Auto-oscillate slightly to encourage wormholes?
        // No, let user control.

        expansion += expansion_speed * dt;
        expansion = expansion.clamp(0.05, 1.0);

        // Update Grid Vertices
        let vertices = grid.get_vertices(expansion);

        // Update Colony
        colony.update(&grid, &vertices, dt);

        // Render
        clear_background(Color::new(0.05, 0.05, 0.1, 1.0));

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
        // We'll draw wireframe + faces
        // Faces color based on pheromones?
        // Or just draw ants.

        let width = grid.cols + 1;

        // Build Mesh
        let mut mesh = Mesh {
            vertices: Vec::new(),
            indices: Vec::new(),
            texture: None,
        };

        for j in 0..grid.rows {
            for i in 0..grid.cols {
                let idx0 = j * width + i;
                let idx1 = j * width + i + 1;
                let idx2 = (j + 1) * width + i + 1;
                let idx3 = (j + 1) * width + i;

                let v0 = vertices[idx0];
                let v1 = vertices[idx1];
                let v2 = vertices[idx2];
                let v3 = vertices[idx3];

                // Check pheromone level at v0 (approx)
                let p = colony.pheromones[idx0];

                // Base color
                let mut color = Color::new(0.2, 0.2, 0.3, 1.0);
                if p > 0.0 {
                    // Green tint for pheromones
                    color.r = (color.r + p * 0.8).min(1.0);
                    color.g = (color.g + p * 0.8).min(1.0);
                }

                let color_bytes: [u8; 4] = color.into();

                // Simple normal calculation (flat shading)
                let normal = (v1 - v0).cross(v2 - v0).normalize();
                let normal_v4 = vec4(normal.x, normal.y, normal.z, 1.0); // Macroquad Vertex expects Vec4 for normal? Usually Vec3 but let's check memory.
                // Memory says: `experiments/type-terrain` the `macroquad` `Vertex` struct requires a `normal` field of type `Vec4`.

                let base_idx = mesh.vertices.len() as u16;

                mesh.vertices.push(Vertex { position: v0, uv: vec2(0.,0.), color: color_bytes, normal: normal_v4 });
                mesh.vertices.push(Vertex { position: v1, uv: vec2(1.,0.), color: color_bytes, normal: normal_v4 });
                mesh.vertices.push(Vertex { position: v2, uv: vec2(1.,1.), color: color_bytes, normal: normal_v4 });
                mesh.vertices.push(Vertex { position: v3, uv: vec2(0.,1.), color: color_bytes, normal: normal_v4 });

                mesh.indices.push(base_idx);
                mesh.indices.push(base_idx + 1);
                mesh.indices.push(base_idx + 2);

                mesh.indices.push(base_idx);
                mesh.indices.push(base_idx + 2);
                mesh.indices.push(base_idx + 3);

                // Wireframe
                draw_line_3d(v0, v1, BLACK);
                draw_line_3d(v1, v2, BLACK);
                draw_line_3d(v2, v3, BLACK);
                draw_line_3d(v3, v0, BLACK);
            }
        }

        draw_mesh(&mesh);

        // Draw Ants
        for ant in &colony.ants {
            match ant.state {
                AntState::Foraging => {
                    // Interpolate position
                    let pos = bilinear_interp(ant.u, ant.v, grid.cols, &vertices);
                    draw_sphere(pos, 0.2, None, RED);
                }
                AntState::Jumping => {
                    // Draw arc
                    // Lerp between start and end
                    let t = ant.jump_progress;
                    let pos = ant.jump_start_pos.lerp(ant.jump_end_pos, t);
                    // Add some arc height
                    let arc_height = 2.0 * (1.0 - (t - 0.5).abs() * 2.0); // Parabola-ish
                    // Up is Y in our camera logic?
                    // Actually, z is height in MiuraGrid logic (h).
                    // In main camera setup, Y is Up.
                    // MiuraGrid returns (x, y, z).
                    // Wait, MiuraGrid `generate_grid` does: `vertices.push(vec3(x, y, z));`
                    // where x is horizontal, y is vertical (on sheet), z is height (fold depth).
                    // So Y is "North-South" on the sheet. Z is "Up/Down" relative to the sheet plane.
                    // But in macroquad 3D, Y is usually Up.
                    // So grid is lying flat on X-Z plane?
                    // No, grid logic: `y` is computed from `j`. `z` is `h`.
                    // So grid is in X-Y plane with Z variation.
                    // Camera is set with Up = (0,1,0).
                    // So the grid is standing up vertically?
                    // Or laying flat?
                    // X is horizontal. Y is vertical. Z is depth.
                    // If camera looks at (0,0,0) from (0,0,40), it sees X-Y plane.
                    // So Z variation is depth (towards/away from camera).
                    // So arc height should be in Z or negative Z (towards camera).

                    let arc_vec = vec3(0.0, 0.0, -arc_height);

                    draw_sphere(pos + arc_vec, 0.3, None, YELLOW);
                    draw_line_3d(ant.jump_start_pos, ant.jump_end_pos, Color::new(1.0, 1.0, 0.0, 0.5));
                }
            }
        }

        set_default_camera();
        draw_text("Folded Colony 🧬", 10.0, 30.0, 30.0, WHITE);
        draw_text(&format!("Expansion: {:.2} (Left/Right to fold)", expansion), 10.0, 50.0, 20.0, LIGHTGRAY);
        draw_text(&format!("Ants: {}", colony.ants.len()), 10.0, 70.0, 20.0, GRAY);

        next_frame().await
    }
}

// Helper need to be duplicated or imported?
// I can import from `ant` but `ant::bilinear_interp` is private.
// I'll make it public in `src/ant.rs`.
// Since I can't edit `src/ant.rs` easily now without another step, I'll just copy it here for now or use `ant` logic if I expose it.
// I'll assume I can copy it here as a local helper.

fn bilinear_interp(u: f32, v: f32, width_cols: usize, vertices: &[Vec3]) -> Vec3 {
    let iu = u.floor() as usize;
    let iv = v.floor() as usize;
    let fu = u - iu as f32;
    let fv = v - iv as f32;

    let w = width_cols + 1;
    let idx00 = iv * w + iu;
    let idx10 = iv * w + (iu + 1);
    let idx01 = (iv + 1) * w + iu;
    let idx11 = (iv + 1) * w + (iu + 1);

    if idx11 >= vertices.len() {
        return Vec3::ZERO;
    }

    let p00 = vertices[idx00];
    let p10 = vertices[idx10];
    let p01 = vertices[idx01];
    let p11 = vertices[idx11];

    let p0 = p00.lerp(p10, fu);
    let p1 = p01.lerp(p11, fu);

    p0.lerp(p1, fv)
}
