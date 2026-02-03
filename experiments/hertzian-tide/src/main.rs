use macroquad::prelude::*;
use hertzian_tide::wave_tank::WaveTank;
use hertzian_tide::audio::Synth;
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};

#[macroquad::main("Hertzian Tide")]
async fn main() {
    // Grid size
    let w = 200;
    let h = 200;
    let mut tank = WaveTank::new(w, h);

    // Audio setup
    let base_freq: f32 = 220.0;
    let shared_freq = Arc::new(AtomicU32::new(base_freq.to_bits()));

    // Try to init synth, log if fails but continue
    let _synth = match Synth::new(shared_freq.clone()) {
        Ok(s) => Some(s),
        Err(e) => {
            eprintln!("Audio initialization failed: {}", e);
            None
        }
    };

    // Visualization buffers
    let mut image = Image::gen_image_color(w as u16, h as u16, BLACK);
    let texture = Texture2D::from_image(&image);
    texture.set_filter(FilterMode::Nearest);

    // Interaction
    let listener_x = w / 2;
    let listener_y = h / 2;

    loop {
        // Physics update (sub-step for stability/speed?)
        tank.step();

        // Audio Modulation
        let val_at_listener = tank.get_height(listener_x, listener_y);
        // Modulate frequency: Base +/- deviation
        // If val is ~1.0, deviation is say 100Hz.
        let target_freq = base_freq + val_at_listener * 100.0;
        // Clamp to safe range
        let safe_freq = target_freq.clamp(50.0, 1000.0);
        shared_freq.store(safe_freq.to_bits(), Ordering::Relaxed);

        // Input
        if is_mouse_button_down(MouseButton::Left) {
            let (mx, my) = mouse_position();
            let sw = screen_width();
            let sh = screen_height();

            // Map to grid
            let gx = (mx / sw * w as f32) as usize;
            let gy = (my / sh * h as f32) as usize;

            // Poke
            // We want continuous poke if holding? Or just one big one?
            // If we add every frame, it might be too much energy.
            // Let's add a small amount every frame.
            tank.poke(gx, gy, 0.5);
        }

        // Draw
        clear_background(BLACK);

        // Update texture
        for y in 0..h {
            for x in 0..w {
                let val = tank.get_height(x, y);
                let intensity = (val * 2.0).clamp(-1.0, 1.0);
                let c = intensity * 0.5 + 0.5;
                image.set_pixel(x as u32, y as u32, Color::new(c, c, c + 0.2, 1.0));
            }
        }

        // Mark listener
        image.set_pixel(listener_x as u32, listener_y as u32, RED);

        texture.update(&image);

        draw_texture_ex(&texture, 0.0, 0.0, WHITE, DrawTextureParams {
            dest_size: Some(vec2(screen_width(), screen_height())),
            ..Default::default()
        });

        draw_text("Hertzian Tide", 20.0, 30.0, 30.0, WHITE);
        draw_text("Click/Drag to ripple", 20.0, 50.0, 20.0, GRAY);
        draw_text(&format!("Freq: {:.1} Hz", safe_freq), 20.0, 70.0, 20.0, RED);

        next_frame().await
    }
}
