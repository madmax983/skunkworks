use macroquad::prelude::*;

mod pbd;
mod mesh;

use pbd::PbdSystem;
use mesh::Mesh as OrigamiMesh;

fn conf() -> Conf {
    Conf {
        window_title: "Genesis: Manifold Origami".to_owned(),
        window_width: 1280,
        window_height: 720,
        high_dpi: true,
        sample_count: 4,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    let mut system = PbdSystem::new();
    let mut origami = OrigamiMesh::new();

    // Generate Pattern
    origami.generate_miura_ori(&mut system, 12, 12);

    let mut cam_dist = 20.0;
    let mut cam_yaw = 0.5f32;
    let mut cam_pitch = 0.8f32;

    let mut fold_rho = 0.0f32; // 0.0 = flat, 1.0 = folded

    // Mouse input handling
    let mut last_mouse_pos = mouse_position();

    loop {
        // Input
        if is_key_down(KeyCode::Up) {
            fold_rho = (fold_rho + 0.01).min(1.0);
        }
        if is_key_down(KeyCode::Down) {
            fold_rho = (fold_rho - 0.01).max(0.0);
        }
        if is_key_pressed(KeyCode::R) {
             system = PbdSystem::new();
             origami.generate_miura_ori(&mut system, 12, 12);
             fold_rho = 0.0;
        }

        // Camera
        let mouse_pos = mouse_position();
        if is_mouse_button_down(MouseButton::Left) {
            let delta_x = mouse_pos.0 - last_mouse_pos.0;
            let delta_y = mouse_pos.1 - last_mouse_pos.1;

            cam_yaw -= delta_x * 0.01;
            cam_pitch = (cam_pitch + delta_y * 0.01).clamp(0.1, 1.5);
        }
        last_mouse_pos = mouse_pos;

        let scroll = mouse_wheel().1;
        cam_dist = (cam_dist - scroll * 1.0).clamp(2.0, 80.0);

        // Update Physics
        origami.update_folds(&mut system, fold_rho);
        system.step(0.016, 20); // More iterations for stiffness

        // Render
        clear_background(Color::new(0.05, 0.05, 0.05, 1.0));

        let cam_pos = vec3(
            cam_yaw.sin() * cam_pitch.cos() * cam_dist,
            cam_pitch.sin() * cam_dist,
            cam_yaw.cos() * cam_pitch.cos() * cam_dist,
        );

        set_camera(&Camera3D {
            position: cam_pos,
            target: vec3(0.0, 0.0, 0.0),
            up: vec3(0.0, 1.0, 0.0),
            ..Default::default()
        });

        // Build Render Mesh
        let mut mq_mesh = Mesh {
            vertices: Vec::new(),
            indices: Vec::new(),
            texture: None,
        };

        let light_dir = vec3(0.5, 1.0, 0.5).normalize();

        for i in (0..origami.indices.len()).step_by(3) {
            let idx0 = origami.indices[i] as usize;
            let idx1 = origami.indices[i+1] as usize;
            let idx2 = origami.indices[i+2] as usize;

            let v0 = system.particles[idx0].pos;
            let v1 = system.particles[idx1].pos;
            let v2 = system.particles[idx2].pos;

            let normal = (v1 - v0).cross(v2 - v0).normalize_or_zero();
            let mut intensity = normal.dot(light_dir).abs();
            intensity = 0.1 + 0.9 * intensity;

            // Color based on position or stress?
            // Let's do "Paper" color (White/Beige) but with shading.
            let base_color = if fold_rho > 0.9 {
                Color::new(1.0, 0.3, 0.3, 1.0) // Red when fully folded (Moonshot "Green/Red" theme?)
            } else {
                Color::new(0.95, 0.95, 0.9, 1.0)
            };

            let color = Color::new(
                base_color.r * intensity,
                base_color.g * intensity,
                base_color.b * intensity,
                1.0
            );
            let color_bytes: [u8; 4] = color.into();

            let base_idx = mq_mesh.vertices.len() as u16;
            // Vertex requires position (Vec3), uv (Vec2), color ([u8;4]), normal (Vec4 usually? or Vec3?)
            // Based on other experiments, let's use what works.
            mq_mesh.vertices.push(Vertex {
                position: v0,
                uv: Vec2::ZERO,
                color: color_bytes,
                normal: Vec4::ZERO
            });
            mq_mesh.vertices.push(Vertex {
                position: v1,
                uv: Vec2::ZERO,
                color: color_bytes,
                normal: Vec4::ZERO
            });
            mq_mesh.vertices.push(Vertex {
                position: v2,
                uv: Vec2::ZERO,
                color: color_bytes,
                normal: Vec4::ZERO
            });
            mq_mesh.indices.push(base_idx);
            mq_mesh.indices.push(base_idx + 1);
            mq_mesh.indices.push(base_idx + 2);

            // Wireframe Overlay (simple lines)
            draw_line_3d(v0, v1, BLACK);
            draw_line_3d(v1, v2, BLACK);
            draw_line_3d(v2, v0, BLACK);
        }

        draw_mesh(&mq_mesh);

        set_default_camera();

        // UI
        draw_text("Genesis: Manifold Origami", 20.0, 30.0, 30.0, WHITE);
        draw_text(&format!("Fold (Rho): {:.2}", fold_rho), 20.0, 60.0, 20.0, WHITE);
        draw_text("Controls: Arrow UP/DOWN to fold. Mouse Drag to Rotate. R to Reset.", 20.0, 90.0, 20.0, GRAY);

        next_frame().await
    }
}
