use macroquad::prelude::*;
mod tiling;
mod dungeon;

use tiling::{TriangleType, generate_initial_tiling, subdivide};
use dungeon::{Dungeon};
use petgraph::graph::NodeIndex;

#[macroquad::main("Penrose Dungeon")]
async fn main() {
    let mut triangles = generate_initial_tiling();
    // 5 iterations -> ~3000 tiles
    for _ in 0..5 {
        triangles = subdivide(&triangles);
    }

    let dungeon = Dungeon::new(triangles);

    // Start at a random node close to center
    let mut current_node = NodeIndex::new(0);
    let mut min_dist = f32::MAX;

    for (idx, center) in &dungeon.room_centers {
        let d = center.length();
        if d < min_dist {
            min_dist = d;
            current_node = *idx;
        }
    }

    let mut player_pos = dungeon.room_centers[&current_node];
    let mut camera_pos = player_pos;
    let mut zoom_level = 0.005; // View scale factor

    loop {
        if is_key_down(KeyCode::Escape) {
            break;
        }

        // Camera controls
        if is_key_down(KeyCode::Z) { zoom_level *= 1.05; }
        if is_key_down(KeyCode::X) { zoom_level /= 1.05; }

        let (_, wheel_y) = mouse_wheel();
        if wheel_y != 0.0 {
            if wheel_y > 0.0 {
                zoom_level *= 1.1;
            } else {
                zoom_level /= 1.1;
            }
        }

        // Smooth camera follow
        camera_pos = camera_pos.lerp(player_pos, 0.1);

        // Movement
        let mut move_dir = Vec2::ZERO;
        // Key press for discrete movement
        let up = is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::W);
        let down = is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::S);
        let left = is_key_pressed(KeyCode::Left) || is_key_pressed(KeyCode::A);
        let right = is_key_pressed(KeyCode::Right) || is_key_pressed(KeyCode::D);

        if up { move_dir.y -= 1.0; }
        if down { move_dir.y += 1.0; }
        if left { move_dir.x -= 1.0; }
        if right { move_dir.x += 1.0; }

        if move_dir != Vec2::ZERO {
            // Find neighbor in direction
            let current_center = dungeon.room_centers[&current_node];
            let target_dir = move_dir.normalize();

            let mut best_neighbor = None;
            let mut max_dot = -1.0;

            for neighbor in dungeon.graph.neighbors(current_node) {
                let neighbor_center = dungeon.room_centers[&neighbor];
                let dir_to_neighbor = (neighbor_center - current_center).normalize_or_zero();
                let dot = dir_to_neighbor.dot(target_dir);

                if dot > max_dot {
                    max_dot = dot;
                    best_neighbor = Some(neighbor);
                }
            }

            if let Some(neighbor) = best_neighbor {
                // Directional threshold
                if max_dot > 0.5 {
                    current_node = neighbor;
                    player_pos = dungeon.room_centers[&current_node];
                }
            }
        }

        clear_background(BLACK);

        let aspect = screen_width() / screen_height();
        set_camera(&Camera2D {
            target: camera_pos,
            zoom: vec2(zoom_level / aspect, zoom_level),
            ..Default::default()
        });

        // Render Visible Tiles
        // Optimization: Cull tiles far from camera?
        // View radius ~ 1.0 / zoom_level
        let view_radius = 1.5 / zoom_level;

        for (idx, tri) in &dungeon.room_triangles {
            let center = dungeon.room_centers[idx];
            if center.distance(camera_pos) > view_radius {
                continue;
            }

            let color = match tri.kind {
                TriangleType::Acute => Color::new(0.2, 0.3, 0.5, 1.0), // Muted Blue
                TriangleType::Obtuse => Color::new(0.5, 0.2, 0.2, 1.0), // Muted Red
            };

            // Draw filled
            draw_triangle(tri.a, tri.b, tri.c, color);

            // Draw outline
            let line_color = Color::new(1.0, 1.0, 1.0, 0.2);
            draw_line(tri.a.x, tri.a.y, tri.b.x, tri.b.y, 0.5 / zoom_level * 0.05, line_color);
            draw_line(tri.b.x, tri.b.y, tri.c.x, tri.c.y, 0.5 / zoom_level * 0.05, line_color);
            draw_line(tri.c.x, tri.c.y, tri.a.x, tri.a.y, 0.5 / zoom_level * 0.05, line_color);
        }

        // Draw Player
        // Size relative to tiles? Tiles are ~1 unit.
        // If zoom_level is small, we see many tiles.
        draw_circle(player_pos.x, player_pos.y, 0.3, YELLOW);

        set_default_camera();
        draw_text("Penrose Dungeon", 20.0, 30.0, 30.0, WHITE);
        draw_text(&format!("Zoom: {:.2}", zoom_level), 20.0, 60.0, 20.0, GRAY);
        draw_text("Arrows/WASD to move", 20.0, 90.0, 20.0, GRAY);
        draw_text("Z/X or Wheel to zoom", 20.0, 110.0, 20.0, GRAY);

        next_frame().await
    }
}
