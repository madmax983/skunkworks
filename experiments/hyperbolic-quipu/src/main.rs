mod fs;
mod layout;
mod tiling;

use fs::{get_repo_statuses, get_view_root};
use layout::{layout_quipu, QuipuNode};
use macroquad::prelude::*;
use poincare_disk::{mobius_add, mobius_sub, Point};
use quipu::Knot;

const DISK_SCALE: f32 = 0.45;

#[macroquad::main("Hyperbolic Quipu")]
async fn main() -> anyhow::Result<()> {
    // Current state
    let current_path = std::env::current_dir()?;
    let git_map = get_repo_statuses(&current_path);

    // Initial Layout
    let root_node = get_view_root(&current_path, 3, &git_map)?; // Depth 3 to limit size
    let quipu_root = layout_quipu(&root_node);

    // Navigation
    let mut view_center = Point::new(0.0, 0.0);
    let mut target_center = Point::new(0.0, 0.0);
    let mut is_dragging = false;
    let mut drag_locked_world_point = Point::new(0.0, 0.0);

    loop {
        clear_background(BLACK);
        let w = screen_width();
        let h = screen_height();
        let min_dim = w.min(h);
        let screen_center = Vec2::new(w / 2.0, h / 2.0);
        let disk_radius = min_dim * DISK_SCALE;

        // --- Input ---
        let (mx, my) = mouse_position();
        let dx = (mx - screen_center.x) / disk_radius;
        let dy = -(my - screen_center.y) / disk_radius;
        let mouse_z = Point::new(dx as f64, dy as f64);
        let is_mouse_in_disk = mouse_z.norm() < 1.0;

        if is_mouse_button_pressed(MouseButton::Left) && is_mouse_in_disk {
            is_dragging = true;
            drag_locked_world_point = mobius_add(mouse_z, view_center);
        }

        if is_mouse_button_down(MouseButton::Left) && is_dragging {
            target_center = mobius_sub(drag_locked_world_point, mouse_z);
            view_center = target_center;
        }

        if is_mouse_button_released(MouseButton::Left) {
            is_dragging = false;
        }

        // --- Animation ---
        let diff = target_center - view_center;
        if diff.norm() > 0.0001 {
            view_center = view_center + diff * 0.1;
        }

        // --- Drawing ---

        // Background Circle
        draw_circle(
            screen_center.x,
            screen_center.y,
            disk_radius,
            Color::new(0.05, 0.05, 0.05, 1.0),
        );

        // Grid
        tiling::draw_tiling(view_center, screen_center, disk_radius);

        // Border
        draw_circle_lines(screen_center.x, screen_center.y, disk_radius, 2.0, DARKGRAY);

        // Quipu
        let mut hover_info = None;
        draw_quipu_recursive(
            &quipu_root,
            view_center,
            screen_center,
            disk_radius,
            mouse_z,
            &mut hover_info,
        );

        // Hover Info
        if let Some((name, info, sx, sy)) = hover_info {
            let text = format!("{} ({})", name, info);
            let dims = measure_text(&text, None, 20, 1.0);
            draw_rectangle(
                sx + 10.0,
                sy - 25.0,
                dims.width + 10.0,
                30.0,
                Color::new(0.0, 0.0, 0.0, 0.8),
            );
            draw_text(&text, sx + 15.0, sy - 5.0, 20.0, WHITE);
        }

        // UI
        draw_text("Hyperbolic Quipu", 20.0, 30.0, 30.0, WHITE);
        draw_text(
            &format!("Path: {}", current_path.display()),
            20.0,
            60.0,
            20.0,
            GRAY,
        );
        draw_text("Drag to Pan", 20.0, h - 30.0, 20.0, DARKGRAY);

        next_frame().await
    }
}

