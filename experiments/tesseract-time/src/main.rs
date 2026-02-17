mod math4d;
mod attractor;

use macroquad::prelude::*;
use math4d::Vec4;
use attractor::Lissajous4D;

fn generate_tesseract() -> (Vec<Vec4>, Vec<(usize, usize)>) {
    let mut verts = Vec::new();
    for i in 0..16 {
        let x = if i & 1 != 0 { 1.0 } else { -1.0 };
        let y = if i & 2 != 0 { 1.0 } else { -1.0 };
        let z = if i & 4 != 0 { 1.0 } else { -1.0 };
        let w = if i & 8 != 0 { 1.0 } else { -1.0 };
        verts.push(Vec4::new(x, y, z, w));
    }

    let mut edges = Vec::new();
    for i in 0..16 {
        for j in (i+1)..16 {
            // Check hamming distance
            let diff: usize = i ^ j;
            if diff.count_ones() == 1 {
                edges.push((i, j));
            }
        }
    }
    (verts, edges)
}

#[macroquad::main("Tesseract Time")]
async fn main() {
    let (base_verts, edges) = generate_tesseract();
    let mut attractor = Lissajous4D::new();
    let mut trail: Vec<Vec4> = Vec::new();
    let max_trail_len = 2000;

    // Camera state
    let mut cam_yaw: f32 = 0.0;
    let mut cam_pitch: f32 = 0.0;
    let mut cam_dist: f32 = 6.0;

    // 4D Rotation state
    let mut angle_xw = 0.0;
    let mut angle_yw = 0.0;
    let mut angle_zw = 0.0;
    let mut auto_rotate = true;

    loop {
        let dt = get_frame_time();

        // Input
        if is_key_down(KeyCode::Left) { cam_yaw += 2.0 * dt; }
        if is_key_down(KeyCode::Right) { cam_yaw -= 2.0 * dt; }
        if is_key_down(KeyCode::Up) { cam_pitch += 2.0 * dt; }
        if is_key_down(KeyCode::Down) { cam_pitch -= 2.0 * dt; }

        // Zoom
        if is_key_down(KeyCode::W) { cam_dist -= 5.0 * dt; }
        if is_key_down(KeyCode::S) { cam_dist += 5.0 * dt; }

        // 4D Rotation Controls
        if is_key_down(KeyCode::Q) { angle_xw += dt; auto_rotate = false; }
        if is_key_down(KeyCode::E) { angle_xw -= dt; auto_rotate = false; }
        if is_key_down(KeyCode::R) { angle_yw += dt; auto_rotate = false; }
        if is_key_down(KeyCode::F) { angle_yw -= dt; auto_rotate = false; }

        if is_key_pressed(KeyCode::Space) { auto_rotate = !auto_rotate; }

        if auto_rotate {
            angle_xw += dt * 0.3;
            angle_zw += dt * 0.2;
        }

        // Update Attractor
        let new_pt = attractor.update(dt);
        trail.push(new_pt);
        if trail.len() > max_trail_len {
            trail.remove(0);
        }

        // Setup 3D Camera
        let cam_pos = vec3(
            cam_dist * cam_pitch.cos() * cam_yaw.sin(),
            cam_dist * cam_pitch.sin(),
            cam_dist * cam_pitch.cos() * cam_yaw.cos(),
        );

        set_camera(&Camera3D {
            position: cam_pos,
            target: vec3(0.0, 0.0, 0.0),
            up: vec3(0.0, 1.0, 0.0),
            ..Default::default()
        });

        clear_background(BLACK);

        // Transformation Pipeline
        // 1. Rotate in 4D (user controls)
        // 2. Project to 3D (perspective)

        let rotate_4d = |v: Vec4| -> Vec4 {
            let mut v = v;
            v = v.rotate_xw(angle_xw);
            v = v.rotate_yw(angle_yw);
            v = v.rotate_zw(angle_zw);
            v
        };

        let transform = |v: Vec4| -> Vec3 {
            rotate_4d(v).project_to_3d(3.0) // Camera at W=3
        };

        // Draw Tesseract Edges
        for &(i, j) in &edges {
            let v1 = base_verts[i];
            let v2 = base_verts[j];

            let p1 = transform(v1);
            let p2 = transform(v2);

            // Color based on W coordinate after full rotation
            let w1 = rotate_4d(v1).w;
            let w2 = rotate_4d(v2).w;
            let depth = (w1 + w2) * 0.5;
            let alpha = ((depth + 2.0) / 4.0).clamp(0.2, 1.0);

            draw_line_3d(p1, p2, Color::new(0.0, 1.0, 1.0, alpha));
        }

        // Draw Trail
        for i in 0..trail.len().saturating_sub(1) {
            let v1 = trail[i];
            let v2 = trail[i+1];

            let p1 = transform(v1);
            let p2 = transform(v2);

            // Color mapping: Map rotated W coordinate to Red/Blue
            let w_rot = rotate_4d(v1).w;

            let r = (w_rot + 1.0) * 0.5;
            let b = 1.0 - r;
            let color = Color::new(r, 0.5, b, 1.0);

            draw_line_3d(p1, p2, color);
        }

        // UI
        set_default_camera();
        draw_text("Genesis: Tesseract Time", 10.0, 20.0, 30.0, WHITE);
        draw_text("Arrows: Rotate 3D Camera", 10.0, 40.0, 20.0, GRAY);
        draw_text("Q/E, R/F: Rotate 4D Planes", 10.0, 60.0, 20.0, GRAY);
        draw_text("Space: Toggle Auto-Rotate", 10.0, 80.0, 20.0, GRAY);
        draw_text(format!("Trail Points: {}", trail.len()).as_str(), 10.0, 100.0, 20.0, LIGHTGRAY);

        next_frame().await
    }
}
