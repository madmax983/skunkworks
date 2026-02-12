use macroquad::prelude::*;
use klein_fs::surface;
use klein_fs::fs_scanner;
use klein_fs::layout;

#[macroquad::main("Klein-FS")]
async fn main() {
    // Generate Surface
    let (mesh_verts, mesh_indices) = surface::generate_wireframe_mesh(40, 40);

    // Scan Directory
    // Limit depth to 5 to avoid too many nodes.
    let nodes = fs_scanner::scan_directory(".", 5);
    let node_positions = layout::layout_nodes(&nodes);

    // Camera Params
    let mut cam_yaw: f32 = 0.0;
    let mut cam_pitch: f32 = 0.5;
    let mut cam_dist: f32 = 10.0;

    // Interaction state
    let mut hovered_node: Option<usize> = None;

    loop {
        clear_background(BLACK);

        // Input Handling
        if is_mouse_button_down(MouseButton::Left) {
            let delta = mouse_delta_position();
            cam_yaw -= delta.x * 2.0;
            cam_pitch += delta.y * 2.0;
            cam_pitch = cam_pitch.clamp(-1.5, 1.5);
        }

        let (_, wheel_y) = mouse_wheel();
        if wheel_y != 0.0 {
            cam_dist -= wheel_y * 0.5;
            cam_dist = cam_dist.clamp(2.0, 50.0);
        }

        // Camera Update
        let cam_pos = vec3(
            cam_dist * cam_yaw.cos() * cam_pitch.cos(),
            cam_dist * cam_pitch.sin(),
            cam_dist * cam_yaw.sin() * cam_pitch.cos(),
        );

        let camera = Camera3D {
            position: cam_pos,
            target: vec3(0.0, 0.0, 0.0),
            up: vec3(0.0, 1.0, 0.0),
            ..Default::default()
        };

        set_camera(&camera);

        // Draw Surface Wireframe (Faint)
        for i in (0..mesh_indices.len()).step_by(2) {
            let idx1 = mesh_indices[i] as usize;
            let idx2 = mesh_indices[i+1] as usize;

            if idx1 < mesh_verts.len() && idx2 < mesh_verts.len() {
                let p1 = mesh_verts[idx1];
                let p2 = mesh_verts[idx2];
                draw_line_3d(p1, p2, Color::new(0.5, 0.5, 0.5, 0.1));
            }
        }

        // Draw Edges (Parent -> Child)
        for (i, node) in nodes.iter().enumerate() {
            if let Some(pidx) = node.parent {
                if i < node_positions.len() && pidx < node_positions.len() {
                    let (p1, _) = node_positions[i];
                    let (p2, _) = node_positions[pidx];
                    draw_line_3d(p1, p2, Color::new(0.3, 0.3, 0.3, 0.4));
                }
            }
        }

        // Raycasting for Hover Detection
        // Using "Center of Screen" approach (closest to camera forward vector)
        let cam_forward = (camera.target - camera.position).normalize();

        let mut best_score = -1.0; // Cosine of angle, -1 to 1.
        let mut best_idx: Option<usize> = None;

        for (i, (pos, _)) in node_positions.iter().enumerate() {
            let to_node = (*pos - camera.position).normalize();
            let score = cam_forward.dot(to_node);

            // Threshold: must be within some cone (e.g. > 0.995 for very small angle ~ 5 degrees)
            if score > 0.995 {
                if score > best_score {
                    best_score = score;
                    best_idx = Some(i);
                }
            }
        }
        hovered_node = best_idx;

        // Draw Nodes
        for (i, (pos, color)) in node_positions.iter().enumerate() {
            let is_hovered = Some(i) == hovered_node;
            let size = if is_hovered { 0.1 } else { 0.05 };
            let draw_color = if is_hovered { YELLOW } else { *color };

            if is_hovered {
                draw_cube(*pos, vec3(size*2.0, size*2.0, size*2.0), None, draw_color);
            } else {
                draw_sphere(*pos, size, None, draw_color);
            }
        }

        set_default_camera();

        // Draw UI
        draw_text("Klein-FS", 10.0, 20.0, 30.0, WHITE);
        draw_text("Left Click + Drag: Rotate | Scroll: Zoom", 10.0, 40.0, 20.0, GRAY);
        draw_text(&format!("Nodes: {}", nodes.len()), 10.0, 60.0, 20.0, LIGHTGRAY);

        if let Some(idx) = hovered_node {
             if idx < nodes.len() {
                 let node = &nodes[idx];
                 draw_text(&format!("{} ({})", node.name, if node.is_dir { "Dir" } else { "File" }),
                     10.0, 80.0, 20.0, YELLOW);
                 draw_text(&format!("Path: {}", node.path), 10.0, 100.0, 15.0, GRAY);
             }
        }

        next_frame().await
    }
}
