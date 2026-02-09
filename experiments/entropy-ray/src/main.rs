use macroquad::prelude::*;
use std::io::Cursor;

mod archaeologist;
mod decay;

use archaeologist::{raw_interpret, recover_image};
use decay::{apply_decay, DecayMode};

#[macroquad::main("Entropy Ray")]
async fn main() {
    // 1. Generate procedural test image (Checkerboard + Gradient)
    let width = 256;
    let height = 256;
    let mut original_img = image::RgbaImage::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let cx = (x as f32 / width as f32) * 255.0;
            let cy = (y as f32 / height as f32) * 255.0;
            let check = ((x / 32) + (y / 32)) % 2 == 0;
            let c = if check { 200 } else { 50 };
            let r = (cx as u8 / 2).wrapping_add(c);
            let g = (cy as u8 / 2).wrapping_add(c);
            let b = c;
            original_img.put_pixel(x, y, image::Rgba([r, g, b, 255]));
        }
    }

    // Encode to PNG buffer
    let mut original_png_data = Vec::new();
    {
        let mut cursor = Cursor::new(&mut original_png_data);
        original_img
            .write_to(&mut cursor, image::ImageOutputFormat::Png)
            .unwrap();
    }

    // Create working buffer
    let mut buffer = original_png_data.clone();

    // State
    let mut decay_mode = DecayMode::Gamma;
    let mut total_entropy = 0;
    let mut show_hex = false;
    // Keep a fallback image
    let last_valid_image = Image {
        bytes: original_img.clone().into_raw(),
        width: width as u16,
        height: height as u16,
    };

    // Initial texture
    let mut texture = Some(Texture2D::from_image(&last_valid_image));

    loop {
        clear_background(BLACK);

        // Input Handling
        if is_key_pressed(KeyCode::Key1) {
            decay_mode = DecayMode::Gamma;
        }
        if is_key_pressed(KeyCode::Key2) {
            decay_mode = DecayMode::XRay;
        }
        if is_key_pressed(KeyCode::Key3) {
            decay_mode = DecayMode::Cosmic;
        }
        if is_key_pressed(KeyCode::Key4) {
            decay_mode = DecayMode::Mold;
        }
        if is_key_pressed(KeyCode::R) {
            // Restore header (first 8 bytes for PNG signature)
            // PNG signature: 89 50 4E 47 0D 0A 1A 0A
            if buffer.len() >= 8 && original_png_data.len() >= 8 {
                buffer[0..8].copy_from_slice(&original_png_data[0..8]);
            }
        }
        if is_key_pressed(KeyCode::F) {
            // Full Reset
            buffer = original_png_data.clone();
            total_entropy = 0;
        }
        if is_key_pressed(KeyCode::H) {
            show_hex = !show_hex;
        }

        // Apply Decay
        // Intensity is low per frame to make it watchable
        let entropy_added = apply_decay(&mut buffer, decay_mode, 0.0001);
        total_entropy += entropy_added;

        // Try to recover
        // We clone buffer because recover_image takes a slice, but raw_interpret needs to iterate it
        // and we want to be safe.
        let display_img = if let Some(img) = recover_image(&buffer) {
            img
        } else {
            // Format broken! Fallback to raw interpretation
            raw_interpret(&buffer, width as u16, height as u16)
        };

        // Update texture
        if let Some(tex) = &texture {
            tex.update(&display_img);
            tex.set_filter(FilterMode::Nearest);
        } else {
            let tex = Texture2D::from_image(&display_img);
            tex.set_filter(FilterMode::Nearest);
            texture = Some(tex);
        }

        // Draw
        if let Some(tex) = &texture {
            draw_texture_ex(
                tex,
                screen_width() / 2.0 - 256.0,
                screen_height() / 2.0 - 256.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(512.0, 512.0)),
                    ..Default::default()
                },
            );
        }

        // UI
        draw_text("ENTROPY RAY", 20.0, 30.0, 30.0, WHITE);
        draw_text(&format!("Mode: {:?}", decay_mode), 20.0, 60.0, 20.0, GREEN);
        draw_text(
            &format!("Entropy: {}", total_entropy),
            20.0,
            80.0,
            20.0,
            RED,
        );
        draw_text(
            "Keys: [1-4] Mode | [R] Fix Header | [F] Reset | [H] Hex",
            20.0,
            screen_height() - 20.0,
            20.0,
            GRAY,
        );

        let status = if recover_image(&buffer).is_some() {
            "VALID PNG"
        } else {
            "CORRUPTED (RAW)"
        };
        draw_text(
            status,
            20.0,
            100.0,
            20.0,
            if status == "VALID PNG" { GREEN } else { RED },
        );

        // Hex Overlay
        if show_hex {
            let hex_start_y = 130.0;
            for i in 0..16 {
                if i * 16 < buffer.len() {
                    let chunk = &buffer[i * 16..(i * 16 + 16).min(buffer.len())];
                    let hex_string: String = chunk.iter().map(|b| format!("{:02X} ", b)).collect();
                    draw_text(
                        &hex_string,
                        20.0,
                        hex_start_y + i as f32 * 15.0,
                        15.0,
                        YELLOW,
                    );
                }
            }
        }

        next_frame().await
    }
}
