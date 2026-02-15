mod logic;

use logic::{assign_target_angles, calculate_strip_transforms, parse_trace, SegmentType};
use macroquad::prelude::*;
use nalgebra::Point3;

#[macroquad::main("Trace Fold")]
async fn main() {
    let raw_trace = "
stack backtrace:
   0: std::backtrace_rs::backtrace::libunwind::trace
             at /rustc/std/src/lib.rs:100
   1: std::backtrace_rs::backtrace::trace_unsynchronized
             at /rustc/std/src/lib.rs:100
   2: std::sys_common::backtrace::_print_fmt
             at /rustc/std/src/lib.rs:100
   3: core::fmt::num::imp::fmt_u64
             at /rustc/core/src/fmt/num.rs:200
   4: alloc::alloc::handle_alloc_error
             at /rustc/alloc/src/alloc.rs:100
   5: my_app::logic::process_data
             at src/logic.rs:45
   6: my_app::main
             at src/main.rs:10
   7: core::ops::function::FnOnce::call_once
             at /rustc/core/src/ops/function.rs:250
    ";

    let segments = parse_trace(raw_trace);
    let angles = assign_target_angles(&segments);

    let mut fold_progress = 0.0f32;

    let mut cam_yaw = 0.0f32;
    let mut cam_pitch = 0.5f32;
    let mut cam_dist = 15.0f32;

    loop {
        clear_background(BLACK);

        if is_key_down(KeyCode::Left) {
            fold_progress = (fold_progress - 0.02).max(0.0);
        }
        if is_key_down(KeyCode::Right) {
            fold_progress = (fold_progress + 0.02).min(1.0);
        }

        if is_key_down(KeyCode::A) {
            cam_yaw += 0.05;
        }
        if is_key_down(KeyCode::D) {
            cam_yaw -= 0.05;
        }
        if is_key_down(KeyCode::W) {
            cam_pitch += 0.05;
        }
        if is_key_down(KeyCode::S) {
            cam_pitch -= 0.05;
        }

        cam_dist = (cam_dist + mouse_wheel().1 * -0.5).clamp(5.0, 50.0);

        let cam_pos = vec3(
            cam_dist * cam_yaw.cos() * cam_pitch.cos(),
            cam_dist * cam_pitch.sin(),
            cam_dist * cam_yaw.sin() * cam_pitch.cos(),
        );

        set_camera(&Camera3D {
            position: cam_pos,
            up: vec3(0., 1., 0.),
            target: vec3(0., 5., 0.),
            ..Default::default()
        });

        draw_grid(20, 1.0, DARKGRAY, GRAY);

        let transforms = calculate_strip_transforms(&segments, &angles, fold_progress);

        // Prepare projection for labels
        let aspect = screen_width() / screen_height();
        let fov = 45.0f32.to_radians();
        let proj = Mat4::perspective_rh_gl(fov, aspect, 0.01, 100.0);
        let view = Mat4::look_at_rh(cam_pos, vec3(0., 5., 0.), vec3(0., 1., 0.));
        let view_proj = proj * view;

        let mut labels = Vec::new();

        for (i, transform) in transforms.iter().enumerate() {
            let segment = &segments[i];

            let color = match segment.segment_type {
                SegmentType::User => RED,
                SegmentType::System => BLUE,
            };

            // Draw Wireframe Box
            let w = 2.0; // Width
            let h = 1.0; // Length/Height along Y
            let d = 0.1; // Thickness

            // Local corners (centered on X, base on Y=0, center Z)
            let corners_local = [
                Point3::new(-w / 2., 0., -d / 2.),
                Point3::new(w / 2., 0., -d / 2.),
                Point3::new(w / 2., 0., d / 2.),
                Point3::new(-w / 2., 0., d / 2.),
                Point3::new(-w / 2., h, -d / 2.),
                Point3::new(w / 2., h, -d / 2.),
                Point3::new(w / 2., h, d / 2.),
                Point3::new(-w / 2., h, d / 2.),
            ];

            let c: Vec<Vec3> = corners_local
                .iter()
                .map(|p| {
                    let world = transform * p;
                    vec3(world.x, world.y, world.z)
                })
                .collect();

            // Draw box lines
            draw_line_3d(c[0], c[1], color);
            draw_line_3d(c[1], c[2], color);
            draw_line_3d(c[2], c[3], color);
            draw_line_3d(c[3], c[0], color);
            draw_line_3d(c[4], c[5], color);
            draw_line_3d(c[5], c[6], color);
            draw_line_3d(c[6], c[7], color);
            draw_line_3d(c[7], c[4], color);
            draw_line_3d(c[0], c[4], color);
            draw_line_3d(c[1], c[5], color);
            draw_line_3d(c[2], c[6], color);
            draw_line_3d(c[3], c[7], color);

            // Label position (Center of face)
            let center_local = Point3::new(0.0, h / 2.0, 0.0);
            let center_world = transform * center_local;
            let center_vec3 = vec3(center_world.x, center_world.y, center_world.z);

            // Project to screen
            let clip = view_proj * center_vec3.extend(1.0);
            let ndc = clip.truncate() / clip.w;

            if clip.w > 0.0 && ndc.z < 1.0 {
                let screen_x = (ndc.x + 1.0) * 0.5 * screen_width();
                let screen_y = (1.0 - ndc.y) * 0.5 * screen_height();
                labels.push((screen_x, screen_y, segment.content.clone(), color));
            }
        }

        set_default_camera();

        // Draw Labels
        for (x, y, text, color) in labels {
            // Draw backdrop
            draw_rectangle(
                x,
                y,
                text.len() as f32 * 8.0,
                20.0,
                Color::new(0., 0., 0., 0.5),
            );
            draw_text(&text, x, y + 15.0, 20.0, color);
        }

        draw_text(
            &format!("Fold: {:.2}", fold_progress),
            10.0,
            20.0,
            30.0,
            WHITE,
        );
        draw_text(
            "Controls: Arrows to Fold, WASD+Wheel to Move Camera",
            10.0,
            40.0,
            20.0,
            LIGHTGRAY,
        );

        next_frame().await
    }
}
