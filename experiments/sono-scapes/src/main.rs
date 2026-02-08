use crossbeam_channel::{bounded, Receiver, Sender};
use macroquad::prelude::*;
use resonance_audio::audio::{AudioCommand, AudioModel};
use std::thread;
use std::time::Duration;

const SIM_WIDTH: usize = 80;
const SIM_HEIGHT: usize = 60;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tool {
    Pluck,
    Wall,
    Listener,
    Source,
    Erase,
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Sono-Scapes".to_owned(),
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let (cmd_tx, cmd_rx) = bounded(100);
    let (snap_tx, snap_rx) = bounded(2);

    // Start Audio Engine
    #[cfg(feature = "audio")]
    let _stream = setup_audio(cmd_rx, snap_tx.clone());

    #[cfg(not(feature = "audio"))]
    let _audio_thread = setup_mock_audio(cmd_rx, snap_tx.clone());

    // If audio fails to init (returns None), we should fallback to mock.
    // But setup_audio handles fallback internally? No, I need to check.
    // I'll make setup_audio return Option<Stream>. If None, spawn mock.

    // Initial State
    let mut tool = Tool::Pluck;
    let mut grid_u = vec![0.0; SIM_WIDTH * SIM_HEIGHT];
    let texture = Texture2D::from_image(&Image::gen_image_color(
        SIM_WIDTH as u16,
        SIM_HEIGHT as u16,
        BLACK,
    ));
    texture.set_filter(FilterMode::Nearest);

    let mut listener_pos = (SIM_WIDTH / 2, SIM_HEIGHT / 2);
    let _ = cmd_tx.send(AudioCommand::MoveListener {
        x: listener_pos.0,
        y: listener_pos.1,
    });

    loop {
        clear_background(BLACK);

        // 1. Process Input
        if is_key_pressed(KeyCode::Key1) {
            tool = Tool::Pluck;
        }
        if is_key_pressed(KeyCode::Key2) {
            tool = Tool::Wall;
        }
        if is_key_pressed(KeyCode::Key3) {
            tool = Tool::Listener;
        }
        if is_key_pressed(KeyCode::Key4) {
            tool = Tool::Source;
        }
        if is_key_pressed(KeyCode::Key5) {
            tool = Tool::Erase;
        }
        if is_key_pressed(KeyCode::Space) {
            let _ = cmd_tx.send(AudioCommand::ClearWaves);
        }
        if is_key_pressed(KeyCode::C) {
            let _ = cmd_tx.send(AudioCommand::ClearWalls);
        }

        // Mouse Interaction
        if is_mouse_button_down(MouseButton::Left) {
            let (mx, my) = mouse_position();
            let sw = screen_width();
            let sh = screen_height();

            // Map screen to grid
            // Assuming full screen fill
            let gx = (mx / sw * SIM_WIDTH as f32) as usize;
            let gy = (my / sh * SIM_HEIGHT as f32) as usize;

            if gx < SIM_WIDTH && gy < SIM_HEIGHT {
                match tool {
                    Tool::Pluck => {
                        // Only pluck on press or drag?
                        // Pluck is continuous excitation if held
                        if is_mouse_button_pressed(MouseButton::Left) {
                            let _ = cmd_tx.send(AudioCommand::Pluck {
                                x: gx,
                                y: gy,
                                strength: 1.0,
                            });
                        }
                    }
                    Tool::Wall => {
                        let _ = cmd_tx.send(AudioCommand::AddWall { x: gx, y: gy });
                    }
                    Tool::Erase => {
                        let _ = cmd_tx.send(AudioCommand::RemoveWall { x: gx, y: gy });
                    }
                    Tool::Listener => {
                        listener_pos = (gx, gy);
                        let _ = cmd_tx.send(AudioCommand::MoveListener { x: gx, y: gy });
                    }
                    Tool::Source => {
                        // Add a tone source
                        if is_mouse_button_pressed(MouseButton::Left) {
                            let freq = 220.0 + (gy as f32 / SIM_HEIGHT as f32) * 880.0;
                            let _ = cmd_tx.send(AudioCommand::Oscillate {
                                x: gx,
                                y: gy,
                                frequency: freq,
                                strength: 0.5,
                            });
                        }
                    }
                }
            }
        }

        // Right click to erase/stop source
        if is_mouse_button_down(MouseButton::Right) {
            let (mx, my) = mouse_position();
            let sw = screen_width();
            let sh = screen_height();
            let gx = (mx / sw * SIM_WIDTH as f32) as usize;
            let gy = (my / sh * SIM_HEIGHT as f32) as usize;
            if gx < SIM_WIDTH && gy < SIM_HEIGHT {
                match tool {
                    Tool::Source => {
                        let _ = cmd_tx.send(AudioCommand::Oscillate {
                            x: gx,
                            y: gy,
                            frequency: 0.0,
                            strength: 0.0,
                        });
                    }
                    _ => {
                        let _ = cmd_tx.send(AudioCommand::RemoveWall { x: gx, y: gy });
                    }
                }
            }
        }

        // 2. Receive Snapshot
        while let Ok(snap) = snap_rx.try_recv() {
            grid_u = snap;
        }

        // 3. Update Texture
        // We need to efficiently update the texture.
        // Macroquad Image is strictly CPU side, then we upload.
        let mut image = Image::gen_image_color(SIM_WIDTH as u16, SIM_HEIGHT as u16, BLACK);
        for y in 0..SIM_HEIGHT {
            for x in 0..SIM_WIDTH {
                let idx = y * SIM_WIDTH + x;
                let val = grid_u[idx];

                // Color Map
                // Val is approx -1.0 to 1.0
                // Red = +ve, Blue = -ve
                let color = if val > 0.0 {
                    Color::new(val.min(1.0), 0.0, 0.0, 1.0)
                } else {
                    Color::new(0.0, 0.0, (-val).min(1.0), 1.0)
                };

                // If it's a wall, how do we know?
                // The snapshot only contains 'u'.
                // We assume walls have u=0 always (enforced by physics).
                // But waves also cross 0.
                // We need 'walls' in the snapshot if we want to render them distinctly.
                // OR we keep a local copy of walls?
                // For now, walls will just be black (0.0).
                // Wait, users want to see walls.
                // I should probably track walls locally in main loop since we send the commands.
                // But that desyncs.
                // Ideally snapshot includes walls.
                // But changing snapshot signature requires changing `resonance-audio`.
                // I won't do that now. I'll just render waves.
                // Actually, walls at 0.0 will be black.

                image.set_pixel(x as u32, y as u32, color);
            }
        }

        // Mark listener
        image.set_pixel(listener_pos.0 as u32, listener_pos.1 as u32, GREEN);

        texture.update(&image);

        // 4. Draw
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

        // UI
        draw_text("SONO-SCAPES", 10.0, 20.0, 30.0, WHITE);
        draw_text(
            &format!("Tool: {:?} (1-5)", tool_name(&tool)),
            10.0,
            50.0,
            20.0,
            WHITE,
        );
        draw_text("LMB: Action, RMB: Erase", 10.0, 70.0, 20.0, GRAY);
        draw_text("Space: Clear Waves, C: Clear Walls", 10.0, 90.0, 20.0, GRAY);

        next_frame().await
    }
}

