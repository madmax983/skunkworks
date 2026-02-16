mod heap;
use heap::{generate_mock_heap, TesseractNode};
use macroquad::prelude::*;
use ::rand::rngs::StdRng;
use ::rand::{Rng, SeedableRng};

const ROOM_SIZE: f32 = 40.0; // Half-extent (goes from -40 to 40)
const CHILD_SIZE_BASE: f32 = 8.0; // Half-extent of children in the room

struct LayoutBox {
    center: Vec3,
    half_size: f32,
}

#[macroquad::main("Heap Tesseract")]
async fn main() {
    let heap_root = generate_mock_heap(1337);

    // Camera State (Local to current room)
    let mut cam_pos = vec3(0.0, 0.0, 30.0);
    let mut cam_yaw: f32 = -90.0;
    let mut cam_pitch: f32 = 0.0;

    let mut last_mouse_pos: Vec2 = mouse_position().into();
    let mut grabbed = false;

    // Navigation State
    let mut path: Vec<usize> = Vec::new();
    let mut transition_cooldown = 0.0;

    loop {
        clear_background(Color::new(0.05, 0.05, 0.05, 1.0));

        // --- Context Resolution ---
        let mut current_node = &heap_root;
        for &idx in &path {
            if idx < current_node.children.len() {
                current_node = &current_node.children[idx];
            } else {
                break; // Should not happen
            }
        }

        // --- Input ---
        if is_key_pressed(KeyCode::Tab) {
            grabbed = !grabbed;
            set_cursor_grab(grabbed);
            show_mouse(!grabbed);
        }

        let dt = get_frame_time();
        if transition_cooldown > 0.0 {
            transition_cooldown -= dt;
        }

        let speed = if is_key_down(KeyCode::LeftControl) { 30.0 } else { 10.0 } * dt;

        let current_mouse_pos: Vec2 = mouse_position().into();
        let mouse_delta = current_mouse_pos - last_mouse_pos;
        last_mouse_pos = current_mouse_pos;

        if grabbed {
            cam_yaw += mouse_delta.x * 0.1;
            cam_pitch += mouse_delta.y * 0.1;
            cam_pitch = cam_pitch.clamp(-89.0, 89.0);
        }

        // Movement Vectors
        let front = vec3(
            cam_yaw.to_radians().cos() * cam_pitch.to_radians().cos(),
            cam_pitch.to_radians().sin(),
            cam_yaw.to_radians().sin() * cam_pitch.to_radians().cos(),
        ).normalize();
        let right = front.cross(vec3(0.0, 1.0, 0.0)).normalize();
        let up = vec3(0.0, 1.0, 0.0);

        // Physics/Move
        if is_key_down(KeyCode::W) { cam_pos += front * speed; }
        if is_key_down(KeyCode::S) { cam_pos -= front * speed; }
        if is_key_down(KeyCode::A) { cam_pos -= right * speed; }
        if is_key_down(KeyCode::D) { cam_pos += right * speed; }
        if is_key_down(KeyCode::Space) { cam_pos += up * speed; }
        if is_key_down(KeyCode::LeftShift) { cam_pos -= up * speed; }

        // --- Tardis Logic ---
        if transition_cooldown <= 0.0 {
            // 1. Check if we are Inside a Child
            let mut rng = StdRng::seed_from_u64(current_node.layout_seed);
            let child_layout = generate_layout(&mut rng, current_node.children.len());

            let mut entered_child = None;
            for (i, bbox) in child_layout.iter().enumerate() {
                // Simple AABB check
                let min = bbox.center - vec3(bbox.half_size, bbox.half_size, bbox.half_size);
                let max = bbox.center + vec3(bbox.half_size, bbox.half_size, bbox.half_size);

                if cam_pos.x > min.x && cam_pos.x < max.x &&
                   cam_pos.y > min.y && cam_pos.y < max.y &&
                   cam_pos.z > min.z && cam_pos.z < max.z {
                       entered_child = Some((i, bbox));
                       break;
                   }
            }

            if let Some((idx, bbox)) = entered_child {
                // Enter Child
                if !current_node.children[idx].children.is_empty() {
                    path.push(idx);
                    // Transform position: Map from Box Range to Room Range
                    // P_new = (P_old - Center) * (RoomSize / BoxHalfSize)
                    let scale_factor = ROOM_SIZE / bbox.half_size;
                    cam_pos = (cam_pos - bbox.center) * scale_factor;
                    transition_cooldown = 0.5;
                }
            } else if !path.is_empty() {
                // 2. Check if we are Outside the Room
                if cam_pos.abs().max_element() > ROOM_SIZE + 2.0 {
                    // Exit to Parent
                    let exited_idx = path.pop().unwrap();

                    // We need parent's layout to know where to put the camera
                    // Parent is the node BEFORE the one we just popped
                    let mut parent_node = &heap_root;
                    for &idx in &path {
                        parent_node = &parent_node.children[idx];
                    }

                    // Reconstruct parent layout
                    let mut p_rng = StdRng::seed_from_u64(parent_node.layout_seed);
                    let p_layout = generate_layout(&mut p_rng, parent_node.children.len());
                    let child_bbox = &p_layout[exited_idx];

                    // Transform position: Map from Room Range to Box Range
                    // P_new = (P_old / Scale) + Center
                    let scale_factor = ROOM_SIZE / child_bbox.half_size;
                    cam_pos = (cam_pos / scale_factor) + child_bbox.center;

                    transition_cooldown = 0.5;
                }
            }
        }

        // --- Render ---
        set_camera(&Camera3D {
            position: cam_pos,
            up,
            target: cam_pos + front,
            ..Default::default()
        });

        // Draw Room Boundary
        draw_cube_wires(vec3(0.0, 0.0, 0.0), vec3(ROOM_SIZE * 2.0, ROOM_SIZE * 2.0, ROOM_SIZE * 2.0), GRAY);

        // Draw Children
        let mut rng = StdRng::seed_from_u64(current_node.layout_seed);
        let layout = generate_layout(&mut rng, current_node.children.len());

        // Store screen positions for text overlay
        let mut labels = Vec::new();
        let cam = Camera3D {
            position: cam_pos,
            up,
            target: cam_pos + front,
            ..Default::default()
        };

        for (i, bbox) in layout.iter().enumerate() {
            let child = &current_node.children[i];
            let size = vec3(bbox.half_size * 2.0, bbox.half_size * 2.0, bbox.half_size * 2.0);

            // Visual Pulse
            let pulse = (get_time() * 2.0 + i as f64).sin() as f32 * 0.1 + 0.9;

            // If it has children (is enterable), draw wires + internal glow
            if !child.children.is_empty() {
                draw_cube_wires(bbox.center, size, child.color);

                // "Portal" look - smaller inner cube
                let inner_color = Color::new(child.color.r, child.color.g, child.color.b, 0.15);
                draw_cube(bbox.center, size * 0.9 * pulse, None, inner_color);

                // Core
                draw_sphere(bbox.center, bbox.half_size * 0.4, None, child.color);
            } else {
                // Leaf Node - solid block
                draw_cube(bbox.center, size * pulse, None, child.color);
            }

            // Project for Label
            if let Some(screen_pos) = world_to_screen(&cam, bbox.center) {
                labels.push((screen_pos, &child.name, child.size));
            }
        }

        // Draw some "Data Particles" floating in the room
        let t = get_time();
        for i in 0..50 {
            let offset = i as f64 * 0.5;
            let px = (t * 2.0 + offset).sin() as f32 * (ROOM_SIZE * 0.8);
            let py = (t * 1.3 + offset).cos() as f32 * (ROOM_SIZE * 0.8);
            let pz = (t * 0.7 + offset).sin() as f32 * (ROOM_SIZE * 0.8);

            draw_sphere(vec3(px, py, pz), 0.2, None, Color::new(0.5, 1.0, 1.0, 0.5));
            draw_line_3d(vec3(px, py, pz), vec3(px, py - 1.0, pz), Color::new(0.5, 1.0, 1.0, 0.2));
        }

        set_default_camera();

        // Draw Labels
        for (pos, name, size) in labels {
             // Check if within screen bounds
             if pos.x > 0.0 && pos.x < screen_width() && pos.y > 0.0 && pos.y < screen_height() {
                 draw_text(name, pos.x, pos.y, 20.0, WHITE);
                 draw_text(&format!("{} B", size), pos.x, pos.y + 15.0, 15.0, GRAY);
             }
        }

        // --- UI ---
        draw_text(&format!("Depth: {}", path.len()), 10.0, 30.0, 30.0, WHITE);
        draw_text(&format!("Node: {}", current_node.name), 10.0, 60.0, 20.0, WHITE);
        if !path.is_empty() {
            draw_text("Fly OUTSIDE the box to go UP", 10.0, 90.0, 20.0, YELLOW);
        }
        draw_text("Fly INTO a box to go DEEPER", 10.0, 110.0, 20.0, GREEN);

        next_frame().await
    }
}

