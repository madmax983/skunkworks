mod terrain;

use macroquad::prelude::*;
use terrain::FontTerrain;

#[macroquad::main("Type Terrain")]
async fn main() {
    // Load font
    // Try to find the font file in a few common locations
    let possible_paths = [
        "assets/font.ttf",
        "experiments/type-terrain/assets/font.ttf",
        "../assets/font.ttf",
    ];

    let mut font_bytes = None;
    for path in possible_paths {
        if let Ok(bytes) = load_file(path).await {
            font_bytes = Some(bytes);
            println!("Loaded font from {}", path);
            break;
        }
    }

    let font_bytes = font_bytes.expect("Failed to load font.ttf. Run from experiments/type-terrain or root.");

    // Generate Terrain
    let text = "GENESIS";
    let terrain = FontTerrain::new(&font_bytes, text, 100.0);
    let mesh = terrain.to_mesh();

    // Camera setup
    let mut cam_pos = vec3(terrain.width as f32 / 2.0, 50.0, terrain.height as f32 / 2.0 + 50.0);
    let mut cam_yaw: f32 = 0.0;
    let mut cam_pitch: f32 = -0.5;

    loop {
        // Input
        let speed = 1.0;
        let rot_speed = 0.02;

        if is_key_down(KeyCode::W) {
            cam_pos += vec3(cam_yaw.sin(), 0.0, cam_yaw.cos()) * speed;
        }
        if is_key_down(KeyCode::S) {
            cam_pos -= vec3(cam_yaw.sin(), 0.0, cam_yaw.cos()) * speed;
        }
        if is_key_down(KeyCode::A) {
            cam_pos += vec3(cam_yaw.cos(), 0.0, -cam_yaw.sin()) * speed;
        }
        if is_key_down(KeyCode::D) {
            cam_pos -= vec3(cam_yaw.cos(), 0.0, -cam_yaw.sin()) * speed;
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

        // Render
        clear_background(LIGHTGRAY);

        set_camera(&Camera3D {
            position: cam_pos,
            up: vec3(0., 1., 0.),
            target: cam_pos + vec3(cam_yaw.sin() * cam_pitch.cos(), cam_pitch.sin(), cam_yaw.cos() * cam_pitch.cos()),
            ..Default::default()
        });

        draw_grid(20, 10., BLACK, GRAY);

        // Draw mesh
        // macroquad's draw_mesh consumes the mesh? No, it takes a reference in 0.4.
        // Wait, macroquad 0.4 `draw_mesh` takes `&Mesh`.
        draw_mesh(&mesh);

        set_default_camera();

        draw_text(&format!("FPS: {}", get_fps()), 10.0, 20.0, 30.0, BLACK);
        draw_text("WASD to move, Arrows to look", 10.0, 50.0, 20.0, DARKGRAY);

        next_frame().await
    }
}
