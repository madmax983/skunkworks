use ::rand::Rng;
use macroquad::prelude::*;
use origami::{MiuraOri, MiuraParams, Orientation};

#[macroquad::main("Rigid Origami")]
async fn main() {
    let params = MiuraParams {
        a: 1.0,
        b: 1.0,
        gamma: 80.0f32.to_radians(),
        orientation: Orientation::Vertical,
    };
    let grid = MiuraOri::new(params, (20, 15));
    let (cols, rows) = grid.grid_size;

    // Camera state
    let mut cam_yaw: f32 = 0.0;
    let mut cam_pitch: f32 = 0.5;
    let mut cam_dist: f32 = 40.0;

    let mut last_mouse_pos = mouse_position();

    // Starfield
    let mut rng = ::rand::thread_rng();
    let stars: Vec<Vec3> = (0..1000)
        .map(|_| {
            vec3(
                rng.gen_range(-100.0..100.0),
                rng.gen_range(-100.0..100.0),
                rng.gen_range(-100.0..100.0),
            )
        })
        .collect();

    // Deployment state
    let mut expansion = 0.05;
    let mut deploying = true;
    let deploy_speed = 0.2;

    loop {
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
            expansion = (expansion + 1.0 * dt).min(1.0);
            deploying = false;
        }
        if is_key_down(KeyCode::Left) {
            expansion = (expansion - 1.0 * dt).max(0.01);
            deploying = false;
        }
        if is_key_pressed(KeyCode::Space) {
            deploying = !deploying;
        }

        // Auto-Deployment
        if deploying {
            expansion += deploy_speed * dt;
            if expansion >= 1.0 {
                expansion = 1.0;
                deploying = false;
            }
        }

        // Rendering
        clear_background(Color::new(0.01, 0.01, 0.05, 1.0)); // Deep space

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
            // Simple point rendering
            draw_line_3d(*star, *star + vec3(0.1, 0.0, 0.0), WHITE);
        }

        // 1. Calculate vertices
        let vertices = grid.generate_grid(expansion);

        // 2. Build Mesh
        let mut mesh = Mesh {
            vertices: Vec::new(),
            indices: Vec::new(),
            texture: None,
        };

        let width = cols + 1;

        // Solar Panel Colors
        let face_color = Color::new(0.1, 0.1, 0.4, 1.0); // Dark Blue

        for j in 0..rows {
            for i in 0..cols {
                let idx0 = (j * width + i) as u16;
                let idx1 = (j * width + i + 1) as u16;
                let idx2 = ((j + 1) * width + i + 1) as u16;
                let idx3 = ((j + 1) * width + i) as u16;

                let v0 = vertices[idx0 as usize];
                let v1 = vertices[idx1 as usize];
                let v2 = vertices[idx2 as usize];
                let v3 = vertices[idx3 as usize];

                // Normal calculation for simple shading
                let normal1 = (v1 - v0).cross(v2 - v0).normalize();

                // Add vertices to mesh (duplicated for flat shading if needed, but here sharing is fine for smooth or just use flat)
                // For sharp creases, we need unique vertices per face.
                // Or we can just use `draw_mesh` with raw triangles.
                // Let's use unique vertices per quad to ensure flat shading look

                let base_idx = mesh.vertices.len() as u16;

                let color = face_color;
                let color_bytes: [u8; 4] = color.into();
                let normal_v4 = vec4(normal1.x, normal1.y, normal1.z, 1.0);

                mesh.vertices.push(Vertex {
                    position: v0,
                    uv: vec2(0., 0.),
                    color: color_bytes,
                    normal: normal_v4,
                });
                mesh.vertices.push(Vertex {
                    position: v1,
                    uv: vec2(1., 0.),
                    color: color_bytes,
                    normal: normal_v4,
                });
                mesh.vertices.push(Vertex {
                    position: v2,
                    uv: vec2(1., 1.),
                    color: color_bytes,
                    normal: normal_v4,
                });
                mesh.vertices.push(Vertex {
                    position: v3,
                    uv: vec2(0., 1.),
                    color: color_bytes,
                    normal: normal_v4,
                });

                // Tri 1
                mesh.indices.push(base_idx);
                mesh.indices.push(base_idx + 1);
                mesh.indices.push(base_idx + 2);

                // Tri 2
                mesh.indices.push(base_idx);
                mesh.indices.push(base_idx + 2);
                mesh.indices.push(base_idx + 3);
            }
        }

        draw_mesh(&mesh);

        // 3. Draw Wireframe (Creases)
        // Mountain vs Valley?
        // In Miura-ori:
        // Horizontal lines are ZigZag (Mountain/Valley alternating)
        // Vertical lines are all Mountain (or Valley depending on convention)
        // Actually, let's just draw lines.

        for j in 0..=rows {
            for i in 0..cols {
                let idx1 = j * width + i;
                let idx2 = j * width + i + 1;
                draw_line_3d(vertices[idx1], vertices[idx2], WHITE);
            }
        }
        for j in 0..rows {
            for i in 0..=cols {
                let idx1 = j * width + i;
                let idx2 = (j + 1) * width + i;
                draw_line_3d(vertices[idx1], vertices[idx2], WHITE);
            }
        }

        set_default_camera();

        draw_text("Rigid Origami Simulation 🛰️", 10.0, 30.0, 30.0, WHITE);
        draw_text(
            &format!(
                "Expansion: {:.2} {}",
                expansion,
                if deploying { "(Deploying...)" } else { "" }
            ),
            10.0,
            50.0,
            20.0,
            LIGHTGRAY,
        );
        draw_text(
            "Arrows: Manual | Space: Toggle Deploy | Mouse: Orbit",
            10.0,
            70.0,
            20.0,
            GRAY,
        );

        next_frame().await
    }
}
