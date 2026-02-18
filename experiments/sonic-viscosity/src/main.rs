mod audio;
mod vis;

use audio::{SonicEngine, Spectrum};
use crossbeam_channel::unbounded;
use macroquad::prelude::*;
use noise::Perlin;
use vis::{FluidCanvas, Painter};

#[macroquad::main("Sonic Viscosity")]
async fn main() {
    let (tx, rx) = unbounded();
    let engine = SonicEngine::new(tx);

    // Audio Setup
    start_audio(engine);

    // Vis Setup
    let w = 200;
    let h = 150;
    let mut canvas = FluidCanvas::new(w, h);
    let texture = Texture2D::from_image(&Image::gen_image_color(w as u16, h as u16, BLACK));
    texture.set_filter(FilterMode::Nearest); // or Linear for smoother look? Let's try Linear.
    texture.set_filter(FilterMode::Linear);

    let mut painters = vec![
        Painter::new(w as f32 * 0.5, h as f32 * 0.5, RED), // Low
        Painter::new(w as f32 * 0.5, h as f32 * 0.5, GREEN), // Mid
        Painter::new(w as f32 * 0.5, h as f32 * 0.5, BLUE), // High
    ];

    let noise = Perlin::new(1);
    let mut spectrum = Spectrum {
        low: 0.0,
        mid: 0.0,
        high: 0.0,
        raw: vec![],
    };

    loop {
        // Receive latest spectrum (drain channel to get latest)
        while let Ok(s) = rx.try_recv() {
            spectrum = s;
        }

        let dt = get_frame_time();

        // Normalize spectrum values roughly (heuristic based on FFT size 1024)
        // Magnitudes can be around 10-100.
        let low_n = (spectrum.low * 0.05).clamp(0.0, 1.0);
        let mid_n = (spectrum.mid * 0.05).clamp(0.0, 1.0);
        let high_n = (spectrum.high * 0.05).clamp(0.0, 1.0);

        // Update Painters
        // Low Freq Painter -> Red/Dark, Slow
        painters[0].color = Color::new(0.5 + low_n * 0.5, 0.1, 0.1, 0.05);
        painters[0].update(dt, low_n, &noise, vec2(w as f32, h as f32));

        // Mid Freq Painter -> Green/Yellow, Medium
        painters[1].color = Color::new(0.1, 0.5 + mid_n * 0.5, 0.1, 0.05);
        painters[1].update(dt, mid_n, &noise, vec2(w as f32, h as f32));

        // High Freq Painter -> Blue/Cyan, Fast
        painters[2].color = Color::new(0.1, 0.1, 0.5 + high_n * 0.5, 0.05);
        painters[2].update(dt, high_n, &noise, vec2(w as f32, h as f32));

        // Paint
        for p in &painters {
            canvas.paint(p.pos.x, p.pos.y, p.size, p.color);
        }

        // Diffuse
        canvas.diffuse();

        // Update Texture
        let image = Image {
            bytes: canvas.pixels.clone(),
            width: w as u16,
            height: h as u16,
        };
        texture.update(&image);

        // Draw
        clear_background(BLACK);

        // Draw texture scaled up
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

        // Overlay info
        draw_text("SONIC VISCOSITY", 20.0, 30.0, 30.0, WHITE);
        draw_text("Spectrogram + Oil Painting", 20.0, 50.0, 20.0, LIGHTGRAY);
        draw_text(&format!("FPS: {}", get_fps()), 20.0, 70.0, 20.0, DARKGRAY);

        // Visualizers for spectrum
        let bar_base = screen_height() - 20.0;
        draw_rectangle(20.0, bar_base - low_n * 200.0, 20.0, low_n * 200.0, RED);
        draw_rectangle(50.0, bar_base - mid_n * 200.0, 20.0, mid_n * 200.0, GREEN);
        draw_rectangle(80.0, bar_base - high_n * 200.0, 20.0, high_n * 200.0, BLUE);

        next_frame().await
    }
}

#[cfg(feature = "audio")]
fn start_audio(engine: SonicEngine) {
    std::thread::spawn(move || {
        use rodio::{OutputStream, Sink};
        // Create the stream in this thread and block
        let stream_result = OutputStream::try_default();
        if let Ok((_stream, stream_handle)) = stream_result {
            if let Ok(sink) = Sink::try_new(&stream_handle) {
                sink.append(engine);
                sink.sleep_until_end();
            }
        }
    });
}

#[cfg(not(feature = "audio"))]
fn start_audio(engine: SonicEngine) {
    std::thread::spawn(move || {
        // Just consume the iterator to generate analysis events
        // Throttle it to approx real time
        let frame_time = std::time::Duration::from_secs_f32(1.0 / 44100.0);
        let start = std::time::Instant::now();
        let mut count = 0;

        for _ in engine {
            count += 1;
            if count % 1024 == 0 {
                // Sync roughly
                let expected = frame_time * count;
                let elapsed = start.elapsed();
                if elapsed < expected {
                    std::thread::sleep(expected - elapsed);
                }
            }
        }
    });
}
