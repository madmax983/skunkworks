mod fs_system;
mod safe_gl;
mod world;

use fs_system::scan_dir;
use macroquad::prelude::*;
use safe_gl::{clear_depth_buffer, ScopedScissor};
use world::{Portal, Room};

const MOVE_SPEED: f32 = 0.2;
const LOOK_SPEED: f32 = 0.003;

struct FirstPersonCamera {
    pos: Vec3,
    yaw: f32,
    pitch: f32,
}

impl FirstPersonCamera {
    fn new(pos: Vec3) -> Self {
        Self {
            pos,
            yaw: -90.0f32.to_radians(),
            pitch: 0.0,
        }
    }

    fn update(&mut self) {
        let _delta = get_frame_time();

        // Mouse Look
        let mouse_delta = mouse_delta_position();
        let mx = mouse_delta.x;
        let my = mouse_delta.y;

        self.yaw += mx * LOOK_SPEED * -1.0;
        self.pitch += my * LOOK_SPEED;
        self.pitch = self.pitch.clamp(-1.5, 1.5);

        // Movement
        let front = self.forward();
        let right = front.cross(vec3(0.0, 1.0, 0.0)).normalize();
        let up = vec3(0.0, 1.0, 0.0);

        if is_key_down(KeyCode::W) {
            self.pos += front * MOVE_SPEED;
        }
        if is_key_down(KeyCode::S) {
            self.pos -= front * MOVE_SPEED;
        }
        if is_key_down(KeyCode::A) {
            self.pos -= right * MOVE_SPEED;
        }
        if is_key_down(KeyCode::D) {
            self.pos += right * MOVE_SPEED;
        }
        if is_key_down(KeyCode::Space) {
            self.pos += up * MOVE_SPEED;
        }
        if is_key_down(KeyCode::LeftShift) {
            self.pos -= up * MOVE_SPEED;
        }
    }

    fn forward(&self) -> Vec3 {
        vec3(
            self.yaw.cos() * self.pitch.cos(),
            self.pitch.sin(),
            self.yaw.sin() * self.pitch.cos(),
        )
        .normalize()
    }

    fn up(&self) -> Vec3 {
        vec3(0.0, 1.0, 0.0)
    }
}

fn world_to_screen(world_pos: Vec3, camera: &FirstPersonCamera) -> Option<Vec2> {
    let aspect = screen_width() / screen_height();
    let fov = 45.0f32.to_radians();

    let projection = Mat4::perspective_rh_gl(fov, aspect, 0.01, 1000.0);
    let view = Mat4::look_at_rh(camera.pos, camera.pos + camera.forward(), camera.up());

    let clip_space = projection * view * world_pos.extend(1.0);

    if clip_space.w <= 0.0 {
        return None;
    }

    let ndc = clip_space.truncate() / clip_space.w;

    let x = (ndc.x + 1.0) * 0.5 * screen_width();
    let y = (1.0 - ndc.y) * 0.5 * screen_height();

    Some(vec2(x, y))
}

fn load_neighbors(room: &mut Room) {
    for portal in &mut room.portals {
        if portal.loaded_room.is_none() {
            let sub_room = scan_dir(&portal.target_path);
            portal.loaded_room = Some(Box::new(sub_room));
        }
    }
}

