use macroquad::prelude::*;
use origami::{generate_miura_grid, MiuraParams, Orientation};

mod cipher;
mod glyph;
mod memory;
mod starmap;

use cipher::CipherReveal;
use memory::Memory;

const IMG_WIDTH: u32 = 512;
const IMG_HEIGHT: u32 = 512;

fn window_conf() -> Conf {
    Conf {
        window_title: "Crumpled Cipher 🦢🗝️".to_string(),
        window_width: 1280,
        window_height: 720,
        high_dpi: true,
        sample_count: 4,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // 1. Initialize Memory (Cipher Mode)
    let mut memory = Memory::new(IMG_WIDTH, IMG_HEIGHT);

    // 2. Initialize Miura-Ori
    let cols = 30;
    let rows = 30;

    let params = MiuraParams {
        a: 4.0,
        b: 4.0,
        gamma: 80.0f32.to_radians(),
        orientation: Orientation::Horizontal,
    };

    // Simulation State
    let mut extension: f32 = 1.0; // 0.0 = folded, 1.0 = flat

    // Camera State
    let mut cam_yaw: f32 = 0.0;
    let mut cam_pitch: f32 = 0.5;
    let mut cam_dist: f32 = 150.0;
    let target = vec3(0.0, 0.0, 0.0);

    // Texture
    let mut mq_image = Image {
        bytes: memory.visible_layer.as_raw().clone(),
        width: IMG_WIDTH as u16,
        height: IMG_HEIGHT as u16,
    };
    let texture = Texture2D::from_image(&mq_image);
    texture.set_filter(FilterMode::Linear);

    loop {
        // --- Input ---
        if is_mouse_button_down(MouseButton::Right) {
            let delta = mouse_delta_position();
            cam_yaw -= delta.x * 5.0;
            cam_pitch += delta.y * 5.0;
            cam_pitch = cam_pitch.clamp(0.1, 1.5);
        }
        cam_dist -= mouse_wheel().1 * 10.0;
        cam_dist = cam_dist.clamp(50.0, 800.0);

        // Fold Control
        let old_extension = extension;
        if is_key_down(KeyCode::Up) {
            extension += 0.01;
        }
        if is_key_down(KeyCode::Down) {
            extension -= 0.01;
        }
        extension = extension.clamp(0.05, 1.0);

        // --- Logic: Stress/Reveal Calculation ---
        let stress_factor = (1.0 - extension).max(0.0);
        let fold_delta = (extension - old_extension).abs();

        if fold_delta > 0.0001 || stress_factor > 0.8 {
            let amount = stress_factor * 0.05 + fold_delta * 2.0;

            let dx = IMG_WIDTH as f32 / cols as f32;
            let dy = IMG_HEIGHT as f32 / rows as f32;

            for j in 0..=rows {
                let y = (j as f32 * dy) as u32;
                memory.reveal_line(0, y, IMG_WIDTH, y, amount);
            }

            for i in 0..=cols {
                let x = (i as f32 * dx) as u32;
                memory.reveal_line(x, 0, x, IMG_HEIGHT, amount);
            }
        }

        // --- Update Memory ---
        // memory.erode(); // No erosion, just reveal

        // --- Update Visuals ---
        mq_image
            .bytes
            .copy_from_slice(memory.visible_layer.as_raw());
        texture.update(&mq_image);

        // Update Mesh
        let grid_points = generate_miura_grid(params, (cols, rows), extension);

        // Build Macroquad Mesh (same as crumpled-memory)
        let mut vertices = Vec::with_capacity(cols * rows * 4);
        let mut indices = Vec::with_capacity(cols * rows * 6);
        let mut idx: u16 = 0;
        let width_pts = cols + 1;

        for j in 0..rows {
            for i in 0..cols {
                let p0_idx = j * width_pts + i;
                let p1_idx = j * width_pts + (i + 1);
                let p2_idx = (j + 1) * width_pts + (i + 1);
                let p3_idx = (j + 1) * width_pts + i;

                let v0 = grid_points[p0_idx];
                let v1 = grid_points[p1_idx];
                let v2 = grid_points[p2_idx];
                let v3 = grid_points[p3_idx];

                let u0 = i as f32 / cols as f32;
                let v0_uv = j as f32 / rows as f32;
                let u1 = (i + 1) as f32 / cols as f32;
                let v1_uv = j as f32 / rows as f32;
                let u2 = (i + 1) as f32 / cols as f32;
                let v2_uv = (j + 1) as f32 / rows as f32;
                let u3 = i as f32 / cols as f32;
                let v3_uv = (j + 1) as f32 / rows as f32;

                vertices.push(Vertex {
                    position: v0,
                    uv: vec2(u0, v0_uv),
                    color: [255, 255, 255, 255],
                    normal: vec4(0., 1., 0., 0.),
                });
                vertices.push(Vertex {
                    position: v1,
                    uv: vec2(u1, v1_uv),
                    color: [255, 255, 255, 255],
                    normal: vec4(0., 1., 0., 0.),
                });
                vertices.push(Vertex {
                    position: v2,
                    uv: vec2(u2, v2_uv),
                    color: [255, 255, 255, 255],
                    normal: vec4(0., 1., 0., 0.),
                });
                vertices.push(Vertex {
                    position: v3,
                    uv: vec2(u3, v3_uv),
                    color: [255, 255, 255, 255],
                    normal: vec4(0., 1., 0., 0.),
                });

                indices.push(idx + 0);
                indices.push(idx + 1);
                indices.push(idx + 3);
                indices.push(idx + 1);
                indices.push(idx + 2);
                indices.push(idx + 3);
                idx += 4;
            }
        }

        let mesh = Mesh {
            vertices,
            indices,
            texture: Some(texture.clone()),
        };

        // --- Render ---
        clear_background(Color::new(0.05, 0.05, 0.05, 1.0));

        let cam_pos = vec3(
            target.x + cam_dist * cam_yaw.cos() * cam_pitch.cos(),
            target.y + cam_dist * cam_pitch.sin(),
            target.z + cam_dist * cam_yaw.sin() * cam_pitch.cos(),
        );

        set_camera(&Camera3D {
            position: cam_pos,
            target,
            up: vec3(0.0, 1.0, 0.0),
            ..Default::default()
        });

        draw_mesh(&mesh);

        set_default_camera();

        // UI
        draw_text("Crumpled Cipher 🦢🗝️", 20.0, 30.0, 30.0, WHITE);
        draw_text(
            &format!("Extension: {:.1}%", extension * 100.0),
            20.0,
            50.0,
            20.0,
            YELLOW,
        );
        draw_text(
            "UP/DOWN: Fold/Unfold to Reveal | Mouse: Orbit",
            20.0,
            70.0,
            20.0,
            LIGHTGRAY,
        );
        draw_text("The truth is in the fold.", 20.0, 90.0, 16.0, SKYBLUE);

        next_frame().await
    }
}
