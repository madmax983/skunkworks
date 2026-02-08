use crate::agent::Agent;
use macroquad::prelude::*;
use rayon::prelude::*;
use ::rand::Rng;

pub struct World {
    pub grid: Vec<f32>,
    pub agents: Vec<Agent>,
    pub width: usize,
    pub height: usize,
}

impl World {
    pub fn new(width: usize, height: usize, num_agents: usize) -> Self {
        let grid = vec![0.0; width * height];
        let mut agents = Vec::with_capacity(num_agents);

        let mut rng = ::rand::thread_rng();
        for _ in 0..num_agents {
            let mut pos;
            loop {
                let x = rng.gen_range(-0.9..0.9);
                let y = rng.gen_range(-0.9..0.9);
                pos = vec2(x, y);
                if pos.length_squared() < 0.81 {
                    break;
                }
            }
            let angle = rng.gen_range(0.0..std::f32::consts::PI * 2.0);
            let mut agent = Agent::new(pos, angle);
            // Randomize speed slightly
            agent.base_speed = rng.gen_range(0.003..0.007);
            agents.push(agent);
        }

        Self {
            grid,
            agents,
            width,
            height,
        }
    }

    pub fn update(&mut self) {
        let width = self.width;
        let height = self.height;
        let grid = &self.grid;

        // Parallel Agent Update
        self.agents.par_iter_mut().for_each(|agent| {
            // Need a slice reference that is safe?
            // Agent update only READS grid.
            agent.update(width, height, grid);
        });

        // Deposit Trails
        // Since we can't write to grid in parallel easily without atomics or collecting,
        // we'll do it sequentially here. It's fast enough for 5000 agents.
        // Or use an intermediate buffer if needed.
        for agent in &self.agents {
            let x_norm = (agent.position.x + 1.0) / 2.0;
            let y_norm = (agent.position.y + 1.0) / 2.0;

            if x_norm >= 0.0 && x_norm < 1.0 && y_norm >= 0.0 && y_norm < 1.0 {
                let x = (x_norm * width as f32) as usize;
                let y = (y_norm * height as f32) as usize;

                if x < width && y < height {
                     let idx = y * width + x;
                     self.grid[idx] = (self.grid[idx] + 0.5).min(1.0);
                }
            }
        }

        // Diffuse and Decay
        let mut next_grid = self.grid.clone();
        let w = self.width;
        let h = self.height;
        let grid_ref = &self.grid; // Immutable borrow for reading

        next_grid.par_chunks_mut(w).enumerate().for_each(|(y, row)| {
            for x in 0..w {
                // Map pixel to [-1, 1]
                let wx = (x as f32 / w as f32) * 2.0 - 1.0;
                let wy = (y as f32 / h as f32) * 2.0 - 1.0;

                // Hard clip at r=1.0 to ensure circular boundary visual
                if wx*wx + wy*wy > 1.0 {
                    row[x] = 0.0;
                    continue;
                }

                let mut sum = 0.0;
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        let nx = x as isize + dx;
                        let ny = y as isize + dy;

                        if nx >= 0 && nx < w as isize && ny >= 0 && ny < h as isize {
                             sum += grid_ref[ny as usize * w + nx as usize];
                        }
                    }
                }
                let avg = sum / 9.0;
                row[x] = avg * 0.98; // Decay
            }
        });

        self.grid = next_grid;
    }
}
