use rand::Rng;
use crate::abacus::{Soroban, BeadMove};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Bid,
    Ask,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AbacusId {
    BestBid,
    BestAsk,
    LastPrice,
    Volume,
}

#[derive(Debug, Clone)]
pub enum MarketEvent {
    NewOrder(Side, u64),
    Trade(u64, u64), // Price, Size
    AbacusUpdate(AbacusId, Vec<BeadMove>),
}

pub struct Market {
    pub best_bid: Soroban,
    pub best_ask: Soroban,
    pub last_price: Soroban,
    pub volume: Soroban,

    // Internal simple storage for order book depth (simplified)
    // We just keep track of the "Best" prices for simulation.
    // In a real simulation we'd have a full book.
    // Here we simulate the "Top of Book" moving.
    target_bid: u64,
    target_ask: u64,
}

impl Market {
    pub fn new() -> Self {
        let mut m = Self {
            best_bid: Soroban::new(5), // 5 columns (up to 99999)
            best_ask: Soroban::new(5),
            last_price: Soroban::new(5),
            volume: Soroban::new(6), // 6 columns for volume
            target_bid: 10000,
            target_ask: 10050,
        };
        m.best_bid.set_value(10000);
        m.best_ask.set_value(10050);
        m.last_price.set_value(10025);
        m
    }

    pub fn update(&mut self) -> Vec<MarketEvent> {
        let mut events = Vec::new();
        let mut rng = rand::thread_rng();

        // 1. Randomly move the market
        if rng.gen_bool(0.1) {
            // Move Bid
            let change = rng.gen_range(-50..=50);
            let new_bid = (self.target_bid as i64 + change).max(1) as u64;
            // Ensure spread
            if new_bid < self.target_ask {
                self.target_bid = new_bid;
                self.best_bid.set_value(new_bid); // Instant update for now (or could animate diff?)
                // Let's not animate price setting for now, just volume/trade logic?
                // Actually, let's animate the diff if it's small?
                // Complicated. Let's just set.
            }
        }

        if rng.gen_bool(0.1) {
            // Move Ask
            let change = rng.gen_range(-50..=50);
            let new_ask = (self.target_ask as i64 + change).max(1) as u64;
            if new_ask > self.target_bid {
                self.target_ask = new_ask;
                self.best_ask.set_value(new_ask);
            }
        }

        // 2. Generate a Trade?
        // If we force a cross or just random trade event.
        if rng.gen_bool(0.05) {
            // Trade at Ask (Buy)
            let trade_price = self.target_ask;
            let trade_size = rng.gen_range(1..=10);

            self.last_price.set_value(trade_price);
            events.push(MarketEvent::Trade(trade_price, trade_size));

            // Accumulate Volume using Abacus arithmetic!
            let moves = self.volume.add(trade_size);
            events.push(MarketEvent::AbacusUpdate(AbacusId::Volume, moves));
        } else if rng.gen_bool(0.05) {
             // Trade at Bid (Sell)
            let trade_price = self.target_bid;
            let trade_size = rng.gen_range(1..=10);

            self.last_price.set_value(trade_price);
            events.push(MarketEvent::Trade(trade_price, trade_size));

            let moves = self.volume.add(trade_size);
            events.push(MarketEvent::AbacusUpdate(AbacusId::Volume, moves));
        }

        events
    }
}