// Deterministic Layout Generator (Random Packing)
fn generate_layout(rng: &mut StdRng, count: usize) -> Vec<LayoutBox> {
    let mut boxes: Vec<LayoutBox> = Vec::new();
    for _ in 0..count {
        // Try to find a spot that doesn't overlap too much?
        // For simplicity: Grid or just Random non-overlapping
        let mut best_pos = vec3(0.0, 0.0, 0.0);
        let mut valid = false;

        let size = CHILD_SIZE_BASE * rng.gen_range(0.8..1.2);

        for _attempt in 0..10 {
            let p = vec3(
                rng.gen_range(-ROOM_SIZE + size..ROOM_SIZE - size),
                rng.gen_range(-ROOM_SIZE + size..ROOM_SIZE - size),
                rng.gen_range(-ROOM_SIZE + size..ROOM_SIZE - size),
            );

            // Check overlap
            let mut overlaps = false;
            for b in &boxes {
                if p.distance(b.center) < (size + b.half_size) * 1.2 {
                    overlaps = true;
                    break;
                }
            }
            if !overlaps {
                best_pos = p;
                valid = true;
                break;
            }
        }

        if valid {
            boxes.push(LayoutBox {
                center: best_pos,
                half_size: size,
            });
        }
    }
    boxes
}

// 3D to 2D Projection helper
fn world_to_screen(cam: &Camera3D, pos: Vec3) -> Option<Vec2> {
    let aspect = screen_width() / screen_height();
    let fov = 45.0f32.to_radians(); // Standard FOV for Macroquad?

    // Macroquad uses glam underneath.
    let proj = Mat4::perspective_rh_gl(fov, aspect, 0.01, 1000.0);
    let view = Mat4::look_at_rh(cam.position, cam.target, cam.up);

    // transform
    let clip_space = proj * view * pos.extend(1.0);

    // Behind camera check
    if clip_space.w <= 0.0 {
        return None;
    }

    let ndc = clip_space.truncate() / clip_space.w;

    // Map NDC (-1 to 1) to Screen (0 to w, h to 0)
    // Note: Y is usually inverted in screen coords (0 at top) vs GL (0 at bottom)
    let x = (ndc.x + 1.0) * 0.5 * screen_width();
    let y = (1.0 - ndc.y) * 0.5 * screen_height();

    Some(vec2(x, y))
}
