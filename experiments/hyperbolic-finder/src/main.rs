mod fs;
mod layout;

use fs::{scan_dir, FileType};
use layout::{layout_tree, LayoutNode};
use macroquad::prelude::*;
use poincare_disk::{mobius_add, mobius_sub, Point};

const DISK_SCALE: f32 = 0.45;

#[macroquad::main("Hyperbolic Finder")]
async fn main() -> anyhow::Result<()> {
    // Scan current directory
    let root_path = std::env::current_dir()?;
    let fs_root = scan_dir(root_path, 5)?;
    let layout_root = layout_tree(fs_root);

    // Navigation State
    let mut view_center = Point::new(0.0, 0.0);
    let mut target_center = Point::new(0.0, 0.0);

    // Dragging State
    let mut is_dragging = false;
    let mut drag_start_mouse = Point::new(0.0, 0.0);
    let mut drag_locked_world_point = Point::new(0.0, 0.0);

    // Hover State
    // let mut hovered_node: Option<(String, String, f32)> = None; // Moved inside loop

    loop {
        let mut hovered_node: Option<(String, String, f32)> = None; // Name, Info, Screen Y
        clear_background(BLACK);

        let w = screen_width();
        let h = screen_height();
        let min_dim = w.min(h);
        let screen_center = Vec2::new(w / 2.0, h / 2.0);
        let disk_radius = min_dim * DISK_SCALE;

        // --- Input Handling ---
        let (mx, my) = mouse_position();

        // Map mouse to disk coords (y-up for math, y-down for screen)
        // Screen: (x, y) -> Disk: (x - cx, -(y - cy)) / R
        let dx = (mx - screen_center.x) / disk_radius;
        let dy = -(my - screen_center.y) / disk_radius;
        let mouse_z = Point::new(dx as f64, dy as f64);
        let is_mouse_in_disk = mouse_z.norm() < 1.0;

        // Panning (Dragging)
        if is_mouse_button_pressed(MouseButton::Left) && is_mouse_in_disk {
            is_dragging = true;
            drag_start_mouse = mouse_z;
            // The world point currently under the mouse is P = mobius_add(mouse_z, view_center)
            // Wait, standard convention: view transform maps World -> Screen (Disk).
            // T(p) = mobius_sub(p, view_center).
            // So p = mobius_add(disk_point, view_center). Correct.
            drag_locked_world_point = mobius_add(mouse_z, view_center);
        }

        if is_mouse_button_down(MouseButton::Left) && is_dragging {
            // We want new_view_center such that drag_locked_world_point maps to mouse_z (current)
            // T(P) = mouse_z  => mobius_sub(P, C) = mouse_z
            // => C = mobius_sub(P, mouse_z)
            target_center = mobius_sub(drag_locked_world_point, mouse_z);
            view_center = target_center; // Instant update for responsiveness
        }

        if is_mouse_button_released(MouseButton::Left) {
            is_dragging = false;
            // On release, check if it was a click (short drag)
            if (mouse_z - drag_start_mouse).norm() < 0.02 && is_mouse_in_disk {
                // It was a click! Navigate to node.
                if let Some(clicked_node_pos) = find_closest_node(
                    &layout_root,
                    view_center,
                    mouse_z,
                    0.05, // hit radius
                ) {
                    target_center = clicked_node_pos;
                }
            }
        }

        // Backspace / Reset
        if is_key_pressed(KeyCode::Backspace) {
            target_center = Point::new(0.0, 0.0);
        }

        // --- Animation ---
        let diff = target_center - view_center;
        if diff.norm() > 0.0001 {
            // Lerp in disk for smooth transition if not dragging
            if !is_dragging {
                view_center = view_center + diff * 0.1;
            }
        }

        // --- Drawing ---

        // Disk Boundary
        draw_circle(
            screen_center.x,
            screen_center.y,
            disk_radius,
            Color::new(0.05, 0.05, 0.05, 1.0),
        );
        draw_circle_lines(screen_center.x, screen_center.y, disk_radius, 2.0, DARKGRAY);

        // Nodes & Links
        draw_node_recursive(
            &layout_root,
            view_center,
            screen_center,
            disk_radius,
            mouse_z,
            &mut hovered_node,
        );

        // UI Overlay
        draw_text("Hyperbolic Finder", 20.0, 30.0, 30.0, WHITE);
        draw_text(&format!("View: {:.2}", view_center), 20.0, 50.0, 20.0, GRAY);

        // Buttons
        if draw_button("Reset View", 20.0, h - 50.0, 120.0, 30.0) {
            target_center = Point::new(0.0, 0.0);
        }

        // Hover Info
        if let Some((name, info, sy)) = hovered_node {
            let text = format!("{} ({})", name, info);
            let tw = measure_text(&text, None, 20, 1.0).width;
            draw_rectangle(
                mx + 10.0,
                sy - 25.0,
                tw + 10.0,
                30.0,
                Color::new(0.0, 0.0, 0.0, 0.8),
            );
            draw_text(&text, mx + 15.0, sy - 5.0, 20.0, WHITE);
        }

        next_frame().await
    }
}

