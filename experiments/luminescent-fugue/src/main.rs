mod simulation;

use macroquad::prelude::*;
use simulation::World;

#[macroquad::main("Luminescent Fugue")]
async fn main() {
    // Window configuration
    request_new_screen_size(1000.0, 1000.0);

    let mut world = World::new();

    // Texture setup
    let width = 1000;
    let height = 1000;
    let mut image = Image::gen_image_color(width as u16, height as u16, BLACK);
    let texture = Texture2D::from_image(&image);

    // Inject some chaos at start
    world.inject_chaos();

    loop {
        clear_background(BLACK);

        // Input
        if is_key_pressed(KeyCode::R) {
            world.reset();
        }
        if is_key_pressed(KeyCode::C) {
            world.inject_chaos();
        }

        // Mouse interaction: Inject signal
        if is_mouse_button_down(MouseButton::Left) {
            let (mx, my) = mouse_position();
            // Map screen to world
            // Assuming full screen fit
            let scale_x = world.width / screen_width();
            let scale_y = world.height / screen_height();

            world.inject_signal(mx * scale_x, my * scale_y, 50.0, [1.0, 1.0, 1.0]);
            // White signal
        }

        // Update Physics
        world.update();

        // Render to CPU buffer
        world.render_to_buffer(&mut image.bytes, width, height);

        // Upload to GPU
        texture.update(&image);

        // Draw to screen
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
        draw_text(
            &format!("Agents: {}", world.agents.len()),
            10.0,
            60.0,
            30.0,
            WHITE,
        );
        let consensus = world.calculate_consensus_metric();
        draw_text(
            &format!("Consensus Deviation: {:.4}", consensus),
            10.0,
            90.0,
            30.0,
            if consensus < 0.1 { GREEN } else { RED },
        );

        draw_text(
            "Luminescent Fugue: Color Consensus via Pulse Coupling",
            10.0,
            screen_height() - 20.0,
            20.0,
            LIGHTGRAY,
        );
        draw_text(
            "[R] Reset  [C] Chaos  [Click] Inject Signal",
            10.0,
            screen_height() - 40.0,
            20.0,
            LIGHTGRAY,
        );

        next_frame().await
    }
}
