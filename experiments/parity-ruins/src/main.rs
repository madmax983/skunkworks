mod codec;
use codec::Sarcophagus;
use macroquad::prelude::*;
use macroquad::audio::{load_sound_from_bytes, play_sound, PlaySoundParams};
use image::{RgbaImage, Rgba};

const SHARD_VIEW_WIDTH: usize = 512;

fn generate_test_image(w: u32, h: u32) -> RgbaImage {
    let mut img = RgbaImage::new(w, h);
    for y in 0..h {
        for x in 0..w {
            let cx = x as f32 / w as f32;
            let cy = y as f32 / h as f32;

            let r = (cx * 255.0) as u8;
            let g = (cy * 255.0) as u8;
            let b = ((cx + cy) * 127.0) as u8;

            // Add some pattern
            let pattern = ((x / 20) + (y / 20)) % 2 == 0;
            let r = if pattern { r.wrapping_add(50) } else { r };

            img.put_pixel(x, y, Rgba([r, g, b, 255]));
        }
    }
    img
}

fn generate_click_sound() -> Vec<u8> {
    // Simple short noise burst
    let sample_rate: u32 = 44100;
    let duration = 0.05;
    let num_samples = (sample_rate as f32 * duration) as usize;
    let mut data = Vec::with_capacity(44 + num_samples * 2);

    // RIFF header
    data.extend_from_slice(b"RIFF");
    let file_size = 36 + num_samples * 2;
    data.extend_from_slice(&(file_size as u32).to_le_bytes());
    data.extend_from_slice(b"WAVE");
    data.extend_from_slice(b"fmt ");
    data.extend_from_slice(&16u32.to_le_bytes());
    data.extend_from_slice(&1u16.to_le_bytes());
    data.extend_from_slice(&1u16.to_le_bytes());
    data.extend_from_slice(&sample_rate.to_le_bytes());
    data.extend_from_slice(&(sample_rate * 2).to_le_bytes());
    data.extend_from_slice(&2u16.to_le_bytes());
    data.extend_from_slice(&16u16.to_le_bytes());
    data.extend_from_slice(b"data");
    data.extend_from_slice(&(num_samples as u32 * 2).to_le_bytes());

    for _ in 0..num_samples {
        let sample = (rand::gen_range(-1.0, 1.0) * 0.5) as f32;
        let sample_i16 = (sample * 32767.0) as i16;
        data.extend_from_slice(&sample_i16.to_le_bytes());
    }
    data
}

