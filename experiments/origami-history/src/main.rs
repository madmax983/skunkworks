use anyhow::Result;
use chrono::Utc;
use git_associates::{Commit, GitModel};
use macroquad::prelude::*;
use origami::{MiuraOri, MiuraParams, Orientation};
use std::f32::consts::PI;

fn conf() -> Conf {
    Conf {
        window_title: "Origami History 🦢📜".to_string(),
        window_width: 1280,
        window_height: 720,
        high_dpi: true,
        sample_count: 4,
        ..Default::default()
    }
}

struct HistoryPanel {
    commit: Option<Commit>,
    color: Color,
}

#[macroquad::main(conf)]
async fn main() -> Result<()> {
    // 1. Load Git History
    let commits = match GitModel::open(".") {
        Ok(model) => model.history(100).unwrap_or_default(),
        Err(_) => {
            eprintln!("Failed to open git repository. Using dummy data.");
            generate_dummy_commits(100)
        }
    };

    if commits.is_empty() {
        eprintln!("No commits found.");
        return Ok(());
    }

    // 2. Setup Grid
    let count = commits.len();
    let cols = (count as f32).sqrt().ceil() as usize;
    let rows = ((count as f32) / (cols as f32)).ceil() as usize;

    // Map commits to panels
    let mut panels: Vec<HistoryPanel> = Vec::with_capacity(cols * rows);
    for commit in commits {
        let color = hash_to_color(&commit.author);
        panels.push(HistoryPanel {
            commit: Some(commit),
            color,
        });
    }
    // Fill remainder
    while panels.len() < cols * rows {
        panels.push(HistoryPanel {
            commit: None,
            color: Color::new(0.1, 0.1, 0.1, 1.0),
        });
    }

    // 3. Setup Miura-Ori
    let params = MiuraParams {
        a: 1.0,
        b: 1.0,
        gamma: 80.0f32.to_radians(),
        orientation: Orientation::Horizontal,
    };
    let miura = MiuraOri::new(params, (cols, rows));

    // Camera State
    let mut cam_yaw: f32 = PI / 4.0;
    let mut cam_pitch: f32 = -PI / 6.0;
    let mut cam_dist: f32 = 20.0;
    let target = vec3(0.0, 0.0, 0.0);

    // Deployment State
    let mut extension: f32 = 0.1;

    // Shader for flat shading
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
        if is_mouse_button_down(MouseButton::Right)
            || (is_key_down(KeyCode::LeftShift) && is_mouse_button_down(MouseButton::Left))
        {
            let dy = mouse_delta_position().y;
            extension -= dy * 2.0;
        }

        if is_mouse_button_down(MouseButton::Left) && !is_key_down(KeyCode::LeftShift) {
            let delta = mouse_delta_position();
            cam_yaw -= delta.x * 5.0;
            cam_pitch += delta.y * 5.0;
            cam_pitch = cam_pitch.clamp(-PI / 2.0 + 0.1, PI / 2.0 - 0.1);
        }

        cam_dist -= mouse_wheel().1 * 0.5;
        cam_dist = cam_dist.clamp(5.0, 100.0);
        extension = extension.clamp(0.0, 1.0);

        // --- Render ---
        clear_background(Color::new(0.05, 0.05, 0.05, 1.0));

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

        // Generate Grid Points
        let grid_points = miura.generate_grid(extension);
        let width_pts = cols + 1;

        // Build Exploded Mesh
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        let mut idx_counter = 0;

        for j in 0..rows {
            for i in 0..cols {
                let panel_idx = j * cols + i;
                if panel_idx >= panels.len() {
                    continue;
                }
                let panel = &panels[panel_idx];

                // Grid indices
                let p0_idx = j * width_pts + i;
                let p1_idx = j * width_pts + (i + 1);
                let p2_idx = (j + 1) * width_pts + (i + 1);
                let p3_idx = (j + 1) * width_pts + i;

                let v0 = grid_points[p0_idx];
                let v1 = grid_points[p1_idx];
                let v2 = grid_points[p2_idx];
                let v3 = grid_points[p3_idx];

                // Calculate normal for flat shading
                // Triangle 1: 0-1-3
                // Normal = Cross(1-0, 3-0)
                // Triangle 2: 1-2-3

                // We just use one color per quad
                let color_bytes: [u8; 4] = panel.color.into();

                // Quad vertices (duplicated for flat shading look)
                // 0
                vertices.push(Vertex {
                    position: v0,
                    uv: vec2(0., 0.),
                    color: color_bytes,
                    normal: vec4(0., 1., 0., 0.),
                });
                // 1
                vertices.push(Vertex {
                    position: v1,
                    uv: vec2(1., 0.),
                    color: color_bytes,
                    normal: vec4(0., 1., 0., 0.),
                });
                // 2
                vertices.push(Vertex {
                    position: v2,
                    uv: vec2(1., 1.),
                    color: color_bytes,
                    normal: vec4(0., 1., 0., 0.),
                });
                // 3
                vertices.push(Vertex {
                    position: v3,
                    uv: vec2(0., 1.),
                    color: color_bytes,
                    normal: vec4(0., 1., 0., 0.),
                });

                // Triangles: 0-1-3, 1-2-3
                indices.push(idx_counter + 0);
                indices.push(idx_counter + 1);
                indices.push(idx_counter + 3);

                indices.push(idx_counter + 1);
                indices.push(idx_counter + 2);
                indices.push(idx_counter + 3);

                idx_counter += 4;
            }
        }

        let mesh = Mesh {
            vertices,
            indices,
            texture: None,
        };

        gl_use_material(&material);
        draw_mesh(&mesh);
        gl_use_default_material();

        // Wireframe
        draw_wireframe(&mesh, Color::new(1.0, 1.0, 1.0, 0.2));

        set_default_camera();

        // UI
        draw_text("Origami History", 20.0, 30.0, 30.0, WHITE);
        draw_text(&format!("Commits: {}", count), 20.0, 50.0, 20.0, LIGHTGRAY);
        draw_text(
            &format!("Extension: {:.1}%", extension * 100.0),
            20.0,
            70.0,
            20.0,
            YELLOW,
        );

        // Let's just stick to listing the most recent commits on the side for now.

        let mut y = 100.0;
        for panel in panels.iter().take(10) {
            if let Some(c) = &panel.commit {
                draw_text(
                    &format!("{}: {}", &c.short_hash, c.author),
                    20.0,
                    y,
                    20.0,
                    panel.color,
                );
                y += 20.0;
            }
        }
        if panels.len() > 10 {
            draw_text("...", 20.0, y, 20.0, GRAY);
        }

        next_frame().await
    }
}

fn hash_to_color(s: &str) -> Color {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    let hash = hasher.finish();

    let r = ((hash >> 16) & 0xFF) as f32 / 255.0;
    let g = ((hash >> 8) & 0xFF) as f32 / 255.0;
    let b = (hash & 0xFF) as f32 / 255.0;

    // Boost saturation/brightness
    let max = r.max(g).max(b).max(0.1);
    Color::new(r / max, g / max, b / max, 1.0)
}

fn generate_dummy_commits(n: usize) -> Vec<Commit> {
    let mut v = Vec::new();
    for i in 0..n {
        v.push(Commit {
            hash: format!("dummyhash{}", i),
            short_hash: format!("dmy{:03}", i),
            author: if i % 2 == 0 {
                "Alice".to_string()
            } else {
                "Bob".to_string()
            },
            message: "Fixed a bug".to_string(),
            timestamp: Utc::now(),
            parents: vec![],
            stats: None,
            files: vec![],
        });
    }
    v
}

fn draw_wireframe(mesh: &Mesh, color: Color) {
    if mesh.indices.len() < 3 {
        return;
    }
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
