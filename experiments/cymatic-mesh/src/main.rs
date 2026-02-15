use crossbeam_channel::bounded;
use macroquad::prelude::*;
use resonance_audio::audio::{AudioCommand, AudioSnapshot};
use resonance_audio::physics::Material as AudioMaterial;

mod audio_backend;

const GRID_W: usize = 60;
const GRID_H: usize = 60;

#[macroquad::main("Cymatic Mesh")]
async fn main() {
    // 1. Setup Channels
    let (cmd_tx, cmd_rx) = bounded(1024);
    let (snap_tx, snap_rx) = bounded(1);

    // 2. Init Audio Backend
    // We hold the system to keep the stream/thread alive
    let _audio_system = match audio_backend::init_audio(GRID_W, GRID_H, cmd_rx, snap_tx) {
        Ok(sys) => sys,
        Err(e) => {
            eprintln!("Failed to init audio: {}", e);
            return;
        }
    };

    let mut snapshot: Option<AudioSnapshot> = None;

    loop {
        // 3. Process Input
        let (mx, my) = mouse_position();
        let cell_w = screen_width() / GRID_W as f32;
        let cell_h = screen_height() / GRID_H as f32;

        let gx = (mx / cell_w) as usize;
        let gy = (my / cell_h) as usize;
        let in_bounds = gx < GRID_W && gy < GRID_H;

        if in_bounds {
            if is_mouse_button_pressed(MouseButton::Left) {
                // Pluck
                let _ = cmd_tx.send(AudioCommand::Pluck {
                    x: gx,
                    y: gy,
                    strength: 1.0,
                });
            }
            if is_mouse_button_down(MouseButton::Right) {
                // Draw Wall
                if is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift) {
                    // Erase
                    let _ = cmd_tx.send(AudioCommand::RemoveWall { x: gx, y: gy });
                } else {
                    // Draw
                    let _ = cmd_tx.send(AudioCommand::AddWall { x: gx, y: gy });
                }
            }
            if is_mouse_button_pressed(MouseButton::Middle) {
                // Oscillate (Toggle would be harder without state tracking here, so just Add)
                // Let's make Middle Click add a 440Hz oscillator
                let _ = cmd_tx.send(AudioCommand::Oscillate {
                    x: gx,
                    y: gy,
                    frequency: 440.0,
                    strength: 0.5,
                });
            }
        }

        if is_key_pressed(KeyCode::R) {
            let _ = cmd_tx.send(AudioCommand::ClearWaves);
            let _ = cmd_tx.send(AudioCommand::ClearWalls);
        }
        if is_key_pressed(KeyCode::C) {
            let _ = cmd_tx.send(AudioCommand::ClearWaves);
        }

        // 4. Update State
        if let Ok(snap) = snap_rx.try_recv() {
            snapshot = Some(snap);
        }

        // 5. Draw
        clear_background(BLACK);

        if let Some(snap) = &snapshot {
            for y in 0..GRID_H {
                for x in 0..GRID_W {
                    let idx = y * GRID_W + x;
                    let pressure = snap.pressure[idx];
                    let material = snap.materials[idx];

                    let rect_x = x as f32 * cell_w;
                    let rect_y = y as f32 * cell_h;

                    let color = match material {
                        AudioMaterial::Wall => WHITE,
                        AudioMaterial::Void => DARKGRAY,
                        _ => {
                            // Pressure visualization
                            // -1.0 (Blue) -> 0.0 (Black) -> 1.0 (Red)
                            let p = pressure.clamp(-1.0, 1.0);
                            if p > 0.0 {
                                Color::new(p, 0.0, 0.0, 1.0)
                            } else {
                                Color::new(0.0, 0.0, -p, 1.0)
                            }
                        }
                    };

                    draw_rectangle(rect_x, rect_y, cell_w, cell_h, color);
                }
            }
        } else {
            draw_text("Waiting for audio thread...", 20.0, 20.0, 30.0, WHITE);
        }

        // UI Overlay
        draw_text("L-Click: Pluck", 10.0, 20.0, 20.0, GREEN);
        draw_text("R-Drag: Wall (Shift: Erase)", 10.0, 40.0, 20.0, GREEN);
        draw_text("M-Click: Oscillator", 10.0, 60.0, 20.0, GREEN);
        draw_text("C: Clear Waves | R: Reset All", 10.0, 80.0, 20.0, GREEN);

        #[cfg(not(feature = "audio"))]
        draw_text("AUDIO DISABLED (Simulation Mode)", 10.0, 100.0, 20.0, RED);

        next_frame().await;
    }
}
