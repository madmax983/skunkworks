use macroquad::prelude::*;

mod origami;
mod parser;
mod solver;
mod analysis;

use origami::Mesh;
use solver::solve_constraints;
use parser::{generate_grid_mesh, apply_syntax_creases};
use analysis::check_maekawa;

fn get_orbit_camera(angle_x: f32, angle_y: f32, zoom: f32) -> Camera3D {
    let radius = zoom;
    let y = radius * angle_y.sin();
    let r_xz = radius * angle_y.cos();
    let x = r_xz * angle_x.sin();
    let z = r_xz * angle_x.cos();

    Camera3D {
        position: vec3(x, y, z),
        target: vec3(0.0, 0.0, 0.0),
        up: vec3(0.0, 1.0, 0.0),
        fovy: 45.0,
        projection: Projection::Perspective,
        ..Default::default()
    }
}

// Helper for lerp
fn lerp(start: f32, end: f32, t: f32) -> f32 {
    start + (end - start) * t
}

#[macroquad::main("Syntax Fold")]
async fn main() {
    let mut mesh = generate_grid_mesh(20, 20, 10.0, 10.0);
    let code = "{{}{{{}}}}";
    apply_syntax_creases(&mut mesh, code, 20, 20);

    let mut folding_factor = 0.0;
    let mut target_fold = 0.0;

    let mut cam_angle_x = 0.5f32;
    let mut cam_angle_y = 0.5f32;
    let mut cam_zoom = 15.0f32;

    let mut last_mouse_pos = mouse_position();

    let mut invalid_verts = check_maekawa(&mesh);

    loop {
        // Input
        if is_key_pressed(KeyCode::R) {
            mesh = generate_grid_mesh(20, 20, 10.0, 10.0);
            apply_syntax_creases(&mut mesh, code, 20, 20);
            folding_factor = 0.0;
            target_fold = 0.0;
            invalid_verts = check_maekawa(&mesh);
        }

        if is_key_pressed(KeyCode::Space) {
            target_fold = if target_fold > 0.5 { 0.0 } else { 1.0 };
        }

        // Camera
        let mouse_pos = mouse_position();
        let d = vec2(mouse_pos.0 - last_mouse_pos.0, mouse_pos.1 - last_mouse_pos.1);
        last_mouse_pos = mouse_pos;

        if is_mouse_button_down(MouseButton::Left) {
            cam_angle_x -= d.x * 0.01;
            cam_angle_y = (cam_angle_y + d.y * 0.01).clamp(0.1, 1.5);
        }
        let wheel = mouse_wheel().1;
        cam_zoom = (cam_zoom - wheel * 0.1).clamp(5.0, 50.0);

        let camera = get_orbit_camera(cam_angle_x, cam_angle_y, cam_zoom);

        // Logic
        folding_factor += (target_fold - folding_factor) * 0.05;

        for crease in mesh.creases.iter_mut() {
            crease.current_target = lerp(std::f32::consts::PI, crease.design_angle, folding_factor);
        }

        solve_constraints(&mut mesh, 0.016);

        // Render
        clear_background(BLACK);

        set_camera(&camera);

        // Build Macroquad Mesh for rendering
        let mut mq_mesh = macroquad::models::Mesh {
            vertices: Vec::new(),
            indices: Vec::new(),
            texture: None,
        };

        for face in mesh.faces.iter() {
            let v0 = mesh.vertices[face.indices[0]].pos;
            let v1 = mesh.vertices[face.indices[1]].pos;
            let v2 = mesh.vertices[face.indices[2]].pos;

            // Flat shading normal
            let normal = (v1 - v0).cross(v2 - v0).normalize_or_zero();
            // Note: macroquad uses vec4 for normal?
            // Actually macroquad::models::Vertex depends on version.
            // In 0.4 it might be different. Let's assume standard Vertex struct: pos, uv, color.
            // Wait, rigid-origami used `normal: normal_v4`. So it must have normal.
            let norm_arr = vec4(normal.x, normal.y, normal.z, 0.0);

            let color_arr: [u8; 4] = face.color.into();

            let base = mq_mesh.vertices.len() as u16;

            mq_mesh.vertices.push(macroquad::models::Vertex {
                position: v0, uv: vec2(0.,0.), color: color_arr, normal: norm_arr
            });
            mq_mesh.vertices.push(macroquad::models::Vertex {
                position: v1, uv: vec2(1.,0.), color: color_arr, normal: norm_arr
            });
            mq_mesh.vertices.push(macroquad::models::Vertex {
                position: v2, uv: vec2(0.,1.), color: color_arr, normal: norm_arr
            });

            mq_mesh.indices.push(base);
            mq_mesh.indices.push(base+1);
            mq_mesh.indices.push(base+2);

            // Wireframe
            draw_line_3d(v0, v1, BLACK);
            draw_line_3d(v1, v2, BLACK);
            draw_line_3d(v2, v0, BLACK);
        }

        draw_mesh(&mq_mesh);

        // Creases
        for crease in mesh.creases.iter() {
            let edge = &mesh.edges[crease.edge_index];
            let v_a = mesh.vertices[edge.a].pos;
            let v_b = mesh.vertices[edge.b].pos;
            let color = if crease.is_mountain { RED } else { BLUE };
            // Draw slightly offset to see over wireframe?
            draw_line_3d(v_a, v_b, color);
        }

        // Invalid Vertices
        for &v_idx in &invalid_verts {
            let v = mesh.vertices[v_idx].pos;
            draw_sphere(v, 0.2, None, MAGENTA);
        }

        set_default_camera();

        draw_text(&format!("Folding: {:.2}", folding_factor), 10.0, 20.0, 30.0, WHITE);
        draw_text("Space: Fold/Unfold, R: Reset, Mouse: Orbit", 10.0, 50.0, 20.0, WHITE);

        if !invalid_verts.is_empty() {
             draw_text(&format!("INVALID VERTICES: {}", invalid_verts.len()), 10.0, 80.0, 20.0, MAGENTA);
        } else {
             draw_text("FLAT-FOLDABLE", 10.0, 80.0, 20.0, GREEN);
        }

        next_frame().await;
    }
}
