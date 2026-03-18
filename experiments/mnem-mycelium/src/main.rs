use macroquad::prelude::*;

mod graph;
use graph::Graph;

#[derive(Debug, Clone)]
pub struct Agent {
    pub x: f32,
    pub y: f32,
    pub angle: f32,
}

impl Agent {
    pub fn new(x: f32, y: f32, angle: f32) -> Self {
        Self { x, y, angle }
    }

    /// Senses combined value of Trail + Gradient toward most rotting node
    pub fn sense(&self, trail_map: &[f32], width: usize, height: usize, graph: &Graph, angle_offset: f32, sensor_dist: f32) -> f32 {
        let sensor_angle = self.angle + angle_offset;
        let sensor_x = self.x + sensor_angle.cos() * sensor_dist;
        let sensor_y = self.y + sensor_angle.sin() * sensor_dist;

        let sx = (sensor_x.rem_euclid(width as f32)) as usize;
        let sy = (sensor_y.rem_euclid(height as f32)) as usize;

        let trail_strength = trail_map[sy * width + sx];

        // Find the node with the highest entropy (lowest health) to act as a target
        let mut min_health = 2.0;
        let mut target_pos = vec2(self.x, self.y);
        for node in &graph.nodes {
            if node.health < min_health {
                min_health = node.health;
                target_pos = node.pos;
            }
        }

        let dist_sq = target_pos.distance_squared(vec2(sensor_x, sensor_y));
        let dist = dist_sq.sqrt().max(1.0);
        let gradient_strength = 500.0 / dist; // Attraction toward rotting nodes

        trail_strength + gradient_strength
    }
}

pub struct Simulation {
    pub width: usize,
    pub height: usize,
    pub trail_map: Vec<f32>,
    pub next_trail_map: Vec<f32>,
    pub agents: Vec<Agent>,
    pub graph: Graph,
    pub entropy: f32,
}

impl Simulation {
    pub fn new(width: usize, height: usize) -> Self {
        let mut agents = Vec::new();
        let mut rng = ::rand::thread_rng();

        for _ in 0..5000 {
            agents.push(Agent::new(
                ::rand::Rng::gen_range(&mut rng, 0.0..width as f32),
                ::rand::Rng::gen_range(&mut rng, 0.0..height as f32),
                ::rand::Rng::gen_range(&mut rng, 0.0..std::f32::consts::PI * 2.0),
            ));
        }

        let mut graph = Graph::new();
        // Load some nodes
        graph.scan_directory("experiments/mnem-mycelium/src");
        if graph.nodes.is_empty() {
            graph.scan_directory(".");
        }

        Self {
            width,
            height,
            trail_map: vec![0.0; width * height],
            next_trail_map: vec![0.0; width * height],
            agents,
            graph,
            entropy: 0.0,
        }
    }

