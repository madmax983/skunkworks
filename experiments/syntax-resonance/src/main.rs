use macroquad::prelude::*;
use resonance_audio::audio::{AudioCommand, AudioModel};
use std::path::PathBuf;

mod audio_backend;
mod parser;

const GRID_WIDTH: usize = 128;
const GRID_HEIGHT: usize = 128;
const SAMPLE_RATE: f32 = 44100.0;

#[macroquad::main("Syntax Resonance")]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = if args.len() > 1 {
        PathBuf::from(&args[1])
    } else {
        std::env::current_dir().unwrap()
    };

    println!("Scanning path: {:?}", path);
    // Note: Walker might take time for large repos. Ideally run in thread, but for simplicity run here.
    let events = parser::walk_path(&path, GRID_WIDTH, GRID_HEIGHT);
    println!("Generated {} events.", events.len());

    // Audio Setup
    let (cmd_tx, cmd_rx) = crossbeam_channel::bounded(1024);
    let (snap_tx, snap_rx) = crossbeam_channel::bounded(2);

    #[cfg(feature = "audio")]
    let mut _stream: Option<cpal::Stream> = None;

    #[cfg(feature = "audio")]
    let mut manual_model: Option<AudioModel> = None;

    #[cfg(not(feature = "audio"))]
    let mut manual_model: Option<AudioModel> =
        Some(AudioModel::new(GRID_WIDTH, GRID_HEIGHT, cmd_rx, snap_tx));

    #[cfg(feature = "audio")]
    {
        match audio_backend::init_audio(cmd_rx.clone(), snap_tx.clone(), GRID_WIDTH, GRID_HEIGHT) {
            Ok(s) => {
                println!("Audio initialized.");
                _stream = Some(s);
            }
            Err(e) => {
                eprintln!(
                    "Audio init failed: {}. Falling back to manual simulation.",
                    e
                );
                manual_model = Some(AudioModel::new(GRID_WIDTH, GRID_HEIGHT, cmd_rx, snap_tx));
            }
        }
    }

    // Visual Setup
    let mut image = Image::gen_image_color(GRID_WIDTH as u16, GRID_HEIGHT as u16, BLACK);
    let texture = Texture2D::from_image(&image);
    texture.set_filter(FilterMode::Nearest);

    let mut current_snapshot = vec![0.0; GRID_WIDTH * GRID_HEIGHT];
    let mut walls = vec![false; GRID_WIDTH * GRID_HEIGHT];

    let mut event_idx = 0;
    let mut playback_time = 0.0;
    let mut last_label = String::new();
    let mut last_file = String::new();

    loop {
        let dt = get_frame_time();
        playback_time += dt;

        // Process Events
        while event_idx < events.len() && events[event_idx].time_offset <= playback_time {
            let event = &events[event_idx];
            let _ = cmd_tx.send(event.command.clone());

            match event.command {
                AudioCommand::AddWall { x, y } => {
                    if x < GRID_WIDTH && y < GRID_HEIGHT {
                        walls[y * GRID_WIDTH + x] = true;
                    }
                }
                AudioCommand::RemoveWall { x, y } => {
                    if x < GRID_WIDTH && y < GRID_HEIGHT {
                        walls[y * GRID_WIDTH + x] = false;
                    }
                }
                AudioCommand::ClearWalls => {
                    walls.fill(false);
                }
                _ => {}
            }

            last_label = event.label.clone();
            last_file = event.file.clone();
            event_idx += 1;
        }

        // Manual Simulation Step
        if let Some(model) = &mut manual_model {
            // Run simulation at a reasonable rate for visuals
            // If we run full audio rate (44100), it might be slow.
            // But we need to drain commands.
            // Let's run chunks.
            let steps = (dt * SAMPLE_RATE).min(2000.0) as usize;
            let mut dummy = vec![0.0; steps];
            model.process(&mut dummy);
        }

        // Poll snapshot
        while let Ok(snap) = snap_rx.try_recv() {
            current_snapshot = snap;
        }

        // Render to Image
        for (i, &val) in current_snapshot.iter().enumerate() {
            let x = (i % GRID_WIDTH) as u32;
            let y = (i / GRID_WIDTH) as u32;

            if walls[i] {
                image.set_pixel(x, y, WHITE);
            } else {
                let v = val * 5.0;
                let c = if v > 0.0 {
                    Color::new(v.min(1.0), 0.0, 0.2, 1.0)
                } else {
                    Color::new(0.0, 0.2, (-v).min(1.0), 1.0)
                };
                image.set_pixel(x, y, c);
            }
        }
        texture.update(&image);

        clear_background(BLACK);

        let scale = (screen_width() / GRID_WIDTH as f32).min(screen_height() / GRID_HEIGHT as f32);
        let draw_width = GRID_WIDTH as f32 * scale;
        let draw_height = GRID_HEIGHT as f32 * scale;
        let offset_x = (screen_width() - draw_width) / 2.0;
        let offset_y = (screen_height() - draw_height) / 2.0;

        draw_texture_ex(
            &texture,
            offset_x,
            offset_y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(draw_width, draw_height)),
                ..Default::default()
            },
        );

        draw_text(&format!("File: {}", last_file), 10.0, 20.0, 20.0, WHITE);
        draw_text(&format!("Event: {}", last_label), 10.0, 40.0, 30.0, YELLOW);
        draw_text(
            &format!("Events: {}/{}", event_idx, events.len()),
            10.0,
            70.0,
            20.0,
            LIGHTGRAY,
        );

        let progress = if events.is_empty() {
            0.0
        } else {
            event_idx as f32 / events.len() as f32
        };
        draw_rectangle(
            0.0,
            screen_height() - 10.0,
            screen_width() * progress,
            10.0,
            GREEN,
        );

        next_frame().await
    }
}
