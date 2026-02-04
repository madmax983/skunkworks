mod world;

use macroquad::prelude::*;
use world::World;

// Constants
const MAX_DEPTH: i32 = 5;

#[macroquad::main("Alloc Tardis")]
async fn main() {
    let world = World::mock_memory();

    // Initial Camera State
    // We start looking at Root (0)
    let mut current_room_id = 0;

    // Camera params
    let mut cam_pos = vec2(0.0, 0.0);
    let mut cam_zoom = 1.0; // 1.0 means "Normal View" (fits screen approx)

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
        // If we are zoomed in enough that a portal dominates the view, switch to it.
        // Threshold: 3.0 (Arbitrary, feels like "Close Enough")
        if cam_zoom > 3.0 {
            let room = world.rooms.get(&current_room_id).unwrap();
            for portal in &room.portals {
                if portal.rect.contains(cam_pos) {
                    let target_room = world.rooms.get(&portal.target_room_id).unwrap();

                    // Calculate Scale Factor (Child -> Parent)
                    // Scale = Portal / Target
                    let scale_x = portal.rect.w / target_room.rect.w;
                    // let scale_y = portal.rect.h / target_room.rect.h;

                    // New Zoom
                    // We want visual continuity.
                    // Current View Width = ScreenWidth / Zoom.
                    // Effectively, we are multiplying our coordinate system by (1/Scale).
                    // So Zoom should be multiplied by Scale?
                    // Wait. If we switch to Child, Child is huge (Target Room).
                    // We were looking at Portal (Small).
                    // To make Child look Small (like Portal), we need Low Zoom?
                    // No, `cam_zoom` = 1.0 means "Fits Screen".
                    // If we are at `cam_zoom` = 10.0 (Looking closely at Portal).
                    // And Portal is 0.1 of Child.
                    // Then Child should be rendered at Zoom 1.0 to match?
                    // new_zoom = old_zoom * scale.
                    // 10.0 * 0.1 = 1.0. Correct.

                    let new_zoom = cam_zoom * scale_x;

                    // New Pos
                    // P_c = TargetX + (P_p - PortalX) / S
                    let new_pos_x = target_room.rect.x + (cam_pos.x - portal.rect.x) / scale_x;
                    let new_pos_y = target_room.rect.y + (cam_pos.y - portal.rect.y) / scale_x;

                    current_room_id = portal.target_room_id;
                    cam_pos = vec2(new_pos_x, new_pos_y);
                    cam_zoom = new_zoom;
                    break;
                }
            }
        }

        clear_background(BLACK);

        let aspect = screen_width() / screen_height();

        // Base Camera for the current root room
        // We map the room's coordinate system to the screen
        // cam_zoom = 1.0 -> 1 unit = 1 pixel? No.
        // Let's say zoom 1.0 means the room fills the screen height?
        // macroquad camera zoom is in "screen normalized units" usually?
        // Camera2D.zoom is (2/viewport_w, 2/viewport_h) * scale.

        // Let's define: cam_zoom of 1.0 means -100..100 fills the height.
        // So scale = 1.0 / 100.0?
        let base_scale = 1.0 / 300.0 * cam_zoom;

        let root_cam = Camera2D {
            target: cam_pos,
            zoom: vec2(base_scale, base_scale * aspect),
            rotation: 0.0,
            render_target: None,
            offset: vec2(0.0, 0.0), // center
            viewport: None,
        };

        draw_recursive(&world, current_room_id, root_cam, MAX_DEPTH);

        // Draw HUD
        set_default_camera();
        draw_text(
            format!("Zoom: {:.2}", cam_zoom).as_str(),
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

        next_frame().await
    }
}

