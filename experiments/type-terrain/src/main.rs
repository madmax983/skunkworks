mod sdf;
mod terrain;

use macroquad::prelude::*;
use terrain::FontTerrain;

#[macroquad::main("Type Terrain")]
async fn main() {
    // Load font
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

    // Default font if not found?
    // We can panic, or try to load a system font?
    // Let's just panic with a clear message.
    let font_bytes = font_bytes.expect("Failed to load font.ttf. Please ensure 'assets/font.ttf' exists in the experiment directory or root assets.");

    // Generate Terrain
    let text = "GENESIS";
    let terrain = FontTerrain::new(&font_bytes, text, 100.0);
    let mesh = terrain.to_mesh();

    // Camera setup
    let mut position = vec3(
        terrain.width as f32 / 2.0,
        50.0,
        terrain.height as f32 / 2.0 + 50.0,
    );
    let mut yaw: f32 = 0.0;
    let mut pitch: f32 = -0.5;

    let look_speed = 0.04;
    let mut move_speed = 1.0;

    // Water plane
    let water_mesh = create_water_mesh(terrain.width as f32, terrain.height as f32);

    loop {
        // Input
        if is_key_down(KeyCode::LeftShift) {
            move_speed = 5.0;
        } else {
            move_speed = 1.0;
        }

        let front = vec3(yaw.sin() * pitch.cos(), pitch.sin(), yaw.cos() * pitch.cos()).normalize();
        let right = vec3(yaw.cos(), 0.0, -yaw.sin()).normalize();

        if is_key_down(KeyCode::W) {
            position += front * move_speed;
        }
        if is_key_down(KeyCode::S) {
            position -= front * move_speed;
        }
        if is_key_down(KeyCode::A) {
            position += right * move_speed;
        }
        if is_key_down(KeyCode::D) {
            position -= right * move_speed;
        }
        if is_key_down(KeyCode::Q) {
            position.y -= move_speed;
        }
        if is_key_down(KeyCode::E) {
            position.y += move_speed;
        }

        if is_key_down(KeyCode::Left) {
            yaw += look_speed;
        }
        if is_key_down(KeyCode::Right) {
            yaw -= look_speed;
        }
        if is_key_down(KeyCode::Up) {
            pitch += look_speed;
        }
        if is_key_down(KeyCode::Down) {
            pitch -= look_speed;
        }

        pitch = pitch.clamp(-1.5, 1.5);

        // Render
        clear_background(SKYBLUE);

        set_camera(&Camera3D {
            position,
            up: vec3(0., 1., 0.),
            target: position + front,
            ..Default::default()
        });

        draw_grid(20, 10., BLACK, GRAY);

        // Draw terrain (Opaque)
        draw_mesh(&mesh);

        // Draw water (Transparent)
        // Macroquad doesn't automatically enable blending for 3D primitives unless material is set?
        // Or if we use `draw_mesh` it uses default material.
        // Let's assume default material handles alpha or just draw it.
        draw_mesh(&water_mesh);

        set_default_camera();

        draw_text(&format!("FPS: {}", get_fps()), 10.0, 20.0, 30.0, BLACK);
        draw_text("WASD to move, Arrows to look, Shift to sprint", 10.0, 50.0, 20.0, DARKGRAY);

        next_frame().await
    }
}

fn create_water_mesh(w: f32, h: f32) -> Mesh {
    let water_level = 0.5;
    let vertices = vec![
        Vertex { position: vec3(0.0, water_level, 0.0), uv: vec2(0., 0.), color: Color::new(0.0, 0.0, 1.0, 0.5).into(), normal: vec4(0., 1., 0., 0.) },
        Vertex { position: vec3(w, water_level, 0.0), uv: vec2(1., 0.), color: Color::new(0.0, 0.0, 1.0, 0.5).into(), normal: vec4(0., 1., 0., 0.) },
        Vertex { position: vec3(w, water_level, h), uv: vec2(1., 1.), color: Color::new(0.0, 0.0, 1.0, 0.5).into(), normal: vec4(0., 1., 0., 0.) },
        Vertex { position: vec3(0.0, water_level, h), uv: vec2(0., 1.), color: Color::new(0.0, 0.0, 1.0, 0.5).into(), normal: vec4(0., 1., 0., 0.) },
    ];
    let indices = vec![0, 1, 2, 0, 2, 3];

    Mesh {
        vertices,
        indices,
        texture: None,
    }
}
