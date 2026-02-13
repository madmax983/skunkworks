use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use ratatui::style::Color;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Block {
    pub owner: Option<usize>, // Agent ID
    pub value: f32,
    pub rent: f32,
}

#[derive(Debug, Clone)]
pub struct Agent {
    pub id: usize,
    pub wealth: f32,
    pub color: Color,
    pub desired_blocks: usize,
    pub owned_blocks: Vec<(usize, usize)>, // (x, y)
}

#[derive(Debug)]
pub struct Market {
    pub width: usize,
    pub height: usize,
    pub grid: Vec<Block>,
    pub agents: Vec<Agent>,
    pub tick: u64,
}

impl Market {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            grid: vec![
                Block {
                    owner: None,
                    value: 1.0,
                    rent: 1.0 // Start with decent rent
                };
                width * height
            ],
            agents: Vec::new(),
            tick: 0,
        }
    }

    pub fn update(&mut self) {
        let mut rng = thread_rng();

        // 0. Income
        for agent in self.agents.iter_mut() {
            agent.wealth += rng.gen_range(2.0..8.0);
        }

        // 1. Rent Collection
        for block_idx in 0..self.grid.len() {
            if let Some(owner_id) = self.grid[block_idx].owner {
                let rent = self.grid[block_idx].rent;
                // Find owner and deduct rent
                if let Some(agent) = self.agents.iter_mut().find(|a| a.id == owner_id) {
                    agent.wealth -= rent;
                }
            }
        }

        // 2. Eviction (Bankruptcy)
        // Identify bankrupt agents
        let bankrupt_ids: Vec<usize> = self
            .agents
            .iter()
            .filter(|a| a.wealth < 0.0)
            .map(|a| a.id)
            .collect();

        // Remove bankrupt agents entirely? Or just reset them?
        // Let's remove them to make room for new spawns in main loop.
        self.agents.retain(|a| a.wealth >= 0.0);

        // Clear blocks owned by evicted agents
        for block in self.grid.iter_mut() {
            if let Some(owner) = block.owner {
                if bankrupt_ids.contains(&owner) {
                    block.owner = None;
                }
            }
        }

        // 3. Price Dynamics & Diffusion
        let mut new_rents = vec![0.0; self.grid.len()];
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = y * self.width + x;
                let mut sum = 0.0;
                let mut count = 0.0;

                // Neighbors
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }
                        let nx = x as isize + dx;
                        let ny = y as isize + dy;
                        if nx >= 0
                            && nx < self.width as isize
                            && ny >= 0
                            && ny < self.height as isize
                        {
                            let n_idx = (ny as usize) * self.width + (nx as usize);
                            sum += self.grid[n_idx].rent;
                            count += 1.0;
                        }
                    }
                }

                let avg = if count > 0.0 {
                    sum / count
                } else {
                    self.grid[idx].rent
                };
                let diffusion = 0.1;
                new_rents[idx] = self.grid[idx].rent * (1.0 - diffusion) + avg * diffusion;
            }
        }

        // Apply
        for (i, block) in self.grid.iter_mut().enumerate() {
            block.rent = new_rents[i];
            if block.owner.is_some() {
                block.rent *= 1.05; // Inflation
            } else {
                block.rent *= 0.95; // Deflation
            }
            block.rent = block.rent.clamp(0.1, 200.0);
        }

        // 4. Bidding
        let mut agent_indices: Vec<usize> = (0..self.agents.len()).collect();
        agent_indices.shuffle(&mut rng);

        for &agent_idx in &agent_indices {
            let (wants_more, wealth, id) = {
                let a = &self.agents[agent_idx];
                (a.owned_blocks.len() < a.desired_blocks, a.wealth, a.id)
            };

            if wants_more && wealth > 0.0 {
                // Find cheapest empty block
                let mut best_block_idx = None;
                let mut min_rent = f32::MAX;

                for (i, block) in self.grid.iter().enumerate() {
                    if block.owner.is_none() && block.rent < min_rent && block.rent <= wealth {
                        min_rent = block.rent;
                        best_block_idx = Some(i);
                    }
                }

                if let Some(idx) = best_block_idx {
                    // Buy it (pay first rent/allocation fee)
                    let rent = self.grid[idx].rent;
                    self.grid[idx].owner = Some(id);

                    let x = idx % self.width;
                    let y = idx / self.width;

                    // Mutate agent
                    if let Some(agent) = self.agents.get_mut(agent_idx) {
                        agent.owned_blocks.push((x, y));
                        agent.wealth -= rent;
                    }
                }
            }
        }

        self.tick += 1;
    }
}
