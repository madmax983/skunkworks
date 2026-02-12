use macroquad::models::{Mesh, Vertex};
use macroquad::prelude::*;
use origami_constellation::mesh_gen::{generate_miura_ori, generate_solar_array, generate_yoshimura};
use origami_constellation::pbd::Constraint;

#[derive(PartialEq)]
enum Pattern {
    Miura,
    Yoshimura,
    SolarArray,
}

#[macroquad::main("Origami Constellation")]
async fn main() {
    let mut current_pattern = Pattern::SolarArray; // Default to the moonshot
    let mut mesh_data = generate_solar_array();

    // Camera state
    let mut cam_yaw: f32 = 0.0;
    let mut cam_pitch: f32 = 0.5;
    let mut cam_dist: f32 = 30.0;

    let mut last_mouse_pos = mouse_position();

    // Folding state
    let mut fold_factor: f32 = 1.0; // 1.0 = Deployed (Flat), 0.0 = Folded

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

        // Pattern Switching
        if is_key_pressed(KeyCode::Key1) {
            current_pattern = Pattern::Miura;
            mesh_data = generate_miura_ori(10, 10);
            fold_factor = 1.0;
        }
        if is_key_pressed(KeyCode::Key2) {
            current_pattern = Pattern::Yoshimura;
            mesh_data = generate_yoshimura(5, 4.0);
            fold_factor = 1.0;
        }
        if is_key_pressed(KeyCode::Key3) {
            current_pattern = Pattern::SolarArray;
            mesh_data = generate_solar_array();
            fold_factor = 1.0;
        }

        // Folding Control
        if is_key_down(KeyCode::Right) {
            fold_factor += 0.01;
        }
        if is_key_down(KeyCode::Left) {
            fold_factor -= 0.01;
        }
        fold_factor = fold_factor.clamp(0.0, 1.0);

        // Update Constraints
        for &actuator_idx in &mesh_data.actuators {
            if let Constraint::Actuator { factor, .. } =
                &mut mesh_data.system.constraints[actuator_idx]
            {
                *factor = fold_factor;
            }
        }

        // Physics Step
        let dt = 0.016; // Fixed step
        mesh_data.system.step(dt, 10); // 10 iterations

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
            // Draw point as small line
            draw_line_3d(*star, *star + vec3(0.05, 0.05, 0.05), WHITE);
        }

        // Draw Constraints (Stress)
        for (i, constraint) in mesh_data.system.constraints.iter().enumerate() {
            if let Constraint::Distance { p1, p2, .. } = constraint {
                let p1_pos = mesh_data.system.particles[*p1].pos;
                let p2_pos = mesh_data.system.particles[*p2].pos;

                let stress = mesh_data.system.get_stress(i);
                // Color: Green (0) -> Red (>0.1)
                let t = (stress * 10.0).clamp(0.0, 1.0);
                let color = Color::new(t, 1.0 - t, 0.0, 0.3); // Alpha 0.3

                draw_line_3d(p1_pos, p2_pos, color);
            }
        }

        // Draw Mesh Faces
        let mut mesh = Mesh {
            vertices: Vec::new(),
            indices: Vec::new(),
            texture: None,
        };

        let indices = &mesh_data.indices;
        let system = &mesh_data.system;

        for i in (0..indices.len()).step_by(3) {
            let idx0 = indices[i] as usize;
            let idx1 = indices[i + 1] as usize;
            let idx2 = indices[i + 2] as usize;

            let v0 = system.particles[idx0].pos;
            let v1 = system.particles[idx1].pos;
            let v2 = system.particles[idx2].pos;

            // Flat shading
            let normal = (v1 - v0).cross(v2 - v0).normalize_or_zero();
            let light_dir = vec3(0.5, 1.0, 0.5).normalize();
            let intensity = normal.dot(light_dir).abs() * 0.7 + 0.3;

            // Solar Panel Color: Dark Blue
            let base_color = if current_pattern == Pattern::SolarArray && (idx0 < 8) {
                // Hub is Grey (first 8 particles of SolarArray are Hub)
                Color::new(0.5, 0.5, 0.5, 1.0)
            } else {
                Color::new(0.0, 0.05, 0.4, 0.9)
            };

            let color = Color::new(
                base_color.r * intensity,
                base_color.g * intensity,
                base_color.b * intensity,
                base_color.a,
            );

            let start_idx = mesh.vertices.len() as u16;

            let color_bytes: [u8; 4] = color.into();
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

            // Wireframe Edges on top of mesh
            let wire_color = Color::new(0.0, 0.8, 1.0, 0.5); // Cyan
            draw_line_3d(v0, v1, wire_color);
            draw_line_3d(v1, v2, wire_color);
            draw_line_3d(v2, v0, wire_color);
        }

        draw_mesh(&mesh);

        set_default_camera();

        draw_text(
            &format!("Deploy: {:.0}%", fold_factor * 100.0),
            10.0,
            30.0,
            30.0,
            WHITE,
        );
        draw_text(
            "Controls: Left/Right Arrow to Deploy/Fold",
            10.0,
            60.0,
            20.0,
            WHITE,
        );
        draw_text(
            "Switch: [1] Miura [2] Yoshimura [3] Solar Array",
            10.0,
            80.0,
            20.0,
            YELLOW,
        );
        draw_text("Orbit: Mouse Drag | Zoom: Scroll", 10.0, 100.0, 20.0, WHITE);

        next_frame().await
    }
}
