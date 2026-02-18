use macroquad::prelude::*;
use poincare_disk::{mobius_add, mobius_sub, neighbor_transform_a, Mobius, Point, TilingConsts};
use std::collections::{HashSet, VecDeque};
use std::f64::consts::PI;

// Helper to hash points approximately for visited set
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct GridPoint {
    x: i64,
    y: i64,
}

impl GridPoint {
    fn from_point(p: Point) -> Self {
        // Discretize heavily to merge close points
        Self {
            x: (p.re * 1000.0) as i64,
            y: (p.im * 1000.0) as i64,
        }
    }
}

pub fn draw_tiling(view_center: Point, screen_center: Vec2, disk_radius: f32) {
    let consts = TilingConsts::new_4_5();

    // Generators for the 4 neighbors
    // These are translations in the local frame of the tile
    let generators: Vec<Mobius> = (0..4)
        .map(|i| {
            let offset = neighbor_transform_a(i, &consts);
            Mobius::translation(offset)
        })
        .collect();

    // BFS State
    let mut queue: VecDeque<(Mobius, usize)> = VecDeque::new();
    let mut visited: HashSet<GridPoint> = HashSet::new();

    // Start with Identity (Center Tile)
    // But if we have moved view, we might want to seed with a tile closer to view?
    // For now, start at Origin.
    // Optimization: If view_center is far, we should pathfind to it first?
    // Let's assume user pans from origin.
    let root = Mobius::identity();
    queue.push_back((root, 0));
    visited.insert(GridPoint::from_point(Point::new(0.0, 0.0)));

    let max_tiles = 300; // Limit to keep FPS high
    let mut count = 0;

    // To prevent infinite expansion in wrong direction, prioritize tiles close to view_center
    // We can't easily sort the queue, but we can cull.

    while let Some((m, depth)) = queue.pop_front() {
        if count > max_tiles {
            break;
        }

        let center = m.apply(Point::new(0.0, 0.0));

        // Transform center to View Space (relative to view_center)
        let view_pos = mobius_sub(center, view_center);
        let dist_sq = view_pos.norm_sqr();

        // Cull if center is too close to boundary (invisible)
        if dist_sq > 0.98 {
            continue;
        }

        // Draw edges for this tile
        // A square has 4 edges. We can draw lines to neighbors.
        // Or we can draw the square itself.
        // Vertices of the square in local frame:
        // They are at `vertex_offset` distance, at angles 45, 135, 225, 315 (pi/4 + k*pi/2).
        // Let's draw the edges connecting vertices.
        draw_tile_edges(&m, &consts, view_center, screen_center, disk_radius);

        // Expand to neighbors
        for (_i, gen) in generators.iter().enumerate() {
            // New transform: M_next = M * Gen
            let m_next = m.then(gen);
            let center_next = m_next.apply(Point::new(0.0, 0.0));

            // Check if visited
            if visited.insert(GridPoint::from_point(center_next)) {
                // Heuristic: only add if not too far from view_center?
                // Or if depth is low?
                // If we cull based on view distance, we naturally expand towards view?
                // No, BFS expands radially from origin.
                // If view is far, we waste time expanding near origin.
                // But this is a simple "Finder" app, likely staying near origin or panning slowly.
                // Just limiting max_tiles is fine for now.

                // Allow deep exploration if it's towards the view
                let next_view_pos = mobius_sub(center_next, view_center);
                if next_view_pos.norm() < 0.95 || depth < 5 {
                    queue.push_back((m_next, depth + 1));
                    count += 1;
                }
            }
        }
    }
}

fn draw_tile_edges(
    m: &Mobius,
    consts: &TilingConsts,
    view_center: Point,
    screen_center: Vec2,
    disk_radius: f32,
) {
    // Vertices of the square in Local Frame
    // 4 vertices at angles pi/4, 3pi/4, ...
    // Distance = consts.vertex_offset

    let mut local_vertices = [Point::default(); 4];
    for i in 0..4 {
        let angle = (i as f64 * 2.0 * PI / 4.0) + (PI / 4.0);
        use num_complex::Complex;
        local_vertices[i] = Complex::from_polar(consts.vertex_offset, angle);
    }

    // Transform vertices to World Space, then to Screen Space
    let world_vertices: Vec<Point> = local_vertices.iter().map(|&p| m.apply(p)).collect();

    for i in 0..4 {
        let p1 = world_vertices[i];
        let p2 = world_vertices[(i + 1) % 4];

        // Map to view
        let v1 = mobius_sub(p1, view_center);
        let v2 = mobius_sub(p2, view_center);

        // Cull if both far
        if v1.norm_sqr() > 0.99 && v2.norm_sqr() > 0.99 {
            continue;
        }

        draw_geodesic(
            v1,
            v2,
            screen_center,
            disk_radius,
            Color::new(0.3, 0.3, 0.3, 0.3), // Faint grid
        );
    }
}

fn draw_geodesic(p1: Point, p2: Point, screen_center: Vec2, radius: f32, color: Color) {
    if (p1 - p2).norm_sqr() < 1e-6 {
        return;
    }

    let steps = 10;
    // Interpolate in hyperbolic space
    // Since we don't have a direct "geodesic lerp" function exposed easily,
    // we map p2 to p1's frame, lerp linearly (which is a geodesic through origin), then map back.
    let m_p2 = mobius_sub(p2, p1);

    // Safety check
    if m_p2.norm() > 0.999 {
        return;
    }

    let mut last_pos = to_screen(p1, screen_center, radius);

    for i in 1..=steps {
        let t = i as f64 / steps as f64;
        let q = m_p2 * t;
        let world_pos = mobius_add(q, p1);

        let screen_pos = to_screen(world_pos, screen_center, radius);
        draw_line(
            last_pos.x,
            last_pos.y,
            screen_pos.x,
            screen_pos.y,
            1.5,
            color,
        );
        last_pos = screen_pos;
    }
}

fn to_screen(p: Point, center: Vec2, radius: f32) -> Vec2 {
    Vec2::new(
        center.x + p.re as f32 * radius,
        center.y - p.im as f32 * radius,
    )
}