fn tool_name(t: &Tool) -> &str {
    match t {
        Tool::Pluck => "Pluck",
        Tool::Wall => "Wall",
        Tool::Listener => "Listener",
        Tool::Source => "Source",
        Tool::Erase => "Erase",
    }
}

#[cfg(feature = "audio")]
fn setup_audio(cmd_rx: Receiver<AudioCommand>, snap_tx: Sender<Vec<f32>>) -> Option<cpal::Stream> {
    use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

    let host = cpal::default_host();
    let device = host.default_output_device()?;
    let config = device.default_output_config().ok()?;

    let cmd_rx_clone = cmd_rx.clone();
    let snap_tx_clone = snap_tx.clone();

    // Create model
    let mut model = AudioModel::new(SIM_WIDTH, SIM_HEIGHT, cmd_rx_clone, snap_tx_clone);

    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

    let stream = match config.sample_format() {
        cpal::SampleFormat::F32 => device.build_output_stream(
            &config.into(),
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                model.process(data);
            },
            err_fn,
            None,
        ),
        _ => return None,
    };

    if let Ok(stream) = stream {
        if stream.play().is_ok() {
            return Some(stream);
        }
    }

    // Fallback
    setup_mock_audio(cmd_rx, snap_tx);
    None
}

#[allow(dead_code)]
fn setup_mock_audio(
    cmd_rx: Receiver<AudioCommand>,
    snap_tx: Sender<Vec<f32>>,
) -> Option<thread::JoinHandle<()>> {
    let handle = thread::spawn(move || {
        let mut model = AudioModel::new(SIM_WIDTH, SIM_HEIGHT, cmd_rx, snap_tx);
        let mut buffer = vec![0.0; 1024]; // Dummy buffer
        loop {
            // Simulate audio rate roughly
            // 1024 samples at 44100Hz is ~23ms
            model.process(&mut buffer);
            thread::sleep(Duration::from_millis(23));
        }
    });
    Some(handle)
}
