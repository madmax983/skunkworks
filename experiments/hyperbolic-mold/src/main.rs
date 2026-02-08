use macroquad::prelude::*;
use rayon::prelude::*;

mod agent;
mod world;

use world::World;

#[macroquad::main("Hyperbolic Mold")]
async fn main() {
    let width = 800;
    let height = 800;
    request_new_screen_size(width as f32, height as f32);

    let num_agents = 5000;
    let mut world = World::new(width, height, num_agents);

    let mut image = Image::gen_image_color(width as u16, height as u16, BLACK);
    let texture = Texture2D::from_image(&image);

    loop {
        clear_background(BLACK);

        world.update();

        // Update texture pixels from world grid
        let grid = &world.grid;

        // Use rayon to parallelize pixel processing
        image.bytes.par_chunks_mut(4)
            .zip(grid.par_iter())
            .for_each(|(pixel, &val)| {
                let v = val.clamp(0.0, 1.0);

                // Color Scheme: Deep Space Mold
                // Low value: Black/Deep Blue
                // Mid value: Cyan/Green
                // High value: White

                let r = (v.powf(2.0) * 255.0) as u8; // Red ramps up slowly (white only at high density)
                let g = (v * 200.0) as u8;
                let b = (v * 255.0 + 20.0).min(255.0) as u8;

                pixel[0] = r;
                pixel[1] = g;
                pixel[2] = b;
                pixel[3] = 255;
            });

        texture.update(&image);

        // Draw the texture covering the screen
        draw_texture(&texture, 0.0, 0.0, WHITE);

        // Draw Boundary Circle (Poincaré Disk Limit)
        let cx = width as f32 / 2.0;
        let cy = height as f32 / 2.0;
        let r = width as f32 / 2.0;

        draw_circle_lines(cx, cy, r, 2.0, WHITE);

        draw_text("Hyperbolic Mold", 10.0, 20.0, 30.0, WHITE);
        draw_text(&format!("FPS: {}", get_fps()), 10.0, 50.0, 20.0, WHITE);

        next_frame().await
    }
}
