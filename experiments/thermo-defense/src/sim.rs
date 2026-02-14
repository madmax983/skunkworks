use crate::grid::{Grid, WIDTH, HEIGHT, Material};
use crate::agent::{Agent, AgentType, GridAction};
use rand::Rng;
use rayon::prelude::*;

pub struct World {
    pub grid: Grid,
    pub agents: Vec<Agent>,
    pub step: u64,
}

impl World {
    pub fn new() -> Self {
        Self {
            grid: Grid::new(),
            agents: Vec::new(),
            step: 0,
        }
    }

    pub fn add_server(&mut self, x: usize, y: usize, w: usize, h: usize) {
        for dy in 0..h {
            for dx in 0..w {
                let nx = x + dx;
                let ny = y + dy;
                if nx < WIDTH && ny < HEIGHT {
                    let idx = Grid::get_index(nx, ny);
                    self.grid.cells[idx].material = Material::Server;
                }
            }
        }
    }

    pub fn spawn_agents(&mut self, count: usize, kind: AgentType) {
        let mut rng = rand::thread_rng();
        for _ in 0..count {
            let x = rng.gen_range(0.0..WIDTH as f32);
            let y = rng.gen_range(0.0..HEIGHT as f32);
            self.agents.push(Agent::new(x, y, kind));
        }
    }

    pub fn update(&mut self) {
        self.step += 1;

        // 1. Grid Diffusion (Parallel)
        self.grid.update_diffusion();

        // 2. Agent Updates (Parallel)
        let grid = &self.grid;
        let agents = &mut self.agents;

        let actions: Vec<_> = agents.par_iter_mut()
            .map(|agent| {
                let mut rng = rand::thread_rng();
                agent.update(grid, &mut rng)
            })
            .collect();

        // 3. Apply Actions (Sequential)
        for action in actions {
            if let Some(GridAction::UpdateCell { idx, heat_delta, defense_delta, attack_delta, new_material }) = action {
                 let cell = &mut self.grid.cells[idx];
                 cell.heat += heat_delta;
                 cell.pheromone_defense = (cell.pheromone_defense + defense_delta).min(100.0);
                 cell.pheromone_attack = (cell.pheromone_attack + attack_delta).min(100.0);
                 if let Some(m) = new_material {
                     cell.material = m;
                 }
            }
        }
    }
}
