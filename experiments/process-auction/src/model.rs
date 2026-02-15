use market_sim::{Grid, Particle};
use rand::prelude::*;
use rayon::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProcessState {
    Ready,
    Running(usize), // Running on Core ID
    Blocked,        // Out of wealth or waiting
    Finished,
}

#[derive(Debug, Clone)]
pub struct Process {
    pub id: usize,
    pub wealth: f32,
    pub work_total: u32,
    pub work_done: u32,
    pub state: ProcessState,
    pub color: (u8, u8, u8),
    // Strategy: (Bid Aggressiveness, Patience)
    pub aggressiveness: f32,
}

impl Process {
    pub fn new(id: usize) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            id,
            wealth: rng.gen_range(50.0..500.0),
            work_total: rng.gen_range(100..1000),
            work_done: 0,
            state: ProcessState::Ready,
            color: (rng.gen(), rng.gen(), rng.gen()),
            aggressiveness: rng.gen_range(0.1..1.0),
        }
    }

    pub fn decide_bid(&self, market_height: usize) -> Option<usize> {
        if self.wealth <= 0.0 || self.state == ProcessState::Finished {
            return None;
        }

        // Simple strategy: Bid proportional to wealth and progress
        let urgency = (self.work_done as f32 / self.work_total as f32) + 0.1;
        let bid_price = self.wealth * self.aggressiveness * urgency * 0.1;

        // Map price to Height (0 = High Price, H-1 = Low Price)
        // market-sim: Y=0 is High.
        // If bid_price is high, y should be small.
        // Let's say Max Price is 100.
        let max_price = 50.0;
        let normalized_bid = (bid_price / max_price).clamp(0.0, 1.0);

        // Invert for Y: Higher bid -> Lower Y
        let y = ((1.0 - normalized_bid) * (market_height as f32 - 1.0)) as usize;
        Some(y)
    }
}

#[derive(Debug, Clone)]
pub struct CpuCore {
    pub id: usize,
    pub current_process: Option<usize>, // PID
    pub cycles_remaining: u32,
    pub total_executed: u32,
}

impl CpuCore {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            current_process: None,
            cycles_remaining: 0,
            total_executed: 0,
        }
    }
}

pub struct Simulation {
    pub market: Grid,
    pub processes: Vec<Process>,
    pub cores: Vec<CpuCore>,
    pub frame_count: u64,
}

impl Simulation {
    pub fn new(num_cores: usize, num_processes: usize) -> Self {
        // Width = Cores (channels), Height = 100 (Price granularity)
        let market = Grid::new(num_cores, 50);
        let processes = (0..num_processes).map(Process::new).collect();
        let cores = (0..num_cores).map(CpuCore::new).collect();

        Self {
            market,
            processes,
            cores,
            frame_count: 0,
        }
    }

