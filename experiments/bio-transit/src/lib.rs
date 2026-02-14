use macroquad::prelude::*;
use rayon::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum AgentState {
    CommutingToWork,
    CommutingHome,
}

#[derive(Clone, Copy)]
pub struct Agent {
    pub pos: Vec2,
    pub angle: f32,
    pub home: Vec2,
    pub work: Vec2,
    pub state: AgentState,
    pub species_mask: u32,
}

impl Agent {
    pub fn new(pos: Vec2, angle: f32, home: Vec2, work: Vec2) -> Self {
        Self {
            pos,
            angle,
            home,
            work,
            state: AgentState::CommutingToWork,
            species_mask: 1,
        }
    }

    pub fn update(&mut self, map: &TrailMap, settings: &Settings) {
        let sensor_angle = settings.sensor_angle;
        let sensor_dist = settings.sensor_dist;
        let turn_angle = settings.turn_angle;

        // Sensing (Physarum Logic)
        let sense = |angle_offset: f32| -> f32 {
            let angle = self.angle + angle_offset;
            let dir = Vec2::new(angle.cos(), angle.sin());
            let sensor_pos = self.pos + dir * sensor_dist;

            // Wrap coordinates
            let w = map.width as f32;
            let h = map.height as f32;
            let x = (sensor_pos.x.rem_euclid(w)) as usize;
            let y = (sensor_pos.y.rem_euclid(h)) as usize;

            // Safe access
            if x < map.width && y < map.height {
                map.grid[y * map.width + x]
            } else {
                0.0
            }
        };

        let left = sense(-sensor_angle);
        let center = sense(0.0);
        let right = sense(sensor_angle);

        // Turn based on sensors
        let mut turned = false;
        if center > left && center > right {
            // Stay straight
        } else if center < left && center < right {
            // Randomly turn left or right
            let mut rng = ::rand::thread_rng();
            use ::rand::Rng;
            if rng.gen_bool(0.5) {
                self.angle += turn_angle;
            } else {
                self.angle -= turn_angle;
            }
            turned = true;
        } else if left > right {
            self.angle -= turn_angle;
            turned = true;
        } else if right > left {
            self.angle += turn_angle;
            turned = true;
        }

        // Target Bias (Commuter Logic)
        let target = match self.state {
            AgentState::CommutingToWork => self.work,
            AgentState::CommutingHome => self.home,
        };

        // If close to target, switch state
        if self.pos.distance(target) < 10.0 {
            self.state = match self.state {
                AgentState::CommutingToWork => AgentState::CommutingHome,
                AgentState::CommutingHome => AgentState::CommutingToWork,
            };
            // Turn around 180 degrees roughly
            self.angle += std::f32::consts::PI;
        }

        // Apply bias towards target
        // If we just turned based on trail, apply less bias to respect the trail
        let bias_strength = if turned { 0.02 } else { 0.05 };

        let to_target = target - self.pos;
        let target_angle = to_target.y.atan2(to_target.x);
        let angle_diff = (target_angle - self.angle + std::f32::consts::PI).rem_euclid(std::f32::consts::TAU) - std::f32::consts::PI;

        // Nudge
        self.angle += angle_diff * bias_strength;

        // Move
        let dir = Vec2::new(self.angle.cos(), self.angle.sin());
        self.pos += dir * settings.move_speed;

        // Wrap Position
        self.pos.x = self.pos.x.rem_euclid(map.width as f32);
        self.pos.y = self.pos.y.rem_euclid(map.height as f32);
    }
}

pub struct TrailMap {
    pub width: usize,
    pub height: usize,
    pub grid: Vec<f32>,
}

