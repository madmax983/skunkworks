mod physics;
mod audio;

use macroquad::prelude::*;
use physics::WaveGrid;
use audio::Probe;

// Handle rand conflict if both macroquad and rand crate are used.
// We'll use macroquad's rand for simplicity in the loop.

#[macroquad::main("Hydro-Phonics")]
async fn main() {
    let width = 256;
    let height = 256;
    let mut grid = WaveGrid::new(width, height);

    // Setup Probes at harmonic points
    let probes = vec![
        Probe::new(width / 4, height / 2),
        Probe::new(width / 2, height / 2),
        Probe::new(width * 3 / 4, height / 2),
    ];

    // Initialize Audio (if feature enabled)
    #[cfg(feature = "audio")]
    let (_stream, _stream_handle) = {
        use rodio::Source;
        match rodio::OutputStream::try_default() {
            Ok((stream, handle)) => {
                let p1 = audio::engine::ProbeSource::new(&probes[0], 220.0); // A3
                let p2 = audio::engine::ProbeSource::new(&probes[1], 277.18); // C#4
                let p3 = audio::engine::ProbeSource::new(&probes[2], 329.63); // E4

                // Add Reverb/Delay if possible? Rodio has some filters.
                // Let's just play raw.
                if let Err(e) = handle.play_raw(p1.convert_samples()) { eprintln!("Audio Error: {}", e); }
                if let Err(e) = handle.play_raw(p2.convert_samples()) { eprintln!("Audio Error: {}", e); }
                if let Err(e) = handle.play_raw(p3.convert_samples()) { eprintln!("Audio Error: {}", e); }

                (Some(stream), Some(handle))
            }
            Err(e) => {
                eprintln!("Failed to initialize audio: {}", e);
                (None, None)
            }
        }
    };

    let mut image = Image::gen_image_color(width as u16, height as u16, BLACK);
    let texture = Texture2D::from_image(&image);
    texture.set_filter(FilterMode::Nearest);

    loop {
        // --- Input ---
        if is_mouse_button_down(MouseButton::Left) {
            let (mx, my) = mouse_position();
            let sw = screen_width();
            let sh = screen_height();

            // Map screen coordinates to grid coordinates
            let gx = (mx / sw * width as f32) as i32;
            let gy = (my / sh * height as f32) as i32;

            if gx >= 0 && gx < width as i32 && gy >= 0 && gy < height as i32 {
                grid.perturb(gx as usize, gy as usize, 5.0);
            }
        }

        // --- Rain Simulation ---
        // Random drops to keep the system alive
        if rand::gen_range(0, 100) < 2 {
            let rx = rand::gen_range(2, width - 2);
            let ry = rand::gen_range(2, height - 2);
            grid.perturb(rx, ry, 3.0);
        }

        // --- Physics Step ---
        grid.update();

        // --- Audio Sync ---
        for p in &probes {
            let h = grid.get_height(p.x, p.y);
            p.update(h);
        }

        // --- Render ---
        for y in 0..height {
            for x in 0..width {
                let h = grid.get_height(x, y);
                // Visualization:
                // Height maps to brightness/color.
                // h is roughly -2.0 to 2.0

                let val = (h + 1.0) * 0.5; // shift to 0.0-1.0 roughly
                let val = val.clamp(0.0, 1.0);

                // Palette: Deep Ocean (0, 0, 0.2) -> Cyan (0, 1, 1) -> White (1, 1, 1)
                // Let's do a simple gradient
                let r = val.powf(2.0); // Red only at peaks
                let g = val;
                let b = 0.4 + 0.6 * val;

                image.set_pixel(x as u32, y as u32, Color::new(r, g, b, 1.0));
            }
        }

        // Draw Probes
        for (i, p) in probes.iter().enumerate() {
            let col = if i == 0 { RED } else if i == 1 { GREEN } else { YELLOW };
            // Draw a small cross
            let px = p.x as u32;
            let py = p.y as u32;
            if px > 0 && px < width as u32 - 1 && py > 0 && py < height as u32 - 1 {
                 image.set_pixel(px, py, col);
                 image.set_pixel(px+1, py, col);
                 image.set_pixel(px-1, py, col);
                 image.set_pixel(px, py+1, col);
                 image.set_pixel(px, py-1, col);
            }
        }

        texture.update(&image);

        clear_background(BLACK);
        draw_texture_ex(&texture, 0.0, 0.0, WHITE, DrawTextureParams {
            dest_size: Some(vec2(screen_width(), screen_height())),
            ..Default::default()
        });

        draw_text("Hydro-Phonics", 10.0, 30.0, 30.0, WHITE);
        draw_text("Click to create ripples. Probes modulate Sound.", 10.0, 50.0, 20.0, LIGHTGRAY);

        next_frame().await
    }
}
