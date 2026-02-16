mod grid;

use grid::Grid;
use macroquad::prelude::*;
use ::rand::Rng; // Use the external rand crate

const GRID_WIDTH: usize = 512;
const GRID_HEIGHT: usize = 512;

#[macroquad::main("Sandpile Scheduler")]
async fn main() {
    let mut grid = Grid::new(GRID_WIDTH, GRID_HEIGHT);

    // Texture setup
    let mut image = Image::gen_image_color(GRID_WIDTH as u16, GRID_HEIGHT as u16, BLACK);
    let texture = Texture2D::from_image(&image);
    texture.set_filter(FilterMode::Nearest);

    let mut ddos_mode = false;
    let mut paused = false;
    let mut process_rate = 0.05; // 5% chance to process a task per tick

    loop {
        // Input
        if is_key_pressed(KeyCode::Space) {
            paused = !paused;
        }
        if is_key_pressed(KeyCode::D) {
            ddos_mode = !ddos_mode;
        }
        if is_key_pressed(KeyCode::R) {
            grid = Grid::new(GRID_WIDTH, GRID_HEIGHT);
        }
        if is_key_down(KeyCode::Up) {
            process_rate = (process_rate + 0.001f64).min(1.0f64);
        }
        if is_key_down(KeyCode::Down) {
            process_rate = (process_rate - 0.001f64).max(0.0f64);
        }

        // Mouse
        if is_mouse_button_down(MouseButton::Left) {
            let (mx, my) = mouse_position();
            // Map mouse to grid
            let sw = screen_width();
            let sh = screen_height();
            // Avoid division by zero
            if sw > 0.0 && sh > 0.0 {
                let gx = (mx / sw * GRID_WIDTH as f32) as usize;
                let gy = (my / sh * GRID_HEIGHT as f32) as usize;

                // Add a burst of tasks
                grid.add_load(gx, gy, 50);
            }
        }

        // Update
        if !paused {
            let mut rng = ::rand::thread_rng();
            if ddos_mode {
                // Drop random tasks everywhere
                for _ in 0..100 {
                    let rx = rng.gen_range(0..GRID_WIDTH);
                    let ry = rng.gen_range(0..GRID_HEIGHT);
                    grid.add_load(rx, ry, 1);
                }
                // Focused attack on center
                grid.add_load(GRID_WIDTH/2, GRID_HEIGHT/2, 10);
            }

            grid.update(process_rate);
        }

        // Render
        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                let idx = grid.get_index(x, y);
                let cell = grid.cells[idx];
                let color = match cell.load {
                    0 => BLACK,
                    1 => Color::new(0.0, 0.0, 0.2, 1.0), // Dark Blue
                    2 => Color::new(0.0, 0.0, 0.6, 1.0), // Blue
                    3 => Color::new(0.0, 0.5, 1.0, 1.0), // Sky Blue
                    _ => {
                        // Overloaded!
                        if cell.load < 10 {
                            WHITE
                        } else {
                            RED // Critical
                        }
                    }
                };
                image.set_pixel(x as u32, y as u32, color);
            }
        }

        texture.update(&image);

        clear_background(BLACK);
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
        draw_text("Sandpile Scheduler", 10.0, 20.0, 30.0, WHITE);
        draw_text(&format!("FPS: {}", get_fps()), 10.0, 50.0, 20.0, WHITE);
        draw_text(&format!("Total Load: {}", grid.total_load()), 10.0, 70.0, 20.0, WHITE);
        draw_text(&format!("Process Rate: {:.3}", process_rate), 10.0, 90.0, 20.0, WHITE);
        draw_text(
            if ddos_mode { "DDoS: ON" } else { "DDoS: OFF" },
            10.0, 110.0, 20.0,
            if ddos_mode { RED } else { GREEN }
        );
        draw_text("Controls: [Space] Pause, [D] DDoS, [R] Reset, [Click] Add Load", 10.0, screen_height() - 20.0, 20.0, LIGHTGRAY);

        next_frame().await
    }
}
