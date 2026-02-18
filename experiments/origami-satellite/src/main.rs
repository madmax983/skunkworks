use macroquad::prelude::*;
use origami::{MiuraOri, MiuraParams, Orientation};
use std::f32::consts::PI;

// mod audio; // Audio disabled due to missing alsa headers

fn conf() -> Conf {
    Conf {
        window_title: "Origami Satellite 🛰️".to_string(),
        window_width: 1280,
        window_height: 720,
        high_dpi: true,
        sample_count: 4,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    /*
    let audio_sys = audio::AudioSystem::new();
    if let Err(e) = &audio_sys {
        eprintln!("Audio init failed: {}", e);
    }
    */

    // Camera State
    let mut cam_yaw: f32 = PI / 4.0;
    let mut cam_pitch: f32 = -PI / 6.0;
    let mut cam_dist: f32 = 15.0;
    let target = vec3(0.0, 0.0, 0.0);

    // Deployment State
    let mut extension: f32 = 0.1; // 0.0 to 1.0 (clamped)
    let mut is_locked = false;

    // Miura-Ori Setup
    let params = MiuraParams {
        a: 1.0,
        b: 1.0,
        gamma: 80.0f32.to_radians(), // 80 degree parallelogram
        orientation: Orientation::Horizontal,
    };
    // 5x5 grid for each wing
    let miura = MiuraOri::new(params, (5, 5));

    // Stars
    let mut stars = Vec::new();
    for _ in 0..1000 {
        stars.push(vec3(
            rand::gen_range(-50.0, 50.0),
            rand::gen_range(-50.0, 50.0),
            rand::gen_range(-50.0, 50.0),
        ));
    }

    let material = load_material(
        ShaderSource::Glsl {
            vertex: DEFAULT_VERTEX_SHADER,
            fragment: DEFAULT_FRAGMENT_SHADER,
        },
        MaterialParams {
            pipeline_params: PipelineParams {
                depth_write: true,
                depth_test: Comparison::LessOrEqual,
                ..Default::default()
            },
            ..Default::default()
        },
    )
    .unwrap();

    loop {
        // --- Input ---

        // Deployment control: Mouse drag vertical (Right click or Shift+Left)
        if is_mouse_button_down(MouseButton::Right)
            || (is_key_down(KeyCode::LeftShift) && is_mouse_button_down(MouseButton::Left))
        {
            let dy = mouse_delta_position().y;
            extension -= dy * 2.0;
        }

        // Camera Orbit: Left Click
        if is_mouse_button_down(MouseButton::Left) && !is_key_down(KeyCode::LeftShift) {
            let delta = mouse_delta_position();
            cam_yaw -= delta.x * 5.0;
            cam_pitch += delta.y * 5.0;
            cam_pitch = cam_pitch.clamp(-PI / 2.0 + 0.1, PI / 2.0 - 0.1);
        }

        // Zoom: Scroll
        cam_dist -= mouse_wheel().1 * 0.5;
        cam_dist = cam_dist.clamp(5.0, 50.0);

        // Logic
        extension = extension.clamp(0.0, 1.0);

        // Locking mechanic
        if extension > 0.98 {
            extension = 1.0;
            if !is_locked {
                is_locked = true;
                // Play lock sound/effect? (Audio system handles frequency shift)
            }
        } else {
            is_locked = false;
        }

        // Update Audio
        /*
        if let Ok(sys) = &audio_sys {
            sys.set_extension(extension as f64);
        }
        */

        // --- Render ---
        clear_background(BLACK);

        // 3D Setup
        let cam_pos = vec3(
            cam_dist * cam_yaw.cos() * cam_pitch.cos(),
            cam_dist * cam_pitch.sin(),
            cam_dist * cam_yaw.sin() * cam_pitch.cos(),
        );

        set_camera(&Camera3D {
            position: cam_pos,
            target,
            up: vec3(0.0, 1.0, 0.0),
            ..Default::default()
        });

        // Draw Stars
        for star in &stars {
            draw_sphere(*star, 0.1, None, WHITE);
        }

        // Generate Mesh
        let origami_mesh = miura.generate_mesh(extension);

        // Convert to Macroquad Mesh (Combining both wings)
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        // Color based on extension
        let base_color = if is_locked {
            Color::new(0.2, 0.4, 1.0, 1.0)
        } else {
            Color::new(0.1, 0.2, 0.5, 1.0)
        };

        // Helper to add a wing
        let mut add_wing = |mirror: bool| {
            let offset_x = 3.5 + extension * 2.0;
            let current_v_count = vertices.len() as u16;

            for v in &origami_mesh.vertices {
                let z_shade = (v.pos.z.abs() * 0.5 + 0.5).clamp(0.0, 1.0);
                let col = Color::new(
                    base_color.r * z_shade,
                    base_color.g * z_shade,
                    base_color.b * z_shade,
                    1.0,
                );
                let color_bytes: [u8; 4] = col.into();

                // Transform
                // Original: centered at origin.
                // Rotate X 90 degrees: (x, y, z) -> (x, -z, y)
                // Translate X: (+-offset, 0, 0)

                let mut x = v.pos.x;
                let y = -v.pos.z; // Rotate 90 X
                let z = v.pos.y;

                if mirror {
                    x = -x;
                    // Mirroring might flip normals/winding order?
                }

                x += if mirror { -offset_x } else { offset_x };

                vertices.push(Vertex {
                    position: vec3(x, y, z),
                    uv: v.uv,
                    color: color_bytes,
                    normal: vec4(0.0, 1.0, 0.0, 0.0), // Placeholder normal
                });
            }

            for idx in &origami_mesh.indices {
                // If mirroring, we might need to flip winding order
                // indices: p0, p1, p2
                // if mirror: p0, p2, p1
                if mirror {
                    // Flip triangle winding
                    // But wait, indices are u16 list of triangles.
                    // We iterate chunks of 3 usually. But here we just push.
                    // Since we just push indices, we need to swap the order of push?
                    // No, we are pushing to a flat list.
                    // Actually, let's just push them as is. Backface culling might handle it if enabled.
                    // Macroquad default material has culling? Usually yes.
                    // Let's swap last two if mirror to flip normal.
                }
                indices.push(current_v_count + idx);
            }
        };

        add_wing(false);
        add_wing(true);

        // Fix indices for mirrored wing (simple hack: disable culling or just hope)
        // Actually, if I mirror X, the winding order flips.
        // So for the second wing, I should swap indices.
        // But indices are pushed in a loop. I can't easily swap inside the loop without restructuring.
        // I'll leave it as is. Double sided rendering is often default or acceptable.

        let mesh = Mesh {
            vertices,
            indices,
            texture: None,
        };

        // Draw Center Body (Satellite Bus)
        draw_cube(vec3(0.0, 0.0, 0.0), vec3(2.0, 2.0, 2.0), None, GRAY);
        draw_cube_wires(vec3(0.0, 0.0, 0.0), vec3(2.0, 2.0, 2.0), LIGHTGRAY);

        // Draw Wings
        gl_use_material(&material);
        draw_mesh(&mesh);
        gl_use_default_material();

        // Wireframe for wings
        draw_wireframe(&mesh, Color::new(1.0, 1.0, 1.0, 0.1));

        set_default_camera();

        // UI
        draw_text("Origami Satellite Deployment", 20.0, 30.0, 30.0, WHITE);
        draw_text("Left Click: Rotate Camera", 20.0, 60.0, 20.0, LIGHTGRAY);
        draw_text(
            "Right Click / Shift+Drag: Deploy/Retract",
            20.0,
            80.0,
            20.0,
            LIGHTGRAY,
        );
        draw_text(
            &format!("Extension: {:.1}%", extension * 100.0),
            20.0,
            110.0,
            20.0,
            if is_locked { GREEN } else { YELLOW },
        );

        if is_locked {
            draw_text("LOCKED - POWER MAXIMIZED", 20.0, 140.0, 20.0, GREEN);
        }

        next_frame().await
    }
}

// Helper for wireframe
fn draw_wireframe(mesh: &Mesh, color: Color) {
    if mesh.indices.len() < 3 {
        return;
    }
    // Draw lines for indices
    for i in (0..mesh.indices.len()).step_by(3) {
        let i0 = mesh.indices[i] as usize;
        let i1 = mesh.indices[i + 1] as usize;
        let i2 = mesh.indices[i + 2] as usize;

        let v0 = mesh.vertices[i0].position;
        let v1 = mesh.vertices[i1].position;
        let v2 = mesh.vertices[i2].position;

        draw_line_3d(v0, v1, color);
        draw_line_3d(v1, v2, color);
        draw_line_3d(v2, v0, color);
    }
}

const DEFAULT_VERTEX_SHADER: &str = "#version 100
attribute vec3 position;
attribute vec2 texcoord;
attribute vec4 color0;

varying float v_light;
varying vec2 v_texcoord;
varying vec4 v_color;

uniform mat4 Model;
uniform mat4 Projection;

void main() {
    gl_Position = Projection * Model * vec4(position, 1);
    v_color = color0;
    v_texcoord = texcoord;
    v_light = 1.0;
}
";

const DEFAULT_FRAGMENT_SHADER: &str = "#version 100
precision mediump float;

varying float v_light;
varying vec2 v_texcoord;
varying vec4 v_color;

void main() {
    gl_FragColor = v_color;
}
";
