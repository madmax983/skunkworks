use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use crossbeam_channel::bounded;
use macroquad::prelude::*;
use myco_transit::World as MycoWorld;
use resonance_audio::audio::{AudioCommand, AudioModel};
use std::sync::{Arc, Mutex};
use std::thread;

const GRID_W: usize = 120;
const GRID_H: usize = 120;

fn window_conf() -> macroquad::window::Conf {
    macroquad::window::Conf {
        window_title: "Mycelial Acoustic Resonance".to_owned(),
        window_width: 800,
        window_height: 800,
        ..Default::default()
    }
}

fn main() {
    if std::env::args().any(|arg| arg == "--headless") {
        println!("Running in headless mode. Exiting immediately.");
        return;
    }
    macroquad::Window::from_config(window_conf(), async_main());
}

async fn async_main() {
    let (cmd_tx, cmd_rx) = bounded(1024);
    let (snap_tx, snap_rx) = bounded(2);

    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("No output device available");
    let config = device.default_output_config().unwrap();

    let mut audio_model = AudioModel::new(GRID_W, GRID_H, cmd_rx, snap_tx, None);

    let stream = match config.sample_format() {
        cpal::SampleFormat::F32 => device.build_output_stream(
            &config.into(),
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                audio_model.process(data);
            },
            |err| eprintln!("Audio stream error: {}", err),
            None,
        ),
        _ => panic!("Unsupported sample format"),
    }
    .unwrap();

    stream.play().unwrap();

    let (world, agents) = MycoWorld::with_cities_and_agents(GRID_W, GRID_H, 5);
    let myco_world = Arc::new(Mutex::new(world));
    let myco_agents = Arc::new(Mutex::new(agents));

    let world_clone = Arc::clone(&myco_world);
    let agents_clone = Arc::clone(&myco_agents);

    // Run Myco simulation in background thread
    thread::spawn(move || {
        loop {
            {
                let mut world = world_clone.lock().unwrap();
                let mut agents = agents_clone.lock().unwrap();
                world.diffuse_and_decay();
                world.update_agents_parallel(&mut agents);
            }
            thread::sleep(std::time::Duration::from_millis(16)); // ~60fps logic
        }
    });

    let mut image = Image::gen_image_color(GRID_W as u16, GRID_H as u16, BLANK);
    let texture = Texture2D::from_image(&image);
    texture.set_filter(FilterMode::Nearest);

    let mut frame_count = 0;

    loop {
        clear_background(Color::new(0.05, 0.05, 0.08, 1.0));

        let mut trail_data = vec![0.0f64; GRID_W * GRID_H];

        {
            let world = myco_world.lock().unwrap();

            for y in 0..GRID_H {
                for x in 0..GRID_W {
                    trail_data[y * GRID_W + x] = world.get_trail(x, y);
                }
            }

            // Every 10 frames, have high pheromone paths pluck the audio engine!
            if frame_count % 10 == 0 {
                for y in 0..GRID_H {
                    for x in 0..GRID_W {
                        let t = world.get_trail(x, y);
                        if t > 5.0 {
                            let _ = cmd_tx.send(AudioCommand::Pluck {
                                x,
                                y,
                                strength: (t as f32 / 10.0).clamp(0.0, 1.0),
                            });
                        }
                    }
                }
            }
        }

        if let Ok(snapshot) = snap_rx.try_recv() {
            let pixels = image.get_image_data_mut();
            for y in 0..GRID_H {
                for x in 0..GRID_W {
                    let p_idx = y * GRID_W + x;

                    let pressure = snapshot.pressure[p_idx];
                    let pheromone = trail_data[p_idx];

                    let pressure_visual = ((pressure + 1.0) * 0.5 * 255.0).clamp(0.0, 255.0) as u8;
                    let pheromone_visual = (pheromone * 10.0).clamp(0.0, 255.0) as u8;

                    // Blend
                    pixels[p_idx] = [
                        pheromone_visual.max(pressure_visual),
                        pressure_visual.saturating_sub(pheromone_visual),
                        255 - pheromone_visual,
                        255,
                    ];
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

        draw_text("Myco-Resonance", 10.0, 20.0, 30.0, WHITE);
        draw_text(
            "Pheromone paths disrupt acoustic waves",
            10.0,
            50.0,
            20.0,
            GRAY,
        );

        frame_count += 1;
        next_frame().await;
    }
}
