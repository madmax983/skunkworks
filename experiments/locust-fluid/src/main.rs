use macroquad::prelude::*;
use ::rand::Rng;
use rayon::prelude::*;

const WORLD_SIZE: f32 = 1000.0;
const AGENT_COUNT: usize = 3000;
const SPEED: f32 = 3.0;
const REPULSION_RADIUS: f32 = 12.0;
const REPULSION_FORCE: f32 = 1.0;
const TARGET_ATTRACTION: f32 = 0.5;

#[derive(Clone, Copy)]
struct Agent {
    pos: Vec2,
    vel: Vec2,
    state: u8, // 0 = alive, 1 = dead
}

struct World {
    agents: Vec<Agent>,
    target: Vec2,
    firewalls: Vec<(Vec2, f32)>,
    server_health: f64,
    max_health: f64,
}

impl World {
    fn new() -> Self {
        let mut rng = ::rand::thread_rng();
        let mut agents = Vec::with_capacity(AGENT_COUNT);
        for _ in 0..AGENT_COUNT {
            let side = rng.gen_range(0..4);
            let (x, y) = match side {
                0 => (rng.gen_range(0.0..WORLD_SIZE), 0.0),
                1 => (rng.gen_range(0.0..WORLD_SIZE), WORLD_SIZE),
                2 => (0.0, rng.gen_range(0.0..WORLD_SIZE)),
                _ => (WORLD_SIZE, rng.gen_range(0.0..WORLD_SIZE)),
            };
            agents.push(Agent {
                pos: vec2(x, y),
                vel: vec2(0.0, 0.0),
                state: 0,
            });
        }

        Self {
            agents,
            target: vec2(WORLD_SIZE / 2.0, WORLD_SIZE / 2.0),
            firewalls: Vec::new(),
            server_health: 1000.0,
            max_health: 1000.0,
        }
    }

    fn update(&mut self) {
        let target = self.target;
        let firewalls = &self.firewalls;

        // Fluid mechanics: We'll use a spatial grid to optimize the O(N^2) particle repulsion.
        let grid_size = REPULSION_RADIUS;
        let grid_cols = (WORLD_SIZE / grid_size).ceil() as usize;
        let grid_rows = (WORLD_SIZE / grid_size).ceil() as usize;

        let mut grid: Vec<Vec<usize>> = vec![vec![]; grid_cols * grid_rows];
        for (i, agent) in self.agents.iter().enumerate() {
            if agent.state == 0 {
                let gx = (agent.pos.x / grid_size).clamp(0.0, (grid_cols - 1) as f32) as usize;
                let gy = (agent.pos.y / grid_size).clamp(0.0, (grid_rows - 1) as f32) as usize;
                grid[gy * grid_cols + gx].push(i);
            }
        }

        let agents_ref = &self.agents;

        let updates: Vec<(Vec2, Vec2, u8, f32)> = self
            .agents
            .par_iter()
            .enumerate()
            .map(|(i, agent)| {
                if agent.state == 1 {
                    let mut rng = ::rand::thread_rng();
                    if rng.gen_bool(0.01) {
                        let side = rng.gen_range(0..4);
                        let (x, y) = match side {
                            0 => (rng.gen_range(0.0..WORLD_SIZE), 0.0),
                            1 => (rng.gen_range(0.0..WORLD_SIZE), WORLD_SIZE),
                            2 => (0.0, rng.gen_range(0.0..WORLD_SIZE)),
                            _ => (WORLD_SIZE, rng.gen_range(0.0..WORLD_SIZE)),
                        };
                        return (vec2(x, y), vec2(0.0, 0.0), 0, 0.0);
                    }
                    return (agent.pos, agent.vel, 1, 0.0);
                }

                let mut desire = vec2(0.0, 0.0);
                let to_target = target - agent.pos;
                let dist_target = to_target.length();

                if dist_target < 15.0 {
                    let mut rng = ::rand::thread_rng();
                    let side = rng.gen_range(0..4);
                    let (x, y) = match side {
                        0 => (rng.gen_range(0.0..WORLD_SIZE), 0.0),
                        1 => (rng.gen_range(0.0..WORLD_SIZE), WORLD_SIZE),
                        2 => (0.0, rng.gen_range(0.0..WORLD_SIZE)),
                        _ => (WORLD_SIZE, rng.gen_range(0.0..WORLD_SIZE)),
                    };
                    return (vec2(x, y), vec2(0.0, 0.0), 0, 1.0);
                }

                if dist_target > 0.0 {
                    desire += to_target.normalize() * TARGET_ATTRACTION;
                }

                // Fluid repulsion (O(N) optimized with grid)
                let gx = (agent.pos.x / grid_size).clamp(0.0, (grid_cols - 1) as f32) as isize;
                let gy = (agent.pos.y / grid_size).clamp(0.0, (grid_rows - 1) as f32) as isize;

                let mut repulsion = vec2(0.0, 0.0);

                for dy in -1..=1 {
                    for dx in -1..=1 {
                        let nx = gx + dx;
                        let ny = gy + dy;
                        if nx >= 0 && nx < grid_cols as isize && ny >= 0 && ny < grid_rows as isize {
                            let idx = (ny as usize) * grid_cols + (nx as usize);
                            for &other_idx in &grid[idx] {
                                if other_idx != i {
                                    let other_agent = &agents_ref[other_idx];
                                    let to_me = agent.pos - other_agent.pos;
                                    let dist = to_me.length();
                                    if dist > 0.0 && dist < REPULSION_RADIUS {
                                        let push_strength = 1.0 - (dist / REPULSION_RADIUS);
                                        repulsion += to_me.normalize() * push_strength * REPULSION_FORCE;
                                    }
                                }
                            }
                        }
                    }
                }

                desire += repulsion;

                let steer = (desire - agent.vel).clamp_length_max(0.5);
                let new_vel = (agent.vel + steer).clamp_length_max(SPEED);
                let mut new_pos = agent.pos + new_vel;

                let mut state = 0;
                for (center, radius) in firewalls {
                    if new_pos.distance(*center) < *radius {
                        state = 1; // Blocked by firewall
                        break;
                    }
                }

                if new_pos.x < 0.0 || new_pos.x > WORLD_SIZE || new_pos.y < 0.0 || new_pos.y > WORLD_SIZE {
                    new_pos = new_pos.clamp(vec2(0.0, 0.0), vec2(WORLD_SIZE, WORLD_SIZE));
                }

                (new_pos, new_vel, state, 0.0)
            })
            .collect();

        let mut total_damage = 0.0;
        for (i, (pos, vel, state, damage)) in updates.into_iter().enumerate() {
            self.agents[i].pos = pos;
            self.agents[i].vel = vel;
            self.agents[i].state = state;
            total_damage += damage;
        }

        self.server_health = (self.server_health - total_damage as f64).max(0.0);
    }

