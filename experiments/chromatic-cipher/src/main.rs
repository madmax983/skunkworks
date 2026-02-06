use macroquad::prelude::*;
use image::{RgbaImage, Rgba};
use ::rand::Rng;

mod stega;

#[macroquad::main("Chromatic Cipher")]
async fn main() {
    // 1. Generate Cover Image (Noise)
    let width = 512;
    let height = 512;
    let mut cover = RgbaImage::new(width, height);
    let mut rng = ::rand::thread_rng();

    // Create a noise pattern that looks like static
    for pixel in cover.pixels_mut() {
        *pixel = Rgba([
            rng.gen(),
            rng.gen(),
            rng.gen(),
            255
        ]);
    }

    // 2. Embed Payload (Source Code)
    // Try to read own source code.
    let payload_path = "experiments/chromatic-cipher/src/main.rs";
    let payload_string = std::fs::read_to_string(payload_path)
        .or_else(|_| std::fs::read_to_string("src/main.rs")) // Fallback if running from crate root
        .unwrap_or_else(|_| "Genesis: Source code not found. Using placeholder payload.".to_string());

    let seed = 1337; // Fixed seed for this demo

    // Clone cover for "clean" version (though steganography modifies it)
    let mut stego_image = cover.clone();

    println!("Embedding {} bytes...", payload_string.len());
    match stega::embed(&mut stego_image, payload_string.as_bytes(), seed) {
        Ok(_) => println!("Payload embedded successfully!"),
        Err(e) => eprintln!("Failed to embed: {}", e),
    }

    // Convert to Macroquad Texture
    let texture = Texture2D::from_rgba8(width as u16, height as u16, stego_image.as_raw());
    texture.set_filter(FilterMode::Nearest);

    // Create LSB View Texture (Pre-calculated for performance)
    let mut lsb_view_img = stego_image.clone();
    for pixel in lsb_view_img.pixels_mut() {
        // Boost LSBs to full visibility.
        // 0 -> 0, 1 -> 255
        pixel[0] = (pixel[0] & 1) * 255;
        pixel[1] = (pixel[1] & 1) * 255;
        pixel[2] = (pixel[2] & 1) * 255;
        pixel[3] = 255;
    }
    let lsb_texture = Texture2D::from_rgba8(width as u16, height as u16, lsb_view_img.as_raw());
    lsb_texture.set_filter(FilterMode::Nearest);

    let mut show_lsb = false;
    let mut decoded_text: Option<String> = None;
    let mut show_help = true;

    loop {
        clear_background(BLACK);

        // Input
        if is_key_pressed(KeyCode::Space) {
            show_lsb = !show_lsb;
        }

        if is_key_pressed(KeyCode::Enter) {
            // Decrypt
             match stega::extract(&stego_image, seed) {
                Ok(bytes) => {
                    if let Ok(s) = String::from_utf8(bytes) {
                        decoded_text = Some(s);
                        show_help = false;
                    } else {
                         decoded_text = Some("Error: Decoded data is not valid UTF-8".to_string());
                    }
                }
                Err(e) => {
                    decoded_text = Some(format!("Error: {}", e));
                }
            }
        }

        if is_key_pressed(KeyCode::Escape) {
            decoded_text = None;
            show_help = true;
        }

        // Draw
        let draw_tex = if show_lsb { &lsb_texture } else { &texture };

        // Center image
        let screen_w = screen_width();
        let screen_h = screen_height();
        let scale = (screen_h / height as f32).min(screen_w / width as f32) * 0.9;
        let dest_w = width as f32 * scale;
        let dest_h = height as f32 * scale;
        let dest_x = (screen_w - dest_w) / 2.0;
        let dest_y = (screen_h - dest_h) / 2.0;

        draw_texture_ex(
            draw_tex,
            dest_x,
            dest_y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(dest_w, dest_h)),
                ..Default::default()
            },
        );

        // UI Overlay
        if show_help {
            draw_text("CHROMATIC CIPHER", 20.0, 30.0, 40.0, WHITE);
            draw_text("[SPACE] Toggle LSB View", 20.0, 60.0, 20.0, GRAY);
            draw_text("[ENTER] Decrypt Payload", 20.0, 80.0, 20.0, GRAY);
        }

        if show_lsb {
             draw_text("MODE: CIPHER VIEW (LSB)", 20.0, screen_h - 20.0, 30.0, RED);
        }

        if let Some(text) = &decoded_text {
            // Draw semi-transparent background
            draw_rectangle(50.0, 50.0, screen_w - 100.0, screen_h - 100.0, Color::new(0.0, 0.0, 0.0, 0.95));
            draw_rectangle_lines(50.0, 50.0, screen_w - 100.0, screen_h - 100.0, 2.0, GREEN);

            draw_text("DECODED PAYLOAD (Source Code):", 70.0, 80.0, 30.0, GREEN);

            let lines: Vec<&str> = text.lines().collect();
            let line_height = 20.0;
            let max_lines = ((screen_h - 160.0) / line_height) as usize;

            for (i, line) in lines.iter().take(max_lines).enumerate() {
                draw_text(line, 70.0, 110.0 + i as f32 * line_height, 16.0, GREEN);
            }
            if lines.len() > max_lines {
                draw_text("... (scrolling not implemented) ...", 70.0, 110.0 + max_lines as f32 * line_height, 16.0, GREEN);
            }
             draw_text("[ESC] Close Overlay", 70.0, screen_h - 70.0, 20.0, GRAY);
        }

        next_frame().await
    }
}
