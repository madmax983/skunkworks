mod simulation;

use macroquad::prelude::*;
use simulation::World;

#[macroquad::main("Firefly Synapse")]
async fn main() {
    // Window configuration
    // Try to set a reasonable window size
    request_new_screen_size(1000.0, 1000.0);

    let mut world = World::new();

    // Texture setup
    // We use a fixed internal resolution for the simulation view
    let width = 1000;
    let height = 1000;
    let mut image = Image::gen_image_color(width as u16, height as u16, BLACK);
    let texture = Texture2D::from_image(&image);

    loop {
        clear_background(BLACK);

        // Update Physics
        world.update();

        // Render to CPU buffer
        // We pass the raw bytes slice to our renderer
        world.render_to_buffer(&mut image.bytes, width, height);

        // Upload to GPU
        texture.update(&image);

        // Draw to screen (stretch to fit window)
        draw_texture_ex(
            &texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );

        // UI
        draw_text(&format!("FPS: {}", get_fps()), 10.0, 30.0, 30.0, WHITE);
        draw_text(&format!("Agents: {}", world.agents.len()), 10.0, 60.0, 30.0, WHITE);
        draw_text("Firefly Synapse: Emergent Synchronization", 10.0, screen_height() - 20.0, 20.0, LIGHTGRAY);

        next_frame().await
    }
}
