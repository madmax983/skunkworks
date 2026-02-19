use macroquad::prelude::*;
use gray_scott::GrayScott;
use origami::{MiuraOri, MiuraParams, Orientation};
use ::rand::Rng;

const WIDTH: usize = 120;
const HEIGHT: usize = 120;

fn window_conf() -> Conf {
    Conf {
        window_title: "Origami Terrain 🦢🏔️".to_string(),
        window_width: 1280,
        window_height: 720,
        high_dpi: true,
        sample_count: 4,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // 1. Initialize Gray-Scott
    let mut gs = GrayScott::new(WIDTH, HEIGHT);

    // Seed with random noise
    let mut rng = ::rand::thread_rng();
    for _ in 0..50 {
        let cx = rng.gen_range(20..WIDTH-20);
        let cy = rng.gen_range(20..HEIGHT-20);
        for y in cy-5..cy+5 {
            for x in cx-5..cx+5 {
                gs.add_chemical(x, y, 0.5);
            }
        }
    }

    // 2. Initialize Miura-Ori
    // Use smaller grid dimensions for the fold if needed, but matching GS is easier for mapping
    let cols = WIDTH - 1;
    let rows = HEIGHT - 1;

    let params = MiuraParams {
        a: 1.0,
        b: 1.0,
        gamma: 80.0f32.to_radians(),
        orientation: Orientation::Horizontal,
    };
    let miura = MiuraOri::new(params, (cols, rows));

    // Simulation State
    let mut feed;
    let kill = 0.062;
    let mut extension: f32 = 1.0; // 0.0 = folded, 1.0 = flat

    // Camera State
    let mut cam_yaw: f32 = 0.0;
    let mut cam_pitch: f32 = 0.5;
    let mut cam_dist: f32 = 300.0;
    let target = vec3(WIDTH as f32 / 2.0, 0.0, HEIGHT as f32 / 2.0);

    // Shader for flat shading (optional but nice)
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
    ).unwrap();

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
        if is_key_down(KeyCode::Up) { extension += 0.01; }
        if is_key_down(KeyCode::Down) { extension -= 0.01; }
        extension = extension.clamp(0.1, 1.0); // Don't go to 0.0 fully to avoid singularity

        // Interactive "Rain"
        if is_key_down(KeyCode::Space) {
             let mut rng = ::rand::thread_rng();
             for _ in 0..10 {
                 let cx = rng.gen_range(5..WIDTH-5);
                 let cy = rng.gen_range(5..HEIGHT-5);
                 for y in cy-2..cy+2 {
                     for x in cx-2..cx+2 {
                         gs.add_chemical(x, y, 0.5);
                     }
                 }
             }
        }

        // --- Logic ---

        // Modulate Reaction-Diffusion based on Folding
        // When folded (low extension), increase feed rate (pressure)?
        // Or make it more chaotic?
        // Let's map extension 0.1 -> Feed 0.08, extension 1.0 -> Feed 0.0545
        let base_feed = 0.0545;
        let compressed_feed = 0.08;
        feed = compressed_feed + (base_feed - compressed_feed) * extension;

        // Update GS
        for _ in 0..8 {
            gs.update(feed, kill, 1.0);
        }

        // Update Mesh
        let grid_points = miura.generate_grid(extension);

        // Build Mesh
        // The Miura grid points are flat (y=0 usually, or folding in y).
        // Wait, MiuraOri::generate_grid returns 3D points where folding is in Z usually?
        // Let's check MiuraOri output. Assuming it returns Vec3.
        // We will displace them further based on GS U value.

        let mut vertices = Vec::with_capacity(cols * rows * 4);
        let mut indices = Vec::with_capacity(cols * rows * 6);
        let mut idx: u16 = 0;

        let u = gs.u();
        let v = gs.v();
        let width_pts = cols + 1; // Miura grid points width

        // We iterate quads
        for j in 0..rows {
            for i in 0..cols {
                // Indices in the grid_points array
                let p0_idx = j * width_pts + i;
                let p1_idx = j * width_pts + (i + 1);
                let p2_idx = (j + 1) * width_pts + (i + 1);
                let p3_idx = (j + 1) * width_pts + i;

                // Indices in the GS grid (linear map)
                // We map i,j directly.
                let gs_idx = j * WIDTH + i; // Approximate mapping since cols=WIDTH-1

                // Sample GS value
                // Let's just use gs_idx for the quad color/height
                // (Using average of corners would be better but this is fast)
                let val_u = u[gs_idx];
                let val_v = v[gs_idx];

                // Height displacement from Reaction
                let displacement = val_u * 30.0; // Mountain height

                // Color from Reaction
                let color = if val_v > 0.3 {
                    Color::new(0.8, 0.2 + val_v, 0.2, 1.0) // Magma
                } else if val_u > 0.6 {
                    Color::new(0.1, 0.5 + val_u * 0.4, 0.1, 1.0) // Forest
                } else {
                    Color::new(0.1, 0.1, 0.3, 1.0) // Water
                };
                let color_bytes: [u8; 4] = color.into();

                // Vertices
                let mut v0 = grid_points[p0_idx];
                let mut v1 = grid_points[p1_idx];
                let mut v2 = grid_points[p2_idx];
                let mut v3 = grid_points[p3_idx];

                // Apply displacement along Y (Up)
                // Assuming Miura folds in Y or Z.
                // If Miura lies on XZ plane, we displace Y.
                v0.y += displacement;
                v1.y += displacement;
                v2.y += displacement;
                v3.y += displacement;

                // Push Quad (duplicated for flat shading look)
                vertices.push(Vertex { position: v0, uv: vec2(0.,0.), color: color_bytes, normal: vec4(0.,1.,0.,0.) });
                vertices.push(Vertex { position: v1, uv: vec2(1.,0.), color: color_bytes, normal: vec4(0.,1.,0.,0.) });
                vertices.push(Vertex { position: v2, uv: vec2(1.,1.), color: color_bytes, normal: vec4(0.,1.,0.,0.) });
                vertices.push(Vertex { position: v3, uv: vec2(0.,1.), color: color_bytes, normal: vec4(0.,1.,0.,0.) });

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
            texture: None,
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

        gl_use_material(&material);
        draw_mesh(&mesh);
        gl_use_default_material();

        set_default_camera();

        // UI
        draw_text("Origami Terrain 🦢🏔️", 20.0, 30.0, 30.0, WHITE);
        draw_text(&format!("Extension: {:.1}%", extension * 100.0), 20.0, 50.0, 20.0, YELLOW);
        draw_text(&format!("Feed: {:.4} (Modulated by Fold)", feed), 20.0, 70.0, 20.0, LIGHTGRAY);
        draw_text("Controls: UP/DOWN to Fold | SPACE for Rain | Mouse to Orbit", 20.0, 90.0, 16.0, GRAY);

        next_frame().await
    }
}

const DEFAULT_VERTEX_SHADER: &str = "#version 100
attribute vec3 position;
attribute vec2 texcoord;
attribute vec4 color0;
attribute vec3 normal;

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
