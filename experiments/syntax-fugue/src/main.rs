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
            // Fallback to default if user provided bad path? Or just exit.
            return Err(e);
        }
    };

    if voices.is_empty() {
        eprintln!("No voices found in {}. Ensure the file contains functions.", path);
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

    while !app.should_quit {
        let loop_start = Instant::now();

        // Handle input
        app.handle_events()?;

        // Update voices
        for (i, state) in app.voices.iter_mut().enumerate() {
            // Check if audio finished playing the previous note
            if !audio.is_voice_busy(i) {
                // If previously playing, we finished that note, so advance.
                // If not started yet, we stay at 0.
                if state.started {
                    state.current_token_idx += 1;
                }

                // Check if we reached the end of the voice
                if state.current_token_idx >= state.voice.tokens.len() {
                    continue; // Voice finished
                }

                // Play the current note
                let token = &state.voice.tokens[state.current_token_idx];

                // Skip if duration is 0?
                if token.duration > 0.0 {
                    audio.play_note(i, token.pitch, token.duration, token.velocity);
                    state.started = true;
                    state.last_play_time = loop_start;
                } else {
                    // Immediate advance if duration is 0 (shouldn't happen with our parser)
                    state.started = true; // Mark as started so next loop advances
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

    Ok(())
}
