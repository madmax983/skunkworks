use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use crossbeam_channel::bounded;
use macroquad::prelude::*;
use quipu::{Cord, Knot, Quipu};
use resonance_audio::audio::{AudioCommand, AudioModel};
use resonance_audio::physics::Material;

const GRID_W: usize = 100;
const GRID_H: usize = 100;

#[macroquad::main("Quipu Resonance")]
async fn main() {
    let (cmd_tx, cmd_rx) = bounded(1024);
    let (snap_tx, snap_rx) = bounded(2);

    // Initialize audio
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("No output device available");
    let config = device.default_output_config().unwrap();
    let _sample_rate = config.sample_rate().0;

    // We send a None for UI updates from the audio thread since we are in Macroquad
    let mut model = AudioModel::new(GRID_W, GRID_H, cmd_rx, snap_tx, None);

    let stream = match config.sample_format() {
        cpal::SampleFormat::F32 => device.build_output_stream(
            &config.into(),
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                model.process(data);
            },
            |err| eprintln!("Audio stream error: {}", err),
            None,
        ),
        _ => panic!("Unsupported sample format"),
    }
    .unwrap();

    stream.play().unwrap();

    let mut q = Quipu::new();
    // Add some interesting data
    q.add_cord(Cord::from(42));
    q.add_cord(Cord::from(105));
    q.add_cord(Cord::from(9));
    q.add_cord(Cord::from(300));

    // Setup walls in resonance-audio based on knots
    let mut x_offset = 20;
    for cord in &q.cords {
        let mut y_offset = 20;
        for cluster in &cord.clusters {
            for knot in cluster {
                let size = match knot {
                    Knot::Simple => 2,
                    Knot::Long(v) => *v as usize + 1,
                    Knot::FigureEight => 3,
                };

                // Add wall
                for i in 0..size {
                    for j in 0..size {
                        let wx = (x_offset + i).clamp(0, GRID_W - 1);
                        let wy = (y_offset + j).clamp(0, GRID_H - 1);
                        cmd_tx.send(AudioCommand::AddWall { x: wx, y: wy }).unwrap();
                    }
                }
                y_offset += size + 2;
            }
            y_offset += 5; // space between clusters
        }
        x_offset += 15; // space between cords
    }

    let mut image = Image::gen_image_color(GRID_W as u16, GRID_H as u16, BLANK);
    let texture = Texture2D::from_image(&image);
    texture.set_filter(FilterMode::Nearest);

    let mut frame_count = 0;

    loop {
        clear_background(BLACK);

        // Periodically pluck the quipu cords to make them "ring"
        if frame_count % 60 == 0 {
            cmd_tx
                .send(AudioCommand::Pluck {
                    x: rand::gen_range(20, 80),
                    y: rand::gen_range(20, 80),
                    strength: 1.0,
                })
                .unwrap();
        }

        // Receive snapshot
        if let Ok(snapshot) = snap_rx.try_recv() {
            let pixels = image.get_image_data_mut();
            for y in 0..GRID_H {
                for x in 0..GRID_W {
                    let idx = y * GRID_W + x;
                    let p = snapshot.pressure[idx];
                    let is_wall = snapshot.materials[idx] == Material::Wall;

                    let p_idx = y * GRID_W + x;
                    if is_wall {
                        pixels[p_idx] = [200, 150, 50, 255];
                    } else {
                        // Map pressure [-1.0, 1.0] to color
                        let c = ((p + 1.0) * 0.5 * 255.0).clamp(0.0, 255.0) as u8;
                        pixels[p_idx] = [c, c, 255, 255];
                    }
                }
            }
            texture.update(&image);
        }

        let scale = (screen_height() / GRID_H as f32).min(screen_width() / GRID_W as f32);
        let w = GRID_W as f32 * scale;
        let h = GRID_H as f32 * scale;
        let x = (screen_width() - w) / 2.0;
        let y = (screen_height() - h) / 2.0;

        draw_texture_ex(
            &texture,
            x,
            y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(w, h)),
                ..Default::default()
            },
        );

        frame_count += 1;
        next_frame().await;
    }
}
