use rand::Rng;
use rayon::prelude::*;
use std::f64::consts::PI;

#[derive(Debug, Clone)]
pub struct Agent {
    pub x: f64,
    pub y: f64,
    pub angle: f64,
    pub home_city: usize,
    pub target_city: usize,
    pub commuted_count: u32,
}

pub struct AgentUpdateResult {
    pub deposit_x: usize,
    pub deposit_y: usize,
    pub deposit_amount: f64,
}

impl Agent {
    pub fn new(x: f64, y: f64, angle: f64, home_city: usize, target_city: usize) -> Self {
        Self {
            x,
            y,
            angle,
            home_city,
            target_city,
            commuted_count: 0,
        }
    }

    pub fn sense(&self, world: &World, angle_offset: f64, sensor_dist: f64) -> f64 {
        let sensor_angle = self.angle + angle_offset;
        let sensor_x = self.x + sensor_angle.cos() * sensor_dist;
        let sensor_y = self.y + sensor_angle.sin() * sensor_dist;

        let w = world.width as f64;
        let h = world.height as f64;
        let sx = (sensor_x.rem_euclid(w)) as usize;
        let sy = (sensor_y.rem_euclid(h)) as usize;

        let trail_strength = world.get_trail(sx, sy);

        let (tx, ty) = world.cities[self.target_city];
        let dx = tx - sensor_x;
        let dy = ty - sensor_y;
        let dist_sq = dx * dx + dy * dy;
        let dist = dist_sq.sqrt().max(1.0);

        let gradient_strength = 2000.0 / dist;
        trail_strength + gradient_strength
    }

    pub fn update(&mut self, world: &World) -> AgentUpdateResult {
        let (tx, ty) = world.cities[self.target_city];
        let dx = tx - self.x;
        let dy = ty - self.y;
        let dist_sq = dx * dx + dy * dy;

        if dist_sq < 25.0 {
            std::mem::swap(&mut self.home_city, &mut self.target_city);
            self.angle += PI;
            self.commuted_count += 1;

            return AgentUpdateResult {
                deposit_x: self.x as usize,
                deposit_y: self.y as usize,
                deposit_amount: 0.0,
            };
        }

        let sensor_angle = PI / 4.0;
        let sensor_dist = 9.0;
        let turn_angle = PI / 8.0;
        let speed = 1.0;

        let left = self.sense(world, -sensor_angle, sensor_dist);
        let center = self.sense(world, 0.0, sensor_dist);
        let right = self.sense(world, sensor_angle, sensor_dist);

        let mut rng = rand::thread_rng();

        if center > left && center > right {
            // Keep going
        } else if center < left && center < right {
            if rng.gen_bool(0.5) {
                self.angle += turn_angle;
            } else {
                self.angle -= turn_angle;
            }
        } else if left > right {
            self.angle -= turn_angle;
        } else if right > left {
            self.angle += turn_angle;
        }

        self.x += self.angle.cos() * speed;
        self.y += self.angle.sin() * speed;

        self.x = self.x.rem_euclid(world.width as f64);
        self.y = self.y.rem_euclid(world.height as f64);

        AgentUpdateResult {
            deposit_x: self.x as usize,
            deposit_y: self.y as usize,
            deposit_amount: 5.0,
        }
    }
}

pub struct World {
    pub width: usize,
    pub height: usize,
    pub trails: Vec<f64>,
    pub next_trails: Vec<f64>,
    pub cities: Vec<(f64, f64)>,
    pub city_names: Vec<String>,
}

impl World {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            trails: vec![0.0; width * height],
            next_trails: vec![0.0; width * height],
            cities: Vec::new(),
            city_names: Vec::new(),
        }
    }

    pub fn with_git_cities_and_agents(
        width: usize,
        height: usize,
        cities: Vec<(String, (f64, f64))>, // Name, Position
        connections: Vec<(usize, usize)>,  // Origin city idx, target city idx based on commits
    ) -> (Self, Vec<Agent>) {
        let mut world = Self::new(width, height);

        for (name, pos) in cities {
            world.city_names.push(name);
            world.cities.push(pos);
        }

        let mut agents = Vec::new();
        let mut rng = rand::thread_rng();

        // For each connection (files changed in the same commit), spawn some agents
        // This visualizes the coupling between those files.
        for (src, dst) in connections {
            let (cx, cy) = world.cities[src];
            for _ in 0..100 {
                // Spawn 100 agents per connection to make it visible
                agents.push(Agent::new(cx, cy, rng.gen_range(0.0..2.0 * PI), src, dst));
            }
        }

        (world, agents)
    }

    pub fn get_trail(&self, x: usize, y: usize) -> f64 {
        if x >= self.width || y >= self.height {
            return 0.0;
        }
        self.trails[y * self.width + x]
    }

    pub fn set_trail(&mut self, x: usize, y: usize, value: f64) {
        if x < self.width && y < self.height {
            self.trails[y * self.width + x] = value;
        }
    }

    pub fn update_agents_parallel(&mut self, agents: &mut [Agent]) {
        let deposits: Vec<AgentUpdateResult> = agents
            .par_iter_mut()
            .map(|agent| agent.update(self))
            .collect();

        for deposit in deposits {
            if deposit.deposit_x < self.width && deposit.deposit_y < self.height {
                let idx = deposit.deposit_y * self.width + deposit.deposit_x;
                self.trails[idx] = (self.trails[idx] + deposit.deposit_amount).min(255.0);
            }
        }
    }

    pub fn diffuse_and_decay(&mut self) {
        let decay_factor = 0.9;
        let width = self.width;
        let height = self.height;

        let trails_ref = &self.trails;

        self.next_trails
            .par_chunks_mut(width)
            .enumerate()
            .for_each(|(y, row)| {
                for (x, pixel) in row.iter_mut().enumerate() {
                    let mut sum = 0.0;
                    for dy in -1..=1 {
                        for dx in -1..=1 {
                            let nx = (x as isize + dx).rem_euclid(width as isize) as usize;
                            let ny = (y as isize + dy).rem_euclid(height as isize) as usize;
                            sum += trails_ref[ny * width + nx];
                        }
                    }
                    let avg = sum / 9.0;
                    *pixel = avg * decay_factor;
                }
            });

        std::mem::swap(&mut self.trails, &mut self.next_trails);
    }
}
