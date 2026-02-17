mod layout;
mod math;
mod scanner;

use crate::layout::{layout_tree, LayoutNode};
use crate::math::mobius_add;
use crate::scanner::{scan_fs, NodeType};
use macroquad::input::{set_cursor_grab, show_mouse}; // Ensure input functions are imported
use macroquad::prelude::*;
use std::path::PathBuf;

const MOVE_SPEED: f32 = 0.02; // Hyperbolic step size per frame
const ROT_SPEED: f32 = 0.03;

struct Player {
    pos: Vec3, // Hyperbolic position (Poincaré Ball)
    yaw: f32,
    pitch: f32,
}

#[macroquad::main("Hyperbolic Void")]
async fn main() {
    // 1. Scan FS
    let root_path = std::env::args().nth(1).unwrap_or_else(|| ".".to_string());
    println!("Scanning {}...", root_path);

    let fs_root = match scan_fs(std::path::Path::new(&root_path), 5) {
        Some(node) => node,
        None => {
            println!("Failed to scan path or empty.");
            return;
        }
    };

    // 2. Build Layout
    println!("Building layout...");
    let layout_root = layout_tree(fs_root);
    println!("Layout complete. Nodes generated.");

    // 3. Init Player
    let mut player = Player {
        pos: Vec3::ZERO,
        yaw: -1.57, // Face -Z roughly
        pitch: 0.0,
    };

    // Capture mouse
    show_mouse(false);
    set_cursor_grab(true);

    loop {
        // --- Input ---
        let delta = get_frame_time();

        // Mouse Look
        let mouse_delta = mouse_delta_position();
        player.yaw += mouse_delta.x * ROT_SPEED * -1.0;
        player.pitch += mouse_delta.y * ROT_SPEED;
        player.pitch = player.pitch.clamp(-1.5, 1.5);

        // Movement
        let front = Vec3::new(
            player.yaw.cos() * player.pitch.cos(),
            player.pitch.sin(),
            player.yaw.sin() * player.pitch.cos(),
        )
        .normalize();

        let right = front.cross(Vec3::new(0.0, 1.0, 0.0)).normalize();
        let up = right.cross(front).normalize(); // Local up

        let mut move_dir = Vec3::ZERO;
        if is_key_down(KeyCode::W) {
            move_dir += front;
        }
        if is_key_down(KeyCode::S) {
            move_dir -= front;
        }
        if is_key_down(KeyCode::A) {
            move_dir -= right;
        }
        if is_key_down(KeyCode::D) {
            move_dir += right;
        }
        if is_key_down(KeyCode::Space) {
            move_dir += up;
        }
        if is_key_down(KeyCode::LeftShift) {
            move_dir -= up;
        }

        if move_dir.length_squared() > 0.001 {
            move_dir = move_dir.normalize();
            // Apply movement hyperbolically
            // The step vector is defined in the tangent space of the current position (which we view as origin)
            // So we just add it using mobius_add(player.pos, step)
            // Wait, mobius_add(a, b) treats 'b' as displacement from origin mapped to 'a'.
            // Yes, this is exactly what we want.
            let step = move_dir * MOVE_SPEED;
            player.pos = mobius_add(player.pos, step);
        }

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        // --- Render ---
        clear_background(Color::new(0.05, 0.05, 0.08, 1.0));

        // 3D Camera Setup
        // We render everything relative to the origin (player's view center)
        // So camera is at 0, looking in direction determined by yaw/pitch
        set_camera(&Camera3D {
            position: Vec3::ZERO,
            target: front,
            up: Vec3::new(0.0, 1.0, 0.0),
            fovy: 45.0,
            ..Default::default()
        });

        // Draw Nodes
        draw_layout_recursive(&layout_root, player.pos, 0);

        // Draw HUD
        set_default_camera();
        draw_text("Hyperbolic Void", 10.0, 20.0, 30.0, WHITE);
        draw_text(&format!("Pos: {:.2?}", player.pos), 10.0, 40.0, 20.0, GRAY);
        draw_text(
            &format!("Norm: {:.4}", player.pos.length()),
            10.0,
            60.0,
            20.0,
            GRAY,
        );
        draw_text(
            "WASD to Move, Mouse to Look",
            10.0,
            screen_height() - 20.0,
            20.0,
            WHITE,
        );

        // Reticle
        draw_circle_lines(screen_width() / 2.0, screen_height() / 2.0, 5.0, 2.0, WHITE);

        next_frame().await
    }
}

fn draw_layout_recursive(node: &LayoutNode, view_pos: Vec3, depth: usize) {
    // Transform node position to view space
    // P_view = mobius_add(-view_pos, P_world)
    let p_view = mobius_add(-view_pos, node.position);

    let dist_sq = p_view.length_squared();
    if dist_sq >= 0.999 {
        return;
    }

    // Calculate base size based on file size (log scale)
    let size_bytes = node.node.size.max(1);
    let log_size = (size_bytes as f32).log10().max(1.0);
    let base_radius = 0.05 * (log_size / 3.0); // Normalize roughly

    // Euclidean size ~ (1 - r^2) * base_radius
    let scale = (1.0 - dist_sq) * base_radius;

    if scale < 0.001 {
        return;
    } // Cull small objects

    // Color
    let color = match node.node.node_type {
        NodeType::Directory => Color::new(0.2, 0.6, 1.0, 0.8),
        NodeType::File => Color::new(1.0, 0.5, 0.2, 0.9),
    };

    // Draw Node
    match node.node.node_type {
        NodeType::Directory => {
            draw_sphere(p_view, scale, None, color);
            draw_sphere_wireframe(p_view, scale * 1.1, None, Color::new(1.0, 1.0, 1.0, 0.3));
        }
        NodeType::File => {
            draw_sphere(p_view, scale, None, color);
        }
    }

    // Draw Text if close to center (player is "inside" or near the node)
    // Center of view is (0,0,0). So if p_view is small.
    if dist_sq < 0.1 {
        // Billboard text
        // Need to project position to screen 2D
        let screen_pos = camera_world_to_screen(p_view);
        // camera_world_to_screen is not available directly?
        // We are in 3D mode. We need to switch to 2D to draw text?
        // Or render text in world space? (macroquad doesn't have 3D text easily)
        // We'll skip text in 3D pass.
    }

    // Draw Connections
    for child in &node.children {
        // Transform child pos
        let c_view = mobius_add(-view_pos, child.position);

        if c_view.length_squared() < 0.99 {
            // Draw line
            draw_line_3d(p_view, c_view, Color::new(1.0, 1.0, 1.0, 0.2));
        }

        draw_layout_recursive(child, view_pos, depth + 1);
    }
}

// Helper to project 3D point to screen
fn camera_world_to_screen(pos: Vec3) -> Vec2 {
    // This requires access to the current matrices which macroquad hides partially.
    // simpler to just not draw text or draw it in a separate pass.
    Vec2::ZERO
}
