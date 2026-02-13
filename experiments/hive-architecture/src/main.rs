mod model;
mod simulation;
#[cfg(test)]
mod tests;

use macroquad::prelude::*;
use simulation::World;

#[macroquad::main("Hive Architecture")]
async fn main() {
    let width = 200;
    let height = 200;
    // Scale up for visibility
    let scale = 3.0;
    request_new_screen_size(width as f32 * scale, height as f32 * scale);

    let mut world = World::new(width, height, 5000);

    let mut image = Image::gen_image_color(width as u16, height as u16, BLACK);
    let texture = Texture2D::from_image(&image);

    loop {
        // Update physics multiple times per frame for speed?
        for _ in 0..5 {
            world.update();
        }

        // Render to texture
        for (idx, val) in world.grid.heat.iter().enumerate() {
            let x = idx % width;
            let y = idx / width;
            let heat = val.clamp(0.0, 1.0);

            if world.grid.wall[idx] {
                 image.set_pixel(x as u32, y as u32, WHITE);
            } else {
                 // Heat: Blue -> Red
                 // Cold (0.0): Blue (0, 0, 1)
                 // Hot (1.0): Red (1, 0, 0)
                 image.set_pixel(x as u32, y as u32, Color::new(heat, 0.0, 1.0 - heat, 1.0));
            }
        }

        // Render Termites
        for t in &world.termites {
            let x = t.x as u32;
            let y = t.y as u32;
            if x < width as u32 && y < height as u32 {
                 image.set_pixel(x, y, t.blueprint.color);
            }
        }

        texture.update(&image);

        clear_background(BLACK);
        draw_texture_ex(&texture, 0.0, 0.0, WHITE, DrawTextureParams {
            dest_size: Some(vec2(screen_width(), screen_height())),
            ..Default::default()
        });

        draw_text(&format!("Agents: {}", world.termites.len()), 10.0, 20.0, 30.0, WHITE);
        draw_text(&format!("FPS: {}", get_fps()), 10.0, 50.0, 30.0, WHITE);

        next_frame().await;
    }
}
