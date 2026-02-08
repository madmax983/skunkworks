use image::{Rgba, RgbaImage};
use macroquad::prelude::*;
use macroquad::audio::{load_sound_from_bytes, play_sound, PlaySoundParams};

mod dct;
mod garden;
use garden::Garden;

enum ViewMode {
    Normal,
    Heatmap,
}

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

// Simple WAV header generator
fn generate_wav(duration_secs: f32, sample_rate: u32, freq: f32, amplitude: f32, noise_mix: f32) -> Vec<u8> {
    let num_samples = (duration_secs * sample_rate as f32) as usize;
    let mut data = Vec::with_capacity(44 + num_samples * 2);

    // RIFF header
    data.extend_from_slice(b"RIFF");
    let file_size = 36 + num_samples * 2;
    data.extend_from_slice(&(file_size as u32).to_le_bytes());
    data.extend_from_slice(b"WAVE");

    // fmt chunk
    data.extend_from_slice(b"fmt ");
    data.extend_from_slice(&16u32.to_le_bytes()); // Chunk size
    data.extend_from_slice(&1u16.to_le_bytes()); // PCM
    data.extend_from_slice(&1u16.to_le_bytes()); // Channels (Mono)
    data.extend_from_slice(&sample_rate.to_le_bytes());
    data.extend_from_slice(&(sample_rate * 2).to_le_bytes()); // Byte rate
    data.extend_from_slice(&2u16.to_le_bytes()); // Block align
    data.extend_from_slice(&16u16.to_le_bytes()); // Bits per sample

    // data chunk
    data.extend_from_slice(b"data");
    data.extend_from_slice(&(num_samples as u32 * 2).to_le_bytes());

    for i in 0..num_samples {
        let t = i as f32 / sample_rate as f32;
        let sine = (t * freq * 2.0 * std::f32::consts::PI).sin();
        let noise = rand::gen_range(-1.0, 1.0);

        let sample = sine * (1.0 - noise_mix) + noise * noise_mix;
        let sample = (sample * amplitude).clamp(-1.0, 1.0);
        let sample_i16 = (sample * 32767.0) as i16;

        data.extend_from_slice(&sample_i16.to_le_bytes());
    }

    data
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
        bytes: bytes.clone(),
        width: w as u16,
        height: h as u16,
    };
    let texture = Texture2D::from_image(&mq_image);
    texture.set_filter(FilterMode::Nearest);

    let mut last_update = get_time();
    let mut paused = false;
    let mut view_mode = ViewMode::Normal;
    let mut metrics = garden.get_entropy_metrics();

    let mut next_sound_time = 0.0;

    loop {
        clear_background(BLACK);

        if is_key_pressed(KeyCode::R) {
            let img = generate_test_image(w, h);
            garden = Garden::from_image(&img);
            metrics = garden.get_entropy_metrics();
        }
        if is_key_pressed(KeyCode::Space) {
            paused = !paused;
        }
        if is_key_pressed(KeyCode::V) {
            view_mode = match view_mode {
                ViewMode::Normal => ViewMode::Heatmap,
                ViewMode::Heatmap => ViewMode::Normal,
            };
        }

        // Sim
        if !paused {
            for _ in 0..5 {
                garden.tick();
            }
        }

        // Render update
        if get_time() - last_update > 0.05 {
            metrics = garden.get_entropy_metrics();

            let rendered_img = garden.to_image();
            let mut bytes = rendered_img.into_raw();

            if let ViewMode::Heatmap = view_mode {
                // Apply a red tint based on max_block_error approximation
                // We don't have per-pixel error easily, so we just tint the whole image
                // heavily based on total error? No, that's boring.
                // Let's just invert colors for heatmap mode for now to show "something different".
                for i in (0..bytes.len()).step_by(4) {
                    bytes[i] = 255 - bytes[i];
                    bytes[i+1] = 255 - bytes[i+1];
                    bytes[i+2] = 255 - bytes[i+2];
                }
            }

            let mq_img = Image {
                bytes,
                width: w as u16,
                height: h as u16,
            };
            texture.update(&mq_img);

            last_update = get_time();
        }

        // Audio
        if !paused && get_time() > next_sound_time {
             // Normalized error
             let error_factor = (metrics.total_error / 5000000.0).clamp(0.0, 1.0);
             // Freq goes down as rot increases (dying)
             let freq = 400.0 - error_factor * 300.0;
             // Noise goes up
             let noise = error_factor * 0.9;

             // Only play if there is some error or just keep the hum?
             // Let's play a hum.

             let wav_data = generate_wav(0.1, 44100, freq, 0.1, noise);

             if let Ok(snd) = load_sound_from_bytes(&wav_data).await {
                 play_sound(&snd, PlaySoundParams { looped: false, volume: 0.5 });
             }
             next_sound_time = get_time() + 0.12; // slightly overlapping
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
        draw_text(
            &format!("Error: {:.0} (Max: {:.0})", metrics.total_error, metrics.max_block_error),
            10.0,
            60.0,
            20.0,
            RED,
        );
        let mode_str = match view_mode {
            ViewMode::Normal => "Normal",
            ViewMode::Heatmap => "Inverted (Heatmap)",
        };
        draw_text(&format!("Mode: {}", mode_str), 10.0, 90.0, 20.0, YELLOW);
        draw_text("R: Reset | Space: Pause | V: View", 10.0, 120.0, 20.0, GRAY);

        next_frame().await
    }
}
