mod safe_gl;
mod warden_gl_test;
mod world;

use chimera_lang::ast::{Dna, Gene, Helix, Nucleotide, Strand};
use chimera_lang::opcode::OpCode;
use chimera_lang::vm::ChimeraVM;
use macroquad::prelude::*;
use world::World;

// Constants
const MAX_DEPTH: i32 = 10;

#[macroquad::main("Chimera Tardis")]
async fn main() {
    // 1. Initialize ChimeraVM with recursive DNA
    let strand0 = Strand {
        genes: vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(50)],
            }, // Initial Depth
            Gene {
                op: OpCode::Call,
                args: vec![Nucleotide::Number(1)],
            }, // Call recursive fn
        ],
    };

    // Recursive Function (Strand 1)
    // Stack: [N]
    let strand1 = Strand {
        genes: vec![
            Gene {
                op: OpCode::Dup,
                args: vec![],
            }, // [N, N]
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)],
            }, // [N, N, 0]
            Gene {
                op: OpCode::Brz,
                args: vec![Nucleotide::Number(6)],
            }, // Jump to Return (Gene 6) if N==0. Note: Brz pops. Wait, Brz pops. So [N, N] -> pop 0? No.
            // Brz checks top. If 0, pop and jump.
            // Need to check equality with 0.
            // Let's use simple non-zero check.
            // If N == 0, Brz takes it.
            // So stack is [N].

            // If we are here, N != 0.
            // We need to decrement.
            Gene {
                op: OpCode::Dup,
                args: vec![],
            }, // [N, N]
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            }, // [N, N, 1]
            Gene {
                op: OpCode::Sub,
                args: vec![],
            }, // [N, N-1]
            Gene {
                op: OpCode::Call,
                args: vec![Nucleotide::Number(1)],
            }, // Recurse!
            // Return logic
            Gene {
                op: OpCode::Drop,
                args: vec![],
            }, // Drop the result
            Gene {
                op: OpCode::Ret,
                args: vec![],
            }, // Return
        ],
    };

    let dna = Dna {
        helix: Helix {
            strands: vec![strand0, strand1],
        },
        evolution_config: None,
    };

    let mut vm = ChimeraVM::new(dna);
    // Enable Nova feature flags/structures if needed (handled by feature flag in Cargo.toml)

    // Camera params
    let mut current_room_id = 0;
    let mut cam_pos = vec2(200.0, 150.0); // Center of first room (400x300)
    let mut cam_zoom = 1.0;

    let mut speed_multiplier = 1;

    loop {
        // --- VM Step ---
        if is_key_pressed(KeyCode::Space) {
            vm.halted = !vm.halted;
        }

        if !vm.halted {
            for _ in 0..speed_multiplier {
                vm.step();
                if vm.halted {
                    break;
                }
            }
        }

        // --- World Rebuild ---
        let world = World::from_vm(&vm);

        // Clamp current_room_id
        if current_room_id >= world.rooms.len() {
            current_room_id = world.rooms.len().saturating_sub(1);
        }

        // --- Input ---
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

        if is_key_pressed(KeyCode::F) {
            speed_multiplier *= 2;
        }
        if is_key_pressed(KeyCode::S) {
            if speed_multiplier > 1 {
                speed_multiplier /= 2;
            }
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

        // --- Render ---
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
            "WASD/Arrows: Move | +/-: Zoom | Space: Pause | F/S: Speed",
            20.0,
            50.0,
            20.0,
            LIGHTGRAY,
        );

        draw_text(
            &format!("VM Steps: ? | Energy: {}", vm.energy),
            20.0,
            80.0,
            20.0,
            YELLOW,
        );

        // Show current room info
        if let Some(room) = world.rooms.get(&current_room_id) {
            draw_text(
                &format!("Current: {}", room.description),
                20.0,
                110.0,
                20.0,
                GREEN,
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
    let font_size = room.rect.w * 0.1;
    draw_text(
        &room.label,
        room.rect.x + 10.0,
        center.y - font_size, // Above center
        font_size,
        WHITE,
    );

    let desc_size = room.rect.w * 0.05;
    draw_text(
        &room.description,
        room.rect.x + 10.0,
        center.y + desc_size,
        desc_size,
        LIGHTGRAY,
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

        // Portal Label
        let p_font_size = portal.rect.w * 0.1;
        draw_text(
            "NEXT",
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
