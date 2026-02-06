use macroquad::prelude::*;
use num_complex::Complex;
use poincare_disk::{Mobius, Point};
use crate::fs_map::{FsCache, RoomId};
use crate::tiling::Tiler;
use std::collections::HashSet;

pub struct Renderer {
    pub tiler: Tiler,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            tiler: Tiler::new(),
        }
    }

    pub fn draw_scene(
        &mut self,
        fs_cache: &mut FsCache,
        root_id: &RoomId,
        view_transform: Mobius,
        screen_size: Vec2,
    ) {
        let min_dim = screen_size.x.min(screen_size.y);
        let disk_radius = min_dim * 0.45;
        let screen_center = screen_size * 0.5;

        // Draw Disk Background
        draw_circle(screen_center.x, screen_center.y, disk_radius, BLACK);

        // Recursive Draw
        let mut visited = HashSet::new();
        self.draw_room_recursive(
            fs_cache,
            root_id,
            view_transform,
            screen_center,
            disk_radius,
            0,
            &mut visited
        );

        // Draw Boundary over everything
        draw_circle_lines(screen_center.x, screen_center.y, disk_radius, 3.0, WHITE);
    }

    fn draw_room_recursive(
        &self,
        fs_cache: &mut FsCache,
        room_id: &RoomId,
        transform: Mobius,
        screen_center: Vec2,
        disk_radius: f32,
        depth: usize,
        visited: &mut HashSet<RoomId>,
    ) {
        // Culling: Check center point
        let center_pos = transform.apply(Point::new(0.0, 0.0));
        let dist_sq = center_pos.norm_sqr();
        if dist_sq > 0.99 {
            return;
        }

        // Depth limit
        if depth > 5 {
            return;
        }

        // Avoid cycles
        if !visited.insert(room_id.clone()) {
            return;
        }

        // Get Room Data
        let room = match fs_cache.get_room(room_id) {
            Ok(r) => r.clone(),
            Err(_) => return,
        };

        // Draw Polygon Vertices
        let vertex_dist = self.tiler.consts.vertex_offset;
        let mut vertices = Vec::new();
        for i in 0..4 {
             // 45, 135, 225, 315
             let angle = (i as f64 * 90.0 + 45.0) * std::f64::consts::PI / 180.0;
             let p_local = Complex::from_polar(vertex_dist, angle);
             let p_screen = to_screen(transform.apply(p_local), screen_center, disk_radius);
             vertices.push(p_screen);
        }

        // Draw Quad
        let color = if room.is_dir {
            Color::new(0.1, 0.1, 0.3, 0.6)
        } else {
            Color::new(0.1, 0.3, 0.1, 0.6)
        };

        draw_triangle(vertices[0], vertices[1], vertices[2], color);
        draw_triangle(vertices[0], vertices[2], vertices[3], color);

        // Draw Outlines
        let line_color = Color::new(0.5, 0.5, 0.5, 0.5);
        for i in 0..4 {
            let next = (i + 1) % 4;
            draw_line(vertices[i].x, vertices[i].y, vertices[next].x, vertices[next].y, 2.0, line_color);
        }

        // Draw Name
        let center_screen = to_screen(center_pos, screen_center, disk_radius);
        let scale = (1.0 - dist_sq).sqrt() * 1.5; // Scale based on disk compression
        if scale > 0.1 {
             let font_size = (30.0 * scale) as u16;
             if font_size > 5 {
                let text = &room.name;
                let dims = measure_text(text, None, font_size, 1.0);
                draw_text(
                    text,
                    center_screen.x - dims.width / 2.0,
                    center_screen.y + dims.height / 2.0,
                    font_size as f32,
                    WHITE
                );
             }
        }

        // Recurse
        for (dir, neighbor_opt) in room.neighbors.iter().enumerate() {
            if let Some(neighbor_id) = neighbor_opt {
                let neighbor_trans = self.tiler.get_neighbor_transform(dir);
                let child_transform = transform.then(&neighbor_trans);

                self.draw_room_recursive(
                    fs_cache,
                    neighbor_id,
                    child_transform,
                    screen_center,
                    disk_radius,
                    depth + 1,
                    visited
                );
            }
        }
    }
}

pub fn to_screen(p: Point, center: Vec2, radius: f32) -> Vec2 {
    Vec2::new(
        center.x + p.re as f32 * radius,
        center.y - p.im as f32 * radius,
    )
}
