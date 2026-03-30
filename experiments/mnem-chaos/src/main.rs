use macroquad::prelude::*;


mod graph;
use graph::Graph;

mod glitch;
use glitch::TextGlitcher;

// Simple Double Pendulum state and math
pub struct DoublePendulum {
    pub theta1: f32,
    pub theta2: f32,
    pub p1: f32,
    pub p2: f32,
    pub length1: f32,
    pub length2: f32,
    pub mass1: f32,
    pub mass2: f32,
    pub gravity: f32,
}

impl DoublePendulum {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            theta1: std::f32::consts::PI / 2.0,
            theta2: std::f32::consts::PI / 2.0,
            p1: 0.0,
            p2: 0.0,
            length1: 1.0,
            length2: 1.0,
            mass1: 1.0,
            mass2: 1.0,
            gravity: 9.81,
        }
    }

    pub fn update(&mut self, dt: f32) {
        let l1 = self.length1;
        let l2 = self.length2;
        let m1 = self.mass1;
        let m2 = self.mass2;
        let g = self.gravity;
        let t1 = self.theta1;
        let t2 = self.theta2;
        let p1 = self.p1;
        let p2 = self.p2;

        let delta = t1 - t2;

        let denom1 = m1 + m2 * delta.sin().powi(2);

        let h1 = p1 * p2 * delta.sin() / (l1 * l2 * denom1);
        let h2 = (m2 * l2 * l2 * p1 * p1 + (m1 + m2) * l1 * l1 * p2 * p2 - 2.0 * m2 * l1 * l2 * p1 * p2 * delta.cos()) * delta.sin() * delta.cos() / (2.0 * l1 * l1 * l2 * l2 * denom1 * denom1);

        let d_t1 = (l2 * p1 - l1 * p2 * delta.cos()) / (l1 * l1 * l2 * denom1);
        let d_t2 = ((m1 + m2) * l1 * p2 - m2 * l2 * p1 * delta.cos()) / (m2 * l1 * l2 * l2 * denom1);

        let d_p1 = -(m1 + m2) * g * l1 * t1.sin() - h1 + h2;
        let d_p2 = -m2 * g * l2 * t2.sin() + h1 - h2;

        self.theta1 += d_t1 * dt;
        self.theta2 += d_t2 * dt;
        self.p1 += d_p1 * dt;
        self.p2 += d_p2 * dt;
    }

    pub fn get_positions(&self) -> (f32, f32, f32, f32) {
        let x1 = self.length1 * self.theta1.sin();
        let y1 = self.length1 * self.theta1.cos();
        let x2 = x1 + self.length2 * self.theta2.sin();
        let y2 = y1 + self.length2 * self.theta2.cos();
        (x1, y1, x2, y2)
    }
}

