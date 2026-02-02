use crate::model::{Strategy, ThreadAgent};
use rand::Rng;

impl ThreadAgent {
    pub fn new(id: usize, strategy: Strategy, budget: f64, work: f64, deadline: u64) -> Self {
        Self {
            id,
            strategy,
            credits: budget,
            work_total: work,
            work_remaining: work,
            deadline,
            executed_ticks: 0,
            state: crate::model::AgentState::Active,
        }
    }

    pub fn decide_bid(&self, current_market_price: f64, current_tick: u64) -> Option<f64> {
        if self.credits <= 0.0 || self.work_remaining <= 0.0 {
            return None;
        }

        let mut rng = rand::thread_rng();

        let ticks_until_deadline = self.deadline.saturating_sub(current_tick) as f64;

        // Urgency: Work Remaining / Time Remaining
        let urgency = if ticks_until_deadline <= 0.0 {
            100.0 // Extremely urgent (or dead)
        } else {
            self.work_remaining / ticks_until_deadline
        };

        let bid = match self.strategy {
            Strategy::HighFreq => {
                // Bid slightly above market price to ensure execution
                // HFTs just want to run, they don't care much about price optimization
                let noise = rng.gen_range(0.0..1.0);
                current_market_price + noise
            }
            Strategy::Sniper => {
                // Wait until the last possible moment
                // If we MUST run every tick from now on (urgency >= 1.0), bid high.
                // Otherwise, bid 0 or very low.
                if urgency >= 0.95 {
                    // Panic bid
                    self.credits * 0.5 // Bet half the farm
                } else {
                    // Lowball
                    rng.gen_range(0.1..0.5)
                }
            }
            Strategy::Desperate => {
                // Bid grows exponentially with urgency
                // If urgency is low, bid low. If high, bid insane.
                let factor = urgency.powf(4.0);
                current_market_price * (0.5 + factor)
            }
            Strategy::Value => {
                // Calculate "fair value" per unit of work
                // Budget / Remaining Work
                let fair_value = self.credits / self.work_remaining;

                // If market is cheap, buy. If expensive, wait (unless urgent).
                if current_market_price < fair_value {
                    current_market_price + 0.1
                } else if urgency > 0.8 {
                    // Forced to buy above value
                    current_market_price + 0.1
                } else {
                    fair_value * 0.8 // Lowball
                }
            }
        };

        // Ensure bid is at least minimal
        let bid = bid.max(0.1);

        // Cannot bid more than we have
        Some(bid.min(self.credits))
    }
}
