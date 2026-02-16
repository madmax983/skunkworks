use crate::etymology::CodeRiver;
use macroquad::prelude::*;

pub fn draw_river(river: &CodeRiver, offset: Vec2, zoom: f32) {
    let layer_width = 150.0 * zoom;
    let line_height = 20.0 * zoom;
    let node_radius = 3.0 * zoom;

    // Draw layers
    for (i, layer) in river.layers.iter().enumerate() {
        let x = offset.x + i as f32 * layer_width;

        // Draw mapping to NEXT layer (Parent)
        if i + 1 < river.layers.len() {
            let next_layer = &river.layers[i + 1];
            let next_x = offset.x + (i + 1) as f32 * layer_width;

            for (line_idx, &parent_idx_opt) in layer.mapping.iter().enumerate() {
                if let Some(parent_idx) = parent_idx_opt {
                    let y = offset.y + line_idx as f32 * line_height;
                    let next_y = offset.y + parent_idx as f32 * line_height;

                    let p1 = vec2(x, y);
                    let p2 = vec2(next_x, next_y);
                    let cp1 = vec2(x + layer_width * 0.5, y);
                    let cp2 = vec2(next_x - layer_width * 0.5, next_y);

                    // Determine color
                    // TODO: Use phonology distance. For now, strict equality check.
                    let is_identical =
                        if line_idx < layer.lines.len() && parent_idx < next_layer.lines.len() {
                            layer.lines[line_idx] == next_layer.lines[parent_idx]
                        } else {
                            false
                        };

                    let color = if is_identical {
                        Color::new(0.3, 0.7, 0.3, 0.4) // Stable Green
                    } else {
                        Color::new(0.8, 0.3, 0.3, 0.6) // Mutation Red
                    };

                    draw_cubic_bezier(p1, p2, cp1, cp2, 2.0 * zoom, color);
                }
            }
        }

        // Draw Nodes
        for (line_idx, _) in layer.lines.iter().enumerate() {
            let y = offset.y + line_idx as f32 * line_height;
            // Only draw node if visible
            if y > -100.0 && y < screen_height() + 100.0 {
                draw_circle(
                    x,
                    y,
                    node_radius,
                    match i {
                        0 => SKYBLUE, // HEAD
                        _ => GRAY,
                    },
                );
            }
        }

        // Draw Layer Header (Commit Info)
        // Only if visible horizontally
        if x > -200.0 && x < screen_width() + 200.0 && i < river.commits.len() {
            let short_hash = &river.commits[i][0..7.min(river.commits[i].len())];
            let author = if i < river.authors.len() {
                &river.authors[i]
            } else {
                "?"
            };
            draw_text(short_hash, x - 20.0, offset.y - 30.0, 16.0, WHITE);
            draw_text(author, x - 20.0, offset.y - 15.0, 12.0, LIGHTGRAY);
        }
    }
}

fn draw_cubic_bezier(p0: Vec2, p3: Vec2, p1: Vec2, p2: Vec2, thickness: f32, color: Color) {
    let segments = 20;
    let mut prev = p0;
    for i in 1..=segments {
        let t = i as f32 / segments as f32;
        let t_inv = 1.0 - t;

        // Cubic Bezier Formula: (1-t)^3*P0 + 3(1-t)^2*t*P1 + 3(1-t)t^2*P2 + t^3*P3
        let p = p0 * (t_inv * t_inv * t_inv)
            + p1 * (3.0 * t_inv * t_inv * t)
            + p2 * (3.0 * t_inv * t * t)
            + p3 * (t * t * t);

        draw_line(prev.x, prev.y, p.x, p.y, thickness, color);
        prev = p;
    }
}
