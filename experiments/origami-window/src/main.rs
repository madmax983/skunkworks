use macroquad::prelude::*;
use origami_window::MiuraGrid;

fn generate_ui_texture() -> Texture2D {
    let width = 512;
    let height = 512;
    let mut image = Image::gen_image_color(width, height, WHITE);

    // Draw "Window" UI
    // Header
    for y in 0..60 {
        for x in 0..width {
            image.set_pixel(x as u32, y as u32, BLUE);
        }
    }

    // Title Text (fake)
    for y in 15..45 {
        for x in 20..200 {
            image.set_pixel(x as u32, y as u32, WHITE);
        }
    }

    // Close Button
    for y in 15..45 {
        for x in (width - 50)..(width - 20) {
            image.set_pixel(x as u32, y as u32, RED);
        }
    }

    // Content Area
    // Sidebar
    for y in 60..height {
        for x in 0..150 {
            image.set_pixel(x as u32, y as u32, LIGHTGRAY);
        }
    }

    // Main Content text lines
    for i in 0..10 {
        let y_start = 80 + i * 40;
        if y_start + 20 < height {
            for y in y_start..(y_start + 20) {
                for x in 170..(width - 20) {
                    image.set_pixel(x as u32, y as u32, DARKGRAY);
                }
            }
        }
    }

    Texture2D::from_image(&image)
}

#[macroquad::main("Origami Window")]
async fn main() {
    let texture = generate_ui_texture();

    // Setup Grid
    let rows = 10;
    let cols = 10;
    let a = 2.0;
    let b = 2.0;
    let gamma = 80.0f32.to_radians(); // 80 degrees sector angle

    let grid = MiuraGrid::new(rows, cols, a, b, gamma);

    let mut cam_dist = 30.0;
    let mut cam_yaw: f32 = 0.0;
    let mut cam_pitch: f32 = 0.5;

    let mut last_mouse_pos = mouse_position();

    // Fold state
    // We map mouse X to rho.

    let mut mesh = Mesh {
        vertices: Vec::with_capacity((rows * cols * 4) as usize),
        indices: Vec::with_capacity((rows * cols * 6) as usize),
        texture: Some(texture.clone()),
    };

    loop {
        clear_background(Color::new(0.1, 0.1, 0.1, 1.0));

        let mouse_pos = mouse_position();
        let mouse_delta_x = mouse_pos.0 - last_mouse_pos.0;
        let mouse_delta_y = mouse_pos.1 - last_mouse_pos.1;
        last_mouse_pos = mouse_pos;

        // Input for Camera
        if is_mouse_button_down(MouseButton::Right) {
            cam_yaw -= mouse_delta_x * 0.01;
            cam_pitch += mouse_delta_y * 0.01;
            cam_pitch = cam_pitch.clamp(-1.5, 1.5);
        }
        cam_dist = (cam_dist - mouse_wheel().1).clamp(5.0, 100.0);

        // Input for Fold
        let mouse_x = mouse_position().0;
        let screen_w = screen_width();
        // Map mouse X (0..screen_w) to rho (0.0..sin(gamma))
        // Actually let's map it to (0.1 .. 0.95)
        let t = (mouse_x / screen_w).clamp(0.0, 1.0);
        let max_rho = gamma.sin() - 0.05;
        let min_rho = 0.1;
        let rho = min_rho + t * (max_rho - min_rho);

        let vertices_na = grid.compute_vertices(rho);

        // Rebuild Mesh
        mesh.vertices.clear();
        mesh.indices.clear();

        // The grid has (rows+1) * (cols+1) vertices.
        // We need to generate quads.
        // Each cell (i, j) is a quad formed by:
        // (i, j), (i, j+1), (i+1, j+1), (i+1, j)

        let v_width = cols as f32;
        let v_height = rows as f32;

        // Calculate normal for lighting
        // Simple flat shading per quad
        // Actually, let's just use vertex normals if smooth, or duplicate vertices for flat.
        // For origami, flat shading is better to see the creases.

        for i in 0..rows {
            for j in 0..cols {
                let idx0 = i * (cols + 1) + j;
                let idx1 = i * (cols + 1) + j + 1;
                let idx2 = (i + 1) * (cols + 1) + j + 1;
                let idx3 = (i + 1) * (cols + 1) + j;

                let p0 = vertices_na[idx0];
                let p1 = vertices_na[idx1];
                let p2 = vertices_na[idx2];
                let p3 = vertices_na[idx3];

                let v0 = vec3(p0.x, p0.y, p0.z);
                let v1 = vec3(p1.x, p1.y, p1.z);
                let v2 = vec3(p2.x, p2.y, p2.z);
                let v3 = vec3(p3.x, p3.y, p3.z);

                // Normal
                let normal = (v1 - v0).cross(v2 - v0).normalize();

                // UVs
                // Map (j, i) to (0..1)
                let u0 = j as f32 / v_width;
                let v_tex0 = i as f32 / v_height;
                let u1 = (j + 1) as f32 / v_width;
                let v_tex1 = (i + 1) as f32 / v_height;

                // Vertex colors based on lighting
                let light_dir = vec3(0.5, 1.0, 0.5).normalize();
                let light = normal.dot(light_dir).abs(); // Two-sided lighting
                let color_val = 0.5 + 0.5 * light;
                let color = Color::new(color_val, color_val, color_val, 1.0);

                // Add vertices (4 per quad for flat shading)
                let base_idx = mesh.vertices.len() as u16;

                mesh.vertices.push(Vertex {
                    position: v0,
                    uv: vec2(u0, v_tex0),
                    color: color.into(),
                    normal: vec4(normal.x, normal.y, normal.z, 1.0),
                });
                mesh.vertices.push(Vertex {
                    position: v1,
                    uv: vec2(u1, v_tex0),
                    color: color.into(),
                    normal: vec4(normal.x, normal.y, normal.z, 1.0),
                });
                mesh.vertices.push(Vertex {
                    position: v2,
                    uv: vec2(u1, v_tex1),
                    color: color.into(),
                    normal: vec4(normal.x, normal.y, normal.z, 1.0),
                });
                mesh.vertices.push(Vertex {
                    position: v3,
                    uv: vec2(u0, v_tex1),
                    color: color.into(),
                    normal: vec4(normal.x, normal.y, normal.z, 1.0),
                });

                mesh.indices.push(base_idx);
                mesh.indices.push(base_idx + 1);
                mesh.indices.push(base_idx + 2);

                mesh.indices.push(base_idx);
                mesh.indices.push(base_idx + 2);
                mesh.indices.push(base_idx + 3);
            }
        }

        // Camera
        let cam_pos = vec3(
            cam_dist * cam_yaw.cos() * cam_pitch.cos(),
            cam_dist * cam_pitch.sin(),
            cam_dist * cam_yaw.sin() * cam_pitch.cos(),
        );

        set_camera(&Camera3D {
            position: cam_pos,
            target: vec3(cols as f32 * a * 0.5 * rho, rows as f32 * b * 0.5, 0.0), // Approximate center
            up: vec3(0.0, 1.0, 0.0),
            ..Default::default()
        });

        draw_mesh(&mesh);

        // Draw wireframe overlay (optional, maybe distinct color lines)

        set_default_camera();

        draw_text("Origami Window", 10.0, 30.0, 30.0, WHITE);
        draw_text(&format!("Fold Factor (Mouse X): {:.2}", rho), 10.0, 50.0, 20.0, LIGHTGRAY);
        draw_text("Right Click Drag: Rotate Camera", 10.0, 70.0, 20.0, LIGHTGRAY);

        next_frame().await
    }
}
