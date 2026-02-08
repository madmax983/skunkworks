use kinetic_crease::mesh;
use kinetic_crease::physics::Solver;
use macroquad::prelude::*;

#[macroquad::main("Kinetic Crease")]
async fn main() {
    let mut solver = Solver::new();

    // Generate Miura-ori
    let crease_mesh = mesh::generate_miura_ori(&mut solver, 10, 10, 1.0, 1.0, 84.0);

    let mut fold_factor = 0.0f32; // 0.0 = Flat, 1.0 = Folded

    // Camera state
    let mut cam_yaw = 0.0f32;
    let mut cam_pitch = 0.5f32;
    let mut cam_dist = 20.0f32;

    loop {
        // Input
        if is_key_down(KeyCode::Left) {
            fold_factor = (fold_factor - 0.01).max(0.0);
        }
        if is_key_down(KeyCode::Right) {
            fold_factor = (fold_factor + 0.01).min(1.0);
        }

        // Camera Input
        if is_key_down(KeyCode::A) {
            cam_yaw += 0.02;
        }
        if is_key_down(KeyCode::D) {
            cam_yaw -= 0.02;
        }
        if is_key_down(KeyCode::W) {
            cam_pitch += 0.02;
        }
        if is_key_down(KeyCode::S) {
            cam_pitch -= 0.02;
        }
        cam_dist = (cam_dist + mouse_wheel().1 * -1.0).clamp(5.0, 100.0);

        // Physics Update
        crease_mesh.update_constraints(&mut solver, fold_factor);

        let dt = 1.0 / 60.0;
        solver.step(dt, 10);

        // Render
        clear_background(Color::new(0.1, 0.1, 0.1, 1.0));

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

        draw_grid(20, 1.0, BLACK, GRAY);

        let mut mq_vertices = Vec::new();
        let mut mq_indices = Vec::new();

        let light_dir = vec3(0.5, 1.0, 0.2).normalize();

        for i in (0..crease_mesh.indices.len()).step_by(3) {
            let idx0 = crease_mesh.indices[i] as usize;
            let idx1 = crease_mesh.indices[i + 1] as usize;
            let idx2 = crease_mesh.indices[i + 2] as usize;

            let p0 = solver.particles[idx0].pos;
            let p1 = solver.particles[idx1].pos;
            let p2 = solver.particles[idx2].pos;

            let v0 = vec3(p0.x, p0.y, p0.z);
            let v1 = vec3(p1.x, p1.y, p1.z);
            let v2 = vec3(p2.x, p2.y, p2.z);

            let normal = (v1 - v0).cross(v2 - v0).normalize_or_zero();
            let normal_v4 = vec4(normal.x, normal.y, normal.z, 0.0);

            let diff = normal.dot(light_dir).abs();
            let brightness = 0.2 + 0.8 * diff;

            let base_color = Color::new(0.9, 0.9, 0.95, 1.0);
            let color = Color::new(
                base_color.r * brightness,
                base_color.g * brightness,
                base_color.b * brightness,
                1.0,
            );

            let color_u8: [u8; 4] = [
                (color.r * 255.0) as u8,
                (color.g * 255.0) as u8,
                (color.b * 255.0) as u8,
                (color.a * 255.0) as u8,
            ];

            let base_idx = mq_vertices.len() as u16;
            mq_vertices.push(Vertex {
                position: v0,
                uv: vec2(0., 0.),
                color: color_u8,
                normal: normal_v4,
            });
            mq_vertices.push(Vertex {
                position: v1,
                uv: vec2(1., 0.),
                color: color_u8,
                normal: normal_v4,
            });
            mq_vertices.push(Vertex {
                position: v2,
                uv: vec2(0., 1.),
                color: color_u8,
                normal: normal_v4,
            });

            mq_indices.push(base_idx);
            mq_indices.push(base_idx + 1);
            mq_indices.push(base_idx + 2);
        }

        let mesh = Mesh {
            vertices: mq_vertices,
            indices: mq_indices,
            texture: None,
        };

        draw_mesh(&mesh);

        set_default_camera();

        draw_text(
            &format!("Fold Factor: {:.2}", fold_factor),
            10.0,
            30.0,
            30.0,
            WHITE,
        );
        draw_text(
            "Controls: Left/Right to Fold, WASD+MouseWheel to Move Camera",
            10.0,
            60.0,
            20.0,
            LIGHTGRAY,
        );
        draw_text(&format!("FPS: {}", get_fps()), 10.0, 80.0, 20.0, LIGHTGRAY);

        next_frame().await
    }
}
