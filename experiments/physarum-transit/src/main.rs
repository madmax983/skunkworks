use macroquad::prelude::*;
use rayon::prelude::*;

mod model;
use model::{Agent, City, World};

#[macroquad::main("Physarum Transit")]
async fn main() {
    let width = 800;
    let height = 600;
    request_new_screen_size(width as f32, height as f32);

    let num_agents = 5000;

    let mut world = World::new(width, height, num_agents);

    // Initialize agents randomly
    for _ in 0..num_agents {
        let x = rand::gen_range(0.0, width as f32);
        let y = rand::gen_range(0.0, height as f32);
        let angle = rand::gen_range(0.0, std::f32::consts::PI * 2.0);
        world.agents.push(Agent::new(vec2(x, y), angle));
    }

    // Initialize cities randomly
    for _ in 0..15 {
        let x = rand::gen_range(0.0, width as f32);
        let y = rand::gen_range(0.0, height as f32);
        world.cities.push(City::new(vec2(x, y)));
    }

    let mut image = Image::gen_image_color(width as u16, height as u16, BLACK);
    let texture = Texture2D::from_image(&image);

    loop {
        // Clear background not strictly needed if we draw full screen texture,
        // but good for safety.
        // clear_background(BLACK);

        world.update();

        // Update image data from grid
        // We do this in parallel to speed up the conversion
        let grid = &world.grid;
        let bytes = &mut image.bytes;

        // Ensure bytes and grid match size (4 bytes per float)
        // grid size = w * h
        // bytes size = w * h * 4

        // Use rayon to process chunks
        bytes.par_chunks_mut(4).enumerate().for_each(|(i, pixel)| {
            if i < grid.len() {
                let val = grid[i];
                // Color mapping: Purple/Blueish for "mycelium" vibe
                // R: val * 0.5, G: val * 0.2, B: val * 1.0?
                // Or just white/cyan.
                let v = (val * 255.0).min(255.0) as u8;

                // RGBA
                pixel[0] = v; // R
                pixel[1] = (v as f32 * 0.8) as u8; // G
                pixel[2] = (v as f32 * 1.0) as u8; // B
                pixel[3] = 255; // A
            }
        });

        texture.update(&image);
        draw_texture(&texture, 0.0, 0.0, WHITE);

        draw_text(&format!("FPS: {}", get_fps()), 20.0, 20.0, 30.0, GREEN);
        draw_text(
            &format!("Agents: {}", world.agents.len()),
            20.0,
            50.0,
            30.0,
            GREEN,
        );

        next_frame().await
    }
}
