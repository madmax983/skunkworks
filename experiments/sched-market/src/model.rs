use rand::Rng;

#[derive(Debug, Clone)]
pub struct Thread {
    pub id: usize,
    pub work_needed: u32,
    pub total_work: u32,
    pub deadline: u64,
    pub budget: f64,
    pub start_budget: f64,
    pub value_on_completion: f64,
    pub start_tick: u64,
    pub execution_history: Vec<(u64, f64)>, // (tick, price_paid)
}

impl Thread {
    pub fn new(id: usize, work: u32, deadline: u64, budget: f64, value: f64, start_tick: u64) -> Self {
        Self {
            id,
            work_needed: work,
            total_work: work,
            deadline,
            budget,
            start_budget: budget,
            value_on_completion: value,
            start_tick,
            execution_history: Vec::new(),
        }
    }

    pub fn bid(&self, current_tick: u64) -> f64 {
        if self.work_needed == 0 {
            return 0.0;
        }

        if current_tick >= self.deadline {
            // Desperate mode: bid everything if we can finish?
            // If we are past deadline, maybe we already failed?
            // Let's assume deadline is inclusive.
            return self.budget;
        }

        let ticks_remaining = self.deadline.saturating_sub(current_tick);

        // Urgency: 1.0 means we must run every tick. < 1.0 means we have slack. > 1.0 means we are screwed.
        let urgency = self.work_needed as f64 / ticks_remaining as f64;

        // Valuation: How much is one tick worth to us?
        // Simple strategy: Try to spread budget over needed ticks, weighted by urgency.
        let budget_per_tick = self.budget / self.work_needed as f64;

        // Bid aggressive if urgent
        let bid = budget_per_tick * urgency.powi(2);

        // Cap at budget
        let mut final_bid = bid.min(self.budget);

        // Cap at value? (Rational agent wouldn't pay more than value/work_needed?)
        // But maybe value is high.

        // Add noise to avoid ties
        let mut rng = rand::thread_rng();
        final_bid *= rng.gen_range(0.9..1.1);

        final_bid.min(self.budget)
    }
}

pub struct Scheduler {
    pub current_tick: u64,
    pub threads: Vec<Thread>,
    pub completed_threads: Vec<Thread>,
    pub failed_threads: Vec<Thread>,
    pub history: Vec<TickRecord>,
}

#[derive(Clone, Debug)]
pub struct TickRecord {
    pub tick: u64,
    pub winner_id: Option<usize>,
    pub price: f64,
    pub bids: Vec<(usize, f64)>,
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            current_tick: 0,
            threads: Vec::new(),
            completed_threads: Vec::new(),
            failed_threads: Vec::new(),
            history: Vec::new(),
        }
    }

    pub fn add_thread(&mut self, t: Thread) {
        self.threads.push(t);
    }

    pub fn tick(&mut self) {
        let mut bids: Vec<(usize, f64)> = Vec::new();

        // 1. Collect Bids
        for t in &self.threads {
            bids.push((t.id, t.bid(self.current_tick)));
        }

        // 2. Determine Winner (Highest Bid)
        // Sort by bid descending
        bids.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        let winner_info = bids.first().cloned();
        let mut winner_id = None;
        let mut price = 0.0;

        if let Some((w_id, w_bid)) = winner_info {
            if w_bid > 0.0 {
                winner_id = Some(w_id);
                price = w_bid;

                // Find the thread and update it
                if let Some(thread) = self.threads.iter_mut().find(|t| t.id == w_id) {
                    thread.budget -= price;
                    thread.work_needed -= 1;
                    thread.execution_history.push((self.current_tick, price));
                }
            }
        }

        // 3. Record History
        self.history.push(TickRecord {
            tick: self.current_tick,
            winner_id,
            price,
            bids,
        });

        // 4. Cleanup (Completed/Failed)
        let mut active_threads = Vec::new();
        for t in self.threads.drain(..) {
            if t.work_needed == 0 {
                self.completed_threads.push(t);
            } else if self.current_tick >= t.deadline {
                self.failed_threads.push(t);
            } else if t.budget <= 0.0001 { // Bankrupt
                 self.failed_threads.push(t);
            } else {
                active_threads.push(t);
            }
        }
        self.threads = active_threads;

        self.current_tick += 1;
    }
}
