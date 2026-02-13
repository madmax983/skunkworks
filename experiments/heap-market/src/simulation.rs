use rand::Rng;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Page {
    pub owner: Option<usize>, // Agent ID
    pub rent: f32,
    pub heat: f32, // Demand indicator
}

impl Page {
    pub fn new(base_rent: f32) -> Self {
        Self {
            owner: None,
            rent: base_rent,
            heat: 0.0,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Agent {
    #[allow(dead_code)]
    pub id: usize,
    pub name: String,
    pub balance: f32,
    pub size: usize,
    pub income_rate: f32,
    pub color_idx: u8,
}

pub struct Heap {
    pub pages: Vec<Page>,
    pub agents: HashMap<usize, Agent>, // Map ID -> Agent
    pub width: usize,
    pub height: usize,
    pub next_agent_id: usize,
    pub oom_count: usize,
    pub tick_count: usize,
}

impl Heap {
    pub fn new(width: usize, height: usize) -> Self {
        let size = width * height;
        let pages = (0..size).map(|_| Page::new(1.0)).collect();
        Self {
            pages,
            agents: HashMap::new(),
            width,
            height,
            next_agent_id: 1,
            oom_count: 0,
            tick_count: 0,
        }
    }

    pub fn tick(&mut self) {
        self.tick_count += 1;
        let mut rng = rand::thread_rng();

        // 1. Update Pages (Heat & Rent)
        for page in &mut self.pages {
            if page.owner.is_some() {
                // Occupied pages heat up
                page.heat = (page.heat + 0.05).min(5.0);
            } else {
                // Empty pages cool down
                page.heat = (page.heat - 0.02).max(0.0);
            }
            // Rent dynamic: Base (1.0) + Heat Factor
            // If heat is high, rent explodes.
            page.rent = 1.0 + (page.heat * page.heat);
        }

        // 2. Process Agents (Income & Rent Payment)
        let mut bankrupt_agents = Vec::new();

        // We need to calculate total rent for each agent first
        let mut agent_bills: HashMap<usize, f32> = HashMap::new();

        for page in &self.pages {
            if let Some(owner_id) = page.owner {
                *agent_bills.entry(owner_id).or_insert(0.0) += page.rent;
            }
        }

        for (id, agent) in &mut self.agents {
            // Income
            agent.balance += agent.income_rate;

            // Expense
            let bill = agent_bills.get(id).unwrap_or(&0.0);
            agent.balance -= bill;

            if agent.balance < 0.0 {
                bankrupt_agents.push(*id);
            }
        }

        // 3. OOM Kill (Eviction)
        for id in bankrupt_agents {
            self.kill_agent(id);
            self.oom_count += 1;
        }

        // 4. Spawn New Agents
        if rng.gen_bool(0.15) {
            // 15% chance per tick
            self.spawn_agent();
        }
    }

    fn kill_agent(&mut self, id: usize) {
        // Remove from map
        self.agents.remove(&id);
        // Clear pages
        for page in &mut self.pages {
            if page.owner == Some(id) {
                page.owner = None;
                // Instant cooling effect on free?
                // No, let it cool naturally.
            }
        }
    }

    pub fn spawn_agent(&mut self) {
        let mut rng = rand::thread_rng();

        // Agent Properties
        let size = rng.gen_range(2..=20); // Need 2 to 20 pages
        let initial_balance = rng.gen_range(50.0..500.0);
        let income_rate = rng.gen_range(1.0..10.0) + (size as f32 * 0.5); // Bigger agents earn more?
        let id = self.next_agent_id;
        self.next_agent_id += 1;

        let agent = Agent {
            id,
            name: format!("Proc_{:X}", id),
            balance: initial_balance,
            size,
            income_rate,
            color_idx: rng.r#gen(),
        };

        // Try to allocate
        if let Some(start_idx) = self.find_contiguous_space(size, agent.balance) {
            // Success
            for i in 0..size {
                self.pages[start_idx + i].owner = Some(id);
            }
            self.agents.insert(id, agent);
        } else {
            // Failed to allocate (OOM on spawn?)
            // Just don't spawn.
        }
    }

    // Find a contiguous block of free pages
    // Optional strategy: Find CHEAPEST block or FIRST block.
    // Let's go with First Fit that is Affordable.
    fn find_contiguous_space(&self, size: usize, budget: f32) -> Option<usize> {
        let mut current_run = 0;
        let mut run_start = 0;

        // Naive 1D search.
        // Improve: The heap is 2D conceptually (width*height), but we treat it as 1D linear memory for allocation
        // because that's how RAM works mostly.

        for (i, page) in self.pages.iter().enumerate() {
            if page.owner.is_none() {
                if current_run == 0 {
                    run_start = i;
                }
                current_run += 1;

                if current_run == size {
                    // Check affordability (Rent for 1 tick vs Balance?)
                    // Agents usually want to survive at least a few ticks.
                    // Let's say they need to afford at least 5 ticks of rent.
                    let estimated_cost: f32 =
                        self.pages[run_start..=i].iter().map(|p| p.rent).sum();
                    if budget > (estimated_cost * 2.0) {
                        return Some(run_start);
                    }
                    // If too expensive, we keep searching?
                    // Actually, if we reset `current_run`, we might miss a sub-block.
                    // But for now, let's just reset to keep it simple.
                    // Ideally we slide the window.
                    current_run = 0; // Reset. This is inefficient but simple.
                }
            } else {
                current_run = 0;
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heap_tick_increases_rent() {
        let mut heap = Heap::new(10, 10);

        // Spawn an agent manually
        let agent = Agent {
            id: 1,
            name: "Test".to_string(),
            balance: 1000.0,
            size: 5,
            income_rate: 10.0,
            color_idx: 0,
        };
        heap.agents.insert(1, agent);
        for i in 0..5 {
            heap.pages[i].owner = Some(1);
        }

        let initial_rent = heap.pages[0].rent;
        heap.tick();
        assert!(
            heap.pages[0].rent > initial_rent,
            "Rent should increase for occupied pages"
        );
    }
}
