use crate::suanpan::Suanpan;
use rand::Rng;
use std::collections::VecDeque;

pub struct Market {
    pub price: Suanpan,
    pub history: VecDeque<u64>,
    pub trend: f64,
}

impl Market {
    pub fn new(start_price: u64) -> Self {
        let mut history = VecDeque::new();
        history.push_back(start_price);
        Self {
            price: Suanpan::from(start_price),
            history,
            trend: 0.0,
        }
    }

    pub fn tick(&mut self) {
        let mut rng = rand::thread_rng();
        let current_u64 = self.price.to_u64();

        // Random Walk
        let change: i64 = rng.gen_range(-5..=5);
        // Add trend
        self.trend += rng.gen_range(-0.1..=0.1);
        let trend_impact = self.trend as i64;

        let new_price_i64 = (current_u64 as i64) + change + trend_impact;
        let new_price = if new_price_i64 < 1 { 1 } else { new_price_i64 as u64 };

        self.price = Suanpan::from(new_price);
        self.history.push_back(new_price);
        if self.history.len() > 100 {
            self.history.pop_front();
        }
    }
}

pub struct Scalper {
    pub balance: Suanpan, // Cash
    pub position: i64,    // Units held (can be negative/short, but let's stick to simple long for now)
    pub window: VecDeque<Suanpan>,
    pub sum: Suanpan,     // Running sum of window
    pub sma: Suanpan,     // Current SMA
    pub window_size: usize,
}

impl Scalper {
    pub fn new(initial_balance: u64) -> Self {
        Self {
            balance: Suanpan::from(initial_balance),
            position: 0,
            window: VecDeque::new(),
            sum: Suanpan::from(0),
            sma: Suanpan::from(0),
            window_size: 10, // Fixed to 10 for easy division
        }
    }

    pub fn update(&mut self, price: &Suanpan) {
        // Add new price to window
        self.window.push_back(price.clone());
        self.sum += price.clone();

        // Remove old if full
        if self.window.len() > self.window_size {
            if let Some(old) = self.window.pop_front() {
                self.sum -= old;
            }
        }

        // Calculate SMA = Sum / 10
        // Division by 10 on Abacus: Shift decimal point left (or rods right)
        // Since we are integers, we just drop the last rod (index 0) and shift others down.
        // Actually, Suanpan struct is 0-indexed from right.
        // So Rod 1 becomes Rod 0. Rod 2 becomes Rod 1.

        let mut sma_rods = self.sum.rods.clone();
        // Remove first element (Rod 0, units)
        if !sma_rods.is_empty() {
            sma_rods.remove(0);
        }
        // Add a zero rod at top to maintain length if needed, or just let it be shorter.
        // Suanpan::from construction ensures padding, but here we manually constructed.
        // Let's rely on standard logic.
        self.sma = Suanpan { rods: sma_rods }; // This might be shorter, but Add/Sub handles varying lengths.
    }

    pub fn trade(&mut self, current_price: &Suanpan) -> Option<String> {
        // Trading Strategy:
        // If Price > SMA + Threshold, Sell.
        // If Price < SMA - Threshold, Buy.
        // Threshold? Let's say 0 for now.

        let price_val = current_price.to_u64();
        let sma_val = self.sma.to_u64();

        if self.window.len() < self.window_size {
            return None; // Wait for full window
        }

        if price_val > sma_val {
            // Price is high. Sell if we have position.
            if self.position > 0 {
                self.position -= 1;
                self.balance += current_price.clone();
                return Some("SELL".to_string());
            } else if self.position == 0 {
                // Short?
                self.position -= 1;
                self.balance += current_price.clone();
                return Some("SHORT".to_string());
            }
        } else if price_val < sma_val {
            // Price is low. Buy.
            // Check if we have money?
            // Since Suanpan doesn't support easy "Less Than" without subtraction,
            // we'll just assume margin/credit or check u64 for logic.
            // But strict Abacus adherence would require `balance - price >= 0`.
            // Let's use the u64 for decision logic (Mental Math) but Suanpan for accounting.

             self.position += 1;
             self.balance -= current_price.clone();
             return Some("BUY".to_string());
        }

        None
    }
}
