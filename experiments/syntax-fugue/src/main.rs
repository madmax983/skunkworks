mod audio;
mod parser;
mod tui;

use anyhow::Result;
use audio::AudioEngine;
use parser::{CodeParser, VoiceType};
use std::env;
use std::time::{Duration, Instant};
use tui::TuiApp;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    // Default to a file that has good structure, like parser.rs itself
    let default_path = "experiments/syntax-fugue/src/parser.rs";
    let path = if args.len() > 1 {
        args[1].as_str()
    } else {
        default_path
    };

    // Parse the code
    println!("Parsing {}...", path);
    let voices = match CodeParser::parse_file(path) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to parse {}: {}", path, e);
            return Err(e);
        }
    };

    if voices.is_empty() {
        eprintln!(
            "No voices found in {}. Ensure the file contains functions, structs, enums, or impls.",
            path
        );
        return Ok(());
    }

    println!("Found {} voices.", voices.len());
    // Short sleep to let user read
    std::thread::sleep(Duration::from_millis(1000));

    // Initialize systems
    let mut audio = AudioEngine::new()?;
    audio.ensure_voice_capacity(voices.len());

    let mut app = TuiApp::new(voices)?;

    // Configure Fugue delays
    let fugue_start = Instant::now();

    // We want staggered entries:
    // Bass (Structs) starts immediately.
    // Tenor (Enums) enters after 4 seconds (approx 2 bars).
    // Alto (Impls) enters after 8 seconds.
    // Soprano (Functions) enters after 12 seconds.

    for state in &mut app.voices {
        match state.voice.voice_type {
            VoiceType::Bass => state.start_delay = Duration::from_secs(0),
            VoiceType::Tenor => state.start_delay = Duration::from_secs(4),
            VoiceType::Alto => state.start_delay = Duration::from_secs(8),
            VoiceType::Soprano => state.start_delay = Duration::from_secs(12),
        }
        // Reset last_play_time to now so elapsed works correctly once started
        state.last_play_time = fugue_start;
    }

    // Playback loop
    let tick_rate = Duration::from_millis(16); // ~60 FPS
    let mut last_tick = Instant::now();

    while !app.should_quit {
        let loop_start = Instant::now();
        let dt = loop_start.duration_since(last_tick).as_secs_f32();
        last_tick = loop_start;
        let time_since_start = loop_start.duration_since(fugue_start);

        // Handle input
        app.handle_events()?;

        // Advance virtual audio engine
        audio.update(dt);

        // Update voices
        for (i, state) in app.voices.iter_mut().enumerate() {
            let mut play_next = false;

            if !state.started {
                // Check if it's time to start
                if time_since_start >= state.start_delay {
                    state.started = true;
                    play_next = true;
                    // Reset last_play_time so the first note plays immediately
                    // Actually, we set play_next=true which plays immediately.
                    // We need to set last_play_time to now AFTER playing.
                }
            } else {
                // Check if current note finished
                if state.current_token_idx < state.voice.tokens.len() {
                    let current_token = &state.voice.tokens[state.current_token_idx];
                    if state.last_play_time.elapsed().as_secs_f32() >= current_token.duration {
                        state.current_token_idx += 1;
                        play_next = true;
                    }
                }
            }

            if play_next && state.current_token_idx < state.voice.tokens.len() {
                let token = &state.voice.tokens[state.current_token_idx];

                // Skip tokens with <= 0 duration (if any)
                if token.duration > 0.0 {
                    audio.play_synth_note(
                        i,
                        token.pitch,
                        token.duration,
                        token.velocity,
                        token.waveform,
                        token.adsr,
                    );
                    state.last_play_time = loop_start;
                } else {
                    // Immediate skip
                    state.current_token_idx += 1;
                    // Loop again? For simplicity, just wait next frame or use recursion.
                    // But since 16ms is fast enough, next frame is fine usually.
                    // Unless we have many 0-duration tokens.
                    // Let's assume parser handles duration >= 0.1s.
                }
            }
        }

        // Render TUI
        app.draw()?;

        // Cap frame rate
        let loop_duration = loop_start.elapsed();
        if loop_duration < tick_rate {
            std::thread::sleep(tick_rate - loop_duration);
        }
    }

    // Save audio on exit
    println!("Saving composition to output.wav...");
    if let Err(e) = audio.save_wav("output.wav") {
        eprintln!("Failed to save WAV: {}", e);
    } else {
        println!("Saved output.wav successfully.");
    }

    Ok(())
}
