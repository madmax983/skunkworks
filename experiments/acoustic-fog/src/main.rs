use crossbeam_channel::bounded;
use hound::WavSpec;
use macroquad::prelude::*;
use resonance_audio::audio::{AudioCommand, AudioModel};
use std::path::Path;
use std::thread;
use std::time::{Duration, Instant};

mod treemap;
use treemap::{generate_treemap, FileRect};

const SIM_WIDTH: usize = 120;
const SIM_HEIGHT: usize = 90;
const SAMPLE_RATE: u32 = 44100;
const BUFFER_SIZE: usize = 735; // ~60Hz at 44.1kHz

#[macroquad::main("Acoustic Fog")]
async fn main() {
    // 1. Setup Channels
    let (cmd_tx, cmd_rx) = bounded(2048);
    let (snap_tx, snap_rx) = bounded(2);

    // 2. Setup Audio Thread (Simulation + WAV Writer)
    thread::spawn(move || {
        // Initialize Audio Model
        let mut model = AudioModel::new(SIM_WIDTH, SIM_HEIGHT, cmd_rx, snap_tx);

        // Setup WAV Writer
        let spec = WavSpec {
            channels: 1,
            sample_rate: SAMPLE_RATE,
            bits_per_sample: 32,
            sample_format: hound::SampleFormat::Float,
        };
        let mut writer = match hound::WavWriter::create("acoustic_fog_output.wav", spec) {
            Ok(w) => Some(w),
            Err(e) => {
                eprintln!("Failed to create WAV writer: {}", e);
                None
            }
        };

        // Simulation Loop
        let mut buffer = vec![0.0; BUFFER_SIZE];

        loop {
            let start = Instant::now();

            // Process simulation
            model.process(&mut buffer);

            // Write to WAV
            if let Some(w) = &mut writer {
                for &sample in &buffer {
                    if let Err(e) = w.write_sample(sample) {
                         eprintln!("WAV write error: {}", e);
                    }
                }
            }

            // Throttle to real-time
            let elapsed = start.elapsed();
            let frame_duration = Duration::from_micros((BUFFER_SIZE as u64 * 1_000_000) / SAMPLE_RATE as u64);

            if frame_duration > elapsed {
                thread::sleep(frame_duration - elapsed);
            }
        }
    });

    // 3. Generate Map
    // Use current directory as root, or "experiments" if we can find it.
    let root = if Path::new("experiments").exists() {
        Path::new("experiments")
    } else {
        Path::new(".")
    };

    println!("Generating Acoustic Map from: {:?}", root);
    let map_rects = generate_treemap(root, SIM_WIDTH as f32, SIM_HEIGHT as f32);
    println!("Generated {} file blocks.", map_rects.len());

    // Send walls to simulation
    for file_rect in &map_rects {
        let rx = file_rect.rect.x.floor() as usize;
        let ry = file_rect.rect.y.floor() as usize;
        let rw = file_rect.rect.w.ceil() as usize;
        let rh = file_rect.rect.h.ceil() as usize;

        // Only add walls for files that are big enough to block a cell
        if rw > 0 && rh > 0 {
             for y in ry..(ry + rh).min(SIM_HEIGHT) {
                for x in rx..(rx + rw).min(SIM_WIDTH) {
                    let _ = cmd_tx.send(AudioCommand::AddWall { x, y });
                }
            }
        }
    }

    // 4. Main Loop
    let mut grid_u = vec![0.0; SIM_WIDTH * SIM_HEIGHT];
    let mut listener_pos = (SIM_WIDTH / 2, SIM_HEIGHT / 2);
    let mut last_pulse = 0.0;

    // Initial Listener
    let _ = cmd_tx.send(AudioCommand::MoveListener { x: listener_pos.0, y: listener_pos.1 });

    let texture = Texture2D::from_image(&Image::gen_image_color(
        SIM_WIDTH as u16,
        SIM_HEIGHT as u16,
        BLACK,
    ));
    texture.set_filter(FilterMode::Nearest);

    loop {
        clear_background(BLACK);

        // Input
        let (mx, my) = mouse_position();
        let sw = screen_width();
        let sh = screen_height();

        // Calculate Grid Coordinates
        let gx = (mx / sw * SIM_WIDTH as f32) as usize;
        let gy = (my / sh * SIM_HEIGHT as f32) as usize;

        if is_mouse_button_pressed(MouseButton::Left) {
             if gx < SIM_WIDTH && gy < SIM_HEIGHT {
                 let _ = cmd_tx.send(AudioCommand::Pluck { x: gx, y: gy, strength: 1.0 });
             }
        }

        if is_mouse_button_down(MouseButton::Right) {
             if gx < SIM_WIDTH && gy < SIM_HEIGHT {
                 listener_pos = (gx, gy);
                 let _ = cmd_tx.send(AudioCommand::MoveListener { x: gx, y: gy });
             }
        }

        // Pulse Mode
        if is_key_down(KeyCode::P) {
             let now = get_time();
             if now - last_pulse > 0.5 {
                 last_pulse = now;
                 // Pulse from listener position
                 let _ = cmd_tx.send(AudioCommand::Pluck { x: listener_pos.0, y: listener_pos.1, strength: 1.0 });
             }
        }

        // Receive Snapshot
        while let Ok(snap) = snap_rx.try_recv() {
            grid_u = snap;
        }

        // Update Texture
        let mut image = Image::gen_image_color(SIM_WIDTH as u16, SIM_HEIGHT as u16, BLACK);
        for y in 0..SIM_HEIGHT {
            for x in 0..SIM_WIDTH {
                let idx = y * SIM_WIDTH + x;
                let val = grid_u[idx];

                // Color Map
                // Val is approx -1.0 to 1.0
                // Red = +ve, Blue = -ve
                // Boost visibility
                let amp = 5.0;
                let v = val * amp;

                let color = if v > 0.0 {
                    Color::new(v.min(1.0), 0.0, 0.0, 1.0)
                } else {
                    Color::new(0.0, 0.0, (-v).min(1.0), 1.0)
                };
                image.set_pixel(x as u32, y as u32, color);
            }
        }

        texture.update(&image);

        // Draw Grid
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

        // Draw Walls (Overlay)
        let sx = sw / SIM_WIDTH as f32;
        let sy = sh / SIM_HEIGHT as f32;

        let mut hovered_file: Option<&FileRect> = None;

        for file in &map_rects {
            let rx = file.rect.x * sx;
            let ry = file.rect.y * sy;
            let rw = file.rect.w * sx;
            let rh = file.rect.h * sy;

            // Draw gray block with transparency
            draw_rectangle(rx, ry, rw, rh, Color::new(0.5, 0.5, 0.5, 0.2));
            draw_rectangle_lines(rx, ry, rw, rh, 1.0, GRAY);

            if mx >= rx && mx <= rx + rw && my >= ry && my <= ry + rh {
                hovered_file = Some(file);
            }
        }

        // Draw Listener
        draw_circle(
            listener_pos.0 as f32 * sx + sx/2.0,
            listener_pos.1 as f32 * sy + sy/2.0,
            5.0,
            GREEN
        );

        // Pulse Indicator
        if get_time() - last_pulse < 0.1 {
            draw_circle_lines(
                listener_pos.0 as f32 * sx + sx/2.0,
                listener_pos.1 as f32 * sy + sy/2.0,
                20.0,
                1.0,
                WHITE
            );
        }

        // UI
        draw_text("LMB: Pluck | RMB: Move Listener | Hold P: Pulse", 10.0, 20.0, 20.0, WHITE);
        draw_text("Recording to acoustic_fog_output.wav...", 10.0, 40.0, 20.0, RED);

        if let Some(file) = hovered_file {
             let name = file.path.file_name().unwrap_or_default().to_string_lossy();
             draw_text(&format!("File: {}", name), mx + 10.0, my - 10.0, 20.0, YELLOW);
             draw_text(&format!("Size: {} bytes", file.size), mx + 10.0, my + 10.0, 16.0, LIGHTGRAY);
        }

        next_frame().await
    }
}
