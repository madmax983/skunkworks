use crossbeam_channel::bounded;
use macroquad::prelude::*;
use resonance_audio::audio::{AudioCommand, AudioModel, AudioSnapshot};

#[cfg(feature = "audio")]
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
#[cfg(feature = "audio")]
use crossbeam_channel::Sender;

const GRID_WIDTH: usize = 128;
const GRID_HEIGHT: usize = 128;
const SAMPLE_RATE: f32 = 44100.0;

#[macroquad::main("Ripple Tank")]
async fn main() {
    // 1. Audio / Simulation Setup
    let (cmd_tx, cmd_rx) = bounded(1024);
    let (snap_tx, snap_rx) = bounded(2);

    #[cfg(feature = "audio")]
    let _stream = match init_audio(cmd_rx, snap_tx.clone()) {
        Ok(s) => Some(s),
        Err(e) => {
            eprintln!("Audio init failed: {}. Running in silent mode.", e);
            None
        }
    };

    // If audio is not active, we need to drive the simulation manually
    #[cfg(not(feature = "audio"))]
    let mut model = AudioModel::new(GRID_WIDTH, GRID_HEIGHT, cmd_rx, snap_tx, None);

    // 2. Visual Setup
    let mut image = Image::gen_image_color(GRID_WIDTH as u16, GRID_HEIGHT as u16, BLACK);
    let texture = Texture2D::from_image(&image);
    texture.set_filter(FilterMode::Nearest);

    let mut current_snapshot: Vec<f32> = vec![0.0; GRID_WIDTH * GRID_HEIGHT];
    let mut listener_pos = (GRID_WIDTH / 2, GRID_HEIGHT / 2);

    loop {
        // Poll for snapshot
        while let Ok(snap) = snap_rx.try_recv() {
            current_snapshot = snap.pressure;
        }

        #[cfg(not(feature = "audio"))]
        {
            // Simulate steps for this frame
            let dt = get_frame_time();
            let steps = (dt * SAMPLE_RATE) as usize;
            // Limit steps to avoid spiral of death
            let steps = steps.min(2000);

            let mut dummy_buffer = vec![0.0; steps];
            model.process(&mut dummy_buffer);
        }

        // Update texture
        for (i, &val) in current_snapshot.iter().enumerate() {
            let x = (i % GRID_WIDTH) as u32;
            let y = (i / GRID_WIDTH) as u32;

            let color = if val > 0.0 {
                Color::new(val.min(1.0), 0.0, 0.0, 1.0)
            } else {
                Color::new(0.0, 0.0, (-val).min(1.0), 1.0)
            };
            image.set_pixel(x, y, color);
        }
        texture.update(&image);

        // Draw
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

        // Draw Listener
        let lx = offset_x + listener_pos.0 as f32 * scale;
        let ly = offset_y + listener_pos.1 as f32 * scale;
        draw_circle(lx + scale / 2.0, ly + scale / 2.0, scale / 2.0, GREEN);

        // Input
        let mouse_pos = mouse_position();
        if mouse_pos.0 >= offset_x
            && mouse_pos.0 < offset_x + draw_width
            && mouse_pos.1 >= offset_y
            && mouse_pos.1 < offset_y + draw_height
        {
            let grid_x = ((mouse_pos.0 - offset_x) / scale) as usize;
            let grid_y = ((mouse_pos.1 - offset_y) / scale) as usize;

            if is_mouse_button_down(MouseButton::Left) {
                let _ = cmd_tx.send(AudioCommand::Pluck {
                    x: grid_x,
                    y: grid_y,
                    strength: 0.5,
                });
            }
            if is_mouse_button_down(MouseButton::Right) {
                let _ = cmd_tx.send(AudioCommand::AddWall {
                    x: grid_x,
                    y: grid_y,
                });
            }

            if is_mouse_button_down(MouseButton::Middle) {
                listener_pos = (grid_x, grid_y);
                let _ = cmd_tx.send(AudioCommand::MoveListener {
                    x: grid_x,
                    y: grid_y,
                });
            }
        }

        if is_key_pressed(KeyCode::Space) {
            let _ = cmd_tx.send(AudioCommand::ClearWaves);
        }
        if is_key_pressed(KeyCode::C) {
            let _ = cmd_tx.send(AudioCommand::ClearWalls);
        }

        draw_text(
            "L-Click: Pluck | R-Click: Wall | M-Click: Ear | Space: Clear Waves | C: Clear Walls",
            10.0,
            20.0,
            20.0,
            WHITE,
        );

        next_frame().await
    }
}

#[cfg(feature = "audio")]
fn init_audio(
    cmd_rx: crossbeam_channel::Receiver<AudioCommand>,
    snap_tx: crossbeam_channel::Sender<AudioSnapshot>,
) -> anyhow::Result<cpal::Stream> {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .ok_or_else(|| anyhow::anyhow!("No output device available"))?;

    let config = device.default_output_config()?;

    let mut model = AudioModel::new(GRID_WIDTH, GRID_HEIGHT, cmd_rx, snap_tx, None);

    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

    let stream = match config.sample_format() {
        cpal::SampleFormat::F32 => device.build_output_stream(
            &config.into(),
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                model.process(data);
            },
            err_fn,
            None,
        )?,
        _ => return Err(anyhow::anyhow!("Only F32 sample format supported")),
    };

    stream.play()?;
    Ok(stream)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossbeam_channel::bounded;

    #[test]
    fn test_simulation_loop() {
        // Setup channels
        let (cmd_tx, cmd_rx) = bounded(1024);
        let (snap_tx, snap_rx) = bounded(2);

        // Create model
        let mut model = AudioModel::new(GRID_WIDTH, GRID_HEIGHT, cmd_rx, snap_tx, None);

        // Send a pluck
        cmd_tx
            .send(AudioCommand::Pluck {
                x: 50,
                y: 50,
                strength: 1.0,
            })
            .unwrap();

        // Step simulation
        let mut buffer = vec![0.0; 1000];
        model.process(&mut buffer);

        // Check buffer has non-zero values (sound generated)
        assert!(buffer.iter().any(|&x| x.abs() > 0.0001));

        // Check snapshot received
        assert!(snap_rx.try_recv().is_ok());
    }
}
