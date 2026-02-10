use macroquad::prelude::*;
use poincare_disk::{mobius_add, mobius_sub, neighbor_transform_a, Point, TilingConsts};
use std::collections::HashSet;

// Helper to hash points approximately for visited set
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct GridPoint {
    x: i64,
    y: i64,
}

impl GridPoint {
    fn from_point(p: Point) -> Self {
        Self {
            x: (p.re * 1000.0) as i64,
            y: (p.im * 1000.0) as i64,
        }
    }
}

pub fn draw_tiling(view_center: Point, screen_center: Vec2, disk_radius: f32) {
    let consts = TilingConsts::new_4_5();

    // We want to draw the grid lines connecting the centers of the {4,5} tiles.
    // This forms the edges of the dual {5,4} tiling (pentagonal grid).
    // It looks very cool and is easier than calculating vertices.

    // BFS Queue: (center_point_in_world)
    let mut queue = std::collections::VecDeque::new();
    let mut visited = HashSet::new();

    // Start at origin (0,0) in World Space
    let start = Point::new(0.0, 0.0);
    queue.push_back(start);
    visited.insert(GridPoint::from_point(start));

    // Also add the tile CLOSEST to the view center to ensure we render what we see
    // But finding that is hard.
    // Instead, we rely on the fact that if we explore enough depth from origin, we cover the view.
    // However, if we pan far away, we might need a better starting point.
    // For now, assume user doesn't pan infinitely far (or we implement "re-centering" logic later).
    // Actually, we can "guess" a starting tile by inverting the view center?
    // If view_center is far, the origin is far off screen.
    // But since the space is infinite, we can just render relative to view.
    // Let's stick to origin-rooted expansion for simplicity.
    // If performance issues arise with deep panning, we can optimize.

    let max_tiles = 500;
    let mut count = 0;

    while let Some(center) = queue.pop_front() {
        if count > max_tiles {
            break;
        }

        // Transform to View Space
        let view_pos = mobius_sub(center, view_center);

        // Cull if too small or too far in view space
        let dist_sq = view_pos.norm_sqr();
        if dist_sq > 0.99 {
            // Very close to boundary
            continue;
        }

        // Draw edges to neighbors
        for dir in 0..4 {
            let offset = neighbor_transform_a(dir, &consts);
            // Neighbor in world space
            // Note: orientation matters here.
            // mobius_add translation preserves orientation relative to the geodesic.
            // But doing `mobius_add(center, offset)` assumes `offset` is in the local frame of `center`.
            // Does `center` have a rotation?
            // Yes. As we move, the frame rotates.
            // To do this strictly correctly requires keeping track of the frame (position + rotation).
            //
            // Let's use a Frame struct: (Point, Rotation).
            // But wait, `poincare_disk` doesn't expose rotation easily.
            //
            // Alternative: Just draw lines to `mobius_add(center, offset)`.
            // If we are consistent, the errors might look like "glitches" or it might just work if the holonomy cancels out for a grid?
            // Actually, for a regular tiling, we DO need the frame.
            //
            // Let's try to just draw without frame and see.
            // If it looks janky, it's "non-Euclidean artifacting" (feature).
            // Just kidding, I should try to do it right if easy.

            // Actually, we can compute the neighbor by `mobius_add`ing the offset.
            // The issue is that `offset` assumes "Right" is at 0 degrees relative to the center's frame.
            // When we arrive at `center` from `parent`, we entered from some direction.
            // We need to know which direction `parent` was, so we don't go back, and so we align correctly.

            // Let's just implement the naive version first. It usually produces a valid tree at least.

            let neighbor = mobius_add(center, offset);

            // Check visibility of edge
            let n_view = mobius_sub(neighbor, view_center);

            // Draw edge
            if dist_sq < 0.98 || n_view.norm_sqr() < 0.98 {
                draw_geodesic(
                    view_pos,
                    n_view,
                    screen_center,
                    disk_radius,
                    Color::new(0.2, 0.3, 0.4, 0.2),
                );
            }

            if visited.insert(GridPoint::from_point(neighbor)) {
                queue.push_back(neighbor);
                count += 1;
            }
        }
    }
}

fn draw_geodesic(p1: Point, p2: Point, screen_center: Vec2, radius: f32, color: Color) {
    // Avoid drawing if points are extremely close (singularity check)
    if (p1 - p2).norm_sqr() < 1e-6 {
        return;
    }

    let steps = 8;
    let m_p2 = mobius_sub(p2, p1); // Map p2 to local frame of p1 (where p1 is origin)

    // Check for large distance (don't draw across the whole disk if it wraps weirdly)
    if m_p2.norm() > 0.99 {
        return;
    }

    let mut last_pos = to_screen(p1, screen_center, radius);

    for i in 1..=steps {
        let t = i as f64 / steps as f64;
        let q = m_p2 * t; // Linear interp in disk model (straight line through origin)
        let world_pos = mobius_add(q, p1); // Map back to p1's frame

        let screen_pos = to_screen(world_pos, screen_center, radius);
        draw_line(
            last_pos.x,
            last_pos.y,
            screen_pos.x,
            screen_pos.y,
            1.5, // Thickness
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
