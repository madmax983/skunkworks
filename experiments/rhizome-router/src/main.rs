use macroquad::prelude::*;
use rhizome_router::{Map, Rhizome};

#[macroquad::main("Rhizome Router")]
async fn main() {
    let width = 100;
    let height = 100;
    let mut map = Map::new(width, height);

    // Start in the center
    let mut rhizome = Rhizome::new(width / 2, height / 2, &mut map);

    // Auto-add some nutrients to start with
    map.add_nutrient(width / 4, height / 4, 15.0, 5.0);
    map.add_nutrient(width * 3 / 4, height * 3 / 4, 15.0, 5.0);

    let mut is_paused = false;

    loop {
        if is_key_pressed(KeyCode::Space) {
            is_paused = !is_paused;
        }

        // Interaction
        if is_mouse_button_down(MouseButton::Left) {
            let (mx, my) = mouse_position();
            let screen_w = screen_width();
            let screen_h = screen_height();

            // Map mouse to grid
            let cell_w = screen_w / width as f32;
            let cell_h = screen_h / height as f32;

            let gx = (mx / cell_w) as usize;
            let gy = (my / cell_h) as usize;

            if gx < width && gy < height {
                // Add nutrients continuously while holding
                map.add_nutrient(gx, gy, 8.0, 1.0);
            }
        }

        if !is_paused {
            rhizome.step(&mut map, 100); // Speed up growth
        }

        clear_background(BLACK);

        let screen_w = screen_width();
        let screen_h = screen_height();
        let cell_w = screen_w / width as f32;
        let cell_h = screen_h / height as f32;
        let time = get_time() as f32;

        // Draw Soil / Cost Map
        for y in 0..height {
            for x in 0..width {
                let cell = &map.cells[y][x];

                // If resistance is low (< 2.0), mix in blue.
                let color = if cell.resistance < 2.0 {
                    let t = (2.0 - cell.resistance) / 2.0;
                    let pulse = (time * 3.0 + x as f32 * 0.1).sin() * 0.2 + 0.8;
                    Color::new(
                        0.1 * (1.0 - t),
                        0.1 * (1.0 - t),
                        (0.4 * t + 0.2) * pulse,
                        1.0
                    )
                } else {
                    let brightness = 0.1 + (1.0 / cell.resistance) * 0.4;
                    // Earthy tones
                    Color::new(0.3 * brightness, 0.2 * brightness, 0.05 * brightness, 1.0)
                };

                draw_rectangle(
                    x as f32 * cell_w,
                    y as f32 * cell_h,
                    cell_w,
                    cell_h,
                    color
                );
            }
        }

        // Draw Roots
        for y in 0..height {
            for x in 0..width {
                let cell = &map.cells[y][x];
                if cell.visited {
                    if let Some((px, py)) = cell.parent {
                        // Organic jitter
                        let jitter_x = ((x as f32 * 13.1 + y as f32 * 17.7 + time).sin()) * cell_w * 0.2;
                        let jitter_y = ((x as f32 * 19.3 + y as f32 * 11.9).cos()) * cell_h * 0.2;

                        let start_pos = Vec2::new(
                            x as f32 * cell_w + cell_w/2.0 + jitter_x,
                            y as f32 * cell_h + cell_h/2.0 + jitter_y
                        );

                        let pjitter_x = ((px as f32 * 13.1 + py as f32 * 17.7 + time).sin()) * cell_w * 0.2;
                        let pjitter_y = ((px as f32 * 19.3 + py as f32 * 11.9).cos()) * cell_h * 0.2;

                        let end_pos = Vec2::new(
                            px as f32 * cell_w + cell_w/2.0 + pjitter_x,
                            py as f32 * cell_h + cell_h/2.0 + pjitter_y
                        );

                        let thickness = if cell.is_taproot { 2.5 } else { 0.8 };
                        // Taproots are gold, feeders are white/grey
                        let color = if cell.is_taproot {
                            Color::new(1.0, 0.8, 0.2, 1.0)
                        } else {
                            Color::new(0.8, 0.8, 0.7, 0.6)
                        };

                        draw_line(start_pos.x, start_pos.y, end_pos.x, end_pos.y, thickness, color);
                    }
                }
            }
        }

        draw_text("Rhizome Router", 20.0, 30.0, 40.0, WHITE);
        draw_text("Hold Left Click to water soil", 20.0, 60.0, 20.0, WHITE);
        draw_text("Space to Pause", 20.0, 80.0, 20.0, WHITE);

        next_frame().await
    }
}
