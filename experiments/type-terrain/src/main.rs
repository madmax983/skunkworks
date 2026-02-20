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

    let font_bytes =
        font_bytes.expect("Failed to load font.ttf. Run from experiments/type-terrain or root.");

    // Generate Terrain
    let mut current_text = String::from("GENESIS");
    let mut terrain = FontTerrain::new(&font_bytes, &current_text, 100.0);
    let mut mesh = terrain.to_mesh();

    // Camera setup
    let mut cam_pos = vec3(
        terrain.width as f32 / 2.0,
        50.0,
        terrain.height as f32 / 2.0 + 50.0,
    );
    let mut cam_yaw: f32 = 0.0;
    let mut cam_pitch: f32 = -0.5;

    let mut erosion_active = false;
    let mut total_erosion_steps = 0;
    let mut last_mesh_update = 0.0;

    // Interactive Text Input
    let mut typing_mode = false;
    let mut input_text = String::new();

    loop {
        // Input
        let speed = 1.0; // Normal speed
        let fast_speed = 5.0; // Shift for speed
        let rot_speed = 0.02;

        let move_speed = if is_key_down(KeyCode::LeftShift) {
            fast_speed
        } else {
            speed
        };

        if !typing_mode {
            if is_key_down(KeyCode::W) {
                cam_pos += vec3(cam_yaw.sin(), 0.0, cam_yaw.cos()) * move_speed;
            }
            if is_key_down(KeyCode::S) {
                cam_pos -= vec3(cam_yaw.sin(), 0.0, cam_yaw.cos()) * move_speed;
            }
            if is_key_down(KeyCode::A) {
                cam_pos += vec3(cam_yaw.cos(), 0.0, -cam_yaw.sin()) * move_speed;
            }
            if is_key_down(KeyCode::D) {
                cam_pos -= vec3(cam_yaw.cos(), 0.0, -cam_yaw.sin()) * move_speed;
            }
            if is_key_down(KeyCode::Q) {
                cam_pos.y -= move_speed;
            }
            if is_key_down(KeyCode::E) {
                cam_pos.y += move_speed;
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
                erosion_active = !erosion_active;
            }

            if is_key_pressed(KeyCode::Enter) {
                typing_mode = true;
                input_text = current_text.clone();
            }
        } else {
            // Typing mode
            if let Some(char) = get_char_pressed() {
                if char >= ' ' && char <= '~' {
                    input_text.push(char);
                }
            }
            if is_key_pressed(KeyCode::Backspace) {
                input_text.pop();
            }
            if is_key_pressed(KeyCode::Enter) {
                typing_mode = false;
                if !input_text.is_empty() {
                    current_text = input_text.clone();
                    // Regenerate terrain
                    terrain = FontTerrain::new(&font_bytes, &current_text, 100.0);
                    mesh = terrain.to_mesh();
                    total_erosion_steps = 0;
                    // Reset camera slightly if needed, or keep it
                }
            }
            if is_key_pressed(KeyCode::Escape) {
                typing_mode = false;
            }
        }

        // Logic
        if erosion_active {
            // Run multiple steps per frame for speed
            let steps = 100;
            terrain.erode(steps);
            total_erosion_steps += steps;

            // Update mesh periodically (every 0.1s or so to save CPU)
            if get_time() - last_mesh_update > 0.1 {
                mesh = terrain.to_mesh();
                last_mesh_update = get_time();
            }
        }

        // Render
        clear_background(Color::new(0.1, 0.1, 0.12, 1.0)); // Dark sky

        // 3D Scene
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

        draw_grid(100, 10., GRAY, DARKGRAY);

        // Draw mesh
        draw_mesh(&mesh);

        // UI Overlay
        set_default_camera();

        let ui_y = 20.0;
        let line_h = 25.0;

        draw_text(&format!("FPS: {}", get_fps()), 10.0, ui_y, 30.0, WHITE);
        draw_text(
            &format!("Terrain: {}", current_text),
            10.0,
            ui_y + line_h,
            30.0,
            YELLOW,
        );

        if typing_mode {
            draw_rectangle(
                0.0,
                ui_y + line_h * 2.0 - 20.0,
                screen_width(),
                40.0,
                Color::new(0.0, 0.0, 0.0, 0.8),
            );
            draw_text(
                &format!("Type new text: {}_", input_text),
                10.0,
                ui_y + line_h * 2.0,
                30.0,
                WHITE,
            );
        } else {
            draw_text(
                "WASD+Arrows to move/look. Shift for speed.",
                10.0,
                ui_y + line_h * 2.0,
                20.0,
                LIGHTGRAY,
            );
            draw_text(
                "SPACE: Toggle Erosion",
                10.0,
                ui_y + line_h * 3.0,
                20.0,
                if erosion_active { GREEN } else { RED },
            );
            draw_text(
                "ENTER: Change Text",
                10.0,
                ui_y + line_h * 4.0,
                20.0,
                LIGHTGRAY,
            );
            draw_text(
                &format!("Erosion Steps: {}", total_erosion_steps),
                10.0,
                ui_y + line_h * 5.0,
                20.0,
                SKYBLUE,
            );
        }

        next_frame().await
    }
}
