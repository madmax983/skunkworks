use macroquad::prelude::*;
use bio_transit::Simulation;

#[macroquad::main("Bio-Transit")]
async fn main() {
    // Simulation Grid Size (Independent of Screen Resolution)
    let grid_w = 400;
    let grid_h = 300;

    let num_agents = 50_000;

    let mut sim = Simulation::new(grid_w, grid_h, num_agents);

    // Adjust settings for better visuals with high agent count
    sim.settings.move_speed = 1.0;
    sim.settings.decay_rate = 0.95; // Slower decay for longer trails
    sim.settings.sensor_dist = 5.0;

    // Texture for visualization
    let mut image = Image::gen_image_color(grid_w as u16, grid_h as u16, BLACK);
    let texture = Texture2D::from_image(&image);
    texture.set_filter(FilterMode::Linear);

    loop {
        // Input
        if is_key_pressed(KeyCode::Q) {
            break;
        }

        // Logic
        sim.step();

        // Render
        clear_background(BLACK);

        // Update Texture from TrailMap
        // This loop is the bottleneck if serial.
        // We can parallelize image construction if needed, but Image::set_pixel is not thread-safe on shared image.
        // Instead, construct buffer in parallel then load to image.
        // For 400x300 = 120k pixels, serial is fast enough (~2ms).
        for (i, &val) in sim.map.grid.iter().enumerate() {
            let x = (i % grid_w) as u32;
            let y = (i / grid_w) as u32;

            // Color Mapping
            // Value 0..1 (can exceed 1.0 due to additive deposit, clamped in deposit but multiple agents can add up)
            // Wait, deposit uses min(1.0).
            // So value is 0..1.

            let v = val;

            // Palette: Deep Space -> Bioluminescent Blue -> White
            let color = if v < 0.05 {
                Color::new(0.0, 0.0, 0.1 * v * 20.0, 1.0) // Deep fade
            } else if v < 0.5 {
                // Blue-ish
                Color::new(0.0, v * 0.8, v + 0.2, 1.0)
            } else {
                // Cyan to White
                Color::new((v - 0.5) * 2.0, 1.0, 1.0, 1.0)
            };

            image.set_pixel(x, y, color);
        }

        texture.update(&image);

        // Draw Map Scaled to Screen
        let screen_w = screen_width();
        let screen_h = screen_height();

        draw_texture_ex(&texture, 0.0, 0.0, WHITE, DrawTextureParams {
            dest_size: Some(vec2(screen_w, screen_h)),
            ..Default::default()
        });

        // Draw Cities
        let scale_x = screen_w / grid_w as f32;
        let scale_y = screen_h / grid_h as f32;

        for city in &sim.cities {
            let pos = city.pos; // In grid coords
            let screen_pos = vec2(pos.x * scale_x, pos.y * scale_y);
            let radius = city.radius * scale_x; // Scale radius

            // Draw Glow
            draw_circle(screen_pos.x, screen_pos.y, radius * 1.5, Color::new(city.color.r, city.color.g, city.color.b, 0.3));
            draw_circle(screen_pos.x, screen_pos.y, radius, city.color);
            draw_circle_lines(screen_pos.x, screen_pos.y, radius, 2.0, WHITE);
        }

        // UI
        draw_text("BIO-TRANSIT // SLIME MOLD URBANISM", 20.0, 30.0, 30.0, WHITE);
        draw_text(&format!("FPS: {}", get_fps()), 20.0, 60.0, 20.0, LIGHTGRAY);
        draw_text(&format!("AGENTS: {}", sim.agents.len()), 20.0, 80.0, 20.0, LIGHTGRAY);
        draw_text("Q: Quit", 20.0, screen_h - 20.0, 20.0, DARKGRAY);

        next_frame().await
    }
}
