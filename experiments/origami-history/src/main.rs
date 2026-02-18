use macroquad::prelude::*;
use git_associates::GitModel;
use origami::{MiuraOri, MiuraParams, Orientation};
use std::f32::consts::PI;

fn conf() -> Conf {
    Conf {
        window_title: "Origami History 🦢".to_string(),
        window_width: 1280,
        window_height: 720,
        high_dpi: true,
        sample_count: 4,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    // --- 1. Git Data Extraction ---
    println!("Loading Git History...");
    let repo_path = "."; // Current directory (root of repo)
    let git_model = GitModel::open(repo_path).unwrap_or_else(|e| {
        eprintln!("Failed to open git repo: {}", e);
        // Fallback or exit? For now let's just panic or create a dummy model if possible.
        // But GitModel doesn't have a dummy constructor.
        // We'll panic for now, as this experiment requires a repo.
        panic!("Failed to open git repo: {}", e);
    });

    // Fetch history
    let limit = 100;
    let history = git_model.history_with_diffs(limit).unwrap_or_else(|e| {
        eprintln!("Failed to fetch history: {}", e);
        Vec::new()
    });

    // Reverse history so index 0 is the oldest (or newest? 0 is usually newest from git log).
    // Let's keep 0 as newest (Top-Left).

    // Grid Setup
    let cols = 10;
    let rows = 10;

    // --- 2. Origami Setup ---
    let params = MiuraParams {
        a: 1.0,
        b: 1.0,
        gamma: 80.0f32.to_radians(),
        orientation: Orientation::Horizontal,
    };
    let miura = MiuraOri::new(params, (cols, rows));

    // --- 3. State ---
    let mut cam_yaw: f32 = PI / 4.0;
    let mut cam_pitch: f32 = -PI / 6.0;
    let mut cam_dist: f32 = 15.0;
    let target = vec3(0.0, 0.0, 0.0);
    let mut extension: f32 = 0.5;

    // Pre-calculate colors
    let mut commit_colors: Vec<Color> = Vec::with_capacity(cols * rows);
    for i in 0..(cols * rows) {
        if i < history.len() {
            let commit = &history[i];
            let stats = commit.stats.as_ref().unwrap(); // We used history_with_diffs

            // Normalize stats for color
            // Heuristic: 50 lines is "intense"
            let max_lines = 50.0;

            let ins = (stats.insertions as f32 / max_lines).clamp(0.0, 1.0);
            let del = (stats.deletions as f32 / max_lines).clamp(0.0, 1.0);

            // R = Deletions, G = Insertions, B = Base
            let r = del * 0.8 + 0.2;
            let g = ins * 0.8 + 0.2;
            let b = 0.4;

            commit_colors.push(Color::new(r, g, b, 1.0));
        } else {
            // Empty / Future slot
            commit_colors.push(Color::new(0.1, 0.1, 0.1, 1.0));
        }
    }

    // Material
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
        if is_mouse_button_down(MouseButton::Right) || (is_key_down(KeyCode::LeftShift) && is_mouse_button_down(MouseButton::Left)) {
            let dy = mouse_delta_position().y;
            extension -= dy * 1.0;
            extension = extension.clamp(0.0, 1.0);
        }

        if is_mouse_button_down(MouseButton::Left) && !is_key_down(KeyCode::LeftShift) {
            let delta = mouse_delta_position();
            cam_yaw -= delta.x * 3.0;
            cam_pitch += delta.y * 3.0;
            cam_pitch = cam_pitch.clamp(-PI / 2.0 + 0.1, PI / 2.0 - 0.1);
        }

        cam_dist -= mouse_wheel().1 * 1.0;
        cam_dist = cam_dist.clamp(5.0, 50.0);

        // --- Logic & Mesh Generation ---

        // 1. Generate base mesh (shared vertices)
        let base_mesh = miura.generate_mesh(extension);

        // 2. Explode mesh for flat shading / per-cell coloring
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut indices: Vec<u16> = Vec::new();
        let mut current_idx = 0;

        let width_v = cols + 1;

        for j in 0..rows {
            for i in 0..cols {
                // Indices in base_mesh.vertices
                let idx00 = j * width_v + i;
                let idx10 = j * width_v + (i + 1);
                let idx11 = (j + 1) * width_v + (i + 1);
                let idx01 = (j + 1) * width_v + i;

                let v00 = base_mesh.vertices[idx00].pos;
                let v10 = base_mesh.vertices[idx10].pos;
                let v11 = base_mesh.vertices[idx11].pos;
                let v01 = base_mesh.vertices[idx01].pos;

                // Color
                let cell_idx = j * cols + i;
                let color = commit_colors.get(cell_idx).cloned().unwrap_or(BLACK);
                let color_bytes: [u8; 4] = color.into();

                // Normal Calculation (Flat)
                // Triangle 1: 00 -> 10 -> 01
                let edge1 = v10 - v00;
                let edge2 = v01 - v00;
                let normal = edge1.cross(edge2).normalize();
                let normal_vec4 = vec4(normal.x, normal.y, normal.z, 0.0);

                // Push 4 unique vertices
                vertices.push(Vertex { position: v00, uv: vec2(0., 0.), color: color_bytes, normal: normal_vec4 }); // 0
                vertices.push(Vertex { position: v10, uv: vec2(1., 0.), color: color_bytes, normal: normal_vec4 }); // 1
                vertices.push(Vertex { position: v11, uv: vec2(1., 1.), color: color_bytes, normal: normal_vec4 }); // 2
                vertices.push(Vertex { position: v01, uv: vec2(0., 1.), color: color_bytes, normal: normal_vec4 }); // 3

                // Push indices (Two triangles)
                // 0, 1, 3
                indices.push(current_idx + 0);
                indices.push(current_idx + 1);
                indices.push(current_idx + 3);

                // 1, 2, 3
                indices.push(current_idx + 1);
                indices.push(current_idx + 2);
                indices.push(current_idx + 3);

                current_idx += 4;
            }
        }

        let mesh = Mesh {
            vertices,
            indices,
            texture: None,
        };

        // --- Render ---
        clear_background(Color::new(0.05, 0.05, 0.05, 1.0));

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

        // Draw Mesh
        gl_use_material(&material);
        draw_mesh(&mesh);
        gl_use_default_material();

        // Draw Wireframe
        draw_wireframe(&mesh, Color::new(1.0, 1.0, 1.0, 0.1));

        set_default_camera();

        // --- UI ---
        draw_text("Origami History 🦢", 20.0, 30.0, 30.0, WHITE);
        draw_text(&format!("Commits: {}", history.len()), 20.0, 60.0, 20.0, LIGHTGRAY);
        draw_text("Drag Vertically to Fold Time", 20.0, 80.0, 20.0, GRAY);

        // Legend
        draw_rectangle(20.0, 100.0, 20.0, 20.0, GREEN);
        draw_text("Insertions", 50.0, 115.0, 20.0, LIGHTGRAY);
        draw_rectangle(20.0, 130.0, 20.0, 20.0, RED);
        draw_text("Deletions", 50.0, 145.0, 20.0, LIGHTGRAY);

        // Find hovered cell?
        // Crude approximation: Just show latest commit details
        if let Some(latest) = history.first() {
            draw_text("Latest Commit:", 20.0, 180.0, 20.0, WHITE);
            draw_text(&latest.short_hash, 20.0, 200.0, 20.0, YELLOW);
            draw_text(&latest.author, 100.0, 200.0, 20.0, BLUE);
            draw_text(&latest.message, 20.0, 220.0, 16.0, LIGHTGRAY);
        }

        next_frame().await
    }
}

fn draw_wireframe(mesh: &Mesh, color: Color) {
    if mesh.indices.len() < 3 { return; }
    for i in (0..mesh.indices.len()).step_by(3) {
        let i0 = mesh.indices[i] as usize;
        let i1 = mesh.indices[i+1] as usize;
        let i2 = mesh.indices[i+2] as usize;

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
attribute vec4 normal;

varying float v_light;
varying vec2 v_texcoord;
varying vec4 v_color;

uniform mat4 Model;
uniform mat4 Projection;

void main() {
    gl_Position = Projection * Model * vec4(position, 1);
    v_color = color0;
    v_texcoord = texcoord;

    // Simple directional lighting
    vec3 light_dir = normalize(vec3(1.0, 1.0, 1.0));
    float diff = max(dot(normal.xyz, light_dir), 0.2);
    v_light = diff;
}
";

const DEFAULT_FRAGMENT_SHADER: &str = "#version 100
precision mediump float;

varying float v_light;
varying vec2 v_texcoord;
varying vec4 v_color;

void main() {
    gl_FragColor = vec4(v_color.rgb * v_light, v_color.a);
}
";
