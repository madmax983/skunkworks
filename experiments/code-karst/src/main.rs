mod graph;
mod layout;
mod karst;

use macroquad::prelude::*;
use std::path::Path;

#[macroquad::main("Code Karst")]
async fn main() {
    let root = Path::new(".");
    let dep_graph = graph::DependencyGraph::build(root);
    let mut layout = layout::Layout::new(&dep_graph);
    let mut grid = karst::VoxelGrid::new();

    let mut cam_dist: f32 = 150.0;
    let mut cam_angle_x: f32 = 0.5;
    let mut cam_angle_y: f32 = 0.5;

    let mut eroding = true;
    let mut physics = true;

    loop {
        // Controls
        if is_key_down(KeyCode::Up) { cam_angle_x += 0.02; }
        if is_key_down(KeyCode::Down) { cam_angle_x -= 0.02; }
        if is_key_down(KeyCode::Left) { cam_angle_y -= 0.02; }
        if is_key_down(KeyCode::Right) { cam_angle_y += 0.02; }
        if is_key_down(KeyCode::W) { cam_dist -= 1.0; }
        if is_key_down(KeyCode::S) { cam_dist += 1.0; }
        if is_key_pressed(KeyCode::Space) { eroding = !eroding; }
        if is_key_pressed(KeyCode::P) { physics = !physics; }

        cam_angle_x = cam_angle_x.clamp(-1.5, 1.5);
        cam_dist = cam_dist.max(10.0);

        // Physics
        if physics {
            layout.update(&dep_graph);
        }

        // Erosion
        if eroding {
            // Pick random edges to erode
            let edges: Vec<_> = dep_graph.graph.edge_indices().collect();
            if !edges.is_empty() {
                for _ in 0..50 { // Erode 50 edges per frame
                    let edge = edges[rand::gen_range(0, edges.len())];
                    if let Some((u, v)) = dep_graph.graph.edge_endpoints(edge) {
                        let pos_u = layout.positions[&u];
                        let pos_v = layout.positions[&v];
                        grid.erode_path(pos_u, pos_v, 0.2);
                    }
                }
            }
        }

        clear_background(BLACK);

        // 3D View
        let cam_pos = vec3(
            cam_angle_y.cos() * cam_dist,
            cam_dist * cam_angle_x.sin(),
            cam_angle_y.sin() * cam_dist,
        );
        let target = vec3(50.0, 50.0, 50.0);

        set_camera(&Camera3D {
            position: cam_pos + target,
            target,
            up: vec3(0.0, 1.0, 0.0),
            fovy: 45.0,
            ..Default::default()
        });

        // Draw Bounding Box
        draw_line_3d(vec3(0.0,0.0,0.0), vec3(100.0,0.0,0.0), RED);
        draw_line_3d(vec3(0.0,0.0,0.0), vec3(0.0,100.0,0.0), GREEN);
        draw_line_3d(vec3(0.0,0.0,0.0), vec3(0.0,0.0,100.0), BLUE);
        draw_line_3d(vec3(100.0,100.0,100.0), vec3(0.0,100.0,100.0), WHITE);
        // ... more lines if needed

        // Draw Nodes
        for pos in layout.positions.values() {
            // Only draw if connected? Or all?
            // Draw small spheres
            draw_cube(*pos, vec3(0.5, 0.5, 0.5), None, Color::new(0.5, 0.5, 0.5, 0.5));
        }

        // Draw Caves (Voxels with low density)
        // Optimization: sparse iteration?
        // We can't iterate 1M voxels every frame efficiently in pure Rust loop + draw calls.
        // But let's try. 100x100x100 is 1M.
        // If density is mostly 1.0, we skip.
        // Only iterate if we know something changed?
        // Or just iterate.
        // 1M iterations is fast in Rust (ms). Draw calls are the bottleneck.
        // Only draw if density < 0.5.
        // Initially 0 cubes.

        let mut cave_voxels = 0;
        for z in (0..karst::GRID_SIZE).step_by(2) { // Step by 2 to reduce count for perf
            for y in (0..karst::GRID_SIZE).step_by(2) {
                for x in (0..karst::GRID_SIZE).step_by(2) {
                    let d = grid.get(x, y, z);
                    if d < 0.5 {
                        let pos = vec3(x as f32, y as f32, z as f32);
                        // Color based on density (lower = darker or brighter?)
                        // Let's make it glow blue/cyan
                        let c = Color::new(0.0, 1.0 - d, 1.0, 0.8);
                        draw_cube(pos, vec3(1.8, 1.8, 1.8), None, c);
                        cave_voxels += 1;
                    }
                }
            }
        }

        set_default_camera();

        // UI
        draw_text("Code Karst: Dependency Cave System", 10.0, 20.0, 30.0, WHITE);
        draw_text(&format!("Nodes: {} | Edges: {}", dep_graph.graph.node_count(), dep_graph.graph.edge_count()), 10.0, 50.0, 20.0, LIGHTGRAY);
        draw_text(&format!("Cave Voxels: {}", cave_voxels), 10.0, 70.0, 20.0, LIGHTGRAY);
        draw_text(&format!("Controls: WASD/Arrows, [Space] Erode: {}, [P] Physics: {}", eroding, physics), 10.0, 90.0, 20.0, LIGHTGRAY);

        next_frame().await;
    }
}
