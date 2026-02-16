mod mesh;
mod pbd;

use macroquad::prelude::*;
use mesh::OrigamiMesh;

#[macroquad::main("Kinetic Folds")]
async fn main() {
    let mut mesh = OrigamiMesh::new_miura(10, 10);
    let mut fold_factor = 0.5;

    loop {
        clear_background(BLACK);

        // Update Physics
        // Map fold_factor to actuators
        for a in &mut mesh.actuators {
            a.target_factor = fold_factor;
        }
        mesh.update(0.016); // 60 FPS update

        // Camera
        set_camera(&Camera3D {
            position: vec3(0.0, -40.0, 30.0),
            target: vec3(50.0, 50.0, 0.0), // Center roughly
            up: vec3(0.0, 0.0, 1.0),
            ..Default::default()
        });

        // Draw Grid
        draw_grid(20, 10.0, DARKGRAY, GRAY);

        // Construct Mesh for Rendering
        let mut mq_mesh = Mesh {
            vertices: Vec::new(),
            indices: Vec::new(),
            texture: None,
        };

        for tri in &mesh.triangles {
            let p1 = mesh.solver.particles[tri[0]].pos;
            let p2 = mesh.solver.particles[tri[1]].pos;
            let p3 = mesh.solver.particles[tri[2]].pos;

            let color = Color::new(0.2, 0.8, 0.8, 0.5); // Translucent blue
                                                        // Ensure color has alpha
            let color_bytes: [u8; 4] = color.into();

            // Normals are needed for lighting if we use it, but for now just dummy
            let normal = vec4(0.0, 0.0, 1.0, 0.0);

            let v1 = Vertex {
                position: p1,
                uv: Vec2::ZERO,
                color: color_bytes,
                normal,
            };
            let v2 = Vertex {
                position: p2,
                uv: Vec2::ZERO,
                color: color_bytes,
                normal,
            };
            let v3 = Vertex {
                position: p3,
                uv: Vec2::ZERO,
                color: color_bytes,
                normal,
            };

            let idx = mq_mesh.vertices.len() as u16;
            mq_mesh.vertices.push(v1);
            mq_mesh.vertices.push(v2);
            mq_mesh.vertices.push(v3);

            mq_mesh.indices.push(idx);
            mq_mesh.indices.push(idx + 1);
            mq_mesh.indices.push(idx + 2);

            // Wireframe
            draw_line_3d(p1, p2, WHITE);
            draw_line_3d(p2, p3, WHITE);
            draw_line_3d(p3, p1, WHITE);
        }

        draw_mesh(&mq_mesh);

        // Draw Constraints (Stress)
        for c in &mesh.solver.distance_constraints {
            let p1 = mesh.solver.particles[c.p1].pos;
            let p2 = mesh.solver.particles[c.p2].pos;
            let current_len = p1.distance(p2);
            let strain = current_len / c.rest_length;

            let color = if strain > 1.05 {
                RED
            } else if strain < 0.95 {
                BLUE
            } else {
                GREEN
            };

            // Draw Actuators with different logic?
            // Just draw all constraints if stressed
            if strain > 1.05 || strain < 0.95 {
                draw_line_3d(p1, p2, color);
            }
        }

        set_default_camera();

        // UI
        draw_text("Kinetic Folds", 10.0, 30.0, 30.0, WHITE);
        draw_text(
            &format!("Fold Factor: {:.2}", fold_factor),
            10.0,
            60.0,
            20.0,
            GRAY,
        );
        draw_text("Arrows: Control Fold | Mouse: None", 10.0, 80.0, 20.0, GRAY);

        if is_key_down(KeyCode::Right) {
            fold_factor = (fold_factor + 0.01).min(1.0);
        }
        if is_key_down(KeyCode::Left) {
            fold_factor = (fold_factor - 0.01).max(0.0);
        }

        next_frame().await
    }
}
