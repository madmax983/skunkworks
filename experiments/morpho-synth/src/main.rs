mod simulation;
mod audio;

use macroquad::prelude::*;
use simulation::Grid;
use audio::AudioEngine;
use ::rand::Rng;

#[macroquad::main("Morpho-Synth")]
async fn main() {
    let width = 256;
    let height = 256;
    let mut grid = Grid::new(width, height);

    // Initial sensors
    grid.add_sensor(128, 128, 220.0); // A3
    grid.add_sensor(148, 128, 277.18); // C#4
    grid.add_sensor(108, 128, 329.63); // E4

    let audio = AudioEngine::new();

    let dt_sim = 1.0;
    let mut feed: f32 = 0.055;
    let mut kill: f32 = 0.062;

    let mut image = Image::gen_image_color(width as u16, height as u16, BLACK);
    let texture = Texture2D::from_image(&image);

    loop {
        // Interaction
        if is_mouse_button_down(MouseButton::Left) {
            let (mx, my) = mouse_position();
            let sw = screen_width();
            let sh = screen_height();

            let gx = (mx / sw * width as f32) as usize;
            let gy = (my / sh * height as f32) as usize;

            // Add V in radius
            let r = 5;
            for dy in -(r as isize)..=r as isize {
                for dx in -(r as isize)..=r as isize {
                    if dx*dx + dy*dy <= r*r {
                        let x = (gx as isize + dx).rem_euclid(width as isize) as usize;
                        let y = (gy as isize + dy).rem_euclid(height as isize) as usize;
                        let idx = y * width + x;
                        grid.v[idx] = 0.5; // Seed
                    }
                }
            }
        }

        if is_mouse_button_pressed(MouseButton::Right) {
            let (mx, my) = mouse_position();
            let sw = screen_width();
            let sh = screen_height();
            let gx = (mx / sw * width as f32) as usize;
            let gy = (my / sh * height as f32) as usize;

            let mut rng = ::rand::thread_rng();
            let scale = [261.63, 293.66, 329.63, 392.00, 440.00, 523.25]; // C Major Pentatonic
            let freq = scale[rng.gen_range(0..scale.len())];

            grid.add_sensor(gx, gy, freq);
        }

        if is_key_down(KeyCode::Up) { feed += 0.0001; }
        if is_key_down(KeyCode::Down) { feed -= 0.0001; }
        if is_key_down(KeyCode::Right) { kill += 0.0001; }
        if is_key_down(KeyCode::Left) { kill -= 0.0001; }

        // Clamp params
        feed = feed.clamp(0.01, 0.1);
        kill = kill.clamp(0.01, 0.1);

        // Simulation steps per frame
        for _ in 0..10 {
            grid.update(feed, kill, dt_sim);
        }

        // Update audio
        let freqs = grid.get_active_frequencies();
        audio.update_active_frequencies(freqs);

        // Update image
        for (i, pixel) in image.bytes.chunks_exact_mut(4).enumerate() {
            let v = grid.v[i];

            // Color mapping:
            // v ranges typically 0.0 to 1.0 (though usually < 0.6 in GS)
            // Map v to a biotic glow

            let t = (v * 3.0).clamp(0.0, 1.0);

            // Color palette: Deep Void -> Biotic Purple -> Neon Cyan -> White Core
            // r: 0 -> 0.5 -> 0.0 -> 1.0
            // g: 0 -> 0.0 -> 1.0 -> 1.0
            // b: 0 -> 0.5 -> 1.0 -> 1.0

            let r = if t < 0.5 { t } else { (t - 0.5) * 2.0 };
            let g = if t < 0.3 { 0.0 } else { (t - 0.3) * 1.4 };
            let b = if t < 0.5 { t + 0.2 } else { 0.7 + (t - 0.5) * 0.6 };

            pixel[0] = (r.clamp(0.0, 1.0) * 255.0) as u8;
            pixel[1] = (g.clamp(0.0, 1.0) * 255.0) as u8;
            pixel[2] = (b.clamp(0.0, 1.0) * 255.0) as u8;
            pixel[3] = 255;
        }

        texture.update(&image);

        clear_background(BLACK);

        // Draw texture scaled to screen
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

        // Render FPS and params
        draw_text(&format!("FPS: {}", get_fps()), 10.0, 20.0, 30.0, WHITE);
        draw_text(&format!("Feed: {:.4}", feed), 10.0, 50.0, 20.0, WHITE);
        draw_text(&format!("Kill: {:.4}", kill), 10.0, 70.0, 20.0, WHITE);

        next_frame().await;
    }
}
