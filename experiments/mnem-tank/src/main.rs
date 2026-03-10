use macroquad::prelude::*;
#[allow(unused_imports)]
use resonance_audio::audio::{AudioModel, AudioCommand, AudioSnapshot};
use crossbeam_channel::bounded;

#[cfg(feature = "audio")]
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

mod graph;
use graph::Graph;
mod glitch;
use glitch::TextGlitcher;

const GRID_SIZE: usize = 200;

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

    let mut model = AudioModel::new(GRID_SIZE, GRID_SIZE, cmd_rx, snap_tx, None);

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

#[macroquad::main("Acoustic Memory Decay")]
async fn main() {
    let mut graph = Graph::new();
    println!("Scanning directory...");
    graph.scan_directory("experiments/mnem-rot/src");
    if graph.nodes.is_empty() {
        graph.scan_directory(".");
    }

    let mut entropy = 0.0;
    let mut hovered_node: Option<usize>;

    let mut offset = vec2(0.0, 0.0);
    let mut zoom = 1.0;
    let mut dragging = false;
    let mut last_mouse = vec2(0.0, 0.0);

    let (cmd_tx, cmd_rx) = bounded(1024);
    let (snap_tx, snap_rx) = bounded(2);

    #[cfg(feature = "audio")]
    let _stream = match init_audio(cmd_rx.clone(), snap_tx.clone()) {
        Ok(s) => Some(s),
        Err(e) => {
            eprintln!("Audio init failed: {}. Running in silent mode.", e);
            None
        }
    };

    #[cfg(not(feature = "audio"))]
    let mut model = AudioModel::new(GRID_SIZE, GRID_SIZE, cmd_rx.clone(), snap_tx.clone(), None);

    let _grid_to_screen = |x: usize, y: usize| -> Vec2 {
        vec2(
            x as f32 / GRID_SIZE as f32 * screen_width(),
            y as f32 / GRID_SIZE as f32 * screen_height(),
        )
    };

    let screen_to_grid = |pos: Vec2| -> (usize, usize) {
        let x = (pos.x / screen_width() * GRID_SIZE as f32) as usize;
        let y = (pos.y / screen_height() * GRID_SIZE as f32) as usize;
        (x.clamp(0, GRID_SIZE - 1), y.clamp(0, GRID_SIZE - 1))
    };

    let mut visual_pressure: Vec<f32> = vec![0.0; GRID_SIZE * GRID_SIZE];

    loop {
        // Fetch snapshot
        while let Ok(snap) = snap_rx.try_recv() {
            visual_pressure = snap.pressure;
        }

        let mouse_pos = mouse_position();
        let mouse_vec = vec2(mouse_pos.0, mouse_pos.1);

        if is_mouse_button_pressed(MouseButton::Right) {
            dragging = true;
            last_mouse = mouse_vec;
        }
        if is_mouse_button_released(MouseButton::Right) {
            dragging = false;
        }
        if dragging {
            let delta = mouse_vec - last_mouse;
            offset += delta;
            last_mouse = mouse_vec;
        }

        let wheel = mouse_wheel();
        if wheel.1 != 0.0 {
            zoom *= if wheel.1 > 0.0 { 1.1 } else { 0.9 };
        }

        entropy += 0.0001;

        if is_mouse_button_pressed(MouseButton::Left) {
            let (gx, gy) = screen_to_grid(mouse_vec);
            let _ = cmd_tx.send(AudioCommand::Pluck { x: gx, y: gy, strength: 5.0 });
        }

        let mut forces = vec![vec2(0.0, 0.0); graph.nodes.len()];

        for i in 0..graph.nodes.len() {
            for j in i + 1..graph.nodes.len() {
                let diff = graph.nodes[i].pos - graph.nodes[j].pos;
                let dist_sq = diff.length_squared().max(1.0);
                if dist_sq < 250000.0 {
                    let force = diff.normalize() * (5000.0 / dist_sq);
                    forces[i] += force;
                    forces[j] -= force;
                }
            }
            let center = vec2(screen_width() / 2.0, screen_height() / 2.0) - offset;
            let to_center = (center - graph.nodes[i].pos) * 0.01;
            forces[i] += to_center;
        }

        for edge in &graph.edges {
            if edge.from < graph.nodes.len() && edge.to < graph.nodes.len() {
                let n1 = graph.nodes[edge.from].pos;
                let n2 = graph.nodes[edge.to].pos;
                let diff = n2 - n1;
                let dist = diff.length();
                let desired = 100.0;
                let force = diff.normalize() * (dist - desired) * 0.05 * edge.strength;
                forces[edge.from] += force;
                forces[edge.to] -= force;
            }
        }

        let dt = get_frame_time();
        hovered_node = None;
        let world_mouse = (mouse_vec - offset) / zoom;

        for (i, node) in graph.nodes.iter_mut().enumerate() {
            node.vel += forces[i] * dt;
            node.vel *= 0.90;
            node.pos += node.vel;

            let screen_node_pos = node.pos * zoom + offset;
            let (gx, gy) = screen_to_grid(screen_node_pos);

            // Wall placement (node acts as a wall if healthy)
            if node.health > 0.5 {
                let _ = cmd_tx.send(AudioCommand::AddWall { x: gx, y: gy });
            } else {
                let _ = cmd_tx.send(AudioCommand::RemoveWall { x: gx, y: gy });
            }

            let idx = gy * GRID_SIZE + gx;
            if idx < visual_pressure.len() {
                let pressure = visual_pressure[idx].abs();

                if pressure > 0.05 {
                    node.health -= pressure * dt * 0.5;
                }
            }

            node.health -= entropy * dt * 0.01;
            if node.health < 0.1 {
                node.health = 0.1;
                if rand::gen_range(0.0, 1.0) < 0.05 { // Reduce pluck frequency to avoid overflowing channel
                    let _ = cmd_tx.send(AudioCommand::Pluck { x: gx, y: gy, strength: 0.5 });
                }
            }

            if node.pos.distance(world_mouse) < 20.0 {
                hovered_node = Some(i);
                node.health += dt * 5.0;
                if node.health > 1.0 {
                    node.health = 1.0;
                }
            }
        }

        clear_background(BLACK);

        let cell_w = screen_width() / GRID_SIZE as f32;
        let cell_h = screen_height() / GRID_SIZE as f32;

        for y in 0..GRID_SIZE {
            for x in 0..GRID_SIZE {
                let p: f32 = visual_pressure[y * GRID_SIZE + x];
                if p.abs() > 0.01 {
                    let c = if p > 0.0 {
                        Color::new(0.0, 0.5, 1.0, (p * 5.0).clamp(0.0, 1.0))
                    } else {
                        Color::new(1.0, 0.0, 0.5, (-p * 5.0).clamp(0.0, 1.0))
                    };
                    draw_rectangle(x as f32 * cell_w, y as f32 * cell_h, cell_w, cell_h, c);
                }
            }
        }

        for edge in &graph.edges {
            if edge.from < graph.nodes.len() && edge.to < graph.nodes.len() {
                let n1 = graph.nodes[edge.from].pos * zoom + offset;
                let n2 = graph.nodes[edge.to].pos * zoom + offset;
                let avg_health =
                    (graph.nodes[edge.from].health + graph.nodes[edge.to].health) / 2.0;

                let jitter = if avg_health < 0.5 {
                    vec2(rand::gen_range(-2.0, 2.0), rand::gen_range(-2.0, 2.0))
                } else {
                    vec2(0.0, 0.0)
                };

                draw_line(
                    n1.x + jitter.x,
                    n1.y + jitter.y,
                    n2.x + jitter.x,
                    n2.y + jitter.y,
                    2.0 * zoom,
                    Color::new(0.5, 0.5, 0.5, avg_health),
                );
            }
        }

        for (i, node) in graph.nodes.iter().enumerate() {
            let pos = node.pos * zoom + offset;
            let color = if node.health > 0.8 {
                GREEN
            } else if node.health > 0.4 {
                YELLOW
            } else {
                RED
            };

            let radius = 5.0 * zoom * (0.5 + node.health);
            draw_circle(pos.x, pos.y, radius, color);

            if node.health > 0.6 || hovered_node == Some(i) {
                draw_text(&node.name, pos.x + 10.0, pos.y, 14.0 * zoom, WHITE);
            }
        }

        if let Some(idx) = hovered_node {
            let node = &graph.nodes[idx];
            draw_rectangle(
                10.0,
                10.0,
                400.0,
                screen_height() - 20.0,
                Color::new(0.05, 0.05, 0.05, 0.95),
            );
            draw_rectangle_lines(10.0, 10.0, 400.0, screen_height() - 20.0, 2.0, WHITE);

            draw_text(&node.name, 20.0, 40.0, 30.0, GREEN);
            draw_text(
                &format!("Health: {:.0}%", node.health * 100.0),
                20.0,
                70.0,
                20.0,
                WHITE,
            );

            let intensity = 1.0 - node.health;
            let corrupted = TextGlitcher::corrupt(&node.content, intensity);

            let lines: Vec<&str> = corrupted.lines().take(35).collect();
            for (j, line) in lines.iter().enumerate() {
                let display_line = if line.len() > 50 { &line[0..50] } else { line };
                draw_text(display_line, 20.0, 100.0 + j as f32 * 15.0, 14.0, LIGHTGRAY);
            }
        }

        draw_text(
            &format!("Entropy: {:.4}", entropy),
            screen_width() - 200.0,
            30.0,
            20.0,
            RED,
        );
        draw_text(
            "Left Click: Pluck | Right Click: Pan | Scroll: Zoom | Hover: Heal",
            10.0,
            screen_height() - 10.0,
            16.0,
            GRAY,
        );

        #[cfg(not(feature = "audio"))]
        {
            let mut dummy = [0.0; 256];
            model.process(&mut dummy);
        }

        next_frame().await
    }
}
