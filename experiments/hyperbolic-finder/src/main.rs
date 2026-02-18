mod fs;
mod layout;
mod tiling;
mod ui;

use fs::{get_repo_statuses, get_view_root, FileType, GitStatus};
use layout::{layout_tree, LayoutNode};
use macroquad::prelude::*;
use poincare_disk::{mobius_add, mobius_sub, Point};
use std::path::PathBuf;
use ui::Button;

const DISK_SCALE: f32 = 0.45;

#[macroquad::main("Hyperbolic Finder")]
async fn main() -> anyhow::Result<()> {
    // Current state
    let mut current_path = std::env::current_dir()?;
    // Git status map
    let mut git_map = get_repo_statuses(&current_path);

    // Root of the current visualization
    let mut fs_root = get_view_root(&current_path, 5, &git_map)?;
    let mut layout_root = layout_tree(fs_root.clone());

    // Navigation State
    let mut view_center = Point::new(0.0, 0.0);
    let mut target_center = Point::new(0.0, 0.0);
    let mut navigating_to: Option<PathBuf> = None;

    // Starfield (Points in the unit disk)
    let stars: Vec<Point> = (0..500)
        .map(|_| {
            let angle = rand::gen_range(0.0, std::f64::consts::PI * 2.0);
            let r = rand::gen_range(0.0f64, 0.99).sqrt(); // Sqrt for uniform distribution
            use num_complex::Complex;
            Complex::from_polar(r, angle)
        })
        .collect();

    // Dragging State
    let mut is_dragging = false;
    let mut drag_start_mouse = Point::new(0.0, 0.0);
    let mut drag_locked_world_point = Point::new(0.0, 0.0);

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
            drag_locked_world_point = mobius_add(mouse_z, view_center);
        }

        if is_mouse_button_down(MouseButton::Left) && is_dragging {
            target_center = mobius_sub(drag_locked_world_point, mouse_z);
            view_center = target_center; // Instant update for responsiveness
        }

        if is_mouse_button_released(MouseButton::Left) {
            is_dragging = false;
            // On release, check if it was a click (short drag)
            if (mouse_z - drag_start_mouse).norm() < 0.02 && is_mouse_in_disk {
                // It was a click! Navigate to node.
                if let Some((clicked_pos, clicked_node)) = find_closest_node(
                    &layout_root,
                    view_center,
                    mouse_z,
                    0.05, // hit radius
                ) {
                    if clicked_node.node.is_dir {
                        // If directory, center on it and prepare to navigate
                        target_center = clicked_pos;
                        navigating_to = Some(clicked_node.node.path.clone());
                    } else {
                        // File action
                        println!("Clicked file: {:?}", clicked_node.node.path);
                    }
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
            if !is_dragging {
                view_center = view_center + diff * 0.1;
            }
        } else if let Some(path) = navigating_to.take() {
            // We reached the target! Switch context.
            // If the path is different from current, reload
            if path != current_path {
                // If path is ".." we need to be careful, but get_view_root adds ".." with the PARENT path.
                // So clicked_node.node.path IS the parent path.
                current_path = path;
                git_map = get_repo_statuses(&current_path);
                match get_view_root(&current_path, 5, &git_map) {
                    Ok(new_root) => {
                        fs_root = new_root;
                        layout_root = layout_tree(fs_root.clone());
                        target_center = Point::new(0.0, 0.0);
                        view_center = Point::new(0.0, 0.0);
                    }
                    Err(e) => {
                        eprintln!("Failed to scan directory: {}", e);
                    }
                }
            }
        }

        // --- Drawing ---

        // Disk Boundary (Background)
        draw_circle(
            screen_center.x,
            screen_center.y,
            disk_radius,
            Color::new(0.05, 0.05, 0.05, 1.0),
        );

        // Draw Starfield
        for star in &stars {
            let star_prime = mobius_sub(*star, view_center);
            if star_prime.norm_sqr() < 0.99 {
                let pos = to_screen(star_prime, screen_center, disk_radius);
                // Twinkle based on position/time
                let alpha = (star_prime.re * 5.0 + get_time()).sin() * 0.5 + 0.5;
                draw_circle(
                    pos.x,
                    pos.y,
                    1.0,
                    Color::new(1.0, 1.0, 1.0, alpha as f32 * 0.5),
                );
            }
        }

        // Draw Tiling (Hyperbolic Grid)
        tiling::draw_tiling(view_center, screen_center, disk_radius);

        // Disk Boundary (Lines)
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
        draw_text(
            &format!("Path: {}", current_path.display()),
            20.0,
            60.0,
            20.0,
            GRAY,
        );

        // Right Click to go Up
        if is_mouse_button_released(MouseButton::Right) {
            if let Some(parent) = current_path.parent() {
                let parent_buf = parent.to_path_buf();
                git_map = get_repo_statuses(&parent_buf);
                if let Ok(new_root) = get_view_root(&parent_buf, 5, &git_map) {
                    current_path = parent_buf;
                    fs_root = new_root;
                    layout_root = layout_tree(fs_root.clone());
                    target_center = Point::new(0.0, 0.0);
                    view_center = Point::new(0.0, 0.0);
                }
            }
        }

        // Buttons
        let btn_w = 120.0;
        let btn_h = 30.0;
        let btn_y = h - 50.0;

        if Button::new("Reset View", 20.0, btn_y, btn_w, btn_h).draw() {
            target_center = Point::new(0.0, 0.0);
        }

        if Button::new("Up (..)", 160.0, btn_y, btn_w, btn_h).draw() {
            if let Some(parent) = current_path.parent() {
                let parent_buf = parent.to_path_buf();
                // Re-scan from parent
                git_map = get_repo_statuses(&parent_buf);
                if let Ok(new_root) = get_view_root(&parent_buf, 5, &git_map) {
                    current_path = parent_buf;
                    fs_root = new_root;
                    layout_root = layout_tree(fs_root.clone());
                    target_center = Point::new(0.0, 0.0);
                    view_center = Point::new(0.0, 0.0);
                }
            }
        }

        // Draw Root Indicator if far
        let root_prime = mobius_sub(Point::new(0.0, 0.0), view_center);
        if root_prime.norm() > 0.1 {
            let screen_pos = to_screen(root_prime, screen_center, disk_radius);
            // Draw arrow
            draw_circle_lines(
                screen_pos.x,
                screen_pos.y,
                10.0,
                2.0,
                Color::new(1.0, 1.0, 1.0, 0.5),
            );
            draw_line(
                screen_pos.x,
                screen_pos.y,
                screen_center.x,
                screen_center.y,
                1.0,
                Color::new(1.0, 1.0, 1.0, 0.2),
            );
        }

        // Hover Info
        if let Some((name, info, sy)) = hovered_node {
            let text = format!("{} ({})", name, info);
            let dims = measure_text(&text, None, 20, 1.0);
            let tw = dims.width;

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

fn find_closest_node<'a>(
    node: &'a LayoutNode,
    view_center: Point,
    click_z: Point, // point in transformed space (screen)
    hit_radius: f64,
) -> Option<(Point, &'a LayoutNode)> {
    let z_prime = mobius_sub(node.pos, view_center);
    let dist = (z_prime - click_z).norm();

    // Adjust effective hit radius based on node size
    let size_factor = (node.node.total_size as f64).max(1.0).log10();
    let effective_radius = hit_radius * (1.0 + size_factor * 0.2);

    let mut best_match = None;
    let mut min_dist = effective_radius;

    if dist < min_dist {
        best_match = Some((node.pos, node));
        min_dist = dist;
    }

    if z_prime.norm() < 0.95 {
        for child in &node.children {
            if let Some((match_pos, match_node)) =
                find_closest_node(child, view_center, click_z, min_dist)
            {
                // Check dist again in screen space
                let child_prime = mobius_sub(match_pos, view_center);
                let child_dist = (child_prime - click_z).norm();
                if child_dist < min_dist {
                    min_dist = child_dist;
                    best_match = Some((match_pos, match_node));
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

    let scale = 1.0 - z_prime.norm_sqr();
    // Scale node size based on total content size (Memory Visualization)
    let size_factor = (node.node.total_size as f64).max(1.0).log10() as f32;
    // Base size depends on hyperbolic scale (distance from center)
    let radius = ((5.0 + size_factor * 1.5) * scale as f32 + 2.0).max(1.0);

    // LOD: If too small, just draw a dot and return
    if radius < 2.0 {
        let color = get_color(node.node.file_type, node.node.git_status);
        draw_circle(screen_pos.x, screen_pos.y, radius.max(1.0), color);
        return;
    }

    // Draw Links first
    for child in &node.children {
        let child_z_prime = mobius_sub(child.pos, view_center);
        // Only cull if both are far invisible
        if child_z_prime.norm_sqr() > 0.999 && z_prime.norm_sqr() > 0.999 {
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

    let color = get_color(node.node.file_type, node.node.git_status);

    // If it's a large directory, draw a "halo" to indicate mass
    if node.node.total_size > 1_000_000 {
        draw_circle(
            screen_pos.x,
            screen_pos.y,
            radius + 2.0 * scale as f32,
            Color::new(color.r, color.g, color.b, 0.3),
        );
    }

    draw_circle(screen_pos.x, screen_pos.y, radius, color);

    // Hover check
    if (z_prime - mouse_z).norm() < (radius / disk_radius) as f64 {
        *hover_state = Some((
            node.node.name.clone(),
            format_size(node.node.total_size),
            screen_pos.y,
        ));
        // Highlight
        draw_circle_lines(screen_pos.x, screen_pos.y, radius + 2.0, 1.0, YELLOW);
    }
}

fn get_color(ft: FileType, git_status: Option<GitStatus>) -> Color {
    if let Some(status) = git_status {
        match status {
            GitStatus::New => return Color::new(0.2, 1.0, 0.2, 1.0), // Bright Green
            GitStatus::Modified => return Color::new(0.2, 0.6, 1.0, 1.0), // Bright Blue
            GitStatus::Ignored => return Color::new(0.4, 0.4, 0.4, 0.5), // Gray
            GitStatus::Conflict => return Color::new(1.0, 0.0, 1.0, 1.0), // Magenta
            GitStatus::Deleted => return Color::new(1.0, 0.0, 0.0, 1.0), // Red
            GitStatus::Renamed => return Color::new(0.6, 0.2, 0.8, 1.0), // Purple
            GitStatus::Clean => {}
        }
    }

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

    // Collect points first
    let mut points = Vec::with_capacity(steps + 1);
    points.push(to_screen(p1, screen_center, radius));

    for i in 1..=steps {
        let t = i as f64 / steps as f64;
        let q = m_p2 * t;
        let world_pos = mobius_add(q, p1); // Map back
        points.push(to_screen(world_pos, screen_center, radius));
    }

    // Draw Glow (3 layers)
    for i in 0..points.len() - 1 {
        let p1 = points[i];
        let p2 = points[i + 1];

        // Layer 1: Wide, Faint
        draw_line(
            p1.x,
            p1.y,
            p2.x,
            p2.y,
            4.0,
            Color::new(color.r, color.g, color.b, 0.1),
        );
        // Layer 2: Medium
        draw_line(
            p1.x,
            p1.y,
            p2.x,
            p2.y,
            2.0,
            Color::new(color.r, color.g, color.b, 0.3),
        );
        // Layer 3: Sharp
        draw_line(p1.x, p1.y, p2.x, p2.y, 1.0, color);
    }
}

fn to_screen(p: Point, center: Vec2, radius: f32) -> Vec2 {
    Vec2::new(
        center.x + p.re as f32 * radius,
        center.y - p.im as f32 * radius, // Flip Y for screen
    )
}
