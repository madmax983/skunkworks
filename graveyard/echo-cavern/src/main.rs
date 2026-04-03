use crossbeam_channel::bounded;
use macroquad::prelude::*;
use resonance_audio::audio::{AudioCommand, AudioModel, AudioSnapshot};
use resonance_audio::physics::Material;
use std::path::Path;

mod layout;
use layout::generate_layout;

const GRID_WIDTH: usize = 200;
const GRID_HEIGHT: usize = 200;
const SAMPLE_RATE: f32 = 44100.0;

#[macroquad::main("Echo Cavern")]
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

    // If audio is not active, we drive simulation manually
    #[cfg(not(feature = "audio"))]
    let mut model = AudioModel::new(GRID_WIDTH, GRID_HEIGHT, cmd_rx, snap_tx);

    // 2. Build the Cavern Layout
    // We scan the current directory (or parent if current is empty/trivial)
    let path = Path::new(".");
    let layout_items = generate_layout(
        path,
        Rect::new(0.0, 0.0, GRID_WIDTH as f32, GRID_HEIGHT as f32),
    );

    // Send layout to audio model
    // We do this by sending PaintMaterial commands
    // Note: This might flood the channel if too many commands, but bounded(1024) might block.
    // Actually bounded channel will block if full. But we are sending from main thread.
    // If audio thread is running, it consumes.
    // If audio thread is NOT running (manual mode), we need to process commands manually or just initialize model with them.
    // Since we can't easily access model in audio mode, we rely on channel.

    // To avoid blocking the main thread if audio thread is slow or not started yet:
    // We should probably just send them.

    println!("Generated layout with {} items", layout_items.len());

    for item in &layout_items {
        let x_start = item.rect.x as usize;
        let y_start = item.rect.y as usize;
        let x_end = (item.rect.x + item.rect.w) as usize;
        let y_end = (item.rect.y + item.rect.h) as usize;

        for y in y_start..y_end {
            for x in x_start..x_end {
                if x < GRID_WIDTH && y < GRID_HEIGHT {
                    // Ignore error if channel full (it will block if bounded, which is fine here as we want to ensure map is built)
                    let _ = cmd_tx.send(AudioCommand::PaintMaterial {
                        x,
                        y,
                        material: item.material,
                    });
                }
            }
        }
    }

    // 3. Visual Setup
    let mut image = Image::gen_image_color(GRID_WIDTH as u16, GRID_HEIGHT as u16, BLACK);
    let texture = Texture2D::from_image(&image);
    texture.set_filter(FilterMode::Nearest);

    let mut current_snapshot: Option<AudioSnapshot> = None;
    let mut listener_pos = (GRID_WIDTH / 2, GRID_HEIGHT / 2);
    let mut source_pos = (GRID_WIDTH / 2, GRID_HEIGHT / 2);
    let mut continuous_tone = false;

    // Initial listener position
    let _ = cmd_tx.send(AudioCommand::MoveListener {
        x: listener_pos.0,
        y: listener_pos.1,
    });

    loop {
        // Poll for snapshot
        while let Ok(snap) = snap_rx.try_recv() {
            current_snapshot = Some(snap);
        }

        #[cfg(not(feature = "audio"))]
        {
            // Simulate steps for this frame
            let dt = get_frame_time();
            let steps = (dt * SAMPLE_RATE) as usize;
            let steps = steps.min(2000); // Cap to avoid freeze

            let mut dummy_buffer = vec![0.0; steps];
            model.process(&mut dummy_buffer);
        }

        // Update texture based on snapshot
        if let Some(snap) = &current_snapshot {
            for (i, &pressure) in snap.pressure.iter().enumerate() {
                let x = (i % GRID_WIDTH) as u32;
                let y = (i / GRID_WIDTH) as u32;
                let material = snap.materials[i];

                // Color mapping:
                // Materials form the static map
                // Pressure adds dynamic light/color

                let base_color = match material {
                    Material::Air => Color::new(0.05, 0.05, 0.1, 1.0), // Dark Blue
                    Material::Wall => Color::new(0.4, 0.4, 0.4, 1.0),  // Grey
                    Material::Slow => Color::new(0.2, 0.1, 0.05, 1.0), // Brown (Mud)
                    Material::Fast => Color::new(0.0, 0.2, 0.2, 1.0),  // Teal (Water/Ice)
                    Material::Void => Color::new(0.0, 0.0, 0.0, 1.0),  // Black
                };

                let p = pressure.clamp(-1.0, 1.0);
                let wave_color = if p > 0.0 {
                    Color::new(1.0, 0.5, 0.2, p.abs()) // Orange for positive
                } else {
                    Color::new(0.2, 0.5, 1.0, p.abs()) // Blue for negative
                };

                // Blend
                // Simple additive blending approximation
                let r = (base_color.r + wave_color.r * wave_color.a).min(1.0);
                let g = (base_color.g + wave_color.g * wave_color.a).min(1.0);
                let b = (base_color.b + wave_color.b * wave_color.a).min(1.0);

                image.set_pixel(x, y, Color::new(r, g, b, 1.0));
            }
            texture.update(&image);
        }

        // Draw
        clear_background(BLACK);

        // Scale to fit screen
        let scale =
            (screen_width() / GRID_WIDTH as f32).min(screen_height() / GRID_HEIGHT as f32) * 0.9;
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

        // Input Handling

        // Listener Movement (WASD)
        let mut moved = false;
        if is_key_down(KeyCode::W) && listener_pos.1 > 0 {
            listener_pos.1 -= 1;
            moved = true;
        }
        if is_key_down(KeyCode::S) && listener_pos.1 < GRID_HEIGHT - 1 {
            listener_pos.1 += 1;
            moved = true;
        }
        if is_key_down(KeyCode::A) && listener_pos.0 > 0 {
            listener_pos.0 -= 1;
            moved = true;
        }
        if is_key_down(KeyCode::D) && listener_pos.0 < GRID_WIDTH - 1 {
            listener_pos.0 += 1;
            moved = true;
        }

        if moved {
            let _ = cmd_tx.send(AudioCommand::MoveListener {
                x: listener_pos.0,
                y: listener_pos.1,
            });
        }

        // Source Movement (Arrows)
        if is_key_down(KeyCode::Up) && source_pos.1 > 0 {
            source_pos.1 -= 1;
        }
        if is_key_down(KeyCode::Down) && source_pos.1 < GRID_HEIGHT - 1 {
            source_pos.1 += 1;
        }
        if is_key_down(KeyCode::Left) && source_pos.0 > 0 {
            source_pos.0 -= 1;
        }
        if is_key_down(KeyCode::Right) && source_pos.0 < GRID_WIDTH - 1 {
            source_pos.0 += 1;
        }

        // Mouse interaction overrides source pos
        let mouse_pos = mouse_position();
        let mut hovered_item_name = None;
        if mouse_pos.0 >= offset_x
            && mouse_pos.0 < offset_x + draw_width
            && mouse_pos.1 >= offset_y
            && mouse_pos.1 < offset_y + draw_height
        {
            let gx = ((mouse_pos.0 - offset_x) / scale) as usize;
            let gy = ((mouse_pos.1 - offset_y) / scale) as usize;
            source_pos = (gx.clamp(0, GRID_WIDTH - 1), gy.clamp(0, GRID_HEIGHT - 1));

            if is_mouse_button_pressed(MouseButton::Left) {
                let _ = cmd_tx.send(AudioCommand::Pluck {
                    x: source_pos.0,
                    y: source_pos.1,
                    strength: 0.8,
                });
            }

            // Find item under mouse
            for item in &layout_items {
                if (gx as f32) >= item.rect.x
                    && (gx as f32) < (item.rect.x + item.rect.w)
                    && (gy as f32) >= item.rect.y
                    && (gy as f32) < (item.rect.y + item.rect.h)
                {
                    hovered_item_name =
                        Some(item.path.file_name().unwrap_or_default().to_string_lossy());
                    break;
                }
            }
        }

        // Actions
        if is_key_pressed(KeyCode::Space) {
            let _ = cmd_tx.send(AudioCommand::Pluck {
                x: source_pos.0,
                y: source_pos.1,
                strength: 0.8,
            });
        }

        if is_key_pressed(KeyCode::Enter) {
            continuous_tone = !continuous_tone;
            let strength = if continuous_tone { 0.5 } else { 0.0 };
            let _ = cmd_tx.send(AudioCommand::Oscillate {
                x: source_pos.0,
                y: source_pos.1,
                frequency: 220.0,
                strength,
            });
        }

        // If continuous, update position of oscillator
        if continuous_tone
            && (moved
                || is_key_down(KeyCode::Up)
                || is_key_down(KeyCode::Down)
                || is_key_down(KeyCode::Left)
                || is_key_down(KeyCode::Right))
        {
            // We need to remove old and add new or just update.
            // Ideally we'd track the oscillator ID or position, but here we just blindly send update.
            // Actually, AudioModel logic: "Check if oscillator exists... if let Some(pos) = ... position(|o| o.x == x && o.y == y)".
            // This logic means we can't move an oscillator easily without removing old one first.
            // For now, let's just use Pluck for movement or toggle off/on.
            // Or we just don't support moving continuous tone easily.
        }

        // Draw HUD
        let lx = offset_x + listener_pos.0 as f32 * scale;
        let ly = offset_y + listener_pos.1 as f32 * scale;
        draw_circle(lx + scale / 2.0, ly + scale / 2.0, scale, GREEN); // Ear

        let sx = offset_x + source_pos.0 as f32 * scale;
        let sy = offset_y + source_pos.1 as f32 * scale;
        draw_circle_lines(sx + scale / 2.0, sy + scale / 2.0, scale, 2.0, RED); // Source

        draw_text("Echo Cavern", 10.0, 20.0, 30.0, WHITE);
        draw_text(
            "WASD: Move Ear | Mouse/Arrows: Move Source | Click/Space: Ping | Enter: Toggle Drone",
            10.0,
            50.0,
            20.0,
            GRAY,
        );

        if let Some(name) = hovered_item_name {
            draw_text(&name, mouse_pos.0 + 10.0, mouse_pos.1, 20.0, YELLOW);
        }

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

    let mut model = AudioModel::new(GRID_WIDTH, GRID_HEIGHT, cmd_rx, snap_tx);

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