fn draw_recursive(world: &World, room_id: usize, cam: Camera2D, depth: i32) {
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
    // draw_text is screen space? No, World space if camera is set.
    // But text size is also world space?
    // In Macroquad, `draw_text` behaves weirdly with Camera2D (font size scales).
    // Let's try `draw_text_ex` with world-space size.
    // A safe size is 10% of room width.
    let font_size = room.rect.w * 0.1;
    draw_text(
        room.label.as_str(),
        room.rect.x + 10.0,
        center.y,
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

        // 4. Calculate Child Camera
        // We need to map the Target Room to fit inside Portal Rect.
        let target_room = match world.rooms.get(&portal.target_room_id) {
            Some(r) => r,
            None => continue,
        };

        // Calculate Scale Factor
        // We want Target.Rect to fit into Portal.Rect
        // ScaleX = Portal.W / Target.W
        // ScaleY = Portal.H / Target.H
        // We probably want to maintain aspect ratio or stretch?
        // Let's stretch for now (Allocators pack tightly).
        let scale_x = portal.rect.w / target_room.rect.w;
        let scale_y = portal.rect.h / target_room.rect.h;

        // We want to compose transforms.
        // Child World Point P_c -> Parent World Point P_p
        // P_p = Portal.TopLeft + (P_c - Target.TopLeft) * Scale
        // P_p = Portal.TopLeft - Target.TopLeft * Scale + P_c * Scale
        // So P_p = P_c * Scale + Offset

        // But Camera works in reverse: World -> Screen.
        // Cam_Parent: P_p -> Screen
        // Cam_Child: P_c -> Screen
        // Cam_Child(P_c) = Cam_Parent(P_p) = Cam_Parent(P_c * Scale + Offset)
        // Since Camera is a linear transform (roughly Scale * (Pos - Target)),
        // composing them means multiplying zoom and shifting target.

        // Let's rely on Macroquad's `zoom` and `target`.
        // Cam_Parent transform matrix M_p.
        // Model Matrix M_m: P_c -> P_p.
        // M_total = M_p * M_m.
        // We can just set the camera zoom/target manually.

        // New Zoom = Parent Zoom * Scale (element wise)
        let new_zoom = vec2(cam.zoom.x * scale_x, cam.zoom.y * scale_y);

        // New Target?
        // We need the screen position of the Portal Center to match the screen position of the Target Center.
        // Or simpler: The Child Camera is centered on the Target Room Center,
        // but offset such that it aligns with the Portal Center in Parent View.

        // Actually, `Camera2D` defines the viewport.
        // If we want to use `gl_scissor` (Screen Rect), we need to know where the portal is on screen.

        let portal_top_left = vec2(portal.rect.x, portal.rect.y);
        let portal_bottom_right =
            vec2(portal.rect.x + portal.rect.w, portal.rect.y + portal.rect.h);

        let screen_tl = cam.world_to_screen(portal_top_left);
        let screen_br = cam.world_to_screen(portal_bottom_right);

        // Scissor Rect
        // Macroquad's world_to_screen returns x in [0, screen_width], y in [0, screen_height].
        // gl_scissor takes x, y (from bottom left), w, h.
        // Macroquad screen coordinates are Y-down (0 at top). GL is Y-up.

        let sx = screen_tl.x.min(screen_br.x) as i32;
        let sy_top = screen_tl.y.max(screen_br.y) as i32; // Bottom in screen coords (higher value)
        let sy_bot = screen_tl.y.min(screen_br.y) as i32; // Top in screen coords (lower value)

        let sw = (screen_tl.x - screen_br.x).abs() as i32;
        let sh = (screen_tl.y - screen_br.y).abs() as i32;

        // GL Scissor Y is from bottom.
        // Screen Height H.
        // y_gl = H - y_screen_bottom
        let gl_y = screen_height() as i32 - sy_top;

        // Skip if too small
        if sw < 1 || sh < 1 {
            continue;
        }

        // Apply Scissor
        unsafe {
            macroquad::miniquad::gl::glEnable(macroquad::miniquad::gl::GL_SCISSOR_TEST);
            macroquad::miniquad::gl::glScissor(sx, gl_y, sw, sh);
        }

        // Calculate new Camera Target
        // We need (Target_Center_World) to map to (Portal_Center_Screen).
        // Since we are setting `zoom` to align the scales, we just need to align the centers.
        // Wait, `Camera2D.target` is the point in World Space that maps to the center of the Screen (or viewport).
        // We don't want to center the Target Room on Screen.
        // We want to center it on the *Portal's* Screen position.

        // This suggests we need `offset`.
        // Camera2D.offset is added after scaling?
        // Proj = Scale * (Pos - Target) + Offset?
        // Docs: "offset: camera position in screen space (0.0 - 1.0)"

        // Let's derive it.
        // We know M_m (Model Matrix) maps Child -> Parent.
        // Child(0,0) (TopLeft) -> Portal(x,y).
        // Child(w,h) -> Portal(x+w, y+h).
        // So P_parent = P_child * Scale + Portal_Pos - Target_Pos * Scale.
        // (Assuming P_child and P_parent are absolute coords in their respective rooms, but here rooms are separate spaces).

        // Let's treat P_child as local to Target Room. P_parent as local to Parent Room.
        // P_parent = (P_child - Target.Rect.TopLeft) * Scale + Portal.Rect.TopLeft.
        // Wait, rooms are positioned in their own world space.
        // Let's assume Room Rect defines its boundaries in its own space.

        // So:
        // P_parent = (P_child - Target.Rect.x) * ScaleX + Portal.Rect.x

        // We want to create a Camera C_child such that C_child(P_child) = C_parent(P_parent).
        // C_parent(P) = (P - T_p) * Z_p
        // C_child(P) = (P - T_c) * Z_c

        // Substitute P_parent:
        // (P_parent - T_p) * Z_p
        // = ( ((P_child - TargetX) * S + PortalX) - T_p ) * Z_p
        // = ( (P_child * S - TargetX * S + PortalX) - T_p ) * Z_p
        // = ( P_child * S - (TargetX * S - PortalX + T_p) ) * Z_p
        // = P_child * (S * Z_p) - (TargetX * S - PortalX + T_p) * Z_p

        // We want this to match (P_child - T_c) * Z_c = P_child * Z_c - T_c * Z_c.

        // So:
        // 1. Z_c = S * Z_p (We already guessed this! `new_zoom`).
        // 2. T_c * Z_c = (TargetX * S - PortalX + T_p) * Z_p
        //    T_c * (S * Z_p) = (TargetX * S - PortalX + T_p) * Z_p
        //    T_c * S = TargetX * S - PortalX + T_p
        //    T_c = TargetX - PortalX / S + T_p / S

        // Let's verify dimensions.
        // T_c (New Target) = TargetX (Child Base) - (PortalX (Parent Base) - T_p (Parent Target)) / S
        // Note: PortalX is in Parent Space. T_p is in Parent Space.
        // (PortalX - T_p) is the vector from Parent Camera Target to Portal Position.
        // We divide by S to map it to Child Space scale.
        // Then we subtract it from Child Base.
        // Seems correct. If Portal is to the right of Parent Target, New Target should be to the left of Child Base?
        // Wait.
        // If Portal is at 10, Target at 0. T_p at 0.
        // Camera looks at 0. Portal is at 10 (Right).
        // Through the portal, we should see Child.
        // The Child's Center (let's say 50) should be at Portal Center (10).
        // So the Camera needs to look at...
        // Actually, T_c is the point in Child Space that maps to screen center.
        // If Screen Center maps to T_p (0), and Portal is at 10.
        // The Portal is shifted +10 relative to center.
        // In Child Space (scaled down), that shift corresponds to +10/S.
        // So the point in Child Space that maps to Screen Center should be "Child Point corresponding to Portal" - 10/S?
        // No.
        // Center of Screen -> T_p.
        // T_p corresponds to some point P_c in Child Space?
        // P_parent = T_p.
        // T_p = (P_c - TargetX) * S + PortalX
        // T_p - PortalX = (P_c - TargetX) * S
        // (T_p - PortalX) / S = P_c - TargetX
        // P_c = TargetX + (T_p - PortalX) / S

        // So T_c = TargetX + (T_p - PortalX) / S.

        // This `P_c` is the point in Child Space that corresponds to `T_p` (Screen Center).
        // So `T_c` (New Camera Target) = `P_c`.

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
        draw_recursive(world, portal.target_room_id, child_cam, depth - 1);

        unsafe {
            macroquad::miniquad::gl::glDisable(macroquad::miniquad::gl::GL_SCISSOR_TEST);
        }
    }
}