impl TrailMap {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            grid: vec![0.0; width * height],
        }
    }

    pub fn deposit(&mut self, x: usize, y: usize, amount: f32) {
        let idx = y * self.width + x;
        if idx < self.grid.len() {
            self.grid[idx] = (self.grid[idx] + amount).min(1.0);
        }
    }

    pub fn diffuse_and_decay(&mut self, settings: &Settings) {
        let mut next_grid = vec![0.0; self.grid.len()];
        let w = self.width;
        let h = self.height;
        let decay = settings.decay_rate;

        // Parallel update using Rayon
        next_grid.par_iter_mut().enumerate().for_each(|(i, cell)| {
            let x = i % w;
            let y = i / w;

            // 3x3 Box Blur
            let mut sum = 0.0;
            for dy in -1..=1 {
                for dx in -1..=1 {
                    let nx = (x as isize + dx).rem_euclid(w as isize) as usize;
                    let ny = (y as isize + dy).rem_euclid(h as isize) as usize;
                    // Use get_unchecked for speed, safe because of rem_euclid
                    sum += unsafe { *self.grid.get_unchecked(ny * w + nx) };
                }
            }

            *cell = (sum / 9.0) * decay;
        });

        self.grid = next_grid;
    }
}

pub struct City {
    pub pos: Vec2,
    pub radius: f32,
    pub color: Color,
}

pub struct Settings {
    pub sensor_angle: f32,
    pub sensor_dist: f32,
    pub turn_angle: f32,
    pub move_speed: f32,
    pub decay_rate: f32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            sensor_angle: std::f32::consts::PI / 4.0,
            sensor_dist: 9.0,
            turn_angle: std::f32::consts::PI / 4.0,
            move_speed: 1.0,
            decay_rate: 0.9,
        }
    }
}

pub struct Simulation {
    pub agents: Vec<Agent>,
    pub map: TrailMap,
    pub cities: Vec<City>,
    pub settings: Settings,
}

impl Simulation {
    pub fn new(width: usize, height: usize, num_agents: usize) -> Self {
        let map = TrailMap::new(width, height);
        let mut agents = Vec::with_capacity(num_agents);
        let mut cities = Vec::new();

        use ::rand::Rng;
        let mut rng = ::rand::thread_rng();

        let w = width as f32;
        let h = height as f32;
        // Ensure bounds are safe
        let padding = 50.0f32.min(w * 0.4).min(h * 0.4);

        // Spawn Cities
        let num_cities = 5;
        for _ in 0..num_cities {
            cities.push(City {
                pos: vec2(
                    rng.gen_range(padding..(w - padding)),
                    rng.gen_range(padding..(h - padding)),
                ),
                radius: rng.gen_range(10.0..30.0),
                color: Color::new(rng.r#gen(), rng.r#gen(), rng.r#gen(), 1.0),
            });
        }

        // Spawn Agents
        for _ in 0..num_agents {
            // Pick Home and Work
            let home_idx = rng.gen_range(0..cities.len());
            let mut work_idx = rng.gen_range(0..cities.len());
            while work_idx == home_idx {
                work_idx = rng.gen_range(0..cities.len());
            }

            let home = cities[home_idx].pos;
            let work = cities[work_idx].pos;

            // Start at home
            // Add some jitter
            let start_pos = home + vec2(rng.gen_range(-10.0..10.0), rng.gen_range(-10.0..10.0));
            let angle: f32 = rng.gen_range(0.0..std::f32::consts::TAU);

            agents.push(Agent::new(start_pos, angle, home, work));
        }

        Self {
            agents,
            map,
            cities,
            settings: Settings::default(),
        }
    }

    pub fn step(&mut self) {
        // Update agents in parallel
        let map = &self.map;
        let settings = &self.settings;
        self.agents.par_iter_mut().for_each(|agent| {
            agent.update(map, settings);
        });

        // Deposit pheromones (Serial for now)
        let w = self.map.width;
        let h = self.map.height;
        for agent in &self.agents {
            let x = agent.pos.x as usize;
            let y = agent.pos.y as usize;
            if x < w && y < h {
                self.map.deposit(x, y, 1.0);
            }
        }

        // Diffuse
        self.map.diffuse_and_decay(&self.settings);
    }
}
