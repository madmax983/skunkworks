mod git_graph;
mod mobius;

use git_graph::GitGraph;
use macroquad::prelude::*;
use mobius::{map_to_mobius, get_normal, get_tangent};

#[macroquad::main("Mobius Git")]
async fn main() -> anyhow::Result<()> {
    // Load git graph from current directory
    let git_graph = match GitGraph::new(std::path::Path::new("."), 200) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("Error loading git graph: {}", e);
            // Just exit, printing error to console is enough for CLI tool
            std::process::exit(1);
        }
    };

    println!("Loaded {} commits into the graph.", git_graph.graph.node_count());

    // Camera State
    let mut cam_u: f32 = 0.0;
    let mut cam_v: f32 = 0.0;
    let mut cam_dist: f32 = 10.0;
    let look_ahead: f32 = 2.0;

    // Scale u to fit nicely on the strip?
    // One loop is 2*PI (~6.28).
    // If u=index, 1 commit = 1 radian ~ 1/6th of a loop.
    // That's reasonable spacing.
    // We can tweak spacing factor.
    let u_scale = 0.5;

    loop {
        let dt = get_frame_time();

        // Input Handling
        if is_key_down(KeyCode::Right) {
            cam_u += 2.0 * dt;
        }
        if is_key_down(KeyCode::Left) {
            cam_u -= 2.0 * dt;
        }
        if is_key_down(KeyCode::Up) {
            cam_v += 5.0 * dt;
        }
        if is_key_down(KeyCode::Down) {
            cam_v -= 5.0 * dt;
        }
        if is_key_down(KeyCode::Equal) { // +
            cam_dist -= 10.0 * dt;
        }
        if is_key_down(KeyCode::Minus) {
            cam_dist += 10.0 * dt;
        }

        // Clamp distance
        cam_dist = cam_dist.clamp(2.0, 50.0);

        // Calculate Camera Vectors
        let strip_pos = map_to_mobius(cam_u * u_scale, cam_v);
        let normal = get_normal(cam_u * u_scale, cam_v);
        let _tangent = get_tangent(cam_u * u_scale, cam_v); // Forward direction

        // First person view flying along the strip
        let position = strip_pos + normal * cam_dist;

        // Target is ahead on the strip
        let target_pos = map_to_mobius((cam_u + look_ahead) * u_scale, cam_v);

        // Up vector: The normal of the strip
        let up = normal;

        let camera = Camera3D {
            position,
            target: target_pos,
            up,
            ..Default::default()
        };

        clear_background(Color::new(0.05, 0.05, 0.1, 1.0));

        set_camera(&camera);

        // Draw Origin Grid (Debug)
        // draw_grid(20, 1.0, BLACK, GRAY);

        // --- Rendering ---

        let render_dist_u = 20.0; // How far ahead/behind to render
        let u_min = cam_u - render_dist_u;
        let u_max = cam_u + render_dist_u;

        // 1. Draw Strip Mesh (Wireframe)
        // We draw a fixed window around the camera
        let mesh_u_step = 0.2;
        let mesh_v_steps = 4; // +/- 4 lanes

        // Longitudinal lines
        for v_idx in -mesh_v_steps..=mesh_v_steps {
            let v = v_idx as f32 * 1.5; // Wider lanes
            let mut u_curr = u_min.floor();
            let mut p_prev = map_to_mobius(u_curr * u_scale, v);

            while u_curr < u_max {
                u_curr += mesh_u_step;
                let p_next = map_to_mobius(u_curr * u_scale, v);
                draw_line_3d(p_prev, p_next, Color::new(0.3, 0.3, 0.3, 0.5));
                p_prev = p_next;
            }
        }

        // Latitudinal lines
        let mut u_curr = u_min.floor();
        while u_curr < u_max {
            for v_idx in -mesh_v_steps..mesh_v_steps {
                let v = v_idx as f32 * 1.5;
                let v_next = (v_idx + 1) as f32 * 1.5;
                let p1 = map_to_mobius(u_curr * u_scale, v);
                let p2 = map_to_mobius(u_curr * u_scale, v_next);
                draw_line_3d(p1, p2, Color::new(0.2, 0.2, 0.2, 0.3));
            }
            u_curr += mesh_u_step;
        }

        // 2. Draw Git Graph Nodes & Edges
        for idx in git_graph.graph.node_indices() {
             let node = &git_graph.graph[idx];

             // Culling
             if node.u < u_min || node.u > u_max {
                 continue;
             }

             // Scale coordinates
             let node_u_scaled = node.u * u_scale;
             let node_v_scaled = node.v * 1.5; // Match mesh lane width

             let pos = map_to_mobius(node_u_scaled, node_v_scaled);

             // Draw Node
             // Color based on branch? (v)
             let color = if node.v.abs() < 0.1 { BLUE } else { GREEN };
             draw_sphere(pos, 0.3, None, color);

             // Draw Edges (to parents)
             for &parent_oid in &node.parent_ids {
                 if let Some(&p_idx) = git_graph.node_map.get(&parent_oid) {
                     let parent_node = &git_graph.graph[p_idx];

                     let parent_u_scaled = parent_node.u * u_scale;
                     let parent_v_scaled = parent_node.v * 1.5;

                     // Subdivide edge for curvature
                     let u_diff = (node_u_scaled - parent_u_scaled).abs();
                     let segments = (u_diff * 10.0).ceil() as i32;
                     let segments = segments.max(2).min(20);

                     let mut last_p = pos;
                     for i in 1..=segments {
                         let t = i as f32 / segments as f32;
                         // Linear interpolation of (u, v) in topological space
                         // This creates a geodesic-like curve on the strip
                         let curr_u = node_u_scaled * (1.0 - t) + parent_u_scaled * t;
                         let curr_v = node_v_scaled * (1.0 - t) + parent_v_scaled * t;

                         let curr_p = map_to_mobius(curr_u, curr_v);
                         draw_line_3d(last_p, curr_p, RED);
                         last_p = curr_p;
                     }
                 }
             }
        }

        set_default_camera();

        // UI Overlay
        draw_text("Mobius Git Explorer", 20.0, 30.0, 30.0, WHITE);
        draw_text(&format!("Commits: {}", git_graph.graph.node_count()), 20.0, 60.0, 20.0, LIGHTGRAY);
        draw_text(&format!("U: {:.2} (Twist: {:.2} pi)", cam_u, (cam_u * u_scale) / std::f32::consts::PI), 20.0, 80.0, 20.0, LIGHTGRAY);
        draw_text(&format!("V: {:.2} | Dist: {:.2}", cam_v, cam_dist), 20.0, 100.0, 20.0, LIGHTGRAY);
        draw_text("Arrows: Move | +/-: Zoom", 20.0, screen_height() - 30.0, 20.0, WHITE);

        next_frame().await
    }
}
