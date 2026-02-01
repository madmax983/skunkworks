use rand::prelude::*;

#[derive(Clone)]
pub struct Agent {
    pub x: f64,
    pub y: f64,
    pub angle: f64,
}

impl Agent {
    pub fn new(x: f64, y: f64, angle: f64) -> Self {
        Self { x, y, angle }
    }

    pub fn update(&mut self, grid: &mut Grid) {
        let sensor_angle = std::f64::consts::FRAC_PI_4;
        let turn_angle = std::f64::consts::FRAC_PI_8;
        let sensor_dist = 2.0; // Look ahead
        let speed = 1.0;

        let width = grid.width as f64;
        let height = grid.height as f64;

        // Helper to sample grid (handling bounds by wrapping or clamping - we'll clamp coords for reading)
        let sense = |g: &Grid, angle: f64| -> f32 {
            let sx = (self.x + angle.cos() * sensor_dist).clamp(0.0, width - 0.1);
            let sy = (self.y + angle.sin() * sensor_dist).clamp(0.0, height - 0.1);
            g.get(sx as usize, sy as usize)
        };

        // 1. Sense
        let v_c = sense(grid, self.angle);
        let v_l = sense(grid, self.angle - sensor_angle);
        let v_r = sense(grid, self.angle + sensor_angle);

        // 2. Turn
        let mut rng = rand::rng();

        if v_c > v_l && v_c > v_r {
            // Continue forward (maybe slight random wobble)
            if rng.random_bool(0.1) {
                self.angle += (rng.random::<f64>() - 0.5) * 0.1;
            }
        } else if v_c < v_l && v_c < v_r {
            // Rotate randomly
            if rng.random_bool(0.5) {
                self.angle += turn_angle;
            } else {
                self.angle -= turn_angle;
            }
        } else if v_l > v_r {
            self.angle -= turn_angle;
        } else if v_r > v_l {
            self.angle += turn_angle;
        }

        // 3. Move
        self.x += self.angle.cos() * speed;
        self.y += self.angle.sin() * speed;

        // 4. Bounds (Bounce)
        if self.x < 0.0 || self.x >= width || self.y < 0.0 || self.y >= height {
            self.angle = rng.random_range(0.0..std::f64::consts::TAU);
            self.x = self.x.clamp(0.0, width - 0.1);
            self.y = self.y.clamp(0.0, height - 0.1);
        }

        // 5. Deposit
        grid.deposit(self.x as usize, self.y as usize, 1.0);
    }
}

pub struct Station {
    pub x: usize,
    pub y: usize,
    pub intensity: f32,
}

pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<f32>,
    buffer: Vec<f32>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        let size = width * height;
        Self {
            width,
            height,
            cells: vec![0.0; size],
            buffer: vec![0.0; size],
        }
    }

    pub fn get(&self, x: usize, y: usize) -> f32 {
        if x >= self.width || y >= self.height {
            return 0.0;
        }
        self.cells[y * self.width + x]
    }

    pub fn deposit(&mut self, x: usize, y: usize, amount: f32) {
        if x >= self.width || y >= self.height {
            return;
        }
        let idx = y * self.width + x;
        self.cells[idx] = (self.cells[idx] + amount).min(10.0); // Cap at 10.0
    }

    pub fn diffuse_and_decay(&mut self, decay_rate: f32) {
        let w = self.width;
        let h = self.height;

        // 3x3 Box Blur
        for y in 0..h {
            for x in 0..w {
                let mut sum = 0.0;
                let mut count = 0.0;

                for dy in -1..=1 {
                    for dx in -1..=1 {
                        let ny = y as isize + dy;
                        let nx = x as isize + dx;

                        if ny >= 0 && ny < h as isize && nx >= 0 && nx < w as isize {
                            let idx = (ny as usize) * w + (nx as usize);
                            sum += self.cells[idx];
                            count += 1.0;
                        }
                    }
                }

                let avg = sum / count;
                self.buffer[y * w + x] = avg * decay_rate;
            }
        }

        std::mem::swap(&mut self.cells, &mut self.buffer);
    }
}

pub struct World {
    pub grid: Grid,
    pub agents: Vec<Agent>,
    pub stations: Vec<Station>,
}

impl World {
    pub fn new(width: usize, height: usize, num_agents: usize) -> Self {
        let mut rng = rand::rng();
        let agents = (0..num_agents)
            .map(|_| {
                Agent::new(
                    rng.random_range(0.0..width as f64),
                    rng.random_range(0.0..height as f64),
                    rng.random_range(0.0..std::f64::consts::TAU),
                )
            })
            .collect();

        Self {
            grid: Grid::new(width, height),
            agents,
            stations: Vec::new(),
        }
    }

    pub fn generate_city(&mut self) {
        let mut rng = rand::rng();
        self.stations.clear();
        let num_stations = rng.random_range(5..15);

        for _ in 0..num_stations {
            let x = rng.random_range(10..self.grid.width - 10);
            let y = rng.random_range(10..self.grid.height - 10);
            self.stations.push(Station {
                x,
                y,
                intensity: 5.0, // Strong attractor
            });
        }
    }

    pub fn tick(&mut self) {
        // 1. Agents Move & Deposit
        for agent in &mut self.agents {
            agent.update(&mut self.grid);
        }

        // 2. Stations Deposit
        for station in &self.stations {
            self.grid.deposit(station.x, station.y, station.intensity);
        }

        // 3. Diffuse & Decay
        self.grid.diffuse_and_decay(0.95);
    }
}