fn draw_button(text: &str, x: f32, y: f32, w: f32, h: f32) -> bool {
    let (mx, my) = mouse_position();
    let is_hover = mx >= x && mx <= x + w && my >= y && my <= y + h;

    draw_rectangle(x, y, w, h, if is_hover { LIGHTGRAY } else { GRAY });
    draw_text(text, x + 10.0, y + 20.0, 20.0, BLACK);

    is_hover && is_mouse_button_pressed(MouseButton::Left)
}

fn find_closest_node(
    node: &LayoutNode,
    view_center: Point,
    click_z: Point, // point in transformed space (screen)
    hit_radius: f64,
) -> Option<Point> {
    let z_prime = mobius_sub(node.pos, view_center);
    let dist = (z_prime - click_z).norm();

    let mut best_match = None;
    let mut min_dist = hit_radius;

    if dist < min_dist {
        best_match = Some(node.pos);
        min_dist = dist;
    }

    if z_prime.norm() < 0.95 {
        for child in &node.children {
            if let Some(match_pos) = find_closest_node(child, view_center, click_z, min_dist) {
                // To compare accurately, we should check distance in screen space
                let child_prime = mobius_sub(match_pos, view_center);
                let child_dist = (child_prime - click_z).norm();
                if child_dist < min_dist {
                    min_dist = child_dist;
                    best_match = Some(match_pos);
                }
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
    mouse_z: Point,
    hover_state: &mut Option<(String, String, f32)>,
) {
    let z_prime = mobius_sub(node.pos, view_center);
    if z_prime.norm_sqr() > 0.99 {
        return;
    }

    let screen_pos = to_screen(z_prime, screen_center, disk_radius);

    // Draw Links first
    for child in &node.children {
        let child_z_prime = mobius_sub(child.pos, view_center);
        if child_z_prime.norm_sqr() > 0.99 && z_prime.norm_sqr() > 0.99 {
            continue;
        }

        draw_geodesic(
            z_prime,
            child_z_prime,
            screen_center,
            disk_radius,
            Color::new(0.4, 0.4, 0.4, 0.5),
        );
        draw_node_recursive(
            child,
            view_center,
            screen_center,
            disk_radius,
            mouse_z,
            hover_state,
        );
    }

    // Draw Node
    let scale = 1.0 - z_prime.norm_sqr();
    let radius = (5.0 * scale as f32 + 2.0).max(1.0);

    let color = get_color(node.node.file_type);

    draw_circle(screen_pos.x, screen_pos.y, radius, color);

    // Hover check
    if (z_prime - mouse_z).norm() < (radius / disk_radius) as f64 {
        *hover_state = Some((
            node.node.name.clone(),
            format_size(node.node.size),
            screen_pos.y,
        ));
        // Highlight
        draw_circle_lines(screen_pos.x, screen_pos.y, radius + 2.0, 1.0, YELLOW);
    }
}

fn get_color(ft: FileType) -> Color {
    match ft {
        FileType::Directory => BLUE,
        FileType::Code => ORANGE,
        FileType::Image => PURPLE,
        FileType::Audio => PINK,
        FileType::Video => RED,
        FileType::Archive => BROWN,
        FileType::Text => WHITE,
        FileType::Other => GREEN,
    }
}

fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

fn draw_geodesic(p1: Point, p2: Point, screen_center: Vec2, radius: f32, color: Color) {
    let steps = 15;
    let m_p2 = mobius_sub(p2, p1);

    let mut last_pos = to_screen(p1, screen_center, radius);

    for i in 1..=steps {
        let t = i as f64 / steps as f64;
        let q = m_p2 * t;
        let world_pos = mobius_add(q, p1); // Map back
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
        center.y - p.im as f32 * radius, // Flip Y for screen
    )
}
