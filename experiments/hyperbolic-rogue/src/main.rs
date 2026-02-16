use macroquad::prelude::*;
use num_complex::Complex;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

mod geometry;
mod tiling;

use geometry::{Mobius, Point};
use tiling::{Polygon, generate_7_3_tiling};

fn calculate_id(point: Point) -> u64 {
    let mut hasher = DefaultHasher::new();
    // Quantize to avoid floating point jitter
    let x = (point.re * 1000.0).round() as i64;
    let y = (point.im * 1000.0).round() as i64;
    x.hash(&mut hasher);
    y.hash(&mut hasher);
    hasher.finish()
}

fn get_room_color(id: u64) -> Color {
    use ::rand::{Rng, SeedableRng};
    let mut rng = ::rand::rngs::StdRng::seed_from_u64(id);

    // Generate a pleasant color
    let h = rng.gen_range(0.0..1.0);
    let s = rng.gen_range(0.4..0.8);
    let l = rng.gen_range(0.2..0.6);

    // Simple HSL to RGB
    hsl_to_rgb(h, s, l)
}

fn hsl_to_rgb(h: f32, s: f32, l: f32) -> Color {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h * 6.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;

    let (r, g, b) = if h < 1.0 / 6.0 {
        (c, x, 0.0)
    } else if h < 2.0 / 6.0 {
        (x, c, 0.0)
    } else if h < 3.0 / 6.0 {
        (0.0, c, x)
    } else if h < 4.0 / 6.0 {
        (0.0, x, c)
    } else if h < 5.0 / 6.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    Color::new(r + m, g + m, b + m, 1.0)
}

