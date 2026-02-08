mod distance;
mod terrain;
mod text;

use macroquad::prelude::*;

const MAP_WIDTH: usize = 200;
const MAP_HEIGHT: usize = 200;

fn window_conf() -> Conf {
    Conf {
        window_title: "Text Erosion".to_owned(),
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let font_data = include_bytes!("../assets/font.ttf");

    let mut current_text = String::from("EROSION");

    // Initial SDF
    let grid = text::rasterize_text(&current_text, font_data, MAP_WIDTH, MAP_HEIGHT);
    let mut sdf_grid = distance::compute_sdf(MAP_WIDTH, MAP_HEIGHT, &grid);
    let mut needs_sdf_update = false;

    // Camera
    let mut cam_angle_x: f32 = 0.8;
    let mut cam_angle_y: f32 = 0.5;
    let mut cam_dist: f32 = 120.0;

    loop {
        // Input Handling
        while let Some(c) = get_char_pressed() {
            // Filter control characters
            if !c.is_control() {
                current_text.push(c);
                needs_sdf_update = true;
            }
        }

        // Handle Backspace
        if is_key_pressed(KeyCode::Backspace) {
            current_text.pop();
            needs_sdf_update = true;
        }

        // Update SDF if needed
        if needs_sdf_update {
            // Avoid empty string causing issues
            let text_to_render = if current_text.is_empty() {
                " "
            } else {
                &current_text
            };
            let grid = text::rasterize_text(text_to_render, font_data, MAP_WIDTH, MAP_HEIGHT);
            sdf_grid = distance::compute_sdf(MAP_WIDTH, MAP_HEIGHT, &grid);
            needs_sdf_update = false;
        }

        // Update Mesh
        let time = get_time();
        let mesh = terrain::generate_mesh(MAP_WIDTH, MAP_HEIGHT, &sdf_grid, time);

        // Render
        clear_background(SKYBLUE);

        // Camera Controls
        if is_key_down(KeyCode::Left) {
            cam_angle_y -= 0.02;
        }
        if is_key_down(KeyCode::Right) {
            cam_angle_y += 0.02;
        }
        if is_key_down(KeyCode::Up) {
            cam_angle_x = (cam_angle_x + 0.02).min(1.5);
        }
        if is_key_down(KeyCode::Down) {
            cam_angle_x = (cam_angle_x - 0.02).max(0.1);
        }
        if is_key_down(KeyCode::W) {
            cam_dist -= 1.0;
        }
        if is_key_down(KeyCode::S) {
            cam_dist += 1.0;
        }

        let cam_pos = vec3(
            cam_dist * cam_angle_x.cos() * cam_angle_y.sin(),
            cam_dist * cam_angle_x.sin(),
            cam_dist * cam_angle_x.cos() * cam_angle_y.cos(),
        );

        set_camera(&Camera3D {
            position: cam_pos,
            target: vec3(0.0, 0.0, 0.0),
            up: vec3(0.0, 1.0, 0.0),
            ..Default::default()
        });

        draw_mesh(&mesh);
        draw_grid(20, 10.0, BLACK, GRAY);

        set_default_camera();

        draw_text("Type to Erode Terrain", 10.0, 30.0, 30.0, BLACK);
        draw_text(
            &format!("Current: {}", current_text),
            10.0,
            60.0,
            20.0,
            DARKGRAY,
        );
        draw_text("ARROWS/WASD to move camera", 10.0, 80.0, 20.0, DARKGRAY);
        draw_text(&format!("FPS: {}", get_fps()), 10.0, 100.0, 20.0, DARKGRAY);

        next_frame().await
    }
}
