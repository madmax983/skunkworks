use macroquad::prelude::*;
use macroquad::audio::{load_sound, play_sound_once};

use spectral_scribe::encoder::{EncoderConfig, generate_audio, save_wav};
use spectral_scribe::decoder::{DecoderConfig, audio_to_spectrogram, recover_text};

enum AppState {
    Editor,
    Playing {
        start_time: f64,
        spectrogram: Vec<Vec<f32>>,
        duration: f64,
        text: String,
    },
}

#[macroquad::main("Spectral Scribe")]
async fn main() {
    let mut state = AppState::Editor;
    let mut input_text = String::from("HELLO WORLD");

    // Configs
    let enc_config = EncoderConfig::default();
    let dec_config = DecoderConfig::default();

    loop {
        clear_background(BLACK);

        let mut next_state = None;

        match &mut state {
            AppState::Editor => {
                draw_text("SPECTRAL SCRIBE", 20.0, 40.0, 40.0, WHITE);
                draw_text("Type your secret message:", 20.0, 80.0, 20.0, GRAY);

                // Input handling
                while let Some(c) = get_char_pressed() {
                    if c.is_ascii_graphic() || c == ' ' {
                        input_text.push(c);
                    }
                }
                if is_key_pressed(KeyCode::Backspace) {
                    input_text.pop();
                }

                draw_text(&input_text, 20.0, 120.0, 30.0, GREEN);
                let cursor_x = 20.0 + measure_text(&input_text, None, 30, 1.0).width;
                if (get_time() * 2.0) as i32 % 2 == 0 {
                    draw_text("_", cursor_x, 120.0, 30.0, GREEN);
                }

                draw_text("Press [ENTER] to Encrypt & Play", 20.0, 200.0, 20.0, WHITE);
                draw_text("Note: Generates 'output.wav' in current dir", 20.0, 230.0, 20.0, DARKGRAY);

                if is_key_pressed(KeyCode::Enter) && !input_text.is_empty() {
                    let samples = generate_audio(&input_text, &enc_config);

                    if save_wav("output.wav", &samples, enc_config.sample_rate).is_ok() {
                        // Load spectrogram
                        let spectrogram = audio_to_spectrogram(&samples, &dec_config);
                        let duration = samples.len() as f64 / enc_config.sample_rate as f64;

                        // Verify decoding
                        let decoded = recover_text(&spectrogram, &dec_config);

                        match load_sound("output.wav").await {
                            Ok(sound) => {
                                play_sound_once(&sound);
                                next_state = Some(AppState::Playing {
                                    start_time: get_time(),
                                    spectrogram,
                                    duration,
                                    text: decoded,
                                });
                            },
                            Err(e) => {
                                eprintln!("Audio load failed: {}", e);
                                next_state = Some(AppState::Playing {
                                    start_time: get_time(),
                                    spectrogram,
                                    duration,
                                    text: decoded,
                                });
                            }
                        }
                    }
                }
            },
            AppState::Playing { start_time, spectrogram, duration, text } => {
                let time = get_time() - *start_time;

                if time > *duration + 2.0 {
                    next_state = Some(AppState::Editor);
                }

                let current_frame = (time * enc_config.sample_rate as f64 / enc_config.fft_size as f64) as usize;
                let frame_width = 4.0;
                let center_x = screen_width() / 2.0;
                let visible_frames = (screen_width() / frame_width) as usize;

                // Draw Spectrogram
                for (i, frame) in spectrogram.iter().enumerate() {
                    let offset = i as i32 - current_frame as i32;
                    if offset.abs() < (visible_frames as i32 / 2) + 10 {
                        let x = center_x + offset as f32 * frame_width;

                        // Optimization: Only draw bins with energy
                        // And zoom in vertically?
                        // Frequencies 0..Nyquist mapped to height.
                        // 512 bins -> 1024 height? Screen is usually 600-800.
                        // Scale y by 1.5

                        for (bin_idx, &mag) in frame.iter().enumerate() {
                            let val = (mag / 40.0).clamp(0.0, 1.0);
                            if val > 0.15 {
                                let color = Color::new(val, val * 0.8, val * 0.2, 1.0);
                                let y = screen_height() - bin_idx as f32 * 1.5 - 50.0;
                                draw_rectangle(x, y, frame_width, 1.5, color);
                            }
                        }
                    }
                }

                draw_text(&format!("BROADCASTING: {}", text), 20.0, 40.0, 30.0, RED);
                draw_line(center_x, 0.0, center_x, screen_height(), 2.0, RED);

                if is_key_pressed(KeyCode::Escape) {
                    next_state = Some(AppState::Editor);
                }
            }
        }

        if let Some(s) = next_state {
            state = s;
        }

        next_frame().await
    }
}
