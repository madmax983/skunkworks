use macroquad::prelude::*;
use std::f32::consts::PI;

mod math;
mod fs;

use math::klein_bottle;
use fs::scan_directory;

const U_SCALE: f32 = 0.2; // How fast u advances per file
const V_SCALE: f32 = 0.1; // How fast v advances per file

#[macroquad::main("Klein FS")]
async fn main() {
    let root = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
    let files = scan_directory(&root);
    let mut scroll_idx: f32 = 0.0;

    // Camera params
    let mut cam_dist: f32 = 8.0;
    let mut cam_rot_x: f32 = PI / 2.0;
    let mut cam_rot_y: f32 = 0.0;

    loop {
        clear_background(BLACK);

        // Input
        // Mouse wheel for scrolling through files
        let (_, mw_y) = mouse_wheel();
        if mw_y != 0.0 {
            scroll_idx -= mw_y; // Scroll down = positive increment
            if scroll_idx < 0.0 { scroll_idx = 0.0; }
            if scroll_idx > (files.len().saturating_sub(1)) as f32 {
                scroll_idx = (files.len().saturating_sub(1)) as f32;
            }
        }

        // Mouse drag for camera orbit
        if is_mouse_button_down(MouseButton::Left) {
            let delta = mouse_delta_position();
            cam_rot_y -= delta.x * 3.0;
            cam_rot_x += delta.y * 3.0;

            // Clamp pitch to avoid gimbal lock flip issues
            cam_rot_x = cam_rot_x.clamp(0.1, PI - 0.1);
        }

        // Zoom
        if is_key_down(KeyCode::W) { cam_dist -= 0.1; }
        if is_key_down(KeyCode::S) { cam_dist += 0.1; }
        cam_dist = cam_dist.clamp(2.0, 50.0);

        // Calculate focus point on the surface
        let u_focus = scroll_idx * U_SCALE;
        let v_focus = scroll_idx * V_SCALE;
        let focus_pos = klein_bottle(u_focus, v_focus);

        // Camera position: Sphere coords relative to focus_pos
        // x = r * sin(theta) * cos(phi)
        // y = r * cos(theta)
        // z = r * sin(theta) * sin(phi)
        // Adjusting for our coordinate system (Y-up)
        let cam_offset = vec3(
            cam_dist * cam_rot_x.sin() * cam_rot_y.cos(),
            cam_dist * cam_rot_x.cos(),
            cam_dist * cam_rot_x.sin() * cam_rot_y.sin()
        );
        let cam_pos = focus_pos + cam_offset;

        set_camera(&Camera3D {
            position: cam_pos,
            target: focus_pos,
            up: vec3(0., 1., 0.),
            ..Default::default()
        });

        // Draw Static Wireframe Grid (Fundamental Domain [0, 2PI] x [0, 2PI])
        // We draw the "canonical" Klein bottle at the origin so the user sees the shape.
        // Wait, if files have large u, they will just wrap around this same shape.
        let grid_steps_u = 40;
        let grid_steps_v = 20;

        for i in 0..grid_steps_u {
            for j in 0..grid_steps_v {
                let u = (i as f32 / grid_steps_u as f32) * 2.0 * PI;
                let v = (j as f32 / grid_steps_v as f32) * 2.0 * PI;

                let p1 = klein_bottle(u, v);
                let p2 = klein_bottle(u + (2.0 * PI / grid_steps_u as f32), v);
                let p3 = klein_bottle(u, v + (2.0 * PI / grid_steps_v as f32));

                draw_line_3d(p1, p2, Color::new(0.2, 0.2, 0.2, 1.0));
                draw_line_3d(p1, p3, Color::new(0.2, 0.2, 0.2, 1.0));
            }
        }

        // Draw Files
        // Show a window around the current scroll index
        let window_size = 50;
        let start_idx = (scroll_idx as isize - window_size).max(0) as usize;
        let end_idx = (scroll_idx as isize + window_size).min(files.len() as isize) as usize;

        for i in start_idx..end_idx {
            let u = i as f32 * U_SCALE;
            let v = i as f32 * V_SCALE;
            let pos = klein_bottle(u, v);

            let is_focused = (i as f32 - scroll_idx).abs() < 0.5;

            let color = if is_focused {
                WHITE
            } else if files[i].is_dir {
                YELLOW
            } else {
                // Color by depth
                let hue = (files[i].depth as f32 * 0.2) % 1.0;
                // Simple hue to rgb approx
                if hue < 0.3 { GREEN } else if hue < 0.6 { BLUE } else { PURPLE }
            };

            let size = if is_focused { 0.15 } else { 0.05 };

            draw_sphere(pos, size, None, color);

            // Draw path connection
            if i < end_idx - 1 {
                let next_pos = klein_bottle((i + 1) as f32 * U_SCALE, (i + 1) as f32 * V_SCALE);
                draw_line_3d(pos, next_pos, Color::new(0.5, 0.5, 0.5, 0.5));
            }
        }

        set_default_camera();

        // UI Overlay
        if !files.is_empty() {
            let current_idx = scroll_idx.round() as usize;
            if let Some(file) = files.get(current_idx) {
                draw_text(&format!("File [{}/{}]: {}", current_idx + 1, files.len(), file.name), 20.0, 30.0, 30.0, WHITE);
                draw_text(&format!("Path: {}", file.path.display()), 20.0, 60.0, 20.0, LIGHTGRAY);
                draw_text(&format!("Depth: {}", file.depth), 20.0, 85.0, 20.0, GRAY);
            }
        } else {
             draw_text("No files found.", 20.0, 30.0, 30.0, RED);
        }

        draw_text("Scroll: Navigate | Drag: Rotate Camera", 20.0, screen_height() - 20.0, 20.0, DARKGRAY);

        next_frame().await
    }
}
