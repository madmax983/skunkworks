use crossbeam_channel::bounded;
use macroquad::prelude::*;
use std::env;

mod audio_backend;
mod scanner;
mod treemap;

use audio_backend::init_audio;
use resonance_audio::audio::{AudioCommand, AudioSnapshot};
use resonance_audio::physics::Material;
use scanner::FileNode;
use treemap::{generate_layout, LayoutNode};

const GRID_W: usize = 120;
const GRID_H: usize = 80;

#[macroquad::main("Code Acoustics")]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let path = if args.len() > 1 { &args[1] } else { "." };

    println!("Scanning {}...", path);
    let root = FileNode::scan(path);
    println!("Scan complete. Total size: {}", root.size);

    // 1. Setup Audio
    let (cmd_tx, cmd_rx) = bounded(2048);
    let (snap_tx, snap_rx) = bounded(2);

    let _audio_system = match init_audio(GRID_W, GRID_H, cmd_rx, snap_tx) {
        Ok(sys) => sys,
        Err(e) => {
            eprintln!("Failed to init audio: {}", e);
            return;
        }
    };

    // 2. Generate Layout
    // We map the treemap to the grid coordinates
    let layout = generate_layout(&root, Rect::new(0.0, 0.0, GRID_W as f32, GRID_H as f32));

    // 3. Build the acoustic map
    // Default to Wall
    let _ = cmd_tx.send(AudioCommand::ClearWaves);
    // Fill with Wall first?
    for y in 0..GRID_H {
        for x in 0..GRID_W {
            let _ = cmd_tx.send(AudioCommand::PaintMaterial {
                x,
                y,
                material: Material::Wall,
            });
        }
    }

    // Carve out rooms for files
    for node in &layout {
        if !node.node.is_dir {
            // Shrink by 1 cell to leave walls
            let r = node.rect;
            let x_start = (r.x.round() as usize).max(0);
            let y_start = (r.y.round() as usize).max(0);
            let x_end = ((r.x + r.w).round() as usize).min(GRID_W);
            let y_end = ((r.y + r.h).round() as usize).min(GRID_H);

            if x_end > x_start + 1 && y_end > y_start + 1 {
                for y in (y_start + 1)..(y_end - 1) {
                    for x in (x_start + 1)..(x_end - 1) {
                        // Determine material based on file extension/size
                        let mat = if node.node.size > 100_000 {
                            Material::Slow // Large files are dense/slow
                        } else {
                            Material::Air // Small files are air
                        };

                        let _ = cmd_tx.send(AudioCommand::PaintMaterial {
                            x,
                            y,
                            material: mat,
                        });
                    }
                }
            } else if x_end > x_start && y_end > y_start {
                // Too small for margin, just make it Air without walls?
                // Or keep as Wall?
                // Let's keep small files as simple Air spots if possible
                let _ = cmd_tx.send(AudioCommand::PaintMaterial {
                    x: x_start,
                    y: y_start,
                    material: Material::Air,
                });
            }
        }
    }

    let mut snapshot: Option<AudioSnapshot> = None;
    let mut hover_node: Option<&LayoutNode> = None;

    loop {
        let (mx, my) = mouse_position();
        let cell_w = screen_width() / GRID_W as f32;
        let cell_h = screen_height() / GRID_H as f32;

        let gx = (mx / cell_w) as usize;
        let gy = (my / cell_h) as usize;
        let in_bounds = gx < GRID_W && gy < GRID_H;

        // Find hovered node
        if in_bounds {
            // Check layout (reverse to find smallest/deepest first)
            hover_node = layout.iter().rev().find(|n| {
                let r = n.rect;
                gx as f32 >= r.x
                    && gx as f32 <= r.x + r.w
                    && gy as f32 >= r.y
                    && gy as f32 <= r.y + r.h
                    && !n.node.is_dir
            });

            if is_mouse_button_pressed(MouseButton::Left) {
                let _ = cmd_tx.send(AudioCommand::Pluck {
                    x: gx,
                    y: gy,
                    strength: 1.0,
                });
            }
            if is_mouse_button_down(MouseButton::Right) {
                let _ = cmd_tx.send(AudioCommand::PaintMaterial {
                    x: gx,
                    y: gy,
                    material: Material::Wall,
                });
            }
        }

        if is_key_pressed(KeyCode::R) {
            let _ = cmd_tx.send(AudioCommand::ClearWaves);
        }

        // Receive snapshot
        if let Ok(snap) = snap_rx.try_recv() {
            snapshot = Some(snap);
        }

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
                        Material::Wall => Color::new(0.2, 0.2, 0.2, 1.0),
                        Material::Slow => Color::new(0.0, 0.1, 0.1, 1.0),
                        Material::Void => BLACK,
                        _ => {
                            // Pressure
                            let p = pressure.clamp(-1.0, 1.0);
                            if p > 0.0 {
                                Color::new(p, 0.0, 0.0, 1.0)
                            } else {
                                Color::new(0.0, 0.0, -p, 1.0)
                            }
                        }
                    };

                    if material != Material::Void {
                        draw_rectangle(rect_x, rect_y, cell_w, cell_h, color);
                    }
                }
            }
        }

        // UI Overlay
        if let Some(node) = hover_node {
            let text = format!("{} ({})", node.node.path.display(), node.node.size);
            draw_text(&text, 10.0, 30.0, 30.0, WHITE);

            // Highlight rect
            let r = node.rect;
            draw_rectangle_lines(
                r.x * cell_w,
                r.y * cell_h,
                r.w * cell_w,
                r.h * cell_h,
                2.0,
                YELLOW,
            );
        } else {
            draw_text("Code Acoustics", 10.0, 30.0, 30.0, WHITE);
        }

        draw_text(
            "L-Click: Pluck | R-Drag: Wall | R: Reset",
            10.0,
            screen_height() - 10.0,
            20.0,
            GRAY,
        );

        next_frame().await;
    }
}
