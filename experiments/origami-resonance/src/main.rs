use crossbeam_channel::bounded;
use macroquad::prelude::*;
use origami::{generate_miura_grid, MiuraParams, Orientation};
use resonance_audio::{AudioCommand, AudioModel, AudioSnapshot};

const GRID_COLS: usize = 20;
const GRID_ROWS: usize = 20;
const AUDIO_SIZE: usize = 100;
const AUDIO_BUFFER_SIZE: usize = 512;

fn window_conf() -> Conf {
    Conf {
        window_title: "Origami Resonance".to_owned(),
        window_width: 800,
        window_height: 800,
        high_dpi: true,
        ..Default::default()
    }
}

// We implement a custom main function to bypass macroquad in headless CI.
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.contains(&"--headless".to_string()) {
        println!("Headless mode enabled. Exiting.");
        return;
    }

    macroquad::Window::from_config(window_conf(), async_main());
}

async fn async_main() {
    let (cmd_tx, cmd_rx) = bounded(1024);
    let (snap_tx, snap_rx) = bounded(1);

    // Create the audio model
    let mut audio_model = AudioModel::new(AUDIO_SIZE, AUDIO_SIZE, cmd_rx, snap_tx, None);

    // Audio thread for processing audio
    std::thread::spawn(move || {
        let mut buffer = vec![0.0; AUDIO_BUFFER_SIZE];
        loop {
            // Process the audio model and consume commands.
            // Normally this is driven by an audio callback like CPAL,
            // but for simulation we just loop with a small sleep to avoid 100% CPU.
            audio_model.process(&mut buffer);
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
    });

    let mut extension_factor;
    let mut time = 0.0;

    let params = MiuraParams {
        a: 0.5,
        b: 0.5,
        gamma: 80.0f32.to_radians(),
        orientation: Orientation::Horizontal,
    };

    let mut last_snapshot: Option<AudioSnapshot> = None;

    loop {
        clear_background(Color::new(0.05, 0.05, 0.05, 1.0));

        // Read snapshot
        while let Ok(snap) = snap_rx.try_recv() {
            last_snapshot = Some(snap);
        }

        // Draw Acoustic Field
        if let Some(snap) = &last_snapshot {
            let cell_w = screen_width() / AUDIO_SIZE as f32;
            let cell_h = screen_height() / AUDIO_SIZE as f32;

            for y in 0..AUDIO_SIZE {
                for x in 0..AUDIO_SIZE {
                    let idx = y * AUDIO_SIZE + x;
                    let pressure = snap.pressure[idx];

                    if pressure.abs() > 0.05 {
                        let c = if pressure > 0.0 {
                            Color::new(0.1, 0.6, 0.9, (pressure * 0.5).clamp(0.0, 1.0))
                        } else {
                            Color::new(0.9, 0.2, 0.1, (-pressure * 0.5).clamp(0.0, 1.0))
                        };
                        draw_rectangle(x as f32 * cell_w, y as f32 * cell_h, cell_w, cell_h, c);
                    }
                }
            }
        }

        // Modulate extension factor with sine wave (breathing)
        time += get_frame_time();
        extension_factor = 0.5 + (time * 1.5).sin() * 0.4;

        // Generate origami points
        let points = generate_miura_grid(params, (GRID_COLS, GRID_ROWS), extension_factor);

        let center_x = screen_width() / 2.0;
        let center_y = screen_height() / 2.0;
        let scale = 150.0; // scale up the origami grid

        for p in points.iter() {
            let screen_x = center_x + p.x * scale;
            let screen_y = center_y + p.y * scale;

            // Map screen coords to Audio Space
            let audio_x = ((screen_x / screen_width()) * AUDIO_SIZE as f32) as usize;
            let audio_y = ((screen_y / screen_height()) * AUDIO_SIZE as f32) as usize;

            if audio_x < AUDIO_SIZE && audio_y < AUDIO_SIZE {
                // If Z is high (mountain fold), send pluck
                if p.z.abs() > 0.1 {
                    let strength = (p.z.abs() * 0.5).clamp(0.0, 1.0);
                    let _ = cmd_tx.try_send(AudioCommand::Pluck {
                        x: audio_x,
                        y: audio_y,
                        strength,
                    });

                    // Draw active node
                    let node_color = Color::new(1.0, 1.0, 1.0, strength);
                    draw_circle(screen_x, screen_y, 2.0, node_color);
                } else {
                    draw_circle(screen_x, screen_y, 1.0, Color::new(0.4, 0.4, 0.4, 0.5));
                }
            }
        }

        draw_text(
            "Origami Resonance - Acoustic Morphogenesis",
            20.0,
            30.0,
            20.0,
            WHITE,
        );
        draw_text(
            format!("Extension Factor: {:.2}", extension_factor).as_str(),
            20.0,
            50.0,
            20.0,
            LIGHTGRAY,
        );

        next_frame().await;
    }
}