#[macroquad::main("Hyperbolic Rogue")]
async fn main() {
    // Generate local template tiling
    // Depth 4 is enough to cover the screen and a bit more
    let local_polygons = generate_7_3_tiling(4);

    // Find neighbors of the central tile for navigation
    // The central tile is the one with id corresponding to (0,0) or transform identity
    // In our generation, the first one is usually the center, but let's be robust.
    // The generators were used in tiling.rs but not exposed.
    // We can infer them from the neighbors of the central tile.
    // Neighbors of center are those at depth 1.
    // We can extract their transforms.

    // Extract generators from depth 1 polygons
    // We need to filter polygons where transform is a generator.
    // Or just re-calculate them here? simpler to re-calc or just find depth 1.
    // Actually, tiling.rs doesn't expose depth.
    // But we know `local_polygons` contains them.
    // Let's identify the neighbors by distance.
    // Center is at 0. Neighbors are at `2 * a_eucl` distance? No.
    // Let's just look at their centers.

    let center_poly = local_polygons
        .iter()
        .find(|p| p.transform.apply(Point::new(0.0, 0.0)).norm() < 0.001)
        .unwrap();

    // Neighbors
    // Sort by distance from origin
    let mut sorted_polys: Vec<&Polygon> = local_polygons.iter().collect();
    sorted_polys.sort_by(|a, b| {
        let da = a.transform.apply(Point::new(0.0, 0.0)).norm_sqr();
        let db = b.transform.apply(Point::new(0.0, 0.0)).norm_sqr();
        da.partial_cmp(&db).unwrap()
    });

    // Skip the first one (center, dist ~ 0) and take next 7
    let neighbors: Vec<Mobius> = sorted_polys
        .iter()
        .skip(1)
        .take(7)
        .map(|p| p.transform)
        .collect();

    // Sort neighbors by angle to be consistent? Not strictly necessary for this logic.

    let mut current_world_transform = Mobius::identity();
    let mut player_pos = Point::new(0.0, 0.0);

    // Player speed
    let speed = 0.5;

    loop {
        let dt = get_frame_time();

        // Input - Move player
        // Movement is hyperbolic translation in the direction of keys
        let mut move_vec = Point::new(0.0, 0.0);
        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
            move_vec.im += 1.0;
        }
        if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            move_vec.im -= 1.0;
        }
        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            move_vec.re += 1.0;
        }
        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            move_vec.re -= 1.0;
        }

        if move_vec.norm_sqr() > 0.001 {
            move_vec = move_vec / move_vec.norm();
            // Amount to move: speed * dt
            // In Poincaré disk, Euclidean movement distance depends on r.
            // But let's apply a small hyperbolic translation.
            // Translation by delta:
            let delta = move_vec * (speed * dt as f64);
            // We want to apply this translation to `player_pos`.
            // The isometry T that maps 0 to `player_pos`.
            // We want new pos P' = T(delta)?
            // Yes, moving "forward" from P means applying translation in the frame of P.

            // Frame at P:
            let t_p = Mobius::translation(player_pos);
            // Apply delta in local frame:
            // new_pos = t_p.apply(delta);
            player_pos = t_p.apply(delta);
        }

        // Check if we need to re-center
        // If player is closer to a neighbor's center than to origin.
        let mut best_dist = player_pos.norm_sqr(); // Dist to 0 (Euclidean comparison works for "closest tile")
        // Actually hyperbolic distance preserves ordering of Euclidean distance from origin?
        // Yes, radial monotonicity.
        let mut best_neighbor_idx = -1;

        for (i, neighbor_transform) in neighbors.iter().enumerate() {
            let center = neighbor_transform.apply(Point::new(0.0, 0.0));
            // Euclidean distance check
            let dist = (player_pos - center).norm_sqr();
            if dist < best_dist {
                best_dist = dist;
                best_neighbor_idx = i as i32;
            }
        }

        if best_neighbor_idx != -1 {
            // Re-center!
            let neighbor = neighbors[best_neighbor_idx as usize];
            let neighbor_inv = neighbor.inverse();

            // Update World Transform: We moved into `neighbor`.
            // So the new center is `neighbor` relative to old center.
            current_world_transform = current_world_transform.compose(neighbor);

            // Update Player Pos: Map back to near 0.
            // P_new = neighbor_inv(P_old)
            player_pos = neighbor_inv.apply(player_pos);
        }

        // Render
        clear_background(BLACK);

        // View Matrix: Maps player_pos to (0,0) (or screen center)
        // Wait, we want player to be at screen center.
        // So world points W map to screen S via:
        // S = T_player_inv(W)
        let view_matrix = Mobius::translation(player_pos).inverse();

        // Screen dimensions
        let w = screen_width();
        let h = screen_height();
        let scale = h.min(w) * 0.45; // Radius of disk
        let offset_x = w / 2.0;
        let offset_y = h / 2.0;

        // Draw Tiling
        for poly in &local_polygons {
            // Calculate absolute ID
            let real_transform = current_world_transform.compose(poly.transform);
            let real_center = real_transform.apply(Point::new(0.0, 0.0));
            let id = calculate_id(real_center);

            let color = get_room_color(id);

            // Transform vertices to screen space
            let mut screen_verts = Vec::new();
            for v in &poly.vertices {
                let local_v = view_matrix.apply(*v);
                screen_verts.push(local_v);
            }

            // Draw Polygon
            // Macroquad draw_poly or draw_triangle_fan
            // We'll use draw_triangle_fan for filled, draw_line_loop for outline

            // Convert to screen coords
            let screen_coords: Vec<(f32, f32)> = screen_verts
                .iter()
                .map(|p| {
                    (
                        offset_x + (p.re as f32) * scale,
                        offset_y - (p.im as f32) * scale, // Flip Y
                    )
                })
                .collect();

            // Draw filled
            // Split into triangles (fan from center of poly?)
            // We can compute center of poly in screen space
            let poly_center = view_matrix.apply(poly.transform.apply(Point::new(0.0, 0.0)));
            let center_screen = (
                offset_x + (poly_center.re as f32) * scale,
                offset_y - (poly_center.im as f32) * scale,
            );

            for i in 0..screen_coords.len() {
                let p1 = screen_coords[i];
                let p2 = screen_coords[(i + 1) % screen_coords.len()];

                draw_triangle(
                    vec2(center_screen.0, center_screen.1),
                    vec2(p1.0, p1.1),
                    vec2(p2.0, p2.1),
                    Color::new(color.r, color.g, color.b, 0.5),
                );
            }

            // Draw outline
            for i in 0..screen_coords.len() {
                let p1 = screen_coords[i];
                let p2 = screen_coords[(i + 1) % screen_coords.len()];
                draw_line(p1.0, p1.1, p2.0, p2.1, 2.0, WHITE);
            }
        }

        // Draw Player (at center)
        draw_circle(offset_x, offset_y, 5.0, RED);

        // Draw Boundary
        draw_circle_lines(offset_x, offset_y, scale, 2.0, GRAY);

        // UI
        draw_text("Hyperbolic Rogue", 20.0, 30.0, 30.0, WHITE);
        draw_text(
            &format!("Pos: {:.2}, {:.2}", player_pos.re, player_pos.im),
            20.0,
            60.0,
            20.0,
            LIGHTGRAY,
        );
        draw_text(
            &format!(
                "ID: {:x}",
                calculate_id(current_world_transform.apply(Point::new(0.0, 0.0)))
            ),
            20.0,
            80.0,
            20.0,
            LIGHTGRAY,
        );

        next_frame().await
    }
}
