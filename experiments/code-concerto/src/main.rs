mod audio;
mod model;
mod parser;
mod visualizer;

use audio::AudioEngine;
use macroquad::prelude::*;
use model::{CodeConcerto, MusicalEvent};
use parser::Parser;
use std::env;
use visualizer::Visualizer;

struct PlaybackState {
    events: Vec<(f64, MusicalEvent)>,
    next_event_idx: usize,
}

fn flatten_concerto(concerto: &CodeConcerto) -> Vec<(f64, MusicalEvent)> {
    let mut abs_events = Vec::new();
    let mut current_time = 0.0;

    for section in &concerto.sections {
        for event in &section.events {
            match event {
                MusicalEvent::Wait(d) => {
                    current_time += d.as_secs_f64();
                }
                MusicalEvent::NoteOn { .. } => {
                    abs_events.push((current_time, event.clone()));
                }
            }
        }
    }
    abs_events
}

#[macroquad::main("Code Concerto")]
async fn main() {
    #[cfg(not(feature = "audio"))]
    {
        println!("Audio disabled. Run with --features audio to hear the concerto.");
    }

    if let Err(e) = run().await {
        eprintln!("Error: {}", e);
    }
}

async fn run() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    let path = if args.len() > 1 {
        &args[1]
    } else {
        "experiments/code-concerto/src/main.rs"
    };

    println!("Parsing {}...", path);
    let concerto = match Parser::parse_file(path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error parsing file: {}", e);
            CodeConcerto::default()
        }
    };
    println!("Generated {} sections.", concerto.sections.len());

    let mut audio = AudioEngine::new()?;
    let mut visualizer = Visualizer::new(concerto);

    let flat_events = flatten_concerto(&visualizer.concerto);

    let mut playback = PlaybackState {
        events: flat_events,
        next_event_idx: 0,
    };

    println!("Starting playback with {} events.", playback.events.len());

    loop {
        if is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::Q) {
            break;
        }

        let dt = get_frame_time() as f64;

        if !is_key_down(KeyCode::Space) {
             visualizer.update(dt);

             let now = visualizer.current_time;
             while playback.next_event_idx < playback.events.len() {
                 let (t, event) = &playback.events[playback.next_event_idx];
                 if *t <= now {
                     audio.play_event(event);
                     playback.next_event_idx += 1;
                 } else {
                     break;
                 }
             }
        }

        clear_background(BLACK);
        visualizer.draw();

        draw_text("Code Concerto", 10.0, 30.0, 30.0, WHITE);
        draw_text(&format!("Target: {}", path), 10.0, 50.0, 20.0, GRAY);
        draw_text("Space to Pause", 10.0, 70.0, 20.0, GRAY);

        next_frame().await;
    }

    Ok(())
}