    pub fn update(&mut self) {
        let dt = get_frame_time();
        self.entropy += 0.0001; // Global rot

        // 1. Update Graph Physics & Decay
        let mut forces = vec![vec2(0.0, 0.0); self.graph.nodes.len()];
        for i in 0..self.graph.nodes.len() {
            for j in i + 1..self.graph.nodes.len() {
                let diff = self.graph.nodes[i].pos - self.graph.nodes[j].pos;
                let dist_sq = diff.length_squared().max(1.0);
                if dist_sq < 250000.0 {
                    let force = diff.normalize() * (5000.0 / dist_sq);
                    forces[i] += force;
                    forces[j] -= force;
                }
            }
            let center = vec2(self.width as f32 / 2.0, self.height as f32 / 2.0);
            let to_center = (center - self.graph.nodes[i].pos) * 0.01;
            forces[i] += to_center;
        }

        for edge in &self.graph.edges {
            if edge.from < self.graph.nodes.len() && edge.to < self.graph.nodes.len() {
                let n1 = self.graph.nodes[edge.from].pos;
                let n2 = self.graph.nodes[edge.to].pos;
                let diff = n2 - n1;
                let dist = diff.length();
                let force = diff.normalize() * (dist - 100.0) * 0.05 * edge.strength;
                forces[edge.from] += force;
                forces[edge.to] -= force;
            }
        }

        for (i, node) in self.graph.nodes.iter_mut().enumerate() {
            node.vel += forces[i] * dt;
            node.vel *= 0.90;
            node.pos += node.vel;

            node.health -= self.entropy * dt * 0.05;
            if node.health < 0.1 {
                node.health = 0.1;
            }
        }

        // 2. Update Slime Mold Agents
        let sensor_angle = std::f32::consts::PI / 4.0;
        let sensor_dist = 9.0;
        let turn_angle = std::f32::consts::PI / 8.0;
        let speed = 2.0;

        let mut rng = ::rand::thread_rng();

        for agent in &mut self.agents {
            let weight_fwd = agent.sense(&self.trail_map, self.width, self.height, &self.graph, 0.0, sensor_dist);
            let weight_left = agent.sense(&self.trail_map, self.width, self.height, &self.graph, -sensor_angle, sensor_dist);
            let weight_right = agent.sense(&self.trail_map, self.width, self.height, &self.graph, sensor_angle, sensor_dist);

            let random_steer = ::rand::Rng::gen_range(&mut rng, 0.0..1.0);

            if weight_fwd > weight_left && weight_fwd > weight_right {
                // Keep going
            } else if weight_fwd < weight_left && weight_fwd < weight_right {
                if random_steer < 0.5 {
                    agent.angle += turn_angle;
                } else {
                    agent.angle -= turn_angle;
                }
            } else if weight_left > weight_right {
                agent.angle -= turn_angle;
            } else if weight_right > weight_left {
                agent.angle += turn_angle;
            }

            // Move
            agent.x += agent.angle.cos() * speed;
            agent.y += agent.angle.sin() * speed;

            // Wrap
            agent.x = agent.x.rem_euclid(self.width as f32);
            agent.y = agent.y.rem_euclid(self.height as f32);

            // Deposit pheromone
            let px = agent.x as usize;
            let py = agent.y as usize;
            if px < self.width && py < self.height {
                self.trail_map[py * self.width + px] += 50.0;
            }

            // If an agent reaches a rotting node, it heals it slightly
            for node in &mut self.graph.nodes {
                if node.pos.distance_squared(vec2(agent.x, agent.y)) < 400.0 {
                    node.health += 0.01;
                    if node.health > 1.0 {
                        node.health = 1.0;
                    }
                }
            }
        }

        // 3. Diffuse & Decay Trail
        let decay_factor = 0.95;
        let w = self.width;
        let h = self.height;

        // Make sure we have the grid bounds correctly
        for y in 0..h {
            for x in 0..w {
                let mut sum = 0.0;
                let mut count = 0.0;

                for dy in -1..=1 {
                    for dx in -1..=1 {
                        let nx = (x as isize + dx).rem_euclid(w as isize) as usize;
                        let ny = (y as isize + dy).rem_euclid(h as isize) as usize;
                        sum += self.trail_map[ny * w + nx];
                        count += 1.0;
                    }
                }

                let blurred = sum / count;
                self.next_trail_map[y * w + x] = blurred * decay_factor;
            }
        }

        std::mem::swap(&mut self.trail_map, &mut self.next_trail_map);
    }

    pub fn draw(&self) {
        // Draw trails
        for y in 0..self.height {
            for x in 0..self.width {
                let val = self.trail_map[y * self.width + x];
                if val > 5.0 {
                    let intensity = (val / 50.0).clamp(0.0, 1.0);
                    draw_rectangle(
                        x as f32,
                        y as f32,
                        1.0,
                        1.0,
                        Color::new(0.0, intensity, intensity * 0.5, 1.0),
                    );
                }
            }
        }

        // Draw graph edges
        for edge in &self.graph.edges {
            if edge.from < self.graph.nodes.len() && edge.to < self.graph.nodes.len() {
                let n1 = self.graph.nodes[edge.from].pos;
                let n2 = self.graph.nodes[edge.to].pos;
                let avg_health = (self.graph.nodes[edge.from].health + self.graph.nodes[edge.to].health) / 2.0;

                let jitter = if avg_health < 0.5 {
                    vec2(
                        ::macroquad::rand::gen_range(-2.0, 2.0),
                        ::macroquad::rand::gen_range(-2.0, 2.0)
                    )
                } else {
                    vec2(0.0, 0.0)
                };

                draw_line(
                    n1.x + jitter.x,
                    n1.y + jitter.y,
                    n2.x + jitter.x,
                    n2.y + jitter.y,
                    2.0,
                    Color::new(0.5, 0.5, 0.5, avg_health * 0.5),
                );
            }
        }

        // Draw graph nodes
        for node in &self.graph.nodes {
            let color = if node.health > 0.8 {
                GREEN
            } else if node.health > 0.4 {
                YELLOW
            } else {
                RED
            };

            let radius = 5.0 * (0.5 + node.health);
            draw_circle(node.pos.x, node.pos.y, radius, color);

            if node.health > 0.6 {
                draw_text(&node.name, node.pos.x + 10.0, node.pos.y, 14.0, WHITE);
            }
        }
    }
}

#[macroquad::main("Mnem-Mycelium")]
async fn main() {
    let width = screen_width() as usize;
    let height = screen_height() as usize;
    let mut sim = Simulation::new(width, height);

    loop {
        // If window resizes, we just run on the original grid size for simplicity,
        // or re-init grid. But for this experiment, static grid size is fine.

        sim.update();

        clear_background(BLACK);
        sim.draw();

        draw_text(
            &format!("Agents: {} | Entropy: {:.4}", sim.agents.len(), sim.entropy),
            10.0,
            20.0,
            20.0,
            WHITE,
        );

        next_frame().await
    }
}
