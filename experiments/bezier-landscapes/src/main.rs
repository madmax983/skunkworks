mod text_mesh;
use macroquad::prelude::*;
use text_mesh::TextMesh;

#[macroquad::main("Bezier Landscapes")]
async fn main() {
    let font_path = "assets/font.ttf";
    let text = "GENESIS";

    let mut mesh_data = TextMesh::from_text(font_path, text).expect("Failed to generate mesh");

    // 1. Rotate to lie on ground (XZ plane)
    // Text X -> World X
    // Text Y -> World Z
    // Text Z -> World Y
    for v in &mut mesh_data.vertices {
        let x = v.position.x;
        let y_text = v.position.y; // Was -y from top-left origin usually, but we flipped it in text_mesh.rs
        let z_text = v.position.z;

        v.position.x = x;
        v.position.z = -y_text; // Flip Z to match reading direction?
                                // If Text Y was Up, and we map to Z, usually Z- is forward.
                                // Let's just try.
        v.position.y = z_text + 20.0; // Lift up so bottom is at 0
    }

    // 2. Apply distortion to Top Face (which is now Y=20)
    for v in &mut mesh_data.vertices {
        if v.position.y >= 19.0 {
            // Top face (allow float error)
            // Simple wave distortion
            let noise = (v.position.x * 0.1).sin() * (v.position.z * 0.1).cos() * 5.0;
            v.position.y += noise;

            // Color based on height
            if v.position.y > 22.0 {
                v.color = WHITE.into();
            } else if v.position.y > 18.0 {
                v.color = GREEN.into();
            } else {
                v.color = BROWN.into();
            }
        } else {
            // Walls/Bottom
            v.color = Color::new(0.5, 0.4, 0.3, 1.0).into();
        }
    }

    let mesh = Mesh {
        vertices: mesh_data.vertices,
        indices: mesh_data.indices,
        texture: None,
    };

    let mut cam_pos = vec3(0., 50., 100.);
    let mut cam_yaw: f32 = -1.57; // Look at text
    let mut cam_pitch: f32 = 0.0;

    loop {
        let dt = get_frame_time();
        let speed = 50.0 * dt;
        let rot_speed = 2.0 * dt;

        // Camera Movement
        let forward = vec3(
            cam_yaw.cos() * cam_pitch.cos(),
            cam_pitch.sin(),
            cam_yaw.sin() * cam_pitch.cos(),
        );
        // Actually right vector: cross(forward, up)
        let right = forward.cross(vec3(0., 1., 0.)).normalize();

        if is_key_down(KeyCode::W) {
            cam_pos += forward * speed;
        }
        if is_key_down(KeyCode::S) {
            cam_pos -= forward * speed;
        }
        if is_key_down(KeyCode::A) {
            cam_pos -= right * speed;
        }
        if is_key_down(KeyCode::D) {
            cam_pos += right * speed;
        }
        if is_key_down(KeyCode::Q) {
            cam_pos.y -= speed;
        }
        if is_key_down(KeyCode::E) {
            cam_pos.y += speed;
        }

        if is_key_down(KeyCode::Left) {
            cam_yaw -= rot_speed;
        }
        if is_key_down(KeyCode::Right) {
            cam_yaw += rot_speed;
        }
        if is_key_down(KeyCode::Up) {
            cam_pitch += rot_speed;
        }
        if is_key_down(KeyCode::Down) {
            cam_pitch -= rot_speed;
        }

        clear_background(SKYBLUE);

        set_camera(&Camera3D {
            position: cam_pos,
            up: vec3(0., 1., 0.),
            target: cam_pos + forward,
            ..Default::default()
        });

        draw_grid(20, 10., BLACK, GRAY);
        draw_mesh(&mesh);

        set_default_camera();

        draw_text("WASD+QE to move, Arrows to look", 10., 20., 20., BLACK);
        draw_text(
            &format!("Pos: {:.1}, {:.1}, {:.1}", cam_pos.x, cam_pos.y, cam_pos.z),
            10.,
            40.,
            20.,
            BLACK,
        );

        next_frame().await
    }
}
