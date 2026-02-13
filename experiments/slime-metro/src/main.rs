use macroquad::prelude::*;
use crate::simulation::Simulation;

mod agent;
mod map;
mod simulation;

#[macroquad::main("Slime Metro")]
async fn main() {
    let width = 1024;
    let height = 1024;
    request_new_screen_size(width as f32, height as f32);

    let num_agents = 200_000;
    let mut sim = Simulation::new(width, height, num_agents);
    let mut paused = false;

    loop {
        // Input
        if is_key_pressed(KeyCode::Space) {
            paused = !paused;
        }
        if is_key_pressed(KeyCode::R) {
            sim = Simulation::new(width, height, num_agents);
        }

        // Update
        if !paused {
            sim.update();
        }

        // Render
        clear_background(BLACK);

        // Draw Map Texture
        // Center on screen
        let screen_w = screen_width();
        let screen_h = screen_height();

        let scale = (screen_w / width as f32).min(screen_h / height as f32);
        let dest_w = width as f32 * scale;
        let dest_h = height as f32 * scale;
        let dest_x = (screen_w - dest_w) / 2.0;
        let dest_y = (screen_h - dest_h) / 2.0;

        draw_texture_ex(
            &sim.map.texture,
            dest_x,
            dest_y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(dest_w, dest_h)),
                ..Default::default()
            },
        );

        // Draw Stations (Food Sources) for debugging/visuals
        // They are already baked into texture as Red channel, but maybe draw circles?
        // No, texture is enough.

        // UI
        draw_text("Slime Metro: Urban Transit Simulation", 20.0, 30.0, 30.0, WHITE);
        draw_text(&format!("FPS: {}", get_fps()), 20.0, 60.0, 20.0, WHITE);
        draw_text(&format!("Agents: {}", num_agents), 20.0, 80.0, 20.0, WHITE);
        draw_text("Space: Pause | R: Reset", 20.0, 100.0, 20.0, GRAY);

        next_frame().await
    }
}
