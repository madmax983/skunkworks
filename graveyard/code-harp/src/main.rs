use macroquad::prelude::*;
mod scanner;
mod audio;

use scanner::StringEntity;
use audio::PluckEvent;

struct VisualString {
    entity: StringEntity,
    vibration: f32,
    phase: f32,
    last_pluck: f64,
}

#[macroquad::main("Code Harp")]
async fn main() {
    let raw_strings = scanner::scan_codebase(".");
    let mut strings: Vec<VisualString> = raw_strings.into_iter().map(|s| VisualString {
        entity: s,
        vibration: 0.0,
        phase: 0.0,
        last_pluck: 0.0,
    }).collect();

    let audio_sender = match audio::init() {
        Ok(s) => Some(s),
        Err(e) => {
            eprintln!("Audio init failed: {}", e);
            None
        }
    };

    let mut camera = Camera3D {
        position: vec3(0.0, 0.0, 30.0),
        target: vec3(0.0, 0.0, 0.0),
        up: vec3(0.0, 1.0, 0.0),
        fovy: 45.0,
        aspect: Some(screen_width() / screen_height()),
        projection: Projection::Perspective,
        render_target: None,
        viewport: None,
        z_near: 0.1,
        z_far: 1000.0,
    };

    let mut yaw: f32 = -90.0;
    let mut pitch: f32 = 0.0;

    let mut last_mouse_pos = mouse_position();
    let mut grabbed = false;

    loop {
        let dt = get_frame_time();

        // Input Handling
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        // Mouse look
        if is_mouse_button_pressed(MouseButton::Right) {
            grabbed = true;
            // set_cursor_grab(true); // Not always reliable in some environments
            show_mouse(false);
            last_mouse_pos = mouse_position();
        }
        if is_mouse_button_released(MouseButton::Right) {
            grabbed = false;
            // set_cursor_grab(false);
            show_mouse(true);
        }

        if grabbed {
            let (mx, my) = mouse_position();
            let dx = mx - last_mouse_pos.0;
            let dy = my - last_mouse_pos.1;

            yaw += dx * 0.1;
            pitch -= dy * 0.1;
            pitch = pitch.clamp(-89.0, 89.0);

            // Re-center mouse logic is tricky without winit access directly
            // So we just update last_mouse_pos
            last_mouse_pos = (mx, my);
        }

        let front = vec3(
            yaw.to_radians().cos() * pitch.to_radians().cos(),
            pitch.to_radians().sin(),
            yaw.to_radians().sin() * pitch.to_radians().cos(),
        ).normalize();

        let right = front.cross(vec3(0.0, 1.0, 0.0)).normalize();
        // let up = right.cross(front).normalize(); // Not used directly for movement usually

        let speed = 10.0 * dt;
        if is_key_down(KeyCode::W) { camera.position += front * speed; }
        if is_key_down(KeyCode::S) { camera.position -= front * speed; }
        if is_key_down(KeyCode::A) { camera.position -= right * speed; }
        if is_key_down(KeyCode::D) { camera.position += right * speed; }
        if is_key_down(KeyCode::Q) { camera.position.y -= speed; }
        if is_key_down(KeyCode::E) { camera.position.y += speed; }

        camera.target = camera.position + front;

        // Logic
        let current_time = get_time();
        let player_pos = camera.position;

        let mut hovered_path = String::new();

        for s in &mut strings {
            // Check collision with player
            let d = distance_point_segment(player_pos, s.entity.start, s.entity.end);

            // Pluck radius
            if d < 1.0 { // 1.0 unit radius
                // If not plucked recently
                if current_time - s.last_pluck > 0.2 {
                    s.last_pluck = current_time;
                    s.vibration = 1.0;

                    if let Some(sender) = &audio_sender {
                        let _ = sender.send(PluckEvent {
                            frequency: s.entity.frequency,
                            damping: s.entity.damping,
                            gain: 0.8,
                        });
                    }
                }
                hovered_path = s.entity.path.clone();
            }

            // Visual decay
            s.vibration *= 0.95; // Visual damping
            s.phase += s.entity.frequency * dt * 0.05; // Slow down visual phase
        }

        // Draw
        clear_background(BLACK);

        set_camera(&camera);

        draw_grid(20, 1.0, BLACK, GRAY);

        for s in &strings {
            let start = s.entity.start;
            let end = s.entity.end;

            // Apply vibration
            if s.vibration > 0.01 {
                let mid = (start + end) * 0.5;
                // Randomize direction or rotate phase
                let offset = vec3(s.phase.sin(), 0.0, s.phase.cos()) * s.vibration * 0.5;

                // Draw as two segments
                draw_line_3d(start, mid + offset, s.entity.color);
                draw_line_3d(mid + offset, end, s.entity.color);
            } else {
                draw_line_3d(start, end, s.entity.color);
            }
        }

        set_default_camera();

        draw_text("CODE HARP", 10.0, 30.0, 30.0, WHITE);
        draw_text("WASD + Right Click to Fly. Touch strings to play.", 10.0, 50.0, 20.0, GRAY);

        if !hovered_path.is_empty() {
             draw_text(&format!("Touching: {}", hovered_path), 10.0, 80.0, 20.0, YELLOW);
        }

        draw_text(&format!("FPS: {}", get_fps()), screen_width() - 100.0, 30.0, 20.0, WHITE);

        next_frame().await;
    }
}

fn distance_point_segment(p: Vec3, a: Vec3, b: Vec3) -> f32 {
    let ab = b - a;
    let len_sq = ab.length_squared();
    if len_sq < 1e-6 {
        return (p - a).length();
    }
    let t = ((p - a).dot(ab) / len_sq).clamp(0.0, 1.0);
    let closest = a + ab * t;
    (p - closest).length()
}
