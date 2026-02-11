mod leaf;
mod erosion;

use macroquad::prelude::*;
use leaf::LeafMap;
use erosion::{erode_step, erode_at};

const MAP_WIDTH: usize = 400;
const MAP_HEIGHT: usize = 400;

#[macroquad::main("Vascular Valley")]
async fn main() {
    let mut map = LeafMap::new(MAP_WIDTH, MAP_HEIGHT);
    map.generate_shape();

    let mut image = Image::gen_image_color(MAP_WIDTH as u16, MAP_HEIGHT as u16, Color::new(0.0, 0.0, 0.0, 0.0));
    let texture = Texture2D::from_image(&image);

    // Auto-erosion on by default
    let mut eroding = true;
    let mut drops_per_frame = 2000;

    loop {
        // Input
        if is_key_pressed(KeyCode::R) {
            map = LeafMap::new(MAP_WIDTH, MAP_HEIGHT);
            map.generate_shape();
        }
        if is_key_pressed(KeyCode::Space) {
            eroding = !eroding;
        }
        if is_key_pressed(KeyCode::Up) {
            drops_per_frame += 500;
        }
        if is_key_pressed(KeyCode::Down) {
            if drops_per_frame > 500 {
                drops_per_frame -= 500;
            }
        }

        // Mouse interaction (Drop water manually)
        if is_mouse_button_down(MouseButton::Left) {
            let mouse_pos = mouse_position();
            // Map screen pos to texture pos
            let screen_w = screen_width();
            let screen_h = screen_height();
            let scale = (screen_w / MAP_WIDTH as f32).min(screen_h / MAP_HEIGHT as f32) * 0.8;
            let offset_x = (screen_w - MAP_WIDTH as f32 * scale) / 2.0;
            let offset_y = (screen_h - MAP_HEIGHT as f32 * scale) / 2.0;

            let mx = (mouse_pos.0 - offset_x) / scale;
            let my = (mouse_pos.1 - offset_y) / scale;

            if mx >= 0.0 && mx < MAP_WIDTH as f32 && my >= 0.0 && my < MAP_HEIGHT as f32 {
                let x = mx as usize;
                let y = my as usize;
                // Drop 100 drops at cursor
                for _ in 0..100 {
                    erode_at(&mut map, x, y);
                }
            }
        }

        // Simulation
        if eroding {
            erode_step(&mut map, drops_per_frame);
        }

        // Visualization
        // Update texture
        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                let idx = y * MAP_WIDTH + x;
                if map.mask[idx] {
                    let h = map.heightmap[idx];
                    let w = map.water[idx];

                    // Color Logic
                    // Base: Green
                    // Veins (Low h): Yellow/Brown
                    // High h: Green

                    let val = h.clamp(0.0, 1.0);

                    let r = (1.0 - val) * 0.8;
                    let g = 0.5 + val * 0.5;
                    let b = 0.1;

                    let mut color = Color::new(r, g, b, 1.0);

                    // Water overlay
                    if w > 0.1 {
                        let water_alpha = (w * 0.5).min(0.8);
                        color = Color::new(
                            color.r * (1.0 - water_alpha) + 0.0 * water_alpha,
                            color.g * (1.0 - water_alpha) + 0.5 * water_alpha,
                            color.b * (1.0 - water_alpha) + 1.0 * water_alpha,
                            1.0
                        );

                        // Decay water
                        map.water[idx] *= 0.9;
                    }

                    image.set_pixel(x as u32, y as u32, color);
                } else {
                    image.set_pixel(x as u32, y as u32, Color::new(0.0, 0.0, 0.0, 0.0));
                }
            }
        }

        texture.update(&image);

        // Draw
        clear_background(Color::new(0.1, 0.1, 0.1, 1.0));

        let screen_w = screen_width();
        let screen_h = screen_height();
        let scale = (screen_w / MAP_WIDTH as f32).min(screen_h / MAP_HEIGHT as f32) * 0.8;
        let offset_x = (screen_w - MAP_WIDTH as f32 * scale) / 2.0;
        let offset_y = (screen_h - MAP_HEIGHT as f32 * scale) / 2.0;

        draw_texture_ex(
            &texture,
            offset_x,
            offset_y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(MAP_WIDTH as f32 * scale, MAP_HEIGHT as f32 * scale)),
                ..Default::default()
            },
        );

        // UI
        draw_text("Vascular Valley", 20.0, 30.0, 30.0, WHITE);
        draw_text(&format!("Drops/Frame: {} (Up/Down)", drops_per_frame), 20.0, 60.0, 20.0, WHITE);
        draw_text("R: Reset | Space: Pause", 20.0, 80.0, 20.0, WHITE);
        draw_text("Click to Erode", 20.0, 100.0, 20.0, WHITE);

        next_frame().await
    }
}
