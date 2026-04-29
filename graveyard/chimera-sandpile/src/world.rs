use crate::agent::Agent;
use crate::grid::SandGrid;
use rand::prelude::*;
use rayon::prelude::*;
use std::collections::HashSet;

pub struct World {
    pub grid: SandGrid,
    pub agents: Vec<Agent>,
    pub width: usize,
    pub height: usize,
}

impl World {
    pub fn new(width: usize, height: usize, agent_count: usize) -> Self {
        let mut grid = SandGrid::new(width, height);
        let mut rng = rand::thread_rng();

        // Initialize with some random sand
        for _ in 0..(width * height / 10) {
            grid.add_load(rng.gen_range(0..width), rng.gen_range(0..height), 4);
        }

        let mut agents = Vec::new();
        for i in 0..agent_count {
            agents.push(Agent::new(
                (rng.gen_range(0..width), rng.gen_range(0..height)),
                i as u64,
            ));
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

        // Phase 1: Agent Action
        // We split borrows to allow parallel iteration over agents while reading grid
        let grid = &mut self.grid;
        let agents = &mut self.agents;

        // Create an immutable view of the grid for the agents to read
        // Note: We can't easily share &mut grid across threads, so we treat it as read-only here
        // by just using the data. But Rayon closures need to capture it.
        // Since `grid` is &mut SandGrid, we can convert it to &SandGrid.
        let grid_view = &*grid;

        let updates: Vec<(usize, usize, i32)> = agents
            .par_iter_mut()
            .flat_map_iter(|agent| {
                let (ax, ay) = agent.pos;
                let start_x = ax.saturating_sub(8);
                let start_y = ay.saturating_sub(8);

                // Extract local view
                let mut local_view = [[0u32; 16]; 16];
                for y in 0..16 {
                    for x in 0..16 {
                        let gx = start_x + x;
                        let gy = start_y + y;
                        if gx < width && gy < height {
                            local_view[y][x] = grid_view.get_load(gx, gy);
                        }
                    }
                }

                agent.sync_input(&local_view);
                agent.step();
                let changes = agent.sync_output(&local_view);

                // Map local changes to global
                changes.into_iter().filter_map(move |(lx, ly, delta)| {
                    let gx = start_x + lx;
                    let gy = start_y + ly;
                    if gx < width && gy < height {
                        Some((gx, gy, delta))
                    } else {
                        None
                    }
                })
            })
            .collect();

        // Apply Agent Changes
        for (gx, gy, delta) in updates {
            if delta > 0 {
                grid.add_load(gx, gy, delta as u32);
            } else if delta < 0 {
                grid.reduce_load(gx, gy, (-delta) as u32);
            }
        }

        // Phase 2: Physics (Avalanche)
        let toppled_indices = grid.update();
        let toppled_set: HashSet<usize> = toppled_indices.into_iter().collect();

        // Phase 3: Transport (Surfing)
        let mut rng = rand::thread_rng();
        for agent in agents {
            let idx = grid.get_index(agent.pos.0, agent.pos.1);
            if toppled_set.contains(&idx) {
                // Move to random neighbor
                let (ax, ay) = agent.pos;
                let neighbors = [
                    (ax.saturating_sub(1), ay),
                    (ax + 1, ay),
                    (ax, ay.saturating_sub(1)),
                    (ax, ay + 1),
                ];
                if let Some(target) = neighbors.choose(&mut rng) {
                    if target.0 < width && target.1 < height {
                        agent.pos = *target;
                    }
                }
            }
        }
    }
}
