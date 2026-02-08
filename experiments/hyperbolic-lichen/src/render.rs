use macroquad::prelude::*;
use poincare_disk::{neighbor_transform_a, Geodesic, Mobius, Point, TilingConsts};
use std::f64::consts::PI;
use crate::lichen::LichenState;

pub fn to_screen(p: Point, scale: f32) -> Vec2 {
    let screen_center = vec2(screen_width() / 2.0, screen_height() / 2.0);
    vec2(
        screen_center.x + (p.re as f32) * scale,
        screen_center.y - (p.im as f32) * scale,
    )
}

pub fn draw_hyperbolic_line(p1: Point, p2: Point, color: Color, thickness: f32, scale: f32) {
    if (p1 - p2).norm() < 1e-4 {
        return;
    }

    let geo = Geodesic::new(p1, p2);

    match geo.euclidean_circle() {
        Some((center, radius)) => {
            let angle1 = (p1.im - center.im).atan2(p1.re - center.re);
            let angle2 = (p2.im - center.im).atan2(p2.re - center.re);

            let start = angle1;
            let end = angle2;
            let mut diff = end - start;
            while diff <= -PI {
                diff += 2.0 * PI;
            }
            while diff > PI {
                diff -= 2.0 * PI;
            }

            let segments = 20;
            for i in 0..segments {
                let t1 = (i as f64) / (segments as f64);
                let t2 = ((i + 1) as f64) / (segments as f64);

                let theta1 = start + diff * t1;
                let theta2 = start + diff * t2;

                let pt1 = Point::new(
                    center.re + radius * theta1.cos(),
                    center.im + radius * theta1.sin(),
                );
                let pt2 = Point::new(
                    center.re + radius * theta2.cos(),
                    center.im + radius * theta2.sin(),
                );

                draw_line(
                    to_screen(pt1, scale).x,
                    to_screen(pt1, scale).y,
                    to_screen(pt2, scale).x,
                    to_screen(pt2, scale).y,
                    thickness,
                    color,
                );
            }
        }
        None => {
            draw_line(
                to_screen(p1, scale).x,
                to_screen(p1, scale).y,
                to_screen(p2, scale).x,
                to_screen(p2, scale).y,
                thickness,
                color,
            );
        }
    }
}

pub fn draw_filled_polygon(vertices: &[Point], color: Color, scale: f32) {
     if vertices.len() < 3 { return; }
     let p0 = to_screen(vertices[0], scale);
     for i in 1..vertices.len()-1 {
         let p1 = to_screen(vertices[i], scale);
         let p2 = to_screen(vertices[i+1], scale);
         draw_triangle(p0, p1, p2, color);
     }
}

pub fn draw_tile_recursive(
    transform: Mobius,
    depth: u32,
    path_hash: u64,
    parent_hash: Option<u64>,
    tiling: &TilingConsts,
    incoming_edge: Option<usize>,
    lichen_state: &mut LichenState,
) {
    let scale = screen_height().min(screen_width()) * 0.45;

    let center = transform.apply(Point::new(0.0, 0.0));
    let v0 = transform.apply(Point::new(tiling.vertex_offset, 0.0));
    let size_approx = (v0 - center).norm();

    if size_approx * (scale as f64) < 3.0 {
        return;
    }
    if depth > 10 {
        return;
    }

    // Register Neighbors
    let mut neighbors = Vec::new();
    if let Some(p) = parent_hash {
        neighbors.push(p);
    }
    for i in 0..4 {
        if let Some(back) = incoming_edge {
            if i == back { continue; }
        }
        // Calculate child hash
        let mut child_hash = path_hash;
        child_hash = child_hash
            .wrapping_mul(6364136223846793005)
            .wrapping_add((i as u64) + 1);
        neighbors.push(child_hash);
    }
    lichen_state.register_node(path_hash, neighbors);

    let mut vertices = Vec::new();
    for i in 0..4 {
        let angle = (i as f64) * PI / 2.0 + PI / 4.0;
        let v_local = Point::from_polar(tiling.vertex_offset, angle);
        vertices.push(transform.apply(v_local));
    }

    // Determine Color from LichenState
    let color = lichen_state.get_color(path_hash);

    if color.a > 0.0 {
        draw_filled_polygon(&vertices, color, scale);
    }

    // Draw edges
    for i in 0..4 {
        draw_hyperbolic_line(vertices[i], vertices[(i + 1) % 4], Color::new(0.2, 0.2, 0.2, 0.5), 1.0, scale);
    }

    // Recurse
    for i in 0..4 {
        if let Some(back) = incoming_edge {
            if i == back {
                continue;
            }
        }

        let mut new_hash = path_hash;
        new_hash = new_hash
            .wrapping_mul(6364136223846793005)
            .wrapping_add((i as u64) + 1);

        let offset = neighbor_transform_a(i, tiling);
        let step = Mobius::translation(offset);
        let next_transform = transform.then(&step);
        let next_incoming = Some((i + 2) % 4);

        draw_tile_recursive(next_transform, depth + 1, new_hash, Some(path_hash), tiling, next_incoming, lichen_state);
    }
}
