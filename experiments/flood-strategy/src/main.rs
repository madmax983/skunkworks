mod fluid;
mod game;

use macroquad::prelude::*;
use game::Game;

const GRID_WIDTH: usize = 128;
const GRID_HEIGHT: usize = 128;
const SIM_STEPS_PER_FRAME: usize = 4;
const DT: f32 = 0.05;

#[macroquad::main("Flood Strategy")]
async fn main() {
    let mut game = Game::new(GRID_WIDTH, GRID_HEIGHT);

    // Initialize terrain with some noise or slope
    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            let idx = game.fluid.index(x, y);
            // Simple slope
            let h = (x as f32 / GRID_WIDTH as f32) * 5.0;
            // Add noise
            let noise = rand::gen_range(0.0, 2.0);
            game.fluid.terrain[idx] = h + noise;
        }
    }

    // Spawn units
    for _ in 0..20 {
        let x = rand::gen_range(0, 10);
        let y = rand::gen_range(0, GRID_HEIGHT);
        let goal_x = GRID_WIDTH - 5;
        let goal_y = rand::gen_range(0, GRID_HEIGHT);
        game.spawn_unit(x, y, goal_x, goal_y);
    }

    // Texture for rendering
    let mut image = Image::gen_image_color(GRID_WIDTH as u16, GRID_HEIGHT as u16, BLACK);
    let texture = Texture2D::from_image(&image);
    texture.set_filter(FilterMode::Nearest);

    let mut running = true;
    let mut tide_enabled = true;
    let mut time = 0.0;
    let mut unit_timer = 0.0;
    let unit_step_interval = 0.2; // Units move every 0.2s

    loop {
        // Handle Input
        if is_key_pressed(KeyCode::Space) {
            running = !running;
        }
        if is_key_pressed(KeyCode::T) {
            tide_enabled = !tide_enabled;
        }
        if is_key_pressed(KeyCode::R) {
             // Reset
             game = Game::new(GRID_WIDTH, GRID_HEIGHT);
             // Re-init terrain...
             for i in 0..game.fluid.terrain.len() {
                 game.fluid.terrain[i] = rand::gen_range(0.0, 5.0);
             }
             // Spawn units
             for _ in 0..20 {
                game.spawn_unit(rand::gen_range(0, 10), rand::gen_range(0, GRID_HEIGHT), GRID_WIDTH-5, rand::gen_range(0, GRID_HEIGHT));
             }
        }

        let mouse_pos = mouse_position();
        let screen_w = screen_width();
        let screen_h = screen_height();
        let scale_x = screen_w / GRID_WIDTH as f32;
        let scale_y = screen_h / GRID_HEIGHT as f32;

        let mx = (mouse_pos.0 / scale_x) as usize;
        let my = (mouse_pos.1 / scale_y) as usize;

        if mx < GRID_WIDTH && my < GRID_HEIGHT {
            let idx = game.fluid.index(mx, my);

            if is_mouse_button_down(MouseButton::Left) {
                // Raise terrain (Brush size 3)
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        let nx = mx as i32 + dx;
                        let ny = my as i32 + dy;
                        if nx >= 0 && nx < GRID_WIDTH as i32 && ny >= 0 && ny < GRID_HEIGHT as i32 {
                            let n_idx = game.fluid.index(nx as usize, ny as usize);
                            game.fluid.terrain[n_idx] += 0.5;
                        }
                    }
                }
            }
            if is_mouse_button_down(MouseButton::Right) {
                // Lower terrain
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        let nx = mx as i32 + dx;
                        let ny = my as i32 + dy;
                        if nx >= 0 && nx < GRID_WIDTH as i32 && ny >= 0 && ny < GRID_HEIGHT as i32 {
                            let n_idx = game.fluid.index(nx as usize, ny as usize);
                            game.fluid.terrain[n_idx] -= 0.5;
                            if game.fluid.terrain[n_idx] < 0.0 { game.fluid.terrain[n_idx] = 0.0; }
                        }
                    }
                }
            }
            if is_mouse_button_down(MouseButton::Middle) {
                // Add Water
                game.fluid.water[idx] += 5.0;
            }
        }

        if running {
            // Physics Steps
            for _ in 0..SIM_STEPS_PER_FRAME {
                game.fluid.step(DT);

                // Tide Logic
                if tide_enabled {
                    time += DT;
                    let tide_level = (time * 0.5).sin() * 5.0 + 5.0; // Oscillates 0 to 10

                    // Apply tide to Left boundary
                    for y in 0..GRID_HEIGHT {
                        let idx = game.fluid.index(0, y);
                        // If water level < tide_level, add water
                        let current_h = game.fluid.terrain[idx] + game.fluid.water[idx];
                        if current_h < tide_level {
                            game.fluid.water[idx] += (tide_level - current_h) * 0.1;
                        }
                    }
                }
            }

            // Unit Steps
            unit_timer += get_frame_time();
            if unit_timer > unit_step_interval {
                unit_timer = 0.0;
                game.update_units();
            }
        }

        // Rendering
        // Update Image
        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                let idx = game.fluid.index(x, y);
                let terrain_h = game.fluid.terrain[idx];
                let water_h = game.fluid.water[idx];

                // Terrain Color (Brown/Green based on height)
                let t_col = if terrain_h < 2.0 {
                    Color::new(0.4, 0.3, 0.1, 1.0) // Low ground
                } else if terrain_h < 8.0 {
                    Color::new(0.2, 0.6, 0.2, 1.0) // Grass
                } else {
                    Color::new(0.5, 0.5, 0.5, 1.0) // Rock
                };

                // Water Overlay
                let mut color = t_col;
                if water_h > 0.01 {
                    let alpha = (water_h * 0.5).min(0.9);
                    let w_col = Color::new(0.0, 0.2, 0.8, alpha);
                    // Blend
                    color = Color::new(
                        t_col.r * (1.0 - alpha) + w_col.r * alpha,
                        t_col.g * (1.0 - alpha) + w_col.g * alpha,
                        t_col.b * (1.0 - alpha) + w_col.b * alpha,
                        1.0
                    );
                }

                image.set_pixel(x as u32, y as u32, color);
            }
        }

        // Draw Units
        // We can't draw units directly onto the texture easily if we want them to move smoothly or be distinct.
        // So draw texture first, then draw circles on top.

        texture.update(&image);

        clear_background(BLACK);

        draw_texture_ex(
            &texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_w, screen_h)),
                ..Default::default()
            },
        );

        // Draw Units
        for unit in &game.units {
            if !unit.alive { continue; }
            let px = unit.x as f32 * scale_x + scale_x / 2.0;
            let py = unit.y as f32 * scale_y + scale_y / 2.0;
            draw_circle(px, py, scale_x * 0.4, RED);
        }

        // UI Info
        draw_text(&format!("Units Alive: {}", game.units.iter().filter(|u| u.alive).count()), 10.0, 20.0, 20.0, WHITE);
        draw_text("L: Raise | R: Lower | M: Water | Space: Pause | T: Tide", 10.0, 40.0, 20.0, WHITE);

        next_frame().await
    }
}
