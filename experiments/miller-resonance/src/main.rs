//! # Miller Resonance 🧬
//!
//! **Acoustic Codebase Morphogenesis**
//!
//! This experiment is a hybrid cross between:
//! - **`miller-lattice`**: The parent crate providing the recursive structural mapping of the codebase into a spatial crystal lattice.
//! - **`resonance-audio`**: The parent crate providing a continuous 2D finite-difference time-domain (FDTD) acoustic wave tank simulation.
//!
//! ## Novel Trait & Lineage
//!
//! This experiment demonstrates **Acoustic Codebase Morphogenesis**.
//!
//! The discrete hierarchical structure of the file system (from `miller-lattice`) is physically instantiated within a continuous sound wave simulator (`resonance-audio`). Specifically, every directory mapped by the lattice acts as a solid, sound-reflecting boundary wall within the acoustic grid.
//!
//! By periodically plucking the center of the repository (or manually plucking specific grid cells), the system generates expanding acoustic pressure waves. These waves diffract, echo, and form standing interference patterns against the unique physical geometry of the repository's structure.
//!
//! The emergent phenotype acts as a sonic fingerprint of the repository—different directory topologies natively produce different acoustic interference patterns and resonances.
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use crossbeam_channel::bounded;
use macroquad::prelude::*;
use miller_lattice::Crystal;
use resonance_audio::audio::{AudioCommand, AudioModel};
use resonance_audio::physics::Material;

const GRID_W: usize = 120;
const GRID_H: usize = 120;

#[macroquad::main("Miller Resonance: Acoustic Codebase Morphogenesis")]
async fn main() {
    let root_path = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
    let crystal = Crystal::build_from_path(&root_path).unwrap_or_else(|_| Crystal::new());

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

    let mut image = Image::gen_image_color(GRID_W as u16, GRID_H as u16, BLANK);
    let texture = Texture2D::from_image(&image);
    texture.set_filter(FilterMode::Nearest);

    // Find crystal bounds to map to the 2D grid
    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;

    for atom in &crystal.atoms {
        min_x = min_x.min(atom.position.x);
        max_x = max_x.max(atom.position.x);
        min_y = min_y.min(atom.position.y);
        max_y = max_y.max(atom.position.y);
    }

    let w = (max_x - min_x).max(1) as f32;
    let h = (max_y - min_y).max(1) as f32;

    // Convert crystal structure to walls
    for atom in &crystal.atoms {
        let normalized_x = (atom.position.x - min_x) as f32 / w;
        let normalized_y = (atom.position.y - min_y) as f32 / h;

        let grid_x = (normalized_x * (GRID_W as f32 - 1.0)).round() as usize;
        let grid_y = (normalized_y * (GRID_H as f32 - 1.0)).round() as usize;

        let grid_x = grid_x.clamp(1, GRID_W - 2);
        let grid_y = grid_y.clamp(1, GRID_H - 2);

        if atom.is_dir {
            // Directories form solid walls
            cmd_tx
                .send(AudioCommand::AddWall {
                    x: grid_x,
                    y: grid_y,
                })
                .unwrap();

            // make dirs thicker
            if grid_x + 1 < GRID_W - 1 {
                cmd_tx
                    .send(AudioCommand::AddWall {
                        x: grid_x + 1,
                        y: grid_y,
                    })
                    .unwrap();
            }
            if grid_y + 1 < GRID_H - 1 {
                cmd_tx
                    .send(AudioCommand::AddWall {
                        x: grid_x,
                        y: grid_y + 1,
                    })
                    .unwrap();
            }
        }
    }

    let mut frame_count = 0;

    loop {
        clear_background(Color::new(0.05, 0.05, 0.08, 1.0));

        if frame_count % 60 == 0 {
            // Drop a pluck near the center to sonify the codebase structure
            cmd_tx
                .send(AudioCommand::Pluck {
                    x: GRID_W / 2,
                    y: GRID_H / 2,
                    strength: 1.0,
                })
                .unwrap();
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            let (mx, my) = mouse_position();
            let scale = (screen_height() / GRID_H as f32).min(screen_width() / GRID_W as f32);
            let draw_w = GRID_W as f32 * scale;
            let draw_h = GRID_H as f32 * scale;
            let offset_x = (screen_width() - draw_w) / 2.0;
            let offset_y = (screen_height() - draw_h) / 2.0;

            let grid_x = ((mx - offset_x) / scale).round() as usize;
            let grid_y = ((my - offset_y) / scale).round() as usize;

            if grid_x > 0 && grid_x < GRID_W - 1 && grid_y > 0 && grid_y < GRID_H - 1 {
                cmd_tx
                    .send(AudioCommand::Pluck {
                        x: grid_x,
                        y: grid_y,
                        strength: 2.0,
                    })
                    .unwrap();
            }
        }

        if let Ok(snapshot) = snap_rx.try_recv() {
            let pixels = image.get_image_data_mut();
            for y in 0..GRID_H {
                for x in 0..GRID_W {
                    let idx = y * GRID_W + x;
                    let p = snapshot.pressure[idx];
                    let is_wall = snapshot.materials[idx] == Material::Wall;

                    if is_wall {
                        pixels[idx] = [100, 255, 100, 255]; // Codebase nodes (walls)
                    } else {
                        // Visualize acoustic wave
                        let c = ((p + 1.0) * 0.5 * 255.0).clamp(0.0, 255.0) as u8;
                        pixels[idx] = [c, c.saturating_sub(50), 255, 255]; // Waves propagating
                    }
                }
            }
            texture.update(&image);
        }

        let scale = (screen_height() / GRID_H as f32).min(screen_width() / GRID_W as f32);
        let draw_w = GRID_W as f32 * scale;
        let draw_h = GRID_H as f32 * scale;
        let x = (screen_width() - draw_w) / 2.0;
        let y = (screen_height() - draw_h) / 2.0;

        draw_texture_ex(
            &texture,
            x,
            y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(draw_w, draw_h)),
                ..Default::default()
            },
        );

        draw_text("Miller Resonance", 10.0, 20.0, 30.0, WHITE);
        draw_text(
            "Acoustic codebase morphogenesis. Codebase directories act as physical walls.",
            10.0,
            50.0,
            20.0,
            GRAY,
        );
        draw_text(
            "Periodic plucks map the repository's topology to an acoustic standing wave.",
            10.0,
            75.0,
            20.0,
            GRAY,
        );
        draw_text("Click to pluck locally.", 10.0, 100.0, 20.0, GRAY);

        frame_count += 1;
        next_frame().await;
    }
}
