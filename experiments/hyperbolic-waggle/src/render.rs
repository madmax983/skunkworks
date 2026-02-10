use macroquad::prelude::*;
use poincare_disk::{neighbor_transform_a, Geodesic, Mobius, Point, TilingConsts};
use std::f64::consts::PI;

pub fn to_screen(p: Point, scale: f32) -> Vec2 {
    let screen_center = vec2(screen_width() / 2.0, screen_height() / 2.0);
    vec2(
        screen_center.x + (p.re as f32) * scale,
        screen_center.y - (p.im as f32) * scale,
    )
}

pub fn to_hyperbolic(screen_pos: Vec2, scale: f32) -> Point {
    let screen_center = vec2(screen_width() / 2.0, screen_height() / 2.0);
    let x = (screen_pos.x - screen_center.x) / scale;
    let y = -(screen_pos.y - screen_center.y) / scale;
    Point::new(x as f64, y as f64)
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

pub fn draw_tile(
    transform: Mobius,
    depth: u32,
    tiling: &TilingConsts,
    incoming_edge: Option<usize>,
) {
    let scale = screen_height().min(screen_width()) * 0.45;

    let center = transform.apply(Point::new(0.0, 0.0));
    let v0 = transform.apply(Point::new(tiling.vertex_offset, 0.0));
    let size_approx = (v0 - center).norm();

    if size_approx * (scale as f64) < 3.0 {
        return;
    }
    if depth > 8 {
        return;
    }

    let mut vertices = Vec::new();
    for i in 0..4 {
        let angle = (i as f64) * PI / 2.0 + PI / 4.0;
        let v_local = Point::from_polar(tiling.vertex_offset, angle);
        vertices.push(transform.apply(v_local));
    }

    // Draw edges
    for i in 0..4 {
        let color = Color::new(0.2, 0.2, 0.2, 0.3); // Faint grid
        draw_hyperbolic_line(vertices[i], vertices[(i + 1) % 4], color, 1.0, scale);
    }

    // Recurse
    for i in 0..4 {
        if let Some(back) = incoming_edge {
            if i == back {
                continue;
            }
        }

        let offset = neighbor_transform_a(i, tiling);
        let step = Mobius::translation(offset);
        let next_transform = transform.then(&step);
        let next_incoming = Some((i + 2) % 4);

        draw_tile(next_transform, depth + 1, tiling, next_incoming);
    }
}

pub fn draw_bee(pos: Point, color: Color, scale: f32) {
    let screen_pos = to_screen(pos, scale);
    let r2 = pos.norm_sqr();
    let visual_scale = 1.0 - r2;
    let radius = 5.0 * visual_scale as f32; // Base radius 5.0

    draw_circle(screen_pos.x, screen_pos.y, radius.max(1.0), color);
}

pub fn draw_source(pos: Point, quality: f64, scale: f32) {
    let screen_pos = to_screen(pos, scale);
    let r2 = pos.norm_sqr();
    let visual_scale = 1.0 - r2;
    let radius = 10.0 * (0.5 + quality) as f32 * visual_scale as f32;

    draw_circle(screen_pos.x, screen_pos.y, radius.max(2.0), Color::new(0.2, 0.8, 0.2, 0.8));
    draw_circle_lines(screen_pos.x, screen_pos.y, radius.max(2.0), 1.0, GREEN);
}
