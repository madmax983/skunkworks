use crossbeam_channel::bounded;
use gray_scott::GrayScott;
use macroquad::prelude::*;
#[cfg(feature = "audio")]
use resonance_audio::audio::AudioSnapshot;
use resonance_audio::audio::{AudioCommand, AudioModel};

#[cfg(feature = "audio")]
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

const GRID_WIDTH: usize = 128;
const GRID_HEIGHT: usize = 128;
const SAMPLE_RATE: f32 = 44100.0;

#[macroquad::main("Acoustic Morphogenesis")]
async fn main() {
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

    #[cfg(not(feature = "audio"))]
    let mut model = AudioModel::new(GRID_WIDTH, GRID_HEIGHT, cmd_rx, snap_tx, None);

    let mut gs = GrayScott::new(GRID_WIDTH, GRID_HEIGHT);

    // Initial seed for Gray-Scott
    for y in (GRID_HEIGHT / 2 - 5)..(GRID_HEIGHT / 2 + 5) {
        for x in (GRID_WIDTH / 2 - 5)..(GRID_WIDTH / 2 + 5) {
            gs.add_chemical(x, y, 1.0);
        }
    }

    let mut image = Image::gen_image_color(GRID_WIDTH as u16, GRID_HEIGHT as u16, BLACK);
    let texture = Texture2D::from_image(&image);
    texture.set_filter(FilterMode::Nearest);

    let mut current_snapshot = vec![0.0; GRID_WIDTH * GRID_HEIGHT];
    let mut listener_pos = (GRID_WIDTH / 2, GRID_HEIGHT / 2);

    let mut frame_count = 0;

    loop {
        // Poll for snapshot
        while let Ok(snap) = snap_rx.try_recv() {
            current_snapshot = snap.pressure;
        }

        #[cfg(not(feature = "audio"))]
        {
            let dt = get_frame_time();
            let steps = (dt * SAMPLE_RATE) as usize;
            let steps = steps.min(2000);
            let mut dummy_buffer = vec![0.0; steps];
            model.process(&mut dummy_buffer);
        }

        // Apply acoustic pressure displacement to Gray-Scott chemicals
        // This is the "novel trait" - wave-driven Turing patterns.
        let mut u_advect = vec![0.0; GRID_WIDTH * GRID_HEIGHT];
        let mut v_advect = vec![0.0; GRID_WIDTH * GRID_HEIGHT];

        let u_orig = gs.u().to_vec();
        let v_orig = gs.v().to_vec();

        // Calculate gradient of pressure to drive flow
        for y in 1..(GRID_HEIGHT - 1) {
            for x in 1..(GRID_WIDTH - 1) {
                let idx = y * GRID_WIDTH + x;

                // Pressure gradient
                let p_left = current_snapshot[y * GRID_WIDTH + (x - 1)];
                let p_right = current_snapshot[y * GRID_WIDTH + (x + 1)];
                let p_up = current_snapshot[(y - 1) * GRID_WIDTH + x];
                let p_down = current_snapshot[(y + 1) * GRID_WIDTH + x];

                let grad_x = (p_right - p_left) * 0.5;
                let grad_y = (p_down - p_up) * 0.5;

                // Displacement vectors
                let flow_scale = 1.0;
                let src_x = (x as f32 - grad_x * flow_scale).clamp(0.0, (GRID_WIDTH - 1) as f32);
                let src_y = (y as f32 - grad_y * flow_scale).clamp(0.0, (GRID_HEIGHT - 1) as f32);

                // Bilinear interpolation for advection
                let x0 = src_x.floor() as usize;
                let x1 = (x0 + 1).min(GRID_WIDTH - 1);
                let y0 = src_y.floor() as usize;
                let y1 = (y0 + 1).min(GRID_HEIGHT - 1);

                let tx = src_x - x0 as f32;
                let ty = src_y - y0 as f32;

                let interpolate = |grid: &[f32]| -> f32 {
                    let v00 = grid[y0 * GRID_WIDTH + x0];
                    let v10 = grid[y0 * GRID_WIDTH + x1];
                    let v01 = grid[y1 * GRID_WIDTH + x0];
                    let v11 = grid[y1 * GRID_WIDTH + x1];

                    let top = v00 * (1.0 - tx) + v10 * tx;
                    let bottom = v01 * (1.0 - tx) + v11 * tx;

                    top * (1.0 - ty) + bottom * ty
                };

                u_advect[idx] = interpolate(&u_orig);
                v_advect[idx] = interpolate(&v_orig);
            }
        }

        // Apply advection back to gs by modifying the current state directly
        gs.u_mut().copy_from_slice(&u_advect);
        gs.v_mut().copy_from_slice(&v_advect);

        // Update Gray-Scott reaction-diffusion
        for _ in 0..10 {
            gs.update(0.055, 0.062, 1.0);
        }

        // Apply wave pressure to GS chemicals (Morphogenesis by pressure)
        // High pressure injects V catalyst
        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                let idx = gs.get_index(x, y);
                let pressure = current_snapshot[idx];

                if pressure > 0.05 {
                    gs.add_chemical(x, y, pressure * 0.01);
                }
            }
        }

        // Update texture
        let u_buffer = gs.u();
        let v_buffer = gs.v();

        for i in 0..(GRID_WIDTH * GRID_HEIGHT) {
            let x = (i % GRID_WIDTH) as u32;
            let y = (i / GRID_WIDTH) as u32;

            let val_u = u_buffer[i];
            let val_v = v_buffer[i];
            let pressure = current_snapshot[i];

            let r = val_u;
            let g = val_v;
            let b = (pressure * 5.0).abs().clamp(0.0, 1.0); // Visualize pressure as blue

            image.set_pixel(x, y, Color::new(r, g, b, 1.0));
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

        next_frame().await;
        frame_count += 1;

        // Auto pluck
        if frame_count % 120 == 0 {
            let _ = cmd_tx.send(AudioCommand::Pluck {
                x: GRID_WIDTH / 2,
                y: GRID_HEIGHT / 2,
                strength: 1.0,
            });
        }
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

    #[test]
    fn test_grid_size() {
        assert_eq!(GRID_WIDTH, 128);
        assert_eq!(GRID_HEIGHT, 128);
    }
}
