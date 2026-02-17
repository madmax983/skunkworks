mod logic;
mod safe_gl;
mod world;

use macroquad::prelude::*;
use world::World;

// Constants
const MAX_DEPTH: i32 = 10;

const RAW_TRACE: &str = "
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
   6: my_app::world::World::from_trace
             at src/world.rs:15
   7: my_app::main
             at src/main.rs:10
   8: core::ops::function::FnOnce::call_once
             at /rustc/core/src/ops/function.rs:250
   9: std::sys_common::backtrace::__rust_begin_short_backtrace
             at /rustc/std/src/sys_common/backtrace.rs:125
  10: std::rt::lang_start::{{closure}}
             at /rustc/std/src/rt.rs:166
  11: std::rt::lang_start_internal
             at /rustc/std/src/rt.rs:166
  12: main
             at src/main.rs:10
";

#[macroquad::main("Trace Tardis")]
async fn main() {
    let world = World::from_trace(RAW_TRACE);

    // Initial Camera State
    // We start looking at Root (0)
    let mut current_room_id = 0;

    // Camera params
    let mut cam_pos = vec2(200.0, 150.0); // Center of first room (400x300)
    let mut cam_zoom = 1.0;

    loop {
        // Handle input
        let move_speed = 5.0 / cam_zoom;
        if is_key_down(KeyCode::Minus) || is_key_down(KeyCode::Q) {
            cam_zoom *= 0.98;
        }
        if is_key_down(KeyCode::Equal) || is_key_down(KeyCode::E) {
            cam_zoom *= 1.02;
        }
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            cam_pos.x -= move_speed;
        }
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            cam_pos.x += move_speed;
        }
        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
            cam_pos.y -= move_speed;
        }
        if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
            cam_pos.y += move_speed;
        }

        // Check for Portal Crossing
        if cam_zoom > 3.0 {
            if let Some(room) = world.rooms.get(&current_room_id) {
                for portal in &room.portals {
                    if portal.rect.contains(cam_pos) {
                        if let Some(target_room) = world.rooms.get(&portal.target_room_id) {
                            let scale_x = portal.rect.w / target_room.rect.w;
                            let new_zoom = cam_zoom * scale_x;

                            let new_pos_x =
                                target_room.rect.x + (cam_pos.x - portal.rect.x) / scale_x;
                            let new_pos_y =
                                target_room.rect.y + (cam_pos.y - portal.rect.y) / scale_x;

                            current_room_id = portal.target_room_id;
                            cam_pos = vec2(new_pos_x, new_pos_y);
                            cam_zoom = new_zoom;
                            break;
                        }
                    }
                }
            }
        }

        // Check for Zoom Out (Crossing back to parent?)
        // Not implemented in alloc-tardis either, just restarts loop usually or needs logic to know parent.
        // For now, we only go deeper. "Trace Tardis" is a one-way trip to the metal.

        clear_background(BLACK);

        let aspect = screen_width() / screen_height();
        let base_scale = 1.0 / 300.0 * cam_zoom;

        let root_cam = Camera2D {
            target: cam_pos,
            zoom: vec2(base_scale, base_scale * aspect),
            rotation: 0.0,
            render_target: None,
            offset: vec2(0.0, 0.0), // center
            viewport: None,
        };

        draw_recursive(&world, current_room_id, root_cam, MAX_DEPTH, None);

        // Draw HUD
        set_default_camera();
        draw_text(
            format!("Zoom: {:.2} | Depth: {}", cam_zoom, current_room_id).as_str(),
            20.0,
            20.0,
            30.0,
            WHITE,
        );
        draw_text(
            "WASD/Arrows to move, +/- to zoom",
            20.0,
            50.0,
            20.0,
            LIGHTGRAY,
        );

        // Show current function name at top
        if let Some(room) = world.rooms.get(&current_room_id) {
            draw_text(
                &format!("Current Frame: {}", room.label),
                20.0,
                80.0,
                20.0,
                YELLOW,
            );
        }

        next_frame().await
    }
}

fn draw_recursive(
    world: &World,
    room_id: usize,
    cam: Camera2D,
    depth: i32,
    parent_scissor: Option<(i32, i32, i32, i32)>,
) {
    if depth <= 0 {
        return;
    }

    let room = match world.rooms.get(&room_id) {
        Some(r) => r,
        None => return,
    };

    // 1. Set Camera
    set_camera(&cam);

    // 2. Draw Room Floor
    draw_rectangle(
        room.rect.x,
        room.rect.y,
        room.rect.w,
        room.rect.h,
        room.color,
    );
    draw_rectangle_lines(
        room.rect.x,
        room.rect.y,
        room.rect.w,
        room.rect.h,
        5.0,
        WHITE,
    );

    // Draw Label (centered)
    let center = vec2(
        room.rect.x + room.rect.w / 2.0,
        room.rect.y + room.rect.h / 2.0,
    );

    // Scale font size so it's readable relative to room
    let font_size = room.rect.w * 0.05;
    draw_text(
        &room.label,
        room.rect.x + 10.0,
        center.y - font_size, // Above center
        font_size,
        WHITE,
    );

    // 3. Draw Portals
    for portal in &room.portals {
        // Draw Portal Frame
        draw_rectangle(
            portal.rect.x,
            portal.rect.y,
            portal.rect.w,
            portal.rect.h,
            portal.color,
        );
        draw_rectangle_lines(
            portal.rect.x,
            portal.rect.y,
            portal.rect.w,
            portal.rect.h,
            2.0,
            BLACK,
        );

        // Portal Label ("Next Call")
        let p_font_size = portal.rect.w * 0.1;
        draw_text(
            "CALL",
            portal.rect.x + 5.0,
            portal.rect.y + p_font_size,
            p_font_size,
            BLACK,
        );

        // 4. Calculate Child Camera
        let target_room = match world.rooms.get(&portal.target_room_id) {
            Some(r) => r,
            None => continue,
        };

        let scale_x = portal.rect.w / target_room.rect.w;
        let scale_y = portal.rect.h / target_room.rect.h;

        let new_zoom = vec2(cam.zoom.x * scale_x, cam.zoom.y * scale_y);

        let portal_top_left = vec2(portal.rect.x, portal.rect.y);
        let portal_bottom_right =
            vec2(portal.rect.x + portal.rect.w, portal.rect.y + portal.rect.h);

        let screen_tl = cam.world_to_screen(portal_top_left);
        let screen_br = cam.world_to_screen(portal_bottom_right);

        let sx = screen_tl.x.min(screen_br.x) as i32;
        let sy_top = screen_tl.y.max(screen_br.y) as i32;
        let _sy_bot = screen_tl.y.min(screen_br.y) as i32;

        let sw = (screen_tl.x - screen_br.x).abs() as i32;
        let sh = (screen_tl.y - screen_br.y).abs() as i32;

        let gl_y = screen_height() as i32 - sy_top;

        if sw < 1 || sh < 1 {
            continue;
        }

        safe_gl::with_scissor(sx, gl_y, sw, sh, parent_scissor, |clipped_rect| {
            let new_target_x = target_room.rect.x + (cam.target.x - portal.rect.x) / scale_x;
            let new_target_y = target_room.rect.y + (cam.target.y - portal.rect.y) / scale_y;

            let child_cam = Camera2D {
                target: vec2(new_target_x, new_target_y),
                zoom: new_zoom,
                rotation: cam.rotation,
                render_target: None,
                offset: cam.offset,
                viewport: None,
            };

            // Recurse
            draw_recursive(
                world,
                portal.target_room_id,
                child_cam,
                depth - 1,
                Some(clipped_rect),
            );
        });
    }
}
