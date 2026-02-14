mod kinematics;

use macroquad::prelude::*;
use kinematics::MiuraGrid;
use market_sim::{Grid as MarketGrid, Particle};

#[macroquad::main("Market Fold")]
async fn main() {
    let cols = 20;
    let rows = 20;

    // Physical Grid
    let grid = MiuraGrid::new(cols, rows);

    // Market Grid (same dimensions)
    let mut market = MarketGrid::new(cols, rows);

    // Camera state
    let mut cam_yaw: f32 = 0.0;
    let mut cam_pitch: f32 = 0.8;
    let mut cam_dist: f32 = 50.0;

    let mut last_mouse_pos = mouse_position();

    // Simulation State
    let mut expansion = 0.5;
    let mut target_expansion = 0.5;
    let mut trade_history: Vec<f32> = Vec::new();

    loop {
        let dt = get_frame_time();

        // --- Input: Camera ---
        if is_mouse_button_down(MouseButton::Left) {
            let mouse_pos = mouse_position();
            let delta = vec2(
                mouse_pos.0 - last_mouse_pos.0,
                mouse_pos.1 - last_mouse_pos.1,
            );
            cam_yaw -= delta.x * 0.01;
            cam_pitch += delta.y * 0.01;
            cam_pitch = cam_pitch.clamp(-1.5, 1.5);
        }
        last_mouse_pos = mouse_position();

        let wheel = mouse_wheel().1;
        cam_dist -= wheel * 0.1 * cam_dist;
        cam_dist = cam_dist.clamp(5.0, 150.0);

        // --- Market Simulation ---
        let mut rng = ::rand::thread_rng();
        use ::rand::Rng;

        // Spawn Bids (at bottom/high index) and Asks (at top/low index)
        // Rate depends on mouse or just random noise
        if rng.gen_bool(0.3) {
            let x = rng.gen_range(0..cols);
            // Spawn Ask at top (y=0)
            market.set(x, 0, Particle::Ask(rng.gen()));
        }
        if rng.gen_bool(0.3) {
            let x = rng.gen_range(0..cols);
            // Spawn Bid at bottom (y=rows-1)
            market.set(x, rows - 1, Particle::Bid(rng.gen()));
        }

        let events = market.update();
        let trade_count = events.len();

        // Calculate Target Expansion
        // More trades = More liquidity = More Stability = Flatter (1.0)
        // No trades = Low liquidity = Volatility/Risk = Folded (< 1.0)

        // Base volatility factor
        let activity = (market.total_bids + market.total_asks) as f32;
        let stability = trade_count as f32 * 5.0 + activity * 0.1;

        let target_val = 0.1 + (stability * 0.05).min(0.9);
        target_expansion = target_val;

        // Smooth interpolation
        expansion += (target_expansion - expansion) * 2.0 * dt;
        expansion = expansion.clamp(0.01, 1.0);

        if trade_count > 0 {
            trade_history.push(1.0);
        } else {
            trade_history.push(0.0);
        }
        if trade_history.len() > 100 {
            trade_history.remove(0);
        }

        // --- Rendering ---
        clear_background(Color::new(0.05, 0.05, 0.1, 1.0));

        let cam_pos = vec3(
            cam_yaw.cos() * cam_pitch.cos() * cam_dist,
            cam_pitch.sin() * cam_dist,
            cam_yaw.sin() * cam_pitch.cos() * cam_dist,
        );

        set_camera(&Camera3D {
            position: cam_pos,
            target: vec3(cols as f32 * grid.params.a * 0.5, rows as f32 * grid.params.b * 0.5, 0.0),
            up: vec3(0.0, 1.0, 0.0),
            ..Default::default()
        });

        // 1. Get Vertices
        let vertices = grid.get_vertices(expansion);

        // 2. Build Mesh
        let mut mesh = Mesh {
            vertices: Vec::new(),
            indices: Vec::new(),
            texture: None,
        };

        let width = cols + 1;

        for j in 0..rows {
            for i in 0..cols {
                let idx0 = (j * width + i) as u16;
                let idx1 = (j * width + i + 1) as u16;
                let idx2 = ((j + 1) * width + i + 1) as u16;
                let idx3 = ((j + 1) * width + i) as u16;

                let v0 = vertices[idx0 as usize];
                let v1 = vertices[idx1 as usize];
                let v2 = vertices[idx2 as usize];
                let v3 = vertices[idx3 as usize];

                let normal = (v1 - v0).cross(v2 - v0).normalize();
                let normal_v4 = vec4(normal.x, normal.y, normal.z, 1.0);

                // Color based on Market State at (i, j)
                // Note: market grid is (x, y) where y=0 is top (Ask spawn)
                // We map market(x, y) to mesh quad (i, j)
                let cell = market.get(i, j);
                let color = match cell {
                    Particle::Empty => Color::new(0.1, 0.1, 0.3, 1.0), // Dark Blue
                    Particle::Bid(_) => Color::new(0.2, 0.8, 0.2, 1.0), // Green
                    Particle::Ask(_) => Color::new(0.8, 0.2, 0.2, 1.0), // Red
                    Particle::Trade { age } => {
                        let intensity = (age as f32 / 5.0).clamp(0.0, 1.0);
                        Color::new(1.0, 1.0, 1.0, intensity) // White Flash
                    }
                };

                let color_bytes: [u8; 4] = color.into();

                let base_idx = mesh.vertices.len() as u16;

                mesh.vertices.push(Vertex { position: v0, uv: vec2(0.,0.), color: color_bytes, normal: normal_v4 });
                mesh.vertices.push(Vertex { position: v1, uv: vec2(1.,0.), color: color_bytes, normal: normal_v4 });
                mesh.vertices.push(Vertex { position: v2, uv: vec2(1.,1.), color: color_bytes, normal: normal_v4 });
                mesh.vertices.push(Vertex { position: v3, uv: vec2(0.,1.), color: color_bytes, normal: normal_v4 });

                mesh.indices.push(base_idx);
                mesh.indices.push(base_idx + 1);
                mesh.indices.push(base_idx + 2);

                mesh.indices.push(base_idx);
                mesh.indices.push(base_idx + 2);
                mesh.indices.push(base_idx + 3);
            }
        }

        draw_mesh(&mesh);

        // Draw Wireframe
        for j in 0..=rows {
            for i in 0..cols {
                 let idx1 = j * width + i;
                 let idx2 = j * width + i + 1;
                 draw_line_3d(vertices[idx1], vertices[idx2], WHITE);
            }
        }
        for j in 0..rows {
            for i in 0..=cols {
                 let idx1 = j * width + i;
                 let idx2 = (j + 1) * width + i;
                 draw_line_3d(vertices[idx1], vertices[idx2], WHITE);
            }
        }

        set_default_camera();

        // UI
        draw_text("Market Fold 📈 Origami Finance", 10.0, 30.0, 30.0, WHITE);
        draw_text(&format!("Expansion: {:.2}", expansion), 10.0, 50.0, 20.0, LIGHTGRAY);
        draw_text(&format!("Bids: {} | Asks: {} | Trades: {}", market.total_bids, market.total_asks, trade_count), 10.0, 70.0, 20.0, YELLOW);

        // Mini graph of activity
        let graph_x = 10.0;
        let graph_y = 150.0;
        let graph_w = 200.0;
        let graph_h = 50.0;
        draw_rectangle(graph_x, graph_y, graph_w, graph_h, Color::new(0.0, 0.0, 0.0, 0.5));
        for (i, val) in trade_history.iter().enumerate() {
            if *val > 0.0 {
                let x = graph_x + (i as f32 / 100.0) * graph_w;
                draw_line(x, graph_y + graph_h, x, graph_y, 2.0, GREEN);
            }
        }

        next_frame().await
    }
}
