mod terrain;

use macroquad::prelude::*;
use terrain::{ErosionParams, HeightMap};

#[macroquad::main("Glyph Erosion")]
async fn main() {
    // Load font
    // Try to find the font file in a few common locations
    let possible_paths = [
        "assets/font.ttf",
        "experiments/type-terrain/assets/font.ttf",
        "../assets/font.ttf",
        "../experiments/type-terrain/assets/font.ttf",
    ];

    let mut font_bytes = None;
    for path in possible_paths {
        if let Ok(bytes) = load_file(path).await {
            font_bytes = Some(bytes);
            println!("Loaded font from {}", path);
            break;
        }
    }

    let font_bytes =
        font_bytes.expect("Failed to load font.ttf. Run from workspace root or experiment folder.");

    // Generate Terrain
    let text = "EROSION";
    let mut terrain = HeightMap::from_text(&font_bytes, text, 100.0, 40);
    let mut mesh = terrain.to_mesh();
    let params = ErosionParams::default();

    // Camera setup
    let mut cam_pos = vec3(
        terrain.width as f32 / 2.0,
        150.0,
        terrain.height as f32 / 2.0 + 150.0,
    );
    let mut cam_yaw: f32 = 0.0;
    let mut cam_pitch: f32 = -0.7;
    let mut cam_target = vec3(terrain.width as f32 / 2.0, 0.0, terrain.height as f32 / 2.0);

    let mut eroding = false;

    loop {
        // Input
        let speed = 2.0;
        let rot_speed = 0.02;

        if is_key_down(KeyCode::W) {
            let forward = vec3(cam_yaw.sin(), 0.0, cam_yaw.cos());
            cam_pos += forward * speed;
            cam_target += forward * speed;
        }
        if is_key_down(KeyCode::S) {
            let forward = vec3(cam_yaw.sin(), 0.0, cam_yaw.cos());
            cam_pos -= forward * speed;
            cam_target -= forward * speed;
        }
        if is_key_down(KeyCode::A) {
            let right = vec3(cam_yaw.cos(), 0.0, -cam_yaw.sin());
            cam_pos += right * speed;
            cam_target += right * speed;
        }
        if is_key_down(KeyCode::D) {
            let right = vec3(cam_yaw.cos(), 0.0, -cam_yaw.sin());
            cam_pos -= right * speed;
            cam_target -= right * speed;
        }
        if is_key_down(KeyCode::Q) {
            cam_pos.y -= speed;
        }
        if is_key_down(KeyCode::E) {
            cam_pos.y += speed;
        }

        if is_key_down(KeyCode::Left) {
            cam_yaw += rot_speed;
        }
        if is_key_down(KeyCode::Right) {
            cam_yaw -= rot_speed;
        }
        if is_key_down(KeyCode::Up) {
            cam_pitch += rot_speed;
        }
        if is_key_down(KeyCode::Down) {
            cam_pitch -= rot_speed;
        }

        if is_key_pressed(KeyCode::Space) {
            eroding = !eroding;
        }

        if is_key_pressed(KeyCode::R) {
            terrain = HeightMap::from_text(&font_bytes, text, 100.0, 40);
            mesh = terrain.to_mesh();
        }

        // Erode
        if eroding {
            terrain.erode(2000, &params); // 2000 droplets per frame
            mesh = terrain.to_mesh(); // Re-generate mesh (slow but accurate)
        }

        // Render
        clear_background(LIGHTGRAY);

        // Update camera position based on orbit if desired, but here we use free cam
        // Let's make camera look at target
        // Actually, let's keep the existing logic but just use `look_at` implicitly via `target`.

        set_camera(&Camera3D {
            position: cam_pos,
            up: vec3(0., 1., 0.),
            target: cam_pos
                + vec3(
                    cam_yaw.sin() * cam_pitch.cos(),
                    cam_pitch.sin(),
                    cam_yaw.cos() * cam_pitch.cos(),
                ),
            ..Default::default()
        });

        draw_grid(20, 10., BLACK, GRAY);

        draw_mesh(&mesh);

        set_default_camera();

        draw_text(&format!("FPS: {}", get_fps()), 10.0, 20.0, 30.0, BLACK);
        draw_text(
            "WASD: Move | Arrows: Look | Q/E: Up/Down",
            10.0,
            50.0,
            20.0,
            DARKGRAY,
        );
        draw_text(
            &format!(
                "SPACE: Toggle Erosion ({})",
                if eroding { "ON" } else { "OFF" }
            ),
            10.0,
            80.0,
            20.0,
            if eroding { RED } else { DARKGRAY },
        );
        draw_text("R: Reset", 10.0, 110.0, 20.0, DARKGRAY);

        next_frame().await
    }
}
