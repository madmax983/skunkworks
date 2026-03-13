use crossbeam_channel::bounded;
use macroquad::prelude::*;

#[cfg(feature = "audio")]
use resonance_audio::audio::AudioSnapshot;
use resonance_audio::audio::{AudioCommand, AudioModel};

#[cfg(feature = "audio")]
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
#[cfg(feature = "audio")]
use crossbeam_channel::Sender;

mod graph;
use graph::Graph;

const GRID_WIDTH: usize = 128;
const GRID_HEIGHT: usize = 128;
const SAMPLE_RATE: f32 = 44100.0;
const WORLD_SIZE: f32 = 128.0;

#[macroquad::main("Mnemonic Tank")]
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

    let mut graph = Graph::new();
    println!("Scanning directory...");

    // Attempt to scan the current crate first
    graph.scan_directory("experiments/mnem-tank/src");

    // If that fails, try current dir
    if graph.nodes.is_empty() {
        println!("Scanning current directory...");
        graph.scan_directory(".");
    }

    let mut image = Image::gen_image_color(GRID_WIDTH as u16, GRID_HEIGHT as u16, BLACK);
    let texture = Texture2D::from_image(&image);
    texture.set_filter(FilterMode::Nearest);

    let mut current_snapshot: Vec<f32> = vec![0.0; GRID_WIDTH * GRID_HEIGHT];

    let mut entropy = 0.0;

    // Initial layout
    for node in &mut graph.nodes {
        node.pos = vec2(
            rand::gen_range(0.0, WORLD_SIZE),
            rand::gen_range(0.0, WORLD_SIZE),
        );
    }

    loop {
        #[cfg(not(feature = "audio"))]
        {
            // Simulate steps for this frame
            let dt = get_frame_time();
            let steps = (dt * SAMPLE_RATE) as usize;
            let steps = steps.min(2000);
            let mut dummy_buffer = vec![0.0; steps];
            model.process(&mut dummy_buffer);
        }

        // Try to get latest snapshot from audio thread
        while let Ok(snap) = snap_rx.try_recv() {
            current_snapshot = snap.pressure;
        }

        let dt = get_frame_time();
        entropy += 0.0001; // Global rot

        // Graph Physics
        let mut forces = vec![vec2(0.0, 0.0); graph.nodes.len()];

        // Repulsion
        for i in 0..graph.nodes.len() {
            for j in i + 1..graph.nodes.len() {
                let diff = graph.nodes[i].pos - graph.nodes[j].pos;
                let dist_sq = diff.length_squared().max(1.0);
                if dist_sq < 400.0 {
                    // Radius scaled for WORLD_SIZE (128x128)
                    let force = diff.normalize() * (50.0 / dist_sq);
                    forces[i] += force;
                    forces[j] -= force;
                }
            }
            // Center attraction (Gravity)
            let center = vec2(WORLD_SIZE / 2.0, WORLD_SIZE / 2.0);
            let to_center = (center - graph.nodes[i].pos) * 0.05;
            forces[i] += to_center;
        }

        // Spring forces
        for edge in &graph.edges {
            if edge.from < graph.nodes.len() && edge.to < graph.nodes.len() {
                let n1 = graph.nodes[edge.from].pos;
                let n2 = graph.nodes[edge.to].pos;
                let diff = n2 - n1;
                let dist = diff.length();
                let desired = 20.0;
                let force = diff.normalize() * (dist - desired) * 0.1 * edge.strength;
                forces[edge.from] += force;
                forces[edge.to] -= force;
            }
        }

        for (i, node) in graph.nodes.iter_mut().enumerate() {
            node.vel += forces[i] * dt;
            node.vel *= 0.90; // Damping
            node.pos += node.vel;

            // Keep within bounds
            node.pos.x = node.pos.x.clamp(0.0, WORLD_SIZE - 1.0);
            node.pos.y = node.pos.y.clamp(0.0, WORLD_SIZE - 1.0);

            // Decay
            node.health -= entropy * dt * 0.01;
            if node.health < 0.1 {
                node.health = 0.1;
            }

            // Acoustic pluck: the more rotted, the more chaotic the pluck
            // node.health ranges from 0.1 to 1.0. Lower health = higher probability of pluck
            let pluck_prob = (1.0 - node.health) * dt * 2.0;
            if rand::gen_range(0.0, 1.0) < pluck_prob {
                let _ = cmd_tx.try_send(AudioCommand::Pluck {
                    x: node.pos.x as usize,
                    y: node.pos.y as usize,
                    strength: (1.0 - node.health) * 0.5,
                });
            }

            // Interaction
            let mouse_pos = mouse_position();
            let world_mouse = vec2(
                mouse_pos.0 / screen_width() * WORLD_SIZE,
                mouse_pos.1 / screen_height() * WORLD_SIZE,
            );
            if node.pos.distance(world_mouse) < 5.0 && is_mouse_button_down(MouseButton::Right) {
                node.health += dt * 2.0; // Fast heal
                if node.health > 1.0 {
                    node.health = 1.0;
                }
            }
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            let mouse_pos = mouse_position();
            let world_mouse = vec2(
                mouse_pos.0 / screen_width() * WORLD_SIZE,
                mouse_pos.1 / screen_height() * WORLD_SIZE,
            );
            let x = world_mouse.x as usize;
            let y = world_mouse.y as usize;
            let _ = cmd_tx.try_send(AudioCommand::Pluck {
                x,
                y,
                strength: 2.0,
            });
        }

        // Render Wave Tank
        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                let h = current_snapshot[y * GRID_WIDTH + x];
                let color_val = ((h + 1.0) * 0.5 * 255.0).clamp(0.0, 255.0) as u8;
                image.set_pixel(
                    x as u32,
                    y as u32,
                    Color::from_rgba(color_val, color_val, 255, 255),
                );
            }
        }
        texture.update(&image);

        clear_background(BLACK);

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

        // Draw Edges
        for edge in &graph.edges {
            if edge.from < graph.nodes.len() && edge.to < graph.nodes.len() {
                let n1 = graph.nodes[edge.from].pos;
                let n2 = graph.nodes[edge.to].pos;

                let sx1 = n1.x / WORLD_SIZE * screen_width();
                let sy1 = n1.y / WORLD_SIZE * screen_height();
                let sx2 = n2.x / WORLD_SIZE * screen_width();
                let sy2 = n2.y / WORLD_SIZE * screen_height();

                let avg_health =
                    (graph.nodes[edge.from].health + graph.nodes[edge.to].health) / 2.0;

                draw_line(
                    sx1,
                    sy1,
                    sx2,
                    sy2,
                    1.0,
                    Color::new(0.0, 1.0, 0.0, avg_health * 0.5),
                );
            }
        }

        // Render Graph Nodes
        for node in &graph.nodes {
            let screen_x = node.pos.x / WORLD_SIZE * screen_width();
            let screen_y = node.pos.y / WORLD_SIZE * screen_height();

            let color = if node.health > 0.8 {
                GREEN
            } else if node.health > 0.4 {
                YELLOW
            } else {
                RED
            };

            draw_circle(screen_x, screen_y, 4.0, color);
            if node.health > 0.5 {
                draw_text(&node.name, screen_x + 5.0, screen_y, 14.0, WHITE);
            }
        }

        draw_text("Mnemonic Tank", 10.0, 20.0, 30.0, WHITE);
        draw_text(&format!("Entropy: {:.4}", entropy), 10.0, 50.0, 20.0, RED);
        draw_text(
            "L-Click: Pluck | R-Click: Heal Node",
            10.0,
            80.0,
            20.0,
            WHITE,
        );

        next_frame().await;
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
        .ok_or_else(|| anyhow::anyhow!("No output device found"))?;

    let config = device.default_output_config()?;
    let sample_rate = config.sample_rate().0 as f32;
    let mut model = AudioModel::new(GRID_WIDTH, GRID_HEIGHT, cmd_rx, snap_tx, Some(sample_rate));

    let stream = device.build_output_stream(
        &config.into(),
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            model.process(data);
        },
        move |err| {
            eprintln!("Audio stream error: {}", err);
        },
        None,
    )?;

    stream.play()?;
    Ok(stream)
}