#[macroquad::main("Impossible Explorer")]
async fn main() {
    let mut camera = FirstPersonCamera::new(vec3(0.0, 0.0, 5.0));

    // Scan current directory
    let current_dir = std::env::current_dir().unwrap();
    let mut root_room = scan_dir(&current_dir);
    load_neighbors(&mut root_room);

    // Set initial position inside room (near center)
    // Room centered at 0,0,0. Floor at -size.y/2 - 0.5.
    // Player height approx 1.7?
    // Floor Y approx -5.5.
    // Cam Y = -4.0.
    camera.pos = vec3(0.0, -2.0, 0.0);

    show_mouse(false);
    set_cursor_grab(true);

    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        camera.update();

        // Portal Collision Logic
        let mut transition_target: Option<(Room, f32)> = None;
        let mut _remove_idx = 0;

        for (i, portal) in root_room.portals.iter_mut().enumerate() {
            // Distance check
            let dist = (camera.pos - portal.pos).length();

            // Simple sphere check for now
            if dist < 2.0 {
                // Determine orientation to adjust yaw
                let p_abs = portal.pos.abs();
                let r_size = root_room.size;
                let mut yaw_adjust = 0.0;

                if (p_abs.z - r_size.z / 2.0).abs() < 1.0 {
                    if portal.pos.z > 0.0 {
                        // Front (+Z)
                        // Walking +Z. Target -Z. Rotate 180.
                        yaw_adjust = std::f32::consts::PI;
                    } else {
                        // Back (-Z)
                        // Walking -Z. Target -Z. Rotate 0.
                        yaw_adjust = 0.0;
                    }
                } else if (p_abs.x - r_size.x / 2.0).abs() < 1.0 {
                    if portal.pos.x > 0.0 {
                        // Right (+X)
                        // Walking +X. Target -Z. Rotate -90.
                        yaw_adjust = -std::f32::consts::FRAC_PI_2;
                    } else {
                        // Left (-X)
                        // Walking -X. Target -Z. Rotate +90.
                        yaw_adjust = std::f32::consts::FRAC_PI_2;
                    }
                }

                if let Some(target_room) = portal.loaded_room.take() {
                    transition_target = Some((*target_room, yaw_adjust));
                    _remove_idx = i;
                }
                break;
            }
        }

        if let Some((new_room, yaw_adjust)) = transition_target {
            // Teleport!
            root_room = new_room;
            load_neighbors(&mut root_room);

            // Set position to entrance of new room
            camera.pos = vec3(0.0, -2.0, root_room.size.z / 2.0 - 2.0);
            camera.yaw += yaw_adjust;
        }

        clear_background(LIGHTGRAY);

        let cam_obj = Camera3D {
            position: camera.pos,
            up: vec3(0.0, 1.0, 0.0),
            target: camera.pos + camera.forward(),
            ..Default::default()
        };

        set_camera(&cam_obj);

        draw_grid(20, 1.0, BLACK, GRAY);

        render_scene(
            &root_room,
            Vec3::ZERO,
            Quat::IDENTITY,
            2,
            &camera,
            &cam_obj,
            None,
        );

        set_default_camera();

        // Crosshair
        draw_line(
            screen_width() / 2.0 - 10.0,
            screen_height() / 2.0,
            screen_width() / 2.0 + 10.0,
            screen_height() / 2.0,
            2.0,
            BLACK,
        );
        draw_line(
            screen_width() / 2.0,
            screen_height() / 2.0 - 10.0,
            screen_width() / 2.0,
            screen_height() / 2.0 + 10.0,
            2.0,
            BLACK,
        );

        draw_text("WASD to Move, Mouse to Look", 10.0, 20.0, 30.0, BLACK);
        draw_text(&format!("Pos: {}", camera.pos), 10.0, 50.0, 20.0, BLACK);
        draw_text(
            &format!("Current: {:?}", root_room.path),
            10.0,
            80.0,
            20.0,
            BLACK,
        );

        next_frame().await
    }
}