#[macroquad::main("Mnemonic Chaos")]
async fn main() {
    let mut pendulum = DoublePendulum::new();
    let mut graph = Graph::new();

    println!("Scanning directory...");
    graph.scan_directory("experiments/mnem-rot/src");
    if graph.nodes.is_empty() {
        graph.scan_directory(".");
    }

    let mut hovered_node: Option<usize>;

    // Camera
    let mut offset = vec2(0.0, 0.0);
    let mut zoom: f32 = 1.0;
    let mut dragging = false;
    let mut last_mouse = vec2(0.0, 0.0);

    // Path tracing for the pendulum
    let mut path_points = std::collections::VecDeque::new();
    let path_length = 500;

    loop {
        let mouse_pos = mouse_position();
        let mouse_vec = vec2(mouse_pos.0, mouse_pos.1);

        // Input: Camera
        if is_mouse_button_pressed(MouseButton::Right) {
            dragging = true;
            last_mouse = mouse_vec;
        }
        if is_mouse_button_released(MouseButton::Right) {
            dragging = false;
        }
        if dragging {
            let delta = mouse_vec - last_mouse;
            offset += delta;
            last_mouse = mouse_vec;
        }

        let wheel = mouse_wheel();
        if wheel.1 != 0.0 {
            let old_zoom = zoom;
            zoom *= if wheel.1 > 0.0 { 1.1 } else { 0.9 };
            zoom = zoom.clamp(0.1, 10.0);

            // Adjust offset to zoom towards mouse
            offset = mouse_vec - (mouse_vec - offset) * (zoom / old_zoom);
        }

        let dt = get_frame_time().min(0.05);

        // Physics & Decay Loop
        let steps = 10;
        let sub_dt = dt / steps as f32;

        let center_x = screen_width() / 2.0;
        let center_y = screen_height() / 2.0;
        let scale = (screen_height() / 4.0).min(screen_width() / 4.0);

        let mut bob2_positions = vec![];

        for _ in 0..steps {
            pendulum.update(sub_dt);
            let (_x1, _y1, x2, y2) = pendulum.get_positions();
            let bob2_pos = Vec2::new(center_x + x2 * scale, center_y + y2 * scale);
            bob2_positions.push(bob2_pos);

            path_points.push_front(bob2_pos);
            if path_points.len() > path_length {
                path_points.pop_back();
            }
        }

        // --- Graph Physics ---
        let mut forces = vec![vec2(0.0, 0.0); graph.nodes.len()];
        for i in 0..graph.nodes.len() {
            for j in i + 1..graph.nodes.len() {
                let diff = graph.nodes[i].pos - graph.nodes[j].pos;
                let dist_sq = diff.length_squared().max(1.0);
                if dist_sq < 250000.0 {
                    let force = diff.normalize() * (5000.0 / dist_sq);
                    forces[i] += force;
                    forces[j] -= force;
                }
            }
            // Center attraction (Gravity)
            let center = vec2(screen_width() / 2.0, screen_height() / 2.0) - offset;
            let to_center = (center - graph.nodes[i].pos) * 0.01;
            forces[i] += to_center;
        }

        for edge in &graph.edges {
            if edge.from < graph.nodes.len() && edge.to < graph.nodes.len() {
                let n1 = graph.nodes[edge.from].pos;
                let n2 = graph.nodes[edge.to].pos;
                let diff = n2 - n1;
                let dist = diff.length();
                let desired = 100.0;
                let force = diff.normalize() * (dist - desired) * 0.05 * edge.strength;
                forces[edge.from] += force;
                forces[edge.to] -= force;
            }
        }

        hovered_node = None;
        let world_mouse = (mouse_vec - offset) / zoom;

        // Pendulum positions in world coordinates for interactions
        let world_bob_positions: Vec<Vec2> = bob2_positions.iter().map(|p| (*p - offset) / zoom).collect();

        for (i, node) in graph.nodes.iter_mut().enumerate() {
            node.vel += forces[i] * dt;
            node.vel *= 0.90; // Damping
            node.pos += node.vel;

            // Chaotic Decay Mapping
            let mut chaotic_decay = 0.0;
            for bob_pos in &world_bob_positions {
                let dist = node.pos.distance(*bob_pos);
                if dist < 100.0 { // Radius of destruction
                    // Exponential decay based on proximity to the chaotic bob
                    chaotic_decay += (1.0 - (dist / 100.0)).powi(2) * 0.05;
                }
            }
            node.health -= chaotic_decay;

            if node.health < 0.1 {
                node.health = 0.1;
            }

            // Interaction
            if node.pos.distance(world_mouse) < 20.0 {
                hovered_node = Some(i);
                // Heal
                node.health += dt * 5.0; // Fast heal
                if node.health > 1.0 {
                    node.health = 1.0;
                }
            }
        }

        // Render
        clear_background(Color::new(0.05, 0.05, 0.05, 1.0));

        // Draw Edges
        for edge in &graph.edges {
            if edge.from < graph.nodes.len() && edge.to < graph.nodes.len() {
                let n1 = graph.nodes[edge.from].pos * zoom + offset;
                let n2 = graph.nodes[edge.to].pos * zoom + offset;
                let avg_health =
                    (graph.nodes[edge.from].health + graph.nodes[edge.to].health) / 2.0;

                // Jitter if low health
                let jitter = if avg_health < 0.5 {
                    vec2(rand::gen_range(-2.0, 2.0), rand::gen_range(-2.0, 2.0))
                } else {
                    vec2(0.0, 0.0)
                };

                draw_line(
                    n1.x + jitter.x,
                    n1.y + jitter.y,
                    n2.x + jitter.x,
                    n2.y + jitter.y,
                    2.0 * zoom,
                    Color::new(0.5, 0.5, 0.5, avg_health),
                );
            }
        }

        // Draw Nodes
        for (i, node) in graph.nodes.iter().enumerate() {
            let pos = node.pos * zoom + offset;
            let color = if node.health > 0.8 {
                GREEN
            } else if node.health > 0.4 {
                YELLOW
            } else {
                RED
            };

            let radius = 5.0 * zoom * (0.5 + node.health);
            draw_circle(pos.x, pos.y, radius, color);

            // Draw name if healthy enough or hovered
            if node.health > 0.6 || hovered_node == Some(i) {
                draw_text(&node.name, pos.x + 10.0, pos.y, 14.0 * zoom, WHITE);
            }
        }

        // Draw Pendulum Path (Trails)
        for i in 1..path_points.len() {
            let alpha = 1.0 - (i as f32 / path_points.len() as f32);
            let p1 = path_points[i - 1];
            let p2 = path_points[i];
            draw_line(p1.x, p1.y, p2.x, p2.y, 2.0, Color::new(1.0, 0.2, 0.2, alpha));
        }

        // Draw Pendulum
        let (x1, y1, x2, y2) = pendulum.get_positions();
        let px1 = center_x + x1 * scale;
        let py1 = center_y + y1 * scale;
        let px2 = center_x + x2 * scale;
        let py2 = center_y + y2 * scale;

        draw_line(center_x, center_y, px1, py1, 3.0, WHITE);
        draw_circle(px1, py1, 8.0, RED);
        draw_line(px1, py1, px2, py2, 3.0, WHITE);
        draw_circle(px2, py2, 8.0, RED);


        // UI Overlay
        if let Some(idx) = hovered_node {
            let node = &graph.nodes[idx];
            // Panel
            draw_rectangle(
                10.0,
                10.0,
                400.0,
                screen_height() - 20.0,
                Color::new(0.05, 0.05, 0.05, 0.95),
            );
            draw_rectangle_lines(10.0, 10.0, 400.0, screen_height() - 20.0, 2.0, WHITE);

            draw_text(&node.name, 20.0, 40.0, 30.0, GREEN);
            draw_text(
                &format!("Health: {:.0}%", node.health * 100.0),
                20.0,
                70.0,
                20.0,
                WHITE,
            );

            // Content
            let intensity = 1.0 - node.health;
            let corrupted = TextGlitcher::corrupt(&node.content, intensity);

            // Render text lines
            let lines: Vec<&str> = corrupted.lines().take(35).collect();
            for (j, line) in lines.iter().enumerate() {
                // Wrap text manually or just truncate
                let display_line = if line.len() > 50 { &line[0..50] } else { line };
                draw_text(display_line, 20.0, 100.0 + j as f32 * 15.0, 14.0, LIGHTGRAY);
            }
        }

        draw_text(
            "Chaotic Attractor Rotting Codebase",
            screen_width() - 350.0,
            30.0,
            20.0,
            RED,
        );
        draw_text(
            "Right Click: Pan | Scroll: Zoom | Hover: Heal",
            10.0,
            screen_height() - 10.0,
            16.0,
            GRAY,
        );

        next_frame().await
    }
}
