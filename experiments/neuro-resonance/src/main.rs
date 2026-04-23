use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use crossbeam_channel::bounded;
use macroquad::prelude::*;
use neuro_sim::Network;
use resonance_audio::audio::{AudioCommand, AudioModel};
use resonance_audio::physics::Material;

const GRID_W: usize = 100;
const GRID_H: usize = 100;
const NUM_NEURONS: usize = 50;

#[macroquad::main("Neuro Resonance")]
async fn main() {
    // Audio initialization
    let (cmd_tx, cmd_rx) = bounded(1024);
    let (snap_tx, snap_rx) = bounded(2);

    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("No output device available");
    let config = device.default_output_config().unwrap();

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

    // Neural Network initialization
    let mut net = Network::new();
    let mut neuron_positions = Vec::new();

    // Add neurons and distribute them randomly
    for _ in 0..NUM_NEURONS {
        net.add_neuron();
        neuron_positions.push((
            rand::gen_range(5, GRID_W - 5),
            rand::gen_range(5, GRID_H - 5),
        ));
    }

    // Fully connect the network with random weights (excitatory and inhibitory)
    for i in 0..NUM_NEURONS {
        for j in 0..NUM_NEURONS {
            if i != j && rand::gen_range(0.0, 1.0) < 0.2 {
                let weight = rand::gen_range(-15.0, 15.0); // Allow inhibitory as well
                net.add_synapse(i, j, weight);
            }
        }
    }

    // Set some initial acoustic walls
    for x in 40..60 {
        cmd_tx.send(AudioCommand::AddWall { x, y: 50 }).unwrap();
    }

    let mut image = Image::gen_image_color(GRID_W as u16, GRID_H as u16, BLANK);
    let texture = Texture2D::from_image(&image);
    texture.set_filter(FilterMode::Nearest);

    let mut frame_count = 0;

    loop {
        clear_background(BLACK);

        // Periodically inject random current into random neurons to sustain activity
        if frame_count % 10 == 0 {
            let n = rand::gen_range(0, NUM_NEURONS);
            net.neurons[n].inject(30.0);
        }

        // Advance neural network
        net.step(&[]);

        // Translate spikes to acoustic plucks
        for i in 0..NUM_NEURONS {
            if net.is_spiking(i) {
                let (nx, ny) = neuron_positions[i];
                cmd_tx
                    .send(AudioCommand::Pluck {
                        x: nx,
                        y: ny,
                        strength: 1.0,
                    })
                    .unwrap();
            }
        }

        // Receive audio grid snapshot and render
        if let Ok(snapshot) = snap_rx.try_recv() {
            let pixels = image.get_image_data_mut();
            for y in 0..GRID_H {
                for x in 0..GRID_W {
                    let idx = y * GRID_W + x;
                    let p = snapshot.pressure[idx];
                    let is_wall = snapshot.materials[idx] == Material::Wall;

                    let p_idx = y * GRID_W + x;
                    if is_wall {
                        pixels[p_idx] = [150, 150, 150, 255]; // Gray wall
                    } else {
                        // Map acoustic pressure [-1.0, 1.0] to a cyan color gradient
                        let c = ((p + 1.0) * 0.5 * 255.0).clamp(0.0, 255.0) as u8;
                        pixels[p_idx] = [0, c, c, 255];
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

        // Overlay neurons on top of the audio grid
        for (i, &(nx, ny)) in neuron_positions.iter().enumerate() {
            let screen_nx = x + (nx as f32 * scale);
            let screen_ny = y + (ny as f32 * scale);

            let color = if net.is_spiking(i) {
                RED
            } else {
                Color::new(1.0, 1.0, 1.0, 0.5)
            };

            let size = if net.is_spiking(i) {
                scale * 1.5
            } else {
                scale * 0.8
            };

            draw_circle(screen_nx, screen_ny, size, color);
        }

        frame_count += 1;
        next_frame().await;
    }
}
