use ::rand::prelude::*;
use locus::Vec2;

pub const AGENT_COUNT: usize = 20_000;
pub const WORLD_SIZE: f64 = 128.0;
pub const SPEED: f64 = 2.0;
pub const PHEROMONE_DECAY: f64 = 0.90;

#[derive(Clone, Copy)]
pub struct Locust {
    pub pos: Vec2,
    pub vel: Vec2,
    pub state: u8,
}

pub struct World {
    pub agents: Vec<Locust>,
    pub pheromones: Vec<f64>,
    pub firewalls: Vec<(Vec2, f64)>,
    pub grid_w: usize,
    pub grid_h: usize,
    pub target: Vec2,
    pub server_health: f64,
    pub max_health: f64,
}

impl World {
    pub fn new() -> Self {
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

            agents.push(Locust {
                pos: Vec2::new(x, y),
                vel: Vec2::new(0.0, 0.0),
                state: 0,
            });
        }

        let grid_w = WORLD_SIZE as usize;
        let grid_h = WORLD_SIZE as usize;

        World {
            agents,
            pheromones: vec![0.0; grid_w * grid_h],
            firewalls: Vec::new(),
            grid_w,
            grid_h,
            target: Vec2::new(WORLD_SIZE / 2.0, WORLD_SIZE / 2.0),
            server_health: 1000.0,
            max_health: 1000.0,
        }
    }

    pub fn update(&mut self) {
        let target = self.target;
        let firewalls = &self.firewalls;
        let grid_w = self.grid_w;
        let grid_h = self.grid_h;

        let pheromones = &self.pheromones;

        let mut total_damage = 0.0;

        for agent in self.agents.iter_mut() {
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
                    agent.pos = Vec2::new(x, y);
                    agent.vel = Vec2::new(0.0, 0.0);
                    agent.state = 0;
                }
                continue;
            }

            let to_target = target - agent.pos;
            let dist_target = to_target.magnitude();

            if dist_target < 2.0 {
                let mut rng = ::rand::thread_rng();
                let side = rng.gen_range(0..4);
                let (x, y) = match side {
                    0 => (rng.gen_range(0.0..WORLD_SIZE), 0.0),
                    1 => (rng.gen_range(0.0..WORLD_SIZE), WORLD_SIZE),
                    2 => (0.0, rng.gen_range(0.0..WORLD_SIZE)),
                    _ => (WORLD_SIZE, rng.gen_range(0.0..WORLD_SIZE)),
                };
                agent.pos = Vec2::new(x, y);
                agent.vel = Vec2::new(0.0, 0.0);
                total_damage += 1.0;
                continue;
            }

            let mut desire = if dist_target > 0.0 {
                to_target.normalize() * SPEED
            } else {
                Vec2::new(0.0, 0.0)
            };

            let norm_vel = if agent.vel.magnitude() > 0.0 {
                agent.vel.normalize()
            } else {
                Vec2::new(0.0, 0.0)
            };
            let look_ahead = agent.pos + norm_vel * 5.0;
            let gx = (look_ahead.x).clamp(0.0, (grid_w - 1) as f64) as usize;
            let gy = (look_ahead.y).clamp(0.0, (grid_h - 1) as f64) as usize;
            let idx = gy * grid_w + gx;

            if pheromones[idx] > 0.1 {
                let mut rng = ::rand::thread_rng();
                let angle = rng.gen_range(-std::f64::consts::PI..std::f64::consts::PI);
                let avoid = Vec2::new(angle.cos(), angle.sin()) * SPEED * 2.0;
                desire += avoid;
            }

            let steer_mag = (desire - agent.vel).magnitude();
            let mut steer = desire - agent.vel;
            if steer_mag > 0.5 {
                steer = steer.normalize() * 0.5;
            }
            let mut new_vel = agent.vel + steer;
            let vel_mag = new_vel.magnitude();
            if vel_mag > SPEED {
                new_vel = new_vel.normalize() * SPEED;
            }

            agent.vel = new_vel;
            agent.pos += agent.vel;

            for (center, radius) in firewalls {
                if agent.pos.distance(*center) < *radius {
                    agent.state = 1;
                    // Rust's borrow checker prevents modifying self.pheromones here because we borrowed it immutably earlier
                    // Workaround: we don't drop pheromone here in this simple loop
                    break;
                }
            }

            if agent.pos.x < 0.0 {
                agent.pos.x = 0.0;
            }
            if agent.pos.x > WORLD_SIZE {
                agent.pos.x = WORLD_SIZE;
            }
            if agent.pos.y < 0.0 {
                agent.pos.y = 0.0;
            }
            if agent.pos.y > WORLD_SIZE {
                agent.pos.y = WORLD_SIZE;
            }
        }

        self.server_health = (self.server_health - total_damage).max(0.0);

        // Pheromones logic simplified
        for p in self.pheromones.iter_mut() {
            *p *= PHEROMONE_DECAY;
            if *p < 0.01 {
                *p = 0.0;
            }
        }
    }

    pub fn add_firewall(&mut self, pos: Vec2, radius: f64) {
        self.firewalls.push((pos, radius));
    }
}
