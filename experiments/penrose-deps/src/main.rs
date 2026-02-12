mod graph;
mod layout;
mod render;

use macroquad::prelude::*;
use petgraph::visit::EdgeRef;
use crate::render::{draw_iso_block, draw_stair_path, iso_project};

enum RenderCmd {
    Block { u: i32, v: i32, w: i32, color: Color, label: String },
    Stair { u1: i32, v1: i32, w1: i32, u2: i32, v2: i32, w2: i32, color: Color },
}

#[macroquad::main("Penrose Deps")]
async fn main() {
    // Load graph
    let graph_res = graph::build_graph();
    if let Err(e) = &graph_res {
        println!("Error building graph: {}", e);
        return;
    }
    let graph = graph_res.unwrap();

    // Layout
    let layout = layout::calculate_layout(&graph);

    // Precompute render commands
    let mut commands: Vec<(i32, RenderCmd)> = Vec::new();

    for (idx, (u, v, w)) in &layout.positions {
        let u = *u; let v = *v; let w = *w;
        let label = graph[*idx].clone();

        // Sort key: In isometric, farther back is smaller x+y
        // We add w to sort key because higher blocks should be drawn after lower blocks if they share (u,v)
        // Actually, simple Painter's algorithm for isometric:
        // Sort by (u + v). If equal, sort by w?
        // Let's try u + v + w.
        let sort_key = u + v + w;

        commands.push((sort_key, RenderCmd::Block {
            u, v, w,
            color: hash_color(&label),
            label
        }));

        // Add edges starting from this node
        for edge in graph.edges(*idx) {
            let target = edge.target();
            if let Some(target_pos) = layout.positions.get(&target) {
                let (u2, v2, w2) = *target_pos;
                // Edges are tricky. Use average position.
                let sort_key_edge = (u + v + w + u2 + v2 + w2) / 2;

                commands.push((sort_key_edge, RenderCmd::Stair {
                    u1: u, v1: v, w1: w,
                    u2, v2, w2,
                    color: DARKGRAY
                }));
            }
        }
    }

    commands.sort_by_key(|(k, _)| *k);

    let mut cam_zoom = 0.01;
    let mut cam_offset = vec2(0.0, 0.0);

    loop {
        clear_background(LIGHTGRAY);

        // Input
        if is_key_down(KeyCode::W) { cam_offset.y -= 10.0 / cam_zoom / 100.0; }
        if is_key_down(KeyCode::S) { cam_offset.y += 10.0 / cam_zoom / 100.0; }
        if is_key_down(KeyCode::A) { cam_offset.x -= 10.0 / cam_zoom / 100.0; }
        if is_key_down(KeyCode::D) { cam_offset.x += 10.0 / cam_zoom / 100.0; }
        if is_key_down(KeyCode::Up) { cam_zoom *= 1.02; }
        if is_key_down(KeyCode::Down) { cam_zoom *= 0.98; }

        // Camera transform
        set_camera(&Camera2D {
            target: cam_offset,
            zoom: vec2(cam_zoom, -cam_zoom * screen_width() / screen_height()),
            ..Default::default()
        });

        for (_, cmd) in &commands {
            match cmd {
                RenderCmd::Block { u, v, w, color, label } => {
                    draw_iso_block(*u, *v, *w, *color);
                    // Draw label on top
                    // Labels need to be drawn in world space but facing camera?
                    // draw_text works in screen space if using default camera.
                    // But we are using a custom camera.
                    // macroquad's draw_text is fixed pixel size.
                    // draw_text_ex allows scaling.

                    let pos = iso_project(*u, *v, *w);
                    // Offset to be above the block
                    let text_pos = vec2(pos.x, pos.y - 40.0);

                    // Simple text scaling inverse to zoom to keep readable?
                    // Or just let it scale with world.
                    draw_text_ex(label, text_pos.x, text_pos.y, TextParams {
                        font_size: 40, // Large font to be visible
                        color: BLACK,
                        ..Default::default()
                    });
                },
                RenderCmd::Stair { u1, v1, w1, u2, v2, w2, color } => {
                    draw_stair_path(*u1, *v1, *w1, *u2, *v2, *w2, *color);
                }
            }
        }

        set_default_camera();
        draw_text("WASD: Pan | Up/Down: Zoom", 20.0, 30.0, 30.0, BLACK);
        draw_text(&format!("Zoom: {:.4}", cam_zoom), 20.0, 60.0, 20.0, DARKGRAY);

        next_frame().await
    }
}

fn hash_color(s: &str) -> Color {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    let hash = hasher.finish();
    let r = ((hash >> 16) & 0xFF) as f32 / 255.0;
    let g = ((hash >> 8) & 0xFF) as f32 / 255.0;
    let b = (hash & 0xFF) as f32 / 255.0;
    // Ensure pastel/nice colors
    Color::new(0.5 + r * 0.5, 0.5 + g * 0.5, 0.5 + b * 0.5, 1.0)
}
