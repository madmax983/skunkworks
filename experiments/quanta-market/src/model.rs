use rand::prelude::*;
use ratatui::style::Color;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProcessState {
    Running,
    Waiting,
    Zombie,
    Dead,
}

#[derive(Debug, Clone)]
pub struct Process {
    pub id: u32,
    pub wealth: f64,
    pub deadline: u64, // Absolute tick
    pub urgency: f64,
    pub state: ProcessState,
    pub color: Color,
    pub time_ran: u64,
}

impl Process {
    pub fn new(id: u32, deadline: u64, color: Color) -> Self {
        Self {
            id,
            wealth: 100.0,
            deadline,
            urgency: 0.0,
            state: ProcessState::Waiting,
            color,
            time_ran: 0,
        }
    }

    pub fn calculate_bid(&mut self, current_tick: u64) -> f64 {
        if self.state == ProcessState::Dead || self.state == ProcessState::Zombie {
            return 0.0;
        }

        // Urgency increases as deadline approaches
        let time_left = self.deadline.saturating_sub(current_tick);
        if time_left == 0 {
            self.urgency = 100.0;
            return self.wealth; // Desperate
        }

        self.urgency = 100.0 / (time_left as f64 + 1.0);
        // Bid is a fraction of wealth scaled by urgency
        let bid = self.wealth * 0.1 * self.urgency;

        bid.min(self.wealth)
    }
}

pub struct Market {
    pub last_clearing_price: f64,
}

impl Market {
    pub fn new() -> Self {
        Self {
            last_clearing_price: 0.0,
        }
    }

    // Returns (Winner Index, Price Paid)
    pub fn run_auction(&mut self, bids: &[(usize, f64)]) -> Option<(usize, f64)> {
        if bids.is_empty() {
            return None;
        }

        let mut sorted_bids = bids.to_vec();
        // Sort descending by bid amount
        sorted_bids.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        let winner = sorted_bids[0];
        let price = if sorted_bids.len() > 1 {
            sorted_bids[1].1 // Second price
        } else {
            sorted_bids[0].1 * 0.5 // Reserve price if only one bidder
        };

        self.last_clearing_price = price;
        Some((winner.0, price))
    }
}

pub struct Scheduler {
    pub tick_count: u64,
    pub processes: Vec<Process>,
    pub market: Market,
    pub history: Vec<(u64, u32, Color)>, // (Tick, ProcessID, Color)
    pub history_len: usize,
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            tick_count: 0,
            processes: Vec::new(),
            market: Market::new(),
            history: Vec::new(),
            history_len: 100,
        }
    }

    pub fn spawn_process(&mut self) {
        let mut rng = rand::thread_rng();
        let id = if let Some(last) = self.processes.last() { last.id + 1 } else { 0 };
        let deadline = self.tick_count + rng.gen_range(50..200);
        let color = Color::Rgb(rng.gen(), rng.gen(), rng.gen());
        self.processes.push(Process::new(id, deadline, color));
    }

    pub fn kill_process(&mut self) {
        if !self.processes.is_empty() {
            let mut rng = rand::thread_rng();
            let idx = rng.gen_range(0..self.processes.len());
            self.processes[idx].state = ProcessState::Dead;
            self.processes[idx].color = Color::DarkGray;
        }
    }

    pub fn tick(&mut self) {
        self.tick_count += 1;

        // Auto spawn
        if self.tick_count % 20 == 0 {
            self.spawn_process();
        }

        // 1. Collect Bids
        let mut bids = Vec::new();
        for (i, p) in self.processes.iter_mut().enumerate() {
            if p.state == ProcessState::Dead { continue; }
            let bid = p.calculate_bid(self.tick_count);
            if bid > 0.0 {
                bids.push((i, bid));
            }
        }

        // 2. Run Auction
        if let Some((winner_idx, price)) = self.market.run_auction(&bids) {
            // Update Winner
            {
                let winner = &mut self.processes[winner_idx];
                winner.wealth -= price;
                winner.time_ran += 1;
                winner.state = ProcessState::Running;
                // Reward for running (Work completed = Income)
                winner.wealth += 10.0;

                self.history.push((self.tick_count, winner.id, winner.color));
            }

            // Update Losers
            for (i, p) in self.processes.iter_mut().enumerate() {
                if i != winner_idx && p.state != ProcessState::Dead {
                    p.state = ProcessState::Waiting;
                    p.wealth -= 0.5; // Cost of waiting (Rent)
                }
            }
        } else {
            // No bids
            self.history.push((self.tick_count, 0, Color::Gray)); // Idle
        }

        // 3. Update Status
        for p in self.processes.iter_mut() {
            if p.wealth <= 0.0 && p.state != ProcessState::Dead {
                p.state = ProcessState::Zombie;
                p.color = Color::Red;
            }
            if self.tick_count > p.deadline && p.state != ProcessState::Dead {
                p.state = ProcessState::Dead; // Missed deadline
                p.color = Color::DarkGray;
            }
        }

        if self.history.len() > self.history_len {
            self.history.remove(0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vickrey_auction() {
        let mut market = Market::new();
        let bids = vec![
            (0, 10.0), // Winner
            (1, 5.0),  // Second Price
            (2, 2.0),
        ];

        let result = market.run_auction(&bids);
        assert!(result.is_some());

        let (winner, price) = result.unwrap();
        assert_eq!(winner, 0);
        assert_eq!(price, 5.0); // Should be second highest bid
    }

    #[test]
    fn test_single_bidder() {
        let mut market = Market::new();
        let bids = vec![
            (0, 10.0),
        ];

        let result = market.run_auction(&bids);
        assert!(result.is_some());
        let (winner, price) = result.unwrap();
        assert_eq!(winner, 0);
        assert_eq!(price, 5.0); // Reserve price logic (0.5 * bid)
    }
}