    fn render_to_buffer(&self, buffer: &mut [u8], width: usize, height: usize) {
        buffer.par_chunks_exact_mut(4).for_each(|pixel| {
            pixel[0] = 5;
            pixel[1] = 5;
            pixel[2] = 20;
            pixel[3] = 255;
        });

        let scale_x = width as f32 / WORLD_SIZE;
        let scale_y = height as f32 / WORLD_SIZE;

        for agent in &self.agents {
            let px = (agent.pos.x * scale_x) as isize;
            let py = (agent.pos.y * scale_y) as isize;

            if px >= 0 && px < width as isize && py >= 0 && py < height as isize {
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        let sx = px + dx;
                        let sy = py + dy;
                        if sx >= 0 && sx < width as isize && sy >= 0 && sy < height as isize {
                            let idx = ((sy as usize) * width + (sx as usize)) * 4;
                            if agent.state == 1 {
                                buffer[idx] = buffer[idx].saturating_add(50);
                                buffer[idx + 1] = buffer[idx + 1].saturating_add(20);
                                buffer[idx + 2] = buffer[idx + 2].saturating_add(20);
                            } else {
                                buffer[idx] = buffer[idx].saturating_add(10);
                                buffer[idx + 1] = buffer[idx + 1].saturating_add(40);
                                buffer[idx + 2] = buffer[idx + 2].saturating_add(100);
                            }
                        }
                    }
                }
            }
        }
    }
}

#[macroquad::main("Locust Fluid")]
async fn main() {
    let mut world = World::new();
    let width = 800;
    let height = 800;
    let mut image = Image::gen_image_color(width as u16, height as u16, BLACK);
    let texture = Texture2D::from_image(&image);

    loop {
        let mouse_pos = mouse_position();
        let world_mouse = vec2(
            mouse_pos.0 / screen_width() * WORLD_SIZE,
            mouse_pos.1 / screen_height() * WORLD_SIZE,
        );

        if is_mouse_button_down(MouseButton::Left) {
            world.firewalls.push((world_mouse, 30.0));
        }

        if is_key_pressed(KeyCode::C) {
            world.firewalls.clear();
        }

        world.update();
        world.render_to_buffer(&mut image.bytes, width, height);
        texture.update(&image);

        clear_background(BLACK);
        draw_texture_ex(
            &texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );

        for (pos, radius) in &world.firewalls {
            let sx = pos.x / WORLD_SIZE * screen_width();
            let sy = pos.y / WORLD_SIZE * screen_height();
            let sr = radius / WORLD_SIZE * screen_width();
            draw_circle(sx, sy, sr, Color::new(1.0, 0.0, 0.0, 0.2));
        }

        let tx = world.target.x / WORLD_SIZE * screen_width();
        let ty = world.target.y / WORLD_SIZE * screen_height();

        let health_pct = (world.server_health / world.max_health).clamp(0.0, 1.0) as f32;
        let server_color = Color::new(1.0 - health_pct, 0.0, health_pct, 1.0);

        draw_circle(tx, ty, 20.0, server_color);
        draw_text("SERVER", tx - 30.0, ty - 25.0, 20.0, WHITE);
        draw_rectangle(tx - 40.0, ty + 20.0, 80.0, 8.0, RED);
        draw_rectangle(tx - 40.0, ty + 20.0, 80.0 * health_pct, 8.0, GREEN);

        draw_text(&format!("FPS: {}", get_fps()), 10.0, 30.0, 30.0, WHITE);
        draw_text(
            &format!("Packets: {}", world.agents.len()),
            10.0,
            60.0,
            30.0,
            WHITE,
        );
        draw_text(
            &format!("Server Health: {:.1}%", health_pct * 100.0),
            10.0,
            90.0,
            30.0,
            if health_pct < 0.2 { RED } else { WHITE },
        );
        draw_text(
            "Left Click: Deploy Firewall | C: Clear Rules",
            10.0,
            screen_height() - 20.0,
            20.0,
            LIGHTGRAY,
        );

        next_frame().await
    }
}
