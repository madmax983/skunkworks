mod graph;
mod terrain;

use macroquad::prelude::*;
use graph::CallGraph;
use terrain::Terrain;

fn window_conf() -> Conf {
    Conf {
        window_title: "Trace Erosion".to_owned(),
        window_width: 1280,
        window_height: 720,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut graph = CallGraph::generate_random();
    let mut terrain = Terrain::new();

    // Initial uplift based on graph structure
    terrain.uplift(&graph);

    let mut camera = Camera3D {
        position: vec3(0., 150., 250.),
        target: vec3(0., 0., 0.),
        up: vec3(0., 1., 0.),
        fovy: 45.,
        aspect: None,
        projection: Projection::Perspective,
        render_target: None,
        viewport: None,
        z_near: 0.1,
        z_far: 1000.0,
    };

    let mut eroding = true;

    loop {
        clear_background(SKYBLUE);

        // Input
        if is_key_pressed(KeyCode::Space) {
            eroding = !eroding;
        }
        if is_key_pressed(KeyCode::R) {
            graph = CallGraph::generate_random();
            terrain = Terrain::new();
            terrain.uplift(&graph);
        }

        // Camera controls
        let speed = 2.0;
        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
            camera.position.z -= speed;
            camera.target.z -= speed;
        }
        if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
            camera.position.z += speed;
            camera.target.z += speed;
        }
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            camera.position.x -= speed;
            camera.target.x -= speed;
        }
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            camera.position.x += speed;
            camera.target.x += speed;
        }
        if is_key_down(KeyCode::Q) {
            camera.position.y -= speed;
        }
        if is_key_down(KeyCode::E) {
            camera.position.y += speed;
        }

        // Update
        if eroding {
            terrain.erode(&graph);
        }

        // Draw
        set_camera(&camera);

        draw_grid(20, 20., BLACK, GRAY);

        // Draw Terrain Mesh
        let mesh = terrain.get_mesh();
        draw_mesh(&mesh);

        // Draw Nodes
        for node in &graph.nodes {
            // Draw nodes floating above potential max height
            draw_sphere(vec3(node.pos.x, 60.0, node.pos.y), 3.0, None, RED);
        }

        // Draw Edges (visualize flow paths as faint lines)
        for edge in &graph.edges {
            let start = graph.nodes[edge.source].pos;
            let end = graph.nodes[edge.target].pos;
            draw_line_3d(
                vec3(start.x, 60.0, start.y),
                vec3(end.x, 60.0, end.y),
                Color::new(0.0, 0.0, 1.0, 0.3)
            );
        }

        set_default_camera();

        draw_text("Trace Erosion", 10.0, 30.0, 30.0, BLACK);
        draw_text("WASD/Arrows: Move, Q/E: Up/Down", 10.0, 50.0, 20.0, DARKGRAY);
        draw_text("Space: Pause/Resume Erosion", 10.0, 70.0, 20.0, DARKGRAY);
        draw_text("R: Reset (New Graph)", 10.0, 90.0, 20.0, DARKGRAY);
        draw_text(&format!("Nodes: {}, Edges: {}", graph.nodes.len(), graph.edges.len()), 10.0, 110.0, 20.0, DARKGRAY);

        next_frame().await
    }
}