fn render_scene(
    room: &Room,
    pos: Vec3,
    rot: Quat,
    depth: i32,
    camera: &FirstPersonCamera,
    cam_obj: &Camera3D,
    parent_scissor: Option<(i32, i32, i32, i32)>,
) {
    if depth < 0 {
        return;
    }

    room.render_transformed(pos, rot);

    // Flush the batch so the room walls are drawn before we change scissor
    set_camera(cam_obj);

    for portal in &room.portals {
        if let Some(target_room) = &portal.loaded_room {
            let mut portal_rot = Quat::IDENTITY;
            let p_abs = portal.pos.abs();
            let r_size = room.size;

            if (p_abs.z - r_size.z / 2.0).abs() < 1.0 {
                if portal.pos.z > 0.0 {
                    portal_rot = Quat::IDENTITY;
                } else {
                    portal_rot = Quat::from_rotation_y(std::f32::consts::PI);
                }
            } else if (p_abs.x - r_size.x / 2.0).abs() < 1.0 {
                if portal.pos.x > 0.0 {
                    portal_rot = Quat::from_rotation_y(-std::f32::consts::FRAC_PI_2);
                } else {
                    portal_rot = Quat::from_rotation_y(std::f32::consts::FRAC_PI_2);
                }
            }

            let total_portal_rot = rot * portal_rot;
            let portal_world_pos = rot * portal.pos + pos;

            if let Some(mut rect) =
                calculate_scissor_rect(portal_world_pos, portal.size, total_portal_rot, camera)
            {
                if let Some(parent) = parent_scissor {
                    rect = intersect_rect(rect, parent);
                }

                if rect.2 <= 0 || rect.3 <= 0 {
                    continue;
                }

                {
                    let _guard = ScopedScissor::new(rect.0, rect.1, rect.2, rect.3);
                    clear_depth_buffer();

                    let target_rot = total_portal_rot;
                    let target_entrance_local = vec3(0.0, -2.0, target_room.size.z / 2.0);

                    let target_pos = portal_world_pos - (target_rot * target_entrance_local);

                    render_scene(
                        target_room,
                        target_pos,
                        target_rot,
                        depth - 1,
                        camera,
                        cam_obj,
                        Some(rect),
                    );

                    // Flush the inner room walls before restoring scissor
                    set_camera(cam_obj);
                }
            }
        }
    }
}

fn intersect_rect(a: (i32, i32, i32, i32), b: (i32, i32, i32, i32)) -> (i32, i32, i32, i32) {
    let x1 = a.0.max(b.0);
    let y1 = a.1.max(b.1);
    let x2 = (a.0 + a.2).min(b.0 + b.2);
    let y2 = (a.1 + a.3).min(b.1 + b.3);

    (x1, y1, (x2 - x1).max(0), (y2 - y1).max(0))
}

fn calculate_scissor_rect(
    pos: Vec3,
    size: Vec2,
    rot: Quat,
    camera: &FirstPersonCamera,
) -> Option<(i32, i32, i32, i32)> {
    let half_w = size.x / 2.0;
    let half_h = size.y / 2.0;

    let corners_local = [
        vec3(-half_w, -half_h, 0.0),
        vec3(half_w, -half_h, 0.0),
        vec3(half_w, half_h, 0.0),
        vec3(-half_w, half_h, 0.0),
    ];

    let mut min_x = f32::MAX;
    let mut max_x = f32::MIN;
    let mut min_y = f32::MAX;
    let mut max_y = f32::MIN;

    let mut all_behind = true;
    let cam_forward = camera.forward();
    let cam_pos = camera.pos;

    for c_local in corners_local {
        let world_pt = pos + rot * c_local;

        let to_pt = world_pt - cam_pos;
        if to_pt.dot(cam_forward) > 0.0 {
            all_behind = false;
        }

        if let Some(screen_pt) = world_to_screen(world_pt, camera) {
            min_x = min_x.min(screen_pt.x);
            max_x = max_x.max(screen_pt.x);
            min_y = min_y.min(screen_pt.y);
            max_y = max_y.max(screen_pt.y);
        }
    }

    if all_behind {
        return None;
    }

    if min_x == f32::MAX {
        return None;
    }

    let sw = screen_width();
    let sh = screen_height();

    min_x = min_x.clamp(0.0, sw);
    max_x = max_x.clamp(0.0, sw);
    min_y = min_y.clamp(0.0, sh);
    max_y = max_y.clamp(0.0, sh);

    if min_x >= max_x || min_y >= max_y {
        return None;
    }

    let x = min_x as i32;
    let h = (max_y - min_y) as i32;
    let y = (sh - max_y) as i32;
    let w = (max_x - min_x) as i32;

    Some((x, y, w, h))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersect_rect() {
        let r1 = (0, 0, 100, 100);
        let r2 = (50, 50, 100, 100);
        let intersection = intersect_rect(r1, r2);
        assert_eq!(intersection, (50, 50, 50, 50));

        let r3 = (200, 200, 50, 50);
        let intersection_none = intersect_rect(r1, r3);
        assert_eq!(intersection_none.2, 0);
        assert_eq!(intersection_none.3, 0);
    }
}
