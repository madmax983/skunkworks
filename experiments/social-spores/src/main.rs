use macroquad::prelude::*;
use ::rand::Rng;

const GRID_SIZE: usize = 40;

#[derive(Clone, Copy)]
pub struct Spore {
    pub pos: Vec2,
    pub vel: Vec2,
    pub color: Color,
    pub life: f32,
    pub max_life: f32,
}

impl Spore {
    pub fn new(pos: Vec2, color: Color) -> Self {
        let mut rng = ::rand::thread_rng();
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        let speed = rng.gen_range(0.5..2.0);
        Self {
            pos,
            vel: vec2(angle.cos() * speed, angle.sin() * speed),
            color,
            life: rng.gen_range(2.0..5.0), // Initial life
            max_life: 5.0,
        }
    }

    pub fn update(&mut self, dt: f32, wind: Vec2) {
        self.vel += wind * dt * 5.0; // Wind influence
        self.vel *= 0.98; // Friction
        self.pos += self.vel;
        self.life -= dt;
    }
}

pub struct Node {
    pub pos: Vec2,
    pub radius: f32,
    pub influence: f32, // Determines radius and spore count
    pub color: Color,
    pub pulse: f32,
}

impl Node {
    pub fn new(pos: Vec2, color: Color) -> Self {
        Self {
            pos,
            radius: 10.0,
            influence: 20.0,
            color,
            pulse: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32) -> Option<Vec<Spore>> {
        self.pulse += dt * (1.0 + self.influence * 0.05);

        // Viral Burst Check
        if self.influence > 50.0 {
            self.influence = 20.0; // Reset
            self.radius = 10.0 + self.influence * 0.5;
            let count = 200;
            let mut spores = Vec::with_capacity(count);
            for _ in 0..count {
                let mut s = Spore::new(self.pos, self.color);
                s.vel *= 3.0; // Fast burst
                spores.push(s);
            }
            return Some(spores);
        }

        if self.pulse > std::f32::consts::TAU {
            self.pulse -= std::f32::consts::TAU;
            // Emit spores based on influence
            let count = (self.influence as usize / 5).max(1);
            let mut spores = Vec::with_capacity(count);
            for _ in 0..count {
                spores.push(Spore::new(self.pos, self.color));
            }
            return Some(spores);
        }
        None
    }
}

pub struct World {
    pub nodes: Vec<Node>,
    pub spores: Vec<Spore>,
    pub wind_grid: [[Vec2; GRID_SIZE]; GRID_SIZE],
    pub time: f64,
    pub width: f32,
    pub height: f32,
}

impl World {
    pub fn new(width: f32, height: f32) -> Self {
        let mut wind_grid = [[Vec2::ZERO; GRID_SIZE]; GRID_SIZE];
        for y in 0..GRID_SIZE {
            for x in 0..GRID_SIZE {
                wind_grid[y][x] = vec2(0.0, 0.0);
            }
        }

        Self {
            nodes: Vec::new(),
            spores: Vec::new(),
            wind_grid,
            time: 0.0,
            width,
            height,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.time += dt as f64;

        // Update wind (simple noise-like behavior)
        for y in 0..GRID_SIZE {
            for x in 0..GRID_SIZE {
                let fx = x as f64 * 0.1 + self.time * 0.5;
                let fy = y as f64 * 0.1 + self.time * 0.2;
                // Simple pseudo-noise
                let angle = (fx.sin() + fy.cos()) * std::f64::consts::PI;
                self.wind_grid[y][x] = vec2(angle.cos() as f32, angle.sin() as f32) * 0.5;
            }
        }

        // Process Spores & Collisions
        let mut hits: Vec<(usize, Color)> = Vec::new(); // (node_index, spore_color)
        let width = self.width;
        let height = self.height;
        let wind_grid = &self.wind_grid;
        let nodes = &self.nodes;

        self.spores.retain_mut(|spore| {
            // Update physics
            let gx = (spore.pos.x / width * GRID_SIZE as f32).clamp(0.0, (GRID_SIZE - 1) as f32);
            let gy = (spore.pos.y / height * GRID_SIZE as f32).clamp(0.0, (GRID_SIZE - 1) as f32);
            let wind = wind_grid[gy as usize][gx as usize];

            spore.update(dt, wind);

            // Bounds check
            if spore.life <= 0.0 || spore.pos.x <= 0.0 || spore.pos.x >= width || spore.pos.y <= 0.0 || spore.pos.y >= height {
                return false;
            }

            // Collision check
            for (idx, node) in nodes.iter().enumerate() {
                if spore.pos.distance_squared(node.pos) < node.radius * node.radius {
                    hits.push((idx, spore.color));
                    return false; // Spore consumed
                }
            }

            true
        });

        // Apply hits to Nodes
        for (node_idx, spore_color) in hits {
            let node = &mut self.nodes[node_idx];
            // Color distance (simple RGB distance)
            let dr = (node.color.r - spore_color.r).abs();
            let dg = (node.color.g - spore_color.g).abs();
            let db = (node.color.b - spore_color.b).abs();
            let dist = dr + dg + db;

            if dist < 0.2 {
                // Agreement: Increase influence
                node.influence += 0.5;
            } else {
                // Disagreement: Change opinion slightly
                let mix = 0.05;
                node.color.r = node.color.r * (1.0 - mix) + spore_color.r * mix;
                node.color.g = node.color.g * (1.0 - mix) + spore_color.g * mix;
                node.color.b = node.color.b * (1.0 - mix) + spore_color.b * mix;
                node.influence += 0.1; // Any engagement is good?
            }
            node.radius = 10.0 + node.influence * 0.5;
        }

        // Update Nodes and spawn spores
        let mut new_spores = Vec::new();
        for node in &mut self.nodes {
            if let Some(mut spores) = node.update(dt) {
                new_spores.append(&mut spores);
            }
        }
        self.spores.append(&mut new_spores);
    }
}

#[macroquad::main("Social Spores")]
async fn main() {
    let mut world = World::new(screen_width(), screen_height());

    // Add some initial nodes
    world.nodes.push(Node::new(vec2(world.width * 0.2, world.height * 0.5), RED));
    world.nodes.push(Node::new(vec2(world.width * 0.8, world.height * 0.5), BLUE));
    world.nodes.push(Node::new(vec2(world.width * 0.5, world.height * 0.2), GREEN));

    loop {
        let dt = get_frame_time();

        // Input
        if is_mouse_button_pressed(MouseButton::Left) {
            let mpos = mouse_position();
            let mut rng = ::rand::thread_rng();
            let color = Color::new(rng.gen(), rng.gen(), rng.gen(), 1.0);
            world.nodes.push(Node::new(vec2(mpos.0, mpos.1), color));
        }

        if is_key_pressed(KeyCode::R) {
             world = World::new(screen_width(), screen_height());
        }

        if is_key_pressed(KeyCode::Space) {
             world.time += 100.0; // Jump time to shift wind
        }

        // Update
        world.update(dt);

        // Draw
        clear_background(BLACK);

        // Draw Wind (Subtle)
        for y in 0..GRID_SIZE {
            for x in 0..GRID_SIZE {
                let wind = world.wind_grid[y][x];
                let px = (x as f32 / GRID_SIZE as f32) * screen_width();
                let py = (y as f32 / GRID_SIZE as f32) * screen_height();
                draw_line(px, py, px + wind.x * 20.0, py + wind.y * 20.0, 1.0, Color::new(0.2, 0.2, 0.2, 0.3));
            }
        }

        // Draw Spores
        for spore in &world.spores {
            let alpha = (spore.life / spore.max_life).clamp(0.0, 1.0);
            let color = Color::new(spore.color.r, spore.color.g, spore.color.b, alpha);
            draw_circle(spore.pos.x, spore.pos.y, 1.5, color);
        }

        // Draw Nodes
        for node in &world.nodes {
            let pulse_scale = 1.0 + (node.pulse.sin() * 0.2);
            draw_circle(node.pos.x, node.pos.y, node.radius * pulse_scale, node.color);
            draw_circle_lines(node.pos.x, node.pos.y, node.radius * pulse_scale, 2.0, WHITE);
        }

        draw_text(format!("Nodes: {}", world.nodes.len()).as_str(), 10.0, 20.0, 20.0, WHITE);
        draw_text(format!("Spores: {}", world.spores.len()).as_str(), 10.0, 40.0, 20.0, WHITE);
        draw_text("Click: Add Node | Space: Shift Wind | R: Reset", 10.0, screen_height() - 20.0, 20.0, GRAY);

        next_frame().await
    }
}
