mod audio;
mod parser;
mod tui;

use anyhow::Result;
use audio::AudioEngine;
use parser::CodeParser;
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
    // Handle potential error if file doesn't exist
    let voices = match CodeParser::parse_file(path) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to parse {}: {}", path, e);
            return Err(e);
        }
    };

    if voices.is_empty() {
        eprintln!(
            "No voices found in {}. Ensure the file contains functions.",
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

    // Playback loop
    let tick_rate = Duration::from_millis(16); // ~60 FPS
    let mut last_tick = Instant::now();

    while !app.should_quit {
        let loop_start = Instant::now();
        let dt = loop_start.duration_since(last_tick).as_secs_f32();
        last_tick = loop_start;

        // Handle input
        app.handle_events()?;

        // Advance virtual audio engine
        audio.update(dt);

        // Update voices
        for (i, state) in app.voices.iter_mut().enumerate() {
            let mut play_next = false;

            if !state.started {
                play_next = true;
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

            if play_next {
                if state.current_token_idx < state.voice.tokens.len() {
                    let token = &state.voice.tokens[state.current_token_idx];

                    if token.duration > 0.0 {
                        audio.play_synth_note(
                            i,
                            token.pitch,
                            token.duration,
                            token.velocity,
                            token.waveform,
                            token.adsr
                        );
                        state.started = true;
                        state.last_play_time = loop_start;
                    } else {
                        // Skip zero duration tokens immediately
                        state.current_token_idx += 1;
                        // Potentially loop again to find next playable token, but simple is fine
                    }
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
