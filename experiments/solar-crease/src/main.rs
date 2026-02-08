mod geometry;
mod kinematics;

use geometry::generate_miura_topology;
use kinematics::calculate_miura_vertices;
use macroquad::prelude::*;
use macroquad::rand;

fn compute_normals(vertices: &[Vec3], indices: &[u16]) -> Vec<Vec3> {
    let mut normals = vec![Vec3::ZERO; vertices.len()];
    for i in (0..indices.len()).step_by(3) {
        let i0 = indices[i] as usize;
        let i1 = indices[i+1] as usize;
        let i2 = indices[i+2] as usize;
        let v0 = vertices[i0];
        let v1 = vertices[i1];
        let v2 = vertices[i2];
        let normal = (v1 - v0).cross(v2 - v0).normalize_or_zero();
        normals[i0] += normal;
        normals[i1] += normal;
        normals[i2] += normal;
    }
    for n in &mut normals {
        *n = n.normalize_or_zero();
    }
    normals
}

#[macroquad::main("Solar Crease")]
async fn main() {
    let mut fold_factor = 0.5f32;
    let rows = 20;
    let cols = 20;
    let a = 1.0;
    let b = 1.0;
    let alpha = 60.0f32.to_radians();

    let topology = generate_miura_topology(rows, cols);

    // Generate stars
    let mut stars = Vec::new();
    for _ in 0..1000 {
        let x = rand::gen_range(0.0, screen_width());
        let y = rand::gen_range(0.0, screen_height());
        stars.push((x, y, rand::gen_range(0.5, 1.0)));
    }

    // Camera state
    let mut cam_dist = 40.0;
    let mut cam_yaw = 0.5;
    let mut cam_pitch = 0.5;
    let mut last_mouse_pos = mouse_position();

    loop {
        clear_background(Color::new(0.05, 0.05, 0.1, 1.0));

        // Draw stars (screenspace)
        for (x, y, alpha) in &stars {
             // Twinkle effect
             let a = *alpha * (0.8 + 0.2 * (get_time() * 5.0 + *x as f64).sin() as f32);
             draw_circle(*x, *y, 1.0, Color::new(1.0, 1.0, 1.0, a));
        }

        // Input
        if is_mouse_button_down(MouseButton::Left) {
            let (mx, my) = mouse_position();
            let dx = mx - last_mouse_pos.0;
            let dy = my - last_mouse_pos.1;
            cam_yaw -= dx * 0.01;
            cam_pitch = (cam_pitch + dy * 0.01).clamp(-1.5, 1.5);
        }
        last_mouse_pos = mouse_position();

        cam_dist = (cam_dist + mouse_wheel().1 * -2.0).clamp(5.0, 100.0);

        if is_key_down(KeyCode::Right) {
            fold_factor = (fold_factor + 0.01).min(1.0);
        }
        if is_key_down(KeyCode::Left) {
            fold_factor = (fold_factor - 0.01).max(0.0);
        }

        // Map mouse X to fold factor if Right Click is held (Interactive folding)
        if is_mouse_button_down(MouseButton::Right) {
             let (mx, _) = mouse_position();
             fold_factor = (mx / screen_width()).clamp(0.0, 1.0);
        }

        // Update Mesh
        let positions = calculate_miura_vertices(rows, cols, a, b, alpha, fold_factor);
        let normals = compute_normals(&positions, &topology.indices);

        let vertices: Vec<Vertex> = positions
            .iter()
            .enumerate()
            .zip(normals.iter())
            .map(|((i, p), n)| {
                // Determine row/col from index i
                let c = i % (cols + 1);
                let r = i / (cols + 1);

                // Checkerboard pattern for "Solar Cells"
                let is_even = (c + r) % 2 == 0;
                let base_color = if is_even {
                    Color::new(0.1, 0.1, 0.6, 1.0) // Deep Blue
                } else {
                    Color::new(0.05, 0.05, 0.4, 1.0) // Darker Blue
                };

                // Add gold tint for the "Back" side? No, simple blue is fine.
                // Let's make it look like Kapton (Gold) on one side and Blue on other?
                // Just blue solar panels.

                let color = base_color;

                Vertex {
                    position: *p,
                    uv: Vec2::ZERO,
                    color: [
                        (color.r * 255.0) as u8,
                        (color.g * 255.0) as u8,
                        (color.b * 255.0) as u8,
                        (color.a * 255.0) as u8,
                    ],
                    normal: vec4(n.x, n.y, n.z, 0.0),
                }
            })
            .collect();

        let mesh = Mesh {
            vertices,
            indices: topology.indices.clone(),
            texture: None,
        };

        // Camera
        let cam_pos = vec3(
            cam_dist * cam_yaw.cos() * cam_pitch.cos(),
            cam_dist * cam_pitch.sin(),
            cam_dist * cam_yaw.sin() * cam_pitch.cos(),
        );

        set_camera(&Camera3D {
            position: cam_pos,
            up: vec3(0., 1., 0.),
            target: vec3(0., 0., 0.),
            ..Default::default()
        });

        draw_grid(20, 1.0, DARKGRAY, GRAY);

        // Draw the mesh
        draw_mesh(&mesh);

        set_default_camera();

        // UI
        draw_text(
            &format!("Deployment: {:.0}%", fold_factor * 100.0),
            10.0,
            30.0,
            30.0,
            WHITE,
        );
        draw_text(
            "Controls: Left/Right Arrow to fold. Mouse drag to rotate.",
            10.0,
            50.0,
            20.0,
            LIGHTGRAY,
        );
        draw_text(
            "Right Mouse Drag to Scrub Fold",
            10.0,
            70.0,
            20.0,
            LIGHTGRAY,
        );

        next_frame().await
    }
}
