use crate::types::{Order, Side};
use rand::Rng;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Strategy {
    Desperate, // Buys high, Sells low (Panic)
    Saver,     // Buys low, Sells high (Limit orders)
    Market,    // Follows market price
}

#[derive(Debug, Clone)]
pub struct Agent {
    pub id: Uuid,
    pub name: String,
    pub credits: f64,
    pub battery: f64, // 0..100. Represents Process Completion/Health.
    pub strategy: Strategy,
    pub is_alive: bool,
}

impl Agent {
    pub fn new(name: String, strategy: Strategy) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            credits: 100.0,
            battery: 50.0,
            strategy,
            is_alive: true,
        }
    }

    pub fn tick(&mut self) {
        if !self.is_alive {
            return;
        }

        // Passive drain (Context switching cost)
        self.battery -= 0.2;

        // Basic Income
        self.credits += 0.5;

        if self.battery <= 0.0 || self.credits < -50.0 {
            // Allow some debt
            self.is_alive = false;
        }
    }

    pub fn decide(&self, market_price: f64, tick: u64) -> Option<Order> {
        if !self.is_alive {
            return None;
        }

        let mut rng = rand::thread_rng();
        let volatility = 0.1;

        // Buying Logic (Low Battery)
        if self.battery < 40.0 {
            let price_mult = match self.strategy {
                Strategy::Desperate => 1.5,
                Strategy::Saver => 0.8,
                Strategy::Market => 1.0 + rng.gen_range(-volatility..volatility),
            };

            let bid_price = (market_price * price_mult).max(0.1);
            return Some(Order::new(Side::Bid, bid_price, 5, self.id, tick));
        }

        // Selling Logic (High Battery - selling excess compute cycles?)
        if self.battery > 80.0 {
            let price_mult = match self.strategy {
                Strategy::Desperate => 0.8, // Sell quick
                Strategy::Saver => 1.2,     // Wait for profit
                Strategy::Market => 1.0 + rng.gen_range(-volatility..volatility),
            };

            let ask_price = (market_price * price_mult).max(0.1);
            return Some(Order::new(Side::Ask, ask_price, 5, self.id, tick));
        }

        None
    }

    pub fn on_trade(&mut self, qty: u32, price: f64, side: Side) {
        match side {
            Side::Bid => {
                // Bought CPU time -> Charge battery
                self.credits -= price * qty as f64;
                self.battery += (qty * 5) as f64;
            }
            Side::Ask => {
                // Sold CPU time -> Drain battery (Work done for others)
                self.credits += price * qty as f64;
                self.battery -= (qty * 5) as f64;
            }
        }
        self.battery = self.battery.clamp(0.0, 100.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bankruptcy() {
        let mut agent = Agent::new("Test".to_string(), Strategy::Market);
        agent.credits = -60.0;
        agent.battery = 10.0;

        agent.tick();
        assert!(!agent.is_alive);
    }

    #[test]
    fn test_trade_update() {
        let mut agent = Agent::new("Test".to_string(), Strategy::Market);
        agent.battery = 50.0;
        agent.credits = 100.0;

        // Buy 10 units at price 2.0
        agent.on_trade(10, 2.0, Side::Bid);

        assert_eq!(agent.credits, 80.0); // 100 - 20
        assert_eq!(agent.battery, 100.0); // 50 + 50
    }
}
