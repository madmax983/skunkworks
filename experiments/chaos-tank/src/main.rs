use crossbeam_channel::bounded;
use macroquad::prelude::*;
use resonance_audio::audio::{AudioCommand, AudioModel};

mod physics;
use physics::PendulumSystem;

#[cfg(feature = "audio")]
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

const GRID_WIDTH: usize = 128;
const GRID_HEIGHT: usize = 128;
const SAMPLE_RATE: f32 = 44100.0;

// Size of the tank visually
const TANK_DRAW_SIZE: f32 = 600.0;

#[macroquad::main("Chaos Tank")]
async fn main() {
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
    let _stream: Option<()> = None;

    let mut model = if cfg!(not(feature = "audio")) || _stream.is_none() {
        Some(AudioModel::new(GRID_WIDTH, GRID_HEIGHT, cmd_rx, snap_tx, None))
    } else {
        None
    };

    let mut image = Image::gen_image_color(GRID_WIDTH as u16, GRID_HEIGHT as u16, BLACK);
    let texture = Texture2D::from_image(&image);
    texture.set_filter(FilterMode::Nearest);

    let mut current_snapshot: Vec<f32> = vec![0.0; GRID_WIDTH * GRID_HEIGHT];

    // Initialize pendulum system
    let mut sys = PendulumSystem::new();

    // We want the pendulum to swing over the wave tank area.
    let center = Vec2::new(screen_width() / 2.0, screen_height() / 2.0);

    // Center top anchor
    let root = sys.add_node(Vec2::new(center.x, center.y - 200.0), 1.0, true, "Root".to_string());

    // Double pendulum nodes
    let node1 = sys.add_node(Vec2::new(center.x + 100.0, center.y - 100.0), 10.0, false, "N1".to_string());
    let node2 = sys.add_node(Vec2::new(center.x + 150.0, center.y), 5.0, false, "N2".to_string());

    sys.add_link(root, node1, 150.0);
    sys.add_link(node1, node2, 120.0);

    // Customize physics for dramatic swings
    sys.gravity = Vec2::new(0.0, 981.0); // stronger gravity for visual scale
    sys.friction = 0.999; // almost frictionless

    loop {
        let dt = get_frame_time();

        // Interaction
        if is_mouse_button_down(MouseButton::Left) {
            let mouse_pos = mouse_position();
            let mouse_vec = Vec2::new(mouse_pos.0, mouse_pos.1);
            let prev = sys.nodes[node2].pos;
            sys.nodes[node2].pos = mouse_vec;
            // set prev_pos slightly behind current position so it gets velocity
            sys.nodes[node2].prev_pos = prev;
        }

        let tip_pos_before = sys.nodes[node2].pos;

        // Step physics
        sys.step(dt);

        let screen_center = Vec2::new(screen_width() / 2.0, screen_height() / 2.0);
        let offset_x = screen_center.x - TANK_DRAW_SIZE / 2.0;
        let offset_y = screen_center.y - TANK_DRAW_SIZE / 2.0;

        // Map pendulum node2 (the tip) to grid
        let tip_pos_after = sys.nodes[node2].pos;
        let frame_distance = (tip_pos_after - tip_pos_before).length();
        let tip_speed = frame_distance / dt;

        // If moving fast, it plunges and plucks
        if tip_speed > 100.0 {
            // Map to grid coordinates
            let grid_x = ((tip_pos_after.x - offset_x) / TANK_DRAW_SIZE * GRID_WIDTH as f32) as isize;
            let grid_y = ((tip_pos_after.y - offset_y) / TANK_DRAW_SIZE * GRID_HEIGHT as f32) as isize;

            if grid_x >= 0 && grid_x < GRID_WIDTH as isize && grid_y >= 0 && grid_y < GRID_HEIGHT as isize {
                // Determine pluck strength based on speed, maxing out at 1.0
                let strength = (tip_speed / 2000.0).clamp(0.0, 1.0);
                if strength > 0.05 {
                     let _ = cmd_tx.send(AudioCommand::Pluck {
                        x: grid_x as usize,
                        y: grid_y as usize,
                        strength,
                    });
                }
            }
        }

        // Run silent model if needed
        if let Some(m) = &mut model {
            let steps = (dt * SAMPLE_RATE) as usize;
            let steps = steps.min(2000);
            let mut dummy_buffer = vec![0.0; steps];
            m.process(&mut dummy_buffer);
        }

        // Receive snapshots
        while let Ok(snap) = snap_rx.try_recv() {
            current_snapshot = snap.pressure;
        }

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

        // Render
        clear_background(BLACK);

        // Draw Tank
        draw_texture_ex(
            &texture,
            offset_x,
            offset_y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(TANK_DRAW_SIZE, TANK_DRAW_SIZE)),
                ..Default::default()
            },
        );

        // Draw tank borders
        draw_rectangle_lines(offset_x, offset_y, TANK_DRAW_SIZE, TANK_DRAW_SIZE, 2.0, GRAY);

        // Draw Pendulum
        for link in &sys.links {
            let pos_a = sys.nodes[link.a].pos;
            let pos_b = sys.nodes[link.b].pos;
            draw_line(pos_a.x, pos_a.y, pos_b.x, pos_b.y, 4.0, LIGHTGRAY);
        }

        for node in &sys.nodes {
            let color = if node.fixed { RED } else { YELLOW };
            let size = if node.fixed { 8.0 } else { (node.mass * 2.0).clamp(5.0, 20.0) };
            draw_circle(node.pos.x, node.pos.y, size, color);
        }

        draw_text(
            "CHAOS TANK (Ripple Tank x Chaos Pendulum)",
            10.0,
            20.0,
            20.0,
            WHITE,
        );
        draw_text("Drag the yellow pendulum tip (L-Click) into the box.", 10.0, 45.0, 20.0, GRAY);

        let avg_pressure: f32 = current_snapshot.iter().map(|v| v.abs()).sum::<f32>() / (GRID_WIDTH * GRID_HEIGHT) as f32;
        draw_text(&format!("Tank Energy: {:.5}", avg_pressure), 10.0, 70.0, 20.0, if avg_pressure > 0.01 { GREEN } else { DARKGRAY });

        next_frame().await
    }
}

#[cfg(feature = "audio")]
fn init_audio(
    cmd_rx: crossbeam_channel::Receiver<AudioCommand>,
    snap_tx: crossbeam_channel::Sender<resonance_audio::audio::AudioSnapshot>,
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
    fn test_world_init() {
        let mut sys = PendulumSystem::new();
        assert_eq!(sys.nodes.len(), 0);
        let n1 = sys.add_node(Vec2::new(0.0, 0.0), 1.0, true, "1".into());
        let n2 = sys.add_node(Vec2::new(1.0, 0.0), 1.0, false, "2".into());
        sys.add_link(n1, n2, 1.0);

        assert_eq!(sys.nodes.len(), 2);
        assert_eq!(sys.links.len(), 1);

        sys.step(0.016);
    }
}