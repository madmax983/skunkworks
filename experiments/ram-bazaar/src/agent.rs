use crate::model::{Agent, Bid, Strategy};
use rand::Rng;

pub trait AgentLogic {
    fn decide_bids(&self, current_market_price: f64) -> Vec<Bid>;
    fn update_budget(&mut self);
}

impl AgentLogic for Agent {
    /// Generates a list of bids based on the agent's strategy and budget.
    fn decide_bids(&self, current_market_price: f64) -> Vec<Bid> {
        let mut bids = Vec::new();
        let mut rng = rand::rng();

        let demand = self.desired_memory;

        // Determine base willingness to pay based on strategy
        let willingness_to_pay = match self.strategy {
            Strategy::Greedy => {
                // Aggressive: Bid above market to steal pages
                (current_market_price * 1.5).max(10.0)
            }
            Strategy::Saver => {
                // Conservative: Only bid if cheap, or slightly above market to maintain
                if self.owned_pages < self.desired_memory / 2 {
                    current_market_price * 1.1 // Desperate for minimum
                } else {
                    current_market_price * 0.9 // Cheapskate for extra
                }
            }
            Strategy::Hoarder => {
                // Irrationally high valuation, sticky
                50.0 + (rng.random::<f64>() * 20.0)
            }
            Strategy::Panic => {
                // High volatility
                if rng.random_bool(0.05) {
                    200.0 // Panic buy
                } else {
                    current_market_price * 0.5 // Dump
                }
            }
        };

        // Generate bids for each desired page
        // Diminishing marginal utility: Each subsequent page is worth slightly less
        for i in 0..demand {
            let utility_decay = 1.0 - (i as f64 * 0.05); // 5% drop per page
            let mut bid_price = willingness_to_pay * utility_decay;

            // Random noise
            bid_price *= rng.random_range(0.95..1.05);

            // Budget constraint check (rough estimate)
            // If they spent this much on all previous bids, would they exceed budget?
            // This is "Rent Budget" per tick.
            let estimated_total_cost = bid_price * (i as f64 + 1.0);

            if estimated_total_cost <= self.budget {
                bids.push(Bid {
                    agent_id: self.id,
                    price: bid_price.max(0.1), // Minimum bid
                });
            } else {
                // Can't afford more pages at this price
                break;
            }
        }

        bids
    }

    fn update_budget(&mut self) {
        // Income logic
        let income = match self.strategy {
            Strategy::Greedy => 50.0,
            Strategy::Saver => 30.0,
            Strategy::Hoarder => 40.0,
            Strategy::Panic => rand::rng().random_range(0.0..100.0),
        };

        // Expenses are deducted in the market resolution (conceptually),
        // but here we just replenish the budget or simulate consumption.
        // Let's say budget is "Current Cash".
        // Rent is deducted *after* the market tick based on ownership.

        self.budget += income;
        self.budget = self.budget.min(self.max_budget); // Cap wealth
    }
}
