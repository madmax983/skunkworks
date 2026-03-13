use ::rand::prelude::*;
use rayon::prelude::*;

// Lineage: Locust DDoS swarm mechanics
pub const AGENT_COUNT: usize = 10_000;

pub const SPEED: f32 = 1.0;
pub const PHEROMONE_DECAY: f32 = 0.90;
pub const GRID_SCALE: usize = 1;

#[derive(Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    pub fn distance(&self, other: &Vec2) -> f32 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
    pub fn normalize(&self) -> Vec2 {
        let len = (self.x.powi(2) + self.y.powi(2)).sqrt();
        if len > 0.0 {
            Vec2::new(self.x / len, self.y / len)
        } else {
            Vec2::new(0.0, 0.0)
        }
    }
}

#[derive(Clone, Copy)]
pub struct Locust {
    pub pos: Vec2,
    pub vel: Vec2,
    pub state: u8, // 0: Normal, 1: Dead (leaves trace)
}

pub struct World {
    pub agents: Vec<Locust>,
    pub pheromones: Vec<f32>,
    pub firewalls: Vec<(Vec2, f32)>,
    pub grid_w: usize,
    pub grid_h: usize,
    pub target: Vec2,
    pub server_health: f32,
    pub max_health: f32,
}

impl World {
    pub fn new() -> Self {
        let mut rng = ::rand::thread_rng();
        let mut agents = Vec::with_capacity(AGENT_COUNT);

        let grid_w = 256;
        let grid_h = 128;

        for _ in 0..AGENT_COUNT {
            let side = rng.gen_range(0..4);
            let (x, y) = match side {
                0 => (rng.gen_range(0.0..grid_w as f32), 0.0),
                1 => (rng.gen_range(0.0..grid_w as f32), grid_h as f32),
                2 => (0.0, rng.gen_range(0.0..grid_h as f32)),
                3 => (grid_w as f32, rng.gen_range(0.0..grid_h as f32)),
                _ => (0.0, 0.0),
            };

            let target = Vec2::new(grid_w as f32 / 2.0, grid_h as f32 / 2.0);
            let vel = Vec2::new(target.x - x, target.y - y).normalize();

            agents.push(Locust {
                pos: Vec2::new(x, y),
                vel,
                state: 0,
            });
        }

        Self {
            agents,
            pheromones: vec![0.0; grid_w * grid_h],
            firewalls: vec![],
            grid_w,
            grid_h,
            target: Vec2::new(grid_w as f32 / 2.0, grid_h as f32 / 2.0),
            server_health: 1000.0,
            max_health: 1000.0,
        }
    }

    pub fn update(&mut self) {
        for p in self.pheromones.iter_mut() {
            *p *= PHEROMONE_DECAY;
        }

        let grid_w = self.grid_w;
        let grid_h = self.grid_h;
        let firewalls = &self.firewalls;
        let mut hits = 0.0;

        let mut agents_data = self.agents.clone();
        let target = self.target;
        let p_grid = &self.pheromones;

        agents_data.par_iter_mut().for_each(|agent| {
            if agent.state == 1 {
                let mut rng = ::rand::thread_rng();
                let side = rng.gen_range(0..4);
                let (x, y) = match side {
                    0 => (rng.gen_range(0.0..grid_w as f32), 0.0),
                    1 => (rng.gen_range(0.0..grid_w as f32), grid_h as f32),
                    2 => (0.0, rng.gen_range(0.0..grid_h as f32)),
                    3 => (grid_w as f32, rng.gen_range(0.0..grid_h as f32)),
                    _ => (0.0, 0.0),
                };
                agent.pos = Vec2::new(x, y);
                agent.vel = Vec2::new(target.x - x, target.y - y).normalize();
                agent.state = 0;
                return;
            }

            let mut hit_firewall = false;
            for (fw_pos, fw_rad) in firewalls {
                if agent.pos.distance(fw_pos) < *fw_rad {
                    hit_firewall = true;
                    break;
                }
            }

            if hit_firewall {
                agent.state = 1;
                return;
            }

            agent.pos.x += agent.vel.x * SPEED;
            agent.pos.y += agent.vel.y * SPEED;

            let gx = (agent.pos.x as usize / GRID_SCALE).clamp(0, grid_w - 1);
            let gy = (agent.pos.y as usize / GRID_SCALE).clamp(0, grid_h - 1);

            let danger = p_grid[gy * grid_w + gx];
            if danger > 0.1 {
                let mut rng = ::rand::thread_rng();
                let scatter_x = rng.gen_range(-1.0..1.0);
                let scatter_y = rng.gen_range(-1.0..1.0);
                agent.vel = Vec2::new(scatter_x, scatter_y).normalize();
            } else {
                let to_target = Vec2::new(target.x - agent.pos.x, target.y - agent.pos.y).normalize();
                agent.vel.x = agent.vel.x * 0.95 + to_target.x * 0.05;
                agent.vel.y = agent.vel.y * 0.95 + to_target.y * 0.05;
                agent.vel = agent.vel.normalize();
            }
        });

        let mut new_pheromones = vec![0.0; self.pheromones.len()];

        for agent in agents_data.iter_mut() {
            if agent.state == 1 {
                let gx = (agent.pos.x as usize / GRID_SCALE).clamp(0, grid_w - 1);
                let gy = (agent.pos.y as usize / GRID_SCALE).clamp(0, grid_h - 1);
                new_pheromones[gy * grid_w + gx] = 1.0;
            }

            if agent.pos.distance(&target) < 10.0 {
                hits += 1.0;
                agent.state = 1;
            }
        }

        self.agents = agents_data;
        for (i, p) in new_pheromones.iter().enumerate() {
            self.pheromones[i] = self.pheromones[i].max(*p);
        }

        self.server_health -= hits;
        if self.server_health < 0.0 {
            self.server_health = 0.0;
            self.max_health += 100.0;
            self.server_health = self.max_health;
            self.firewalls.clear();
        }
    }

    pub fn add_firewall(&mut self, pos: Vec2, radius: f32) {
        self.firewalls.push((pos, radius));
    }
}