    pub fn update(&mut self) {
        self.frame_count += 1;
        let mut rng = rand::thread_rng();

        // 1. Processes income (UBI)
        for p in &mut self.processes {
            if p.state != ProcessState::Finished {
                p.wealth += 0.5; // Basic income

                // Block if poor, unblock if rich enough
                match p.state {
                    ProcessState::Blocked => {
                        if p.wealth > 5.0 {
                            p.state = ProcessState::Ready;
                        }
                    }
                    ProcessState::Ready => {
                        if p.wealth < 1.0 {
                            p.state = ProcessState::Blocked;
                        }
                    }
                    _ => {}
                }
            }
        }

        // 2. Cores work
        for core in &mut self.cores {
            if let Some(pid) = core.current_process {
                if core.cycles_remaining > 0 {
                    core.cycles_remaining -= 1;
                    core.total_executed += 1;
                    // Update process progress
                    // We assume PID is valid.
                    if pid < self.processes.len() {
                        let p = &mut self.processes[pid];
                        p.work_done += 1;
                        if p.work_done >= p.work_total {
                            p.state = ProcessState::Finished;
                            // We need to clear core logic, but we can't mutate core here because we are iterating cores?
                            // Wait, `for core in &mut self.cores` allows modifying core.
                            // But we also modify `self.processes`.
                            // This is fine because `self.processes` is disjoint from `core` (element of `self.cores`).
                            // Rust allows disjoint borrows of fields of `self`, but here we have `&mut self.cores` and `&mut self.processes`.
                            // Wait. `update` takes `&mut self`.
                            // `self.cores` iter takes `&mut cores`.
                            // `self.processes` access takes `&mut processes`.
                            // These are disjoint fields, so it SHOULD be fine IF we split borrow `self`.
                            // But inside the loop, we are accessing `self.processes`.
                            // The compiler might complain if I use `self.processes` while iterating `self.cores` if I didn't split first.
                        }
                    }
                    if self.processes[pid].state == ProcessState::Finished {
                         core.current_process = None;
                    }
                } else {
                    // Time slice expired
                    if pid < self.processes.len() {
                        let p = &mut self.processes[pid];
                        if p.state != ProcessState::Finished {
                            p.state = ProcessState::Ready;
                        }
                    }
                    core.current_process = None;
                }
            }
        }

        // 3. Place Bids (Parallelized decision, Sequential placement)
        // We can't mutate grid in parallel easily, so we collect bids first.
        let bids: Vec<(usize, usize)> = self.processes.par_iter()
            .filter(|p| p.state == ProcessState::Ready)
            .filter_map(|p| {
                // Stochastic bidding: don't bid every frame
                if rand::thread_rng().gen_bool(0.1) {
                     p.decide_bid(50).map(|y| (p.id, y))
                } else {
                    None
                }
            })
            .collect();

        for (pid, y) in bids {
            // Pick a random column (Core affinity or random)
            let x = rng.gen_range(0..self.market.width);
            // Only place if empty (simple collision avoidance)
            if let Particle::Empty = self.market.get(x, y) {
                self.market.set(x, y, Particle::Bid(pid));
            }
        }

        // 4. Cores place Asks (Selling time slices)
        for core in &self.cores {
            if core.current_process.is_none() {
                // If idle, offer services
                // Price drops as it stays idle (Y increases)
                // Let's just place at random "Ask" heights to simulate fluctuating supply curves
                if rng.gen_bool(0.2) {
                    let y = rng.gen_range(0..25); // Top half (High prices)
                    let x = core.id; // Place in own column
                     if let Particle::Empty = self.market.get(x, y) {
                        self.market.set(x, y, Particle::Ask(core.id));
                    }
                }
            }
        }

        // 5. Run Market
        let trades = self.market.update();

        // 6. Execute Trades
        for trade in trades {
            // Buyer = PID, Seller = Core ID
            let pid = trade.buyer;
            let core_id = trade.seller;
            let price = trade.price;

            // 1. Deduct Wealth from winner
            // We assume PID is valid index because we created them.
            if pid < self.processes.len() {
                self.processes[pid].wealth -= price;
                if self.processes[pid].wealth < 0.0 {
                    self.processes[pid].wealth = 0.0;
                }
            }

            // 2. Handle Core assignment and Preemption
            if let Some(core) = self.cores.iter_mut().find(|c| c.id == core_id) {
                // If core is busy, we preempt! (Ruthless market)
                if let Some(old_pid) = core.current_process {
                    if old_pid != pid && old_pid < self.processes.len() {
                        // We must set old process to Ready.
                        // But we cannot borrow processes here if we are going to borrow it later for winner?
                        // Actually, we are holding mutable ref to `core`.
                        // We need to mutate `self.processes`.
                        // We are not holding `self.processes` ref yet inside this block.
                        self.processes[old_pid].state = ProcessState::Ready;
                    }
                }

                core.current_process = Some(pid);
                core.cycles_remaining = 20; // 20 Ticks purchased
            }

            // 3. Update Winner State
            if pid < self.processes.len() {
                self.processes[pid].state = ProcessState::Running(core_id);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simulation_step() {
        let mut sim = Simulation::new(4, 10);
        sim.update();
        assert_eq!(sim.frame_count, 1);

        // Check that processes exist
        assert_eq!(sim.processes.len(), 10);
        assert_eq!(sim.cores.len(), 4);
    }

    #[test]
    fn test_process_bid() {
        let p = Process::new(0);
        let bid = p.decide_bid(50);
        assert!(bid.is_some());
    }
}