fn draw_quipu_recursive(
    node: &QuipuNode,
    view_center: Point,
    screen_center: Vec2,
    disk_radius: f32,
    mouse_z: Point,
    hover_state: &mut Option<(String, String, f32, f32)>,
) {
    let z_start = mobius_sub(node.start_pos, view_center);
    let z_end = mobius_sub(node.end_pos, view_center);

    // Cull if completely off screen (both ends far away)
    // Note: z_start/z_end are in the unit disk.
    // If they are near edge (norm > 0.99), they are effectively invisible or at infinity.
    if z_start.norm_sqr() > 0.99 && z_end.norm_sqr() > 0.99 {
        return;
    }

    // Draw Main Cord (Geodesic from Start to End)
    draw_geodesic(z_start, z_end, screen_center, disk_radius, node.color);

    // Draw Knots along the cord
    let num_clusters = node.cord.clusters.len();
    if num_clusters > 0 {
        let m_diff = mobius_sub(node.end_pos, node.start_pos); // Vector from start to end in start's frame

        for (i, cluster) in node.cord.clusters.iter().enumerate() {
            if cluster.is_empty() {
                continue;
            }

            // Position t along the cord (0.0 to 1.0)
            // Reverse index (Unit at bottom/end, High power at top/start)
            // But usually Quipu: Top is High Power (Start), Bottom is Units (End).
            // So index 0 (Units) is near End (t=1.0).
            let t = 1.0 - (i as f64 / (num_clusters as f64 + 1.0));

            // Map to world
            let p_local = m_diff * t;
            let p_world = mobius_add(node.start_pos, p_local);
            let p_view = mobius_sub(p_world, view_center);

            // Draw Knot
            let pos_screen = to_screen(p_view, screen_center, disk_radius);

            // Draw Cluster Shape
            // Just a circle for now
            draw_circle(pos_screen.x, pos_screen.y, 3.0, WHITE);

            // Check Hover on Cluster
            let dist_mouse = (p_view - mouse_z).norm();
            // hit radius in disk space varies with zoom, but let's approximate
            if dist_mouse < 0.02 {
                *hover_state = Some((
                    node.name.clone(),
                    node.info.clone(),
                    pos_screen.x,
                    pos_screen.y,
                ));
                draw_circle_lines(pos_screen.x, pos_screen.y, 10.0, 1.0, YELLOW);
            }

            // Draw individual knots
            for (k_idx, knot) in cluster.iter().enumerate() {
                let offset = (k_idx as f32) * 4.0;
                match knot {
                    Knot::Simple => draw_circle(pos_screen.x + offset, pos_screen.y, 2.0, YELLOW),
                    Knot::Long(v) => draw_rectangle(
                        pos_screen.x + offset,
                        pos_screen.y - 2.0,
                        3.0,
                        4.0 + *v as f32,
                        BLUE,
                    ),
                    Knot::FigureEight => {
                        draw_circle_lines(pos_screen.x + offset, pos_screen.y, 3.0, 1.0, RED)
                    }
                }
            }
        }
    }

    // Recurse
    for child in &node.children {
        draw_quipu_recursive(
            child,
            view_center,
            screen_center,
            disk_radius,
            mouse_z,
            hover_state,
        );
    }
}

// Helper to draw geodesic arc
fn draw_geodesic(p1: Point, p2: Point, screen_center: Vec2, radius: f32, color: Color) {
    if (p1 - p2).norm_sqr() < 1e-6 {
        return;
    }
    let steps = 15;
    let m_diff = mobius_sub(p2, p1);

    let mut last_pos = to_screen(p1, screen_center, radius);

    for i in 1..=steps {
        let t = i as f64 / steps as f64;
        let q = m_diff * t;
        let world_pos = mobius_add(q, p1); // Map back to p1's frame -> View Frame

        let screen_pos = to_screen(world_pos, screen_center, radius);
        draw_line(
            last_pos.x,
            last_pos.y,
            screen_pos.x,
            screen_pos.y,
            2.0,
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
