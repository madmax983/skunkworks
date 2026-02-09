mod pbd;
mod mesh_gen;

use macroquad::prelude::*;
use macroquad::models::{Mesh, Vertex};
use pbd::Constraint;
use mesh_gen::generate_miura_ori;

#[macroquad::main("Origami Constellation")]
async fn main() {
    let mesh_data = generate_miura_ori(10, 10);
    // Destructure to avoid partial move issues
    let mesh_gen::MeshData { mut system, indices, actuators } = mesh_data;

    // Camera state
    let mut cam_yaw: f32 = 0.0;
    let mut cam_pitch: f32 = 0.5;
    let mut cam_dist: f32 = 20.0;

    let mut last_mouse_pos = mouse_position();

    // Folding state
    let mut fold_factor: f32 = 1.0; // 1.0 = Flat, 0.0 = Folded

    // Stars
    let mut stars = Vec::new();
    for _ in 0..1000 {
        stars.push(vec3(
            rand::gen_range(-100.0, 100.0),
            rand::gen_range(-100.0, 100.0),
            rand::gen_range(-100.0, 100.0),
        ));
    }

    loop {
        // Input
        let mouse_pos = mouse_position();
        let delta = vec2(mouse_pos.0 - last_mouse_pos.0, mouse_pos.1 - last_mouse_pos.1);
        last_mouse_pos = mouse_pos;

        if is_mouse_button_down(MouseButton::Left) {
            cam_yaw -= delta.x * 0.01;
            cam_pitch += delta.y * 0.01;
            cam_pitch = cam_pitch.clamp(-1.5, 1.5);
        }

        let wheel = mouse_wheel().1;
        cam_dist -= wheel * 0.1 * cam_dist;
        cam_dist = cam_dist.clamp(5.0, 150.0);

        // Folding Control
        if is_key_down(KeyCode::Right) {
            fold_factor += 0.01;
        }
        if is_key_down(KeyCode::Left) {
            fold_factor -= 0.01;
        }
        fold_factor = fold_factor.clamp(0.0, 1.0);

        // Update Constraints
        for &actuator_idx in &actuators {
            if let Constraint::Actuator { factor, .. } = &mut system.constraints[actuator_idx] {
                *factor = fold_factor;
            }
        }

        // Physics Step
        let dt = 0.016; // Fixed step
        system.step(dt, 10); // 10 iterations

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

        // Draw Stars
        for star in &stars {
            draw_line_3d(*star, *star + vec3(0.1, 0.0, 0.0), WHITE);
        }

        // Draw Mesh
        // Rebuild mesh every frame
        // Texture is None
        let mut mesh = Mesh {
            vertices: Vec::new(),
            indices: Vec::new(),
            texture: None,
        };

        for i in (0..indices.len()).step_by(3) {
            let idx0 = indices[i] as usize;
            let idx1 = indices[i+1] as usize;
            let idx2 = indices[i+2] as usize;

            let v0 = system.particles[idx0].pos;
            let v1 = system.particles[idx1].pos;
            let v2 = system.particles[idx2].pos;

            // Flat shading
            let normal = (v1 - v0).cross(v2 - v0).normalize_or_zero();
            let light_dir = vec3(0.5, 1.0, 0.5).normalize();
            let intensity = normal.dot(light_dir).abs() * 0.7 + 0.3;

            let color = Color::new(0.0, 0.0, 0.8 * intensity, 1.0); // Deep Blue

            let start_idx = mesh.vertices.len() as u16;

            let color_bytes: [u8; 4] = color.into();
            let normal_v4 = vec4(normal.x, normal.y, normal.z, 1.0);
            mesh.vertices.push(Vertex { position: v0, uv: vec2(0.,0.), color: color_bytes, normal: normal_v4 });
            mesh.vertices.push(Vertex { position: v1, uv: vec2(0.,0.), color: color_bytes, normal: normal_v4 });
            mesh.vertices.push(Vertex { position: v2, uv: vec2(0.,0.), color: color_bytes, normal: normal_v4 });

            mesh.indices.push(start_idx);
            mesh.indices.push(start_idx + 1);
            mesh.indices.push(start_idx + 2);

            // Wireframe
            let wire_color = Color::new(1.0, 0.8, 0.0, 0.5); // Gold
            draw_line_3d(v0, v1, wire_color);
            draw_line_3d(v1, v2, wire_color);
            draw_line_3d(v2, v0, wire_color);
        }

        draw_mesh(&mesh);

        set_default_camera();

        draw_text(&format!("Fold: {:.0}%", (1.0 - fold_factor) * 100.0), 10.0, 30.0, 30.0, WHITE);
        draw_text("Controls: Left/Right Arrow to Fold", 10.0, 60.0, 20.0, WHITE);
        draw_text("Orbit: Mouse Drag | Zoom: Scroll", 10.0, 80.0, 20.0, WHITE);

        next_frame().await
    }
}
