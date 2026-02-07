use image::{Rgba, RgbaImage};
use macroquad::prelude::*;
mod dct;
mod garden;
use garden::Garden;

fn generate_test_image(w: u32, h: u32) -> RgbaImage {
    let mut img = RgbaImage::new(w, h);
    for y in 0..h {
        for x in 0..w {
            let cx = (x as f32 / w as f32) * 255.0;
            let cy = (y as f32 / h as f32) * 255.0;

            // Checkerboard
            let check = ((x / 32) + (y / 32)) % 2 == 0;
            let c = if check { 200 } else { 50 };

            // Gradient overlay
            let r = (cx as u8 / 2).wrapping_add(c);
            let g = (cy as u8 / 2).wrapping_add(c);
            let b = c;

            img.put_pixel(x, y, Rgba([r, g, b, 255]));
        }
    }
    img
}

#[macroquad::main("JPEG Garden")]
async fn main() {
    let w = 512;
    let h = 512;

    // Initial setup
    let initial_img = generate_test_image(w, h);
    let mut garden = Garden::from_image(&initial_img);

    // Create initial texture
    let bytes = initial_img.as_raw().clone();
    let mq_image = Image {
        bytes,
        width: w as u16,
        height: h as u16,
    };
    let texture = Texture2D::from_image(&mq_image);
    // Use nearest neighbor for that "pixelated" digital rot look if desired,
    // but linear is fine too. Let's stick to default.
    texture.set_filter(FilterMode::Nearest);

    let mut last_update = get_time();
    let mut paused = false;

    loop {
        clear_background(BLACK);

        // Input
        if is_key_pressed(KeyCode::R) {
            let img = generate_test_image(w, h);
            garden = Garden::from_image(&img);
        }
        if is_key_pressed(KeyCode::Space) {
            paused = !paused;
        }

        // Sim
        if !paused {
            for _ in 0..10 {
                // Speed up simulation
                garden.tick();
            }
        }

        // Render update
        if get_time() - last_update > 0.05 {
            let rendered_img = garden.to_image();
            let bytes = rendered_img.into_raw();

            let mq_img = Image {
                bytes,
                width: w as u16,
                height: h as u16,
            };
            texture.update(&mq_img);

            last_update = get_time();
        }

        // Draw fitting screen
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

        draw_text("JPEG Garden", 10.0, 30.0, 30.0, WHITE);
        draw_text("R: Reset | Space: Pause", 10.0, 50.0, 20.0, GRAY);

        next_frame().await
    }
}