#[macroquad::main("Parity Ruins")]
async fn main() {
    // 1. Setup Data
    let img_w = 200; // Smallish image for performance
    let img_h = 200;
    let original_img = generate_test_image(img_w, img_h);
    let raw_bytes = original_img.as_raw().clone();

    // Configuration
    let data_shards = 4;
    let parity_shards = 2;

    let mut sarcophagus = Sarcophagus::new(&raw_bytes, data_shards, parity_shards);

    // 2. Visualization Setup
    // Calculate shard view height
    let total_bytes: usize = sarcophagus.shards.iter().map(|s| s.len()).sum();
    let shard_view_h = (total_bytes + SHARD_VIEW_WIDTH - 1) / SHARD_VIEW_WIDTH;

    let mut shard_image = Image {
        bytes: vec![0; SHARD_VIEW_WIDTH * shard_view_h * 4], // RGBA
        width: SHARD_VIEW_WIDTH as u16,
        height: shard_view_h as u16,
    };
    let shard_texture = Texture2D::from_image(&shard_image);
    shard_texture.set_filter(FilterMode::Nearest);

    let recon_texture = Texture2D::from_image(&Image {
        bytes: vec![0; (img_w * img_h * 4) as usize],
        width: img_w as u16,
        height: img_h as u16,
    });
    recon_texture.set_filter(FilterMode::Nearest);

    let click_wav = generate_click_sound();
    let click_sound = load_sound_from_bytes(&click_wav).await.ok();

    let mut entropy_mode = false;
    let mut frame_count = 0;

    loop {
        clear_background(Color::new(0.1, 0.1, 0.1, 1.0));

        // Input Handling
        if is_key_pressed(KeyCode::E) {
            entropy_mode = !entropy_mode;
        }
        if is_key_pressed(KeyCode::R) {
            // Reset
            sarcophagus = Sarcophagus::new(&raw_bytes, data_shards, parity_shards);
        }

        let mouse_pos = mouse_position();
        let mouse_x = mouse_pos.0;
        let mouse_y = mouse_pos.1;

        let mut corrupted_this_frame = false;

        // Draw Shard View
        // We draw it at 10, 10
        let view_x = 10.0;
        let view_y = 40.0;

        // Handle Interaction with Shard View
        if mouse_x >= view_x && mouse_x < view_x + SHARD_VIEW_WIDTH as f32 &&
           mouse_y >= view_y && mouse_y < view_y + shard_view_h as f32
        {
            if is_mouse_button_down(MouseButton::Left) || is_mouse_button_down(MouseButton::Right) {
                let lx = (mouse_x - view_x) as usize;
                let ly = (mouse_y - view_y) as usize;

                // Map x,y back to shard index
                let byte_offset = ly * SHARD_VIEW_WIDTH + lx;
                let mut current_offset = 0;

                for (_shard_idx, shard) in sarcophagus.shards.iter_mut().enumerate() {
                    if byte_offset >= current_offset && byte_offset < current_offset + shard.len() {
                        let local_idx = byte_offset - current_offset;

                        // Burst size
                        let burst = 5;
                        let val = if is_mouse_button_down(MouseButton::Left) { 0 } else { rand::gen_range(0, 255) };

                        // Corrupt a small area
                         for i in 0..burst {
                             if local_idx + i < shard.len() {
                                 shard[local_idx + i] = val;
                             }
                             // Horizontal spill?
                         }
                        corrupted_this_frame = true;
                        break;
                    }
                    current_offset += shard.len();
                }
            }
        }

        // Entropy
        if entropy_mode && frame_count % 10 == 0 {
            let shard_idx = rand::gen_range(0, sarcophagus.shards.len());
            let byte_idx = rand::gen_range(0, sarcophagus.shard_size);
            sarcophagus.corrupt(shard_idx, byte_idx, 0);
            corrupted_this_frame = true;
        }

        if corrupted_this_frame {
            if let Some(snd) = &click_sound {
                 play_sound(snd, PlaySoundParams { looped: false, volume: 0.2 });
            }
        }

        // Update Shard Texture
        // We map bytes to pixels.
        // Data bytes -> Grayscale.
        // Parity bytes -> Red tint.
        let mut current_offset = 0;
        for (i, shard) in sarcophagus.shards.iter().enumerate() {
            let is_parity = i >= sarcophagus.data_shards_count;
            for &b in shard {
                if current_offset >= shard_image.bytes.len() / 4 { break; }

                let pixel_idx = current_offset * 4;
                if is_parity {
                    shard_image.bytes[pixel_idx] = b;     // R
                    shard_image.bytes[pixel_idx + 1] = 0; // G
                    shard_image.bytes[pixel_idx + 2] = 0; // B
                } else {
                    shard_image.bytes[pixel_idx] = b;
                    shard_image.bytes[pixel_idx + 1] = b;
                    shard_image.bytes[pixel_idx + 2] = b;
                }
                shard_image.bytes[pixel_idx + 3] = 255; // Alpha

                current_offset += 1;
            }
        }
        shard_texture.update(&shard_image);

        // Reconstruct
        let recon_result = sarcophagus.reconstruct();

        // Update Recon Texture
        match recon_result {
            Ok(data) => {
                 // Create image from raw bytes
                 // Note: 'data' is RGBA raw bytes
                 // If data length matches expected
                 if data.len() == (img_w * img_h * 4) as usize {
                      let img = Image {
                          bytes: data,
                          width: img_w as u16,
                          height: img_h as u16,
                      };
                      recon_texture.update(&img);
                 }
            },
            Err(_) => {
                // If failed, we could show static or the last good frame with glitch overlay
                // For now, let's just fill with static/noise to indicate failure
                 let mut noise = vec![0u8; (img_w * img_h * 4) as usize];
                 for j in 0..noise.len() {
                     noise[j] = rand::gen_range(0, 255);
                 }
                 let img = Image {
                     bytes: noise,
                     width: img_w as u16,
                     height: img_h as u16,
                 };
                 recon_texture.update(&img);
            }
        }

        // Draw
        draw_text("Parity Ruins: Data Sarcophagus", 10.0, 20.0, 30.0, WHITE);
        draw_text("Left Click: Zero | Right Click: Rand | E: Entropy | R: Reset", 10.0, 35.0, 20.0, GRAY);

        draw_texture(&shard_texture, view_x, view_y, WHITE);

        // Draw separation lines between shards?
        // Visually we can see them (gray vs red)

        draw_text("Reconstructed:", 550.0, 20.0, 30.0, GREEN);
        draw_texture_ex(&recon_texture, 550.0, 40.0, WHITE, DrawTextureParams {
            dest_size: Some(vec2(img_w as f32 * 2.0, img_h as f32 * 2.0)),
            ..Default::default()
        });

        // Status
        let status_color = if sarcophagus.reconstruct().is_ok() { GREEN } else { RED };
        draw_circle(540.0, 30.0, 5.0, status_color);

        frame_count += 1;
        next_frame().await
    }
}
