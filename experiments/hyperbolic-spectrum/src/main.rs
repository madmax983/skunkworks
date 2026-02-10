use macroquad::prelude::*;
use macroquad::audio::{load_sound_from_bytes, play_sound_once, Sound};
use poincare_disk::{mobius_add, mobius_sub, Point};

mod audio;
mod font;
mod spectrogram;

use audio::{AudioConfig, generate_audio_from_text, create_wav_buffer};
use spectrogram::Spectrogram;

#[derive(Clone)]
struct SpectralFrame {
    spectrum: Vec<f32>,
    _timestamp: f64,
}

#[macroquad::main("Hyperbolic Spectrum")]
async fn main() {
    // Config
    let audio_config = AudioConfig::default();
    let mut spectrogram_analyzer = Spectrogram::new(audio_config.fft_size);

    // State
    let mut input_text = String::from("HYPERBOLIC");
    let mut audio_samples: Vec<f32> = Vec::new();
    let mut spectral_history: Vec<SpectralFrame> = Vec::new();
    let mut playback_start_time: Option<f64> = None;
    let mut _current_sound: Option<Sound> = None;

    // Camera
    let mut view_offset = Point::new(0.0, 0.0);
    let zoom = 1.0;

    loop {
        clear_background(BLACK);

        // --- Input Handling ---

        // Typing
        while let Some(c) = get_char_pressed() {
            if c.is_ascii_graphic() || c == ' ' {
                input_text.push(c);
            }
        }
        if is_key_pressed(KeyCode::Backspace) {
            input_text.pop();
        }

        // Trigger Generation
        if is_key_pressed(KeyCode::Enter) && !input_text.is_empty() {
            // 1. Generate Audio
            audio_samples = generate_audio_from_text(&input_text, &audio_config);

            // 2. Create WAV
            if let Ok(wav_bytes) = create_wav_buffer(&audio_samples, audio_config.sample_rate) {
                // 3. Load Sound
                if let Ok(sound) = load_sound_from_bytes(&wav_bytes).await {
                    play_sound_once(&sound);
                    _current_sound = Some(sound);
                    playback_start_time = Some(get_time());
                    spectral_history.clear(); // Start fresh or keep history? Let's clear for clarity.
                }
            }
        }

        // Camera Pan
        let pan_speed = 0.02 * zoom;
        let mut move_vec = Point::new(0.0, 0.0);
        if is_key_down(KeyCode::Up) { move_vec.im += pan_speed; }
        if is_key_down(KeyCode::Down) { move_vec.im -= pan_speed; }
        if is_key_down(KeyCode::Left) { move_vec.re -= pan_speed; }
        if is_key_down(KeyCode::Right) { move_vec.re += pan_speed; }

        if move_vec.norm() > 0.0 {
            view_offset = mobius_add(view_offset, move_vec);
        }

        // --- Audio Analysis ---

        if let Some(start_time) = playback_start_time {
            let now = get_time();
            let elapsed = now - start_time;

            // Analyze current chunk
            let sample_idx = (elapsed * audio_config.sample_rate as f64) as usize;

            if sample_idx < audio_samples.len() {
                // Extract chunk centered on current time
                // Or just the window starting at current time?
                // FFT window size
                let window_size = audio_config.fft_size;
                let chunk_start = sample_idx;
                let chunk_end = (chunk_start + window_size).min(audio_samples.len());

                if chunk_end > chunk_start {
                    let chunk = &audio_samples[chunk_start..chunk_end];
                    let spectrum = spectrogram_analyzer.analyze_chunk(chunk);

                    spectral_history.push(SpectralFrame {
                        spectrum,
                        _timestamp: elapsed,
                    });
                }
            } else {
                // Playback finished
                 playback_start_time = None;
            }
        }

        // --- Rendering ---

        let center_x = screen_width() / 2.0;
        let center_y = screen_height() / 2.0;
        let radius = screen_height().min(screen_width()) * 0.45;

        // Draw Disk Boundary
        draw_circle(center_x, center_y, radius, DARKGRAY);
        draw_circle_lines(center_x, center_y, radius, 2.0, WHITE);

        // Draw Spectral History
        // We iterate backwards from latest frame.
        // Latest frame is at r = 0 (center)
        // Older frames move outwards r -> 1.

        let max_history = 200; // Limit rendering for perf
        let history_len = spectral_history.len();
        let start_idx = if history_len > max_history { history_len - max_history } else { 0 };

        for (i, frame) in spectral_history.iter().enumerate().skip(start_idx) {
            // Calculate "Age" of the frame relative to now (or just latest frame)
            // If we are playing, the latest frame is "Now".
            // So index `i` is older than `history_len - 1`.
            // Wait, history appends. So `history_len - 1` is the NEWEST.
            // `0` is the OLDEST.

            // Visualization Concept:
            // "The Source" is at the center. Sound emanates OUTWARDS.
            // So the NEWEST frame should be at r=0.
            // The OLDEST frame should be at r -> 1.

            let age_idx = history_len - 1 - i; // 0 for newest

            // Map age to hyperbolic radius
            // r = tanh(age * scale)
            let scale = 0.05;
            let r_hyp = (age_idx as f64 * scale).tanh(); // [0, 1)

            // Apply Camera (Möbius transform)
            // We treat the ring as a set of points in the complex plane.
            // But drawing full rings is hard if we transform every point.
            // Optimization: Transform the CENTER? No, the center is 0.
            // If we pan, the center moves.

            // Let's draw arcs.
            // Frequency bins map to angle.
            let num_bins = frame.spectrum.len();

            // We only draw significant bins to save perf
            for (bin_idx, &mag) in frame.spectrum.iter().enumerate() {
                if mag < 0.1 { continue; } // Threshold

                let angle = (bin_idx as f64 / num_bins as f64) * 2.0 * std::f64::consts::PI;

                // Position in source frame (centered at 0)
                let p_source = Point::from_polar(r_hyp, angle);

                // Apply View Offset (Camera)
                // z_screen = mobius_sub(z_world, view_offset)
                // Wait, if I move camera "forward", things should move "back".
                // So sub is correct.
                let p_screen = mobius_sub(p_source, view_offset);

                // Map to Screen Coords
                if p_screen.norm_sqr() < 0.99 {
                    let sx = center_x + p_screen.re as f32 * radius;
                    let sy = center_y - p_screen.im as f32 * radius;

                    // Color based on magnitude and frequency
                    // Low freq = Red, High freq = Blue
                    let hue = bin_idx as f32 / num_bins as f32;
                    let intensity = (mag * 0.05).clamp(0.0, 1.0);

                    let color = Color::new(
                        intensity, // Red
                        intensity * (1.0 - hue), // Green
                        intensity * hue, // Blue
                        0.8
                    );

                    // Size of point decreases with distance from center visually?
                    // Hyperbolic scaling...
                    let size = 3.0 * (1.0 - p_screen.norm() as f32) + 1.0;

                    draw_circle(sx, sy, size, color);
                }
            }
        }

        // UI
        draw_text("HYPERBOLIC SPECTRUM", 20.0, 30.0, 30.0, WHITE);
        draw_text(&format!("Input: {}_", input_text), 20.0, 60.0, 20.0, GREEN);
        draw_text("Press ENTER to transmit.", 20.0, 80.0, 16.0, LIGHTGRAY);
        draw_text("Arrows to pan view.", 20.0, 100.0, 16.0, LIGHTGRAY);

        next_frame().await
    }
}
