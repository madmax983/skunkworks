mod fs;
mod hyperbolic;
mod layout;

use fs::scan_dir;
use hyperbolic::{inverse_mobius_transform, mobius_transform, Point};
use layout::{layout_tree, LayoutNode};
use macroquad::prelude::*;
use num_complex::Complex;

const DISK_SCALE: f32 = 0.45;

#[macroquad::main("Hyperbolic Finder")]
async fn main() -> anyhow::Result<()> {
    let root_path = std::env::current_dir()?;
    let fs_root = scan_dir(root_path, 4)?;
    let layout_root = layout_tree(fs_root);

    let mut view_center = Complex::new(0.0, 0.0);
    let mut target_center = Complex::new(0.0, 0.0);

    loop {
        clear_background(BLACK);

        let w = screen_width();
        let h = screen_height();
        let min_dim = w.min(h);
        let screen_center = Vec2::new(w / 2.0, h / 2.0);
        let disk_radius = min_dim * DISK_SCALE;

        // Input Handling
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mx, my) = mouse_position();
            let _m_vec = Vec2::new(mx, my);

            // Map mouse to disk coords
            let dx = (mx - screen_center.x) / disk_radius;
            let dy = -(my - screen_center.y) / disk_radius; // Flip Y back
            let mouse_z = Complex::new(dx as f64, dy as f64);

            if mouse_z.norm() < 1.0 {
                // Find clicked node
                if let Some(clicked_node_pos) = find_closest_node(
                    &layout_root,
                    view_center,
                    mouse_z,
                    0.1, // hit radius
                ) {
                    target_center = clicked_node_pos;
                }
            }
        }

        // Navigation: Back
        if is_key_pressed(KeyCode::Backspace) {
            // Logic to go up?
            // Without explicit parent links in layout, hard to know "up".
            // But we can just move towards 0,0 (Root).
            target_center = Complex::new(0.0, 0.0);
        }

        // Animation
        let diff = target_center - view_center;
        if diff.norm() > 0.001 {
            view_center = view_center + diff * 0.1;
        }

        // Draw Disk
        draw_circle(screen_center.x, screen_center.y, disk_radius, WHITE);
        draw_circle_lines(screen_center.x, screen_center.y, disk_radius, 2.0, DARKGRAY);

        draw_node_recursive(&layout_root, view_center, screen_center, disk_radius);

        draw_text("Hyperbolic File System", 20.0, 30.0, 30.0, BLACK);
        draw_text(
            &format!("Center: {:.2}", view_center),
            20.0,
            60.0,
            20.0,
            GRAY,
        );
        draw_text(
            "Left Click: Navigate | Backspace: Reset to Root",
            20.0,
            h - 20.0,
            20.0,
            DARKGRAY,
        );

        next_frame().await
    }
}

fn find_closest_node(
    node: &LayoutNode,
    view_center: Point,
    click_z: Point, // point in transformed space (screen)
    hit_radius: f64,
) -> Option<Point> {
    let z_prime = mobius_transform(node.pos, view_center);
    let dist = (z_prime - click_z).norm();

    let mut best_match = None;
    let mut min_dist = hit_radius;

    if dist < min_dist {
        best_match = Some(node.pos);
        min_dist = dist;
    }

    // Check children
    if z_prime.norm() < 0.95 {
        // Optimization: don't check deep children if parent is far
        for child in &node.children {
            if let Some(match_pos) = find_closest_node(child, view_center, click_z, min_dist) {
                // We found a better match in children
                // Re-evaluate distance to be sure?
                // The recursive call uses the updated min_dist, so it only returns if better.
                let child_prime = mobius_transform(match_pos, view_center);
                let child_dist = (child_prime - click_z).norm();
                min_dist = child_dist;
                best_match = Some(match_pos);
            }
        }
    }

    best_match
}

fn draw_node_recursive(
    node: &LayoutNode,
    view_center: Point,
    screen_center: Vec2,
    disk_radius: f32,
) {
    let z_prime = mobius_transform(node.pos, view_center);
    if z_prime.norm_sqr() > 1.001 {
        return;
    }

    let screen_pos = to_screen(z_prime, screen_center, disk_radius);

    for child in &node.children {
        let child_z_prime = mobius_transform(child.pos, view_center);
        draw_geodesic(
            z_prime,
            child_z_prime,
            screen_center,
            disk_radius,
            Color::new(0.5, 0.5, 0.5, 0.5),
        );
        draw_node_recursive(child, view_center, screen_center, disk_radius);
    }

    let scale = 1.0 - z_prime.norm_sqr();
    let r = 5.0 * scale as f32 + 2.0;

    let color = if node.node.is_dir { BLUE } else { GREEN };
    draw_circle(screen_pos.x, screen_pos.y, r, color);

    if scale > 0.05 {
        draw_text(
            &node.node.name,
            screen_pos.x + r,
            screen_pos.y,
            15.0 * scale as f32,
            BLACK,
        );
    }
}

fn draw_geodesic(p1: Point, p2: Point, screen_center: Vec2, radius: f32, color: Color) {
    let steps = 10;
    let m_p2 = mobius_transform(p2, p1);

    let mut last_pos = to_screen(p1, screen_center, radius);

    for i in 1..=steps {
        let t = i as f64 / steps as f64;
        let q = m_p2 * t;
        let world_pos = inverse_mobius_transform(q, p1);
        let screen_pos = to_screen(world_pos, screen_center, radius);
        draw_line(
            last_pos.x,
            last_pos.y,
            screen_pos.x,
            screen_pos.y,
            1.0,
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
