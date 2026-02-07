use macroquad::prelude::*;
use rayon::prelude::*;
use ::rand::Rng; // Import Rng trait for gen_range from crate rand

const SENSOR_ANGLE: f32 = std::f32::consts::PI / 4.0; // 45 degrees
const SENSOR_DIST: f32 = 9.0;
const ROTATION_ANGLE: f32 = std::f32::consts::PI / 8.0; // 22.5 degrees

#[derive(Clone, Copy, Debug)]
pub struct Agent {
    pub position: Vec2,
    pub angle: f32,
    pub speed: f32,
}

impl Agent {
    pub fn new(position: Vec2, angle: f32) -> Self {
        Self { position, angle, speed: 2.0 }
    }

    pub fn sense(&self, sensor_angle_offset: f32, width: usize, height: usize, grid: &[f32]) -> f32 {
        let sensor_angle = self.angle + sensor_angle_offset;
        let sensor_dir = vec2(sensor_angle.cos(), sensor_angle.sin());
        let sensor_pos = self.position + sensor_dir * SENSOR_DIST;

        let x = (sensor_pos.x.round() as isize).rem_euclid(width as isize) as usize;
        let y = (sensor_pos.y.round() as isize).rem_euclid(height as isize) as usize;

        grid[y * width + x]
    }

    pub fn update(&mut self, width: usize, height: usize, grid: &[f32]) {
        let sensor_left = self.sense(-SENSOR_ANGLE, width, height, grid);
        let sensor_center = self.sense(0.0, width, height, grid);
        let sensor_right = self.sense(SENSOR_ANGLE, width, height, grid);

        // Use thread-local RNG for thread safety in rayon loop
        let mut rng = ::rand::thread_rng();
        let random_steer: f32 = rng.gen_range(0.0..1.0);

        if sensor_center > sensor_left && sensor_center > sensor_right {
            // Keep direction
        } else if sensor_center < sensor_left && sensor_center < sensor_right {
            // Rotate random
            if random_steer < 0.5 {
                self.angle -= ROTATION_ANGLE;
            } else {
                self.angle += ROTATION_ANGLE;
            }
        } else if sensor_left < sensor_right {
            self.angle += ROTATION_ANGLE;
        } else if sensor_right < sensor_left {
            self.angle -= ROTATION_ANGLE;
        }

        let direction = vec2(self.angle.cos(), self.angle.sin());
        self.position += direction * self.speed;

        let w = width as f32;
        let h = height as f32;
        if self.position.x < 0.0 { self.position.x += w; }
        if self.position.x >= w { self.position.x -= w; }
        if self.position.y < 0.0 { self.position.y += h; }
        if self.position.y >= h { self.position.y -= h; }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct City {
    pub position: Vec2,
    pub radius: f32,
}

impl City {
    pub fn new(position: Vec2) -> Self {
        Self { position, radius: 3.0 }
    }
}

pub struct World {
    pub grid: Vec<f32>,
    pub agents: Vec<Agent>,
    pub cities: Vec<City>,
    pub width: usize,
    pub height: usize,
}

impl World {
    pub fn new(width: usize, height: usize, num_agents: usize) -> Self {
        let grid = vec![0.0; width * height];
        let agents = Vec::with_capacity(num_agents);
        let cities = Vec::new();

        Self {
            grid,
            agents,
            cities,
            width,
            height,
        }
    }

    pub fn update(&mut self) {
        let width = self.width;
        let height = self.height;
        let grid = &self.grid;

        // Update agents
        self.agents.par_iter_mut().for_each(|agent| {
            agent.update(width, height, grid);
        });

        // Deposit trails from agents
        for agent in &self.agents {
            let x = agent.position.x as usize;
            let y = agent.position.y as usize;
            if x < self.width && y < self.height {
                let idx = y * self.width + x;
                self.grid[idx] = (self.grid[idx] + 0.5).min(1.0);
            }
        }

        // Deposit trails from cities
        for city in &self.cities {
            let cx = city.position.x as isize;
            let cy = city.position.y as isize;
            let r = city.radius as isize;

            for dy in -r..=r {
                for dx in -r..=r {
                    if dx*dx + dy*dy <= r*r {
                        let nx = (cx + dx).rem_euclid(width as isize) as usize;
                        let ny = (cy + dy).rem_euclid(height as isize) as usize;
                        self.grid[ny * width + nx] = 1.0;
                    }
                }
            }
        }

        // Diffuse and Decay
        let mut next_grid = self.grid.clone();
        let w = self.width;
        let h = self.height;
        let grid = &self.grid;

        next_grid.par_chunks_mut(w).enumerate().for_each(|(y, row)| {
            for x in 0..w {
                let mut sum = 0.0;
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        let nx = (x as isize + dx).rem_euclid(w as isize) as usize;
                        let ny = (y as isize + dy).rem_euclid(h as isize) as usize;
                        sum += grid[ny * w + nx];
                    }
                }
                let avg = sum / 9.0;
                row[x] = avg * 0.95;
            }
        });

        self.grid = next_grid;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_moves_forward() {
        let width = 100;
        let height = 100;
        let mut world = World::new(width, height, 1);
        let start_pos = vec2(50.0, 50.0);
        let angle = 0.0;
        let agent = Agent::new(start_pos, angle);
        world.agents.push(agent);

        world.update();

        let new_pos = world.agents[0].position;
        assert_ne!(new_pos, start_pos, "Agent should have moved");
        let new_idx = new_pos.y as usize * width + new_pos.x as usize;
        assert!(world.grid[new_idx] > 0.0, "Trail should be deposited");
    }
}
