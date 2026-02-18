use macroquad::models::Vertex as MqVertex;
use macroquad::prelude::*;
use origami::{MiuraOri, MiuraParams, Orientation};

#[macroquad::main("Origami Constellation")]
async fn main() {
    let params = MiuraParams {
        a: 1.0,
        b: 1.0,
        gamma: 80.0f32.to_radians(),
        orientation: Orientation::Horizontal,
    };
    let grid_size = (20, 20); // 20x20 grid
    let origami = MiuraOri::new(params, grid_size);

    let mut extension = 0.5;

    // Camera state
    let mut cam_yaw: f32 = 0.0;
    let mut cam_pitch: f32 = 0.5;
    let mut cam_dist: f32 = 30.0;
    let mut last_mouse_pos = mouse_position();

    // Star field
    let mut stars = Vec::new();
    for _ in 0..2000 {
        stars.push(vec3(
            rand::gen_range(-100.0, 100.0),
            rand::gen_range(-100.0, 100.0),
            rand::gen_range(-100.0, 100.0),
        ));
    }

    loop {
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

        // Folding Control (Mouse X relative to screen width, or arrows)
        if is_key_down(KeyCode::Right) {
            extension += 0.01;
        }
        if is_key_down(KeyCode::Left) {
            extension -= 0.01;
        }

        // Also allow dragging with Right Mouse Button to fold
        if is_mouse_button_down(MouseButton::Right) {
            extension += delta.x * 0.005;
        }

        extension = extension.clamp(0.0, 1.0);

        // Generate Mesh
        let mesh_data = origami.generate_mesh(extension);

        // Convert to Macroquad Mesh
        // We'll use flat shading per triangle for the "low poly" look
        // Actually, let's use the wireframe + point approach mainly, but fill faces slightly.

        clear_background(BLACK);

        // 3D Setup
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

        // Draw Stars
        for star in &stars {
            // Simple parallax could be added here
            draw_line_3d(*star, *star + vec3(0.05, 0.0, 0.0), WHITE);
        }

        // Draw Origami
        // 1. Faces (Transparent)
        // We need to construct a macroquad Mesh
        let mut mq_mesh = Mesh {
            vertices: Vec::with_capacity(mesh_data.indices.len()), // unindexed
            indices: Vec::with_capacity(mesh_data.indices.len()),
            texture: None,
        };

        // Color based on extension
        let base_color = if extension > 0.8 {
            Color::new(0.0, 0.8, 1.0, 0.3) // Cyan when flat
        } else {
            Color::new(0.8, 0.0, 1.0, 0.4) // Purple when folded
        };

        let light_dir = vec3(0.5, 1.0, 0.5).normalize();

        for i in (0..mesh_data.indices.len()).step_by(3) {
            let idx0 = mesh_data.indices[i] as usize;
            let idx1 = mesh_data.indices[i + 1] as usize;
            let idx2 = mesh_data.indices[i + 2] as usize;

            let v0 = mesh_data.vertices[idx0].pos;
            let v1 = mesh_data.vertices[idx1].pos;
            let v2 = mesh_data.vertices[idx2].pos;

            let uv0 = mesh_data.vertices[idx0].uv;
            let uv1 = mesh_data.vertices[idx1].uv;
            let uv2 = mesh_data.vertices[idx2].uv;

            // Flat Normal
            let normal = (v1 - v0).cross(v2 - v0).normalize_or_zero();
            let intensity = normal.dot(light_dir).abs() * 0.6 + 0.4;

            let color = Color::new(
                base_color.r * intensity,
                base_color.g * intensity,
                base_color.b * intensity,
                base_color.a,
            );
            let color_bytes: [u8; 4] = color.into();

            let start = mq_mesh.vertices.len() as u16;

            mq_mesh.vertices.push(MqVertex {
                position: v0,
                uv: uv0,
                color: color_bytes,
                normal: vec4(normal.x, normal.y, normal.z, 1.0),
            });
            mq_mesh.vertices.push(MqVertex {
                position: v1,
                uv: uv1,
                color: color_bytes,
                normal: vec4(normal.x, normal.y, normal.z, 1.0),
            });
            mq_mesh.vertices.push(MqVertex {
                position: v2,
                uv: uv2,
                color: color_bytes,
                normal: vec4(normal.x, normal.y, normal.z, 1.0),
            });

            mq_mesh.indices.push(start);
            mq_mesh.indices.push(start + 1);
            mq_mesh.indices.push(start + 2);

            // Draw Wireframe manually (Edges)
            let wire_color = Color::new(1.0, 1.0, 1.0, 0.3);
            draw_line_3d(v0, v1, wire_color);
            draw_line_3d(v1, v2, wire_color);
            draw_line_3d(v2, v0, wire_color);
        }

        // Draw Faces
        draw_mesh(&mq_mesh);

        // Draw Vertices (Stars)
        for v in &mesh_data.vertices {
            // Billboard? Nah, just small sphere or point
            // draw_sphere(v.pos, 0.05, None, WHITE); // Expensive for many vertices
            // Use lines for "cross" star
            let s = 0.05;
            draw_line_3d(v.pos - vec3(s, 0., 0.), v.pos + vec3(s, 0., 0.), WHITE);
            draw_line_3d(v.pos - vec3(0., s, 0.), v.pos + vec3(0., s, 0.), WHITE);
            draw_line_3d(v.pos - vec3(0., 0., s), v.pos + vec3(0., 0., s), WHITE);
        }

        set_default_camera();

        // UI Overlay
        draw_text("ORIGAMI CONSTELLATION", 20.0, 30.0, 30.0, WHITE);
        draw_text(
            &format!("Extension: {:.1}%", extension * 100.0),
            20.0,
            60.0,
            20.0,
            WHITE,
        );
        draw_text(
            "Controls: Left/Right Arrow or Right Mouse Drag to Fold",
            20.0,
            80.0,
            20.0,
            GRAY,
        );
        draw_text(
            "Orbit: Left Mouse Drag | Zoom: Scroll",
            20.0,
            100.0,
            20.0,
            GRAY,
        );

        // "Moonshot" flavor text
        let status = if extension > 0.95 {
            "ARRAY DEPLOYED - COLLECTING PHOTONS"
        } else if extension < 0.05 {
            "STOWED FOR TRANSIT"
        } else {
            "DEPLOYMENT IN PROGRESS..."
        };

        let color = if extension > 0.95 { GREEN } else { YELLOW };
        draw_text(status, 20.0, screen_height() - 30.0, 20.0, color);

        next_frame().await
    }
}
