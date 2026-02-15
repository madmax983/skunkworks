use crate::soroban::Soroban;
use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Signal {
    Buy,
    Sell,
    Hold,
}

pub struct Market {
    pub prices: Vec<u64>,
    pub current_price: u64,
}

impl Default for Market {
    fn default() -> Self {
        Self {
            prices: vec![10000], // Start at 100.00
            current_price: 10000,
        }
    }
}

impl Market {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&mut self) {
        let mut rng = rand::thread_rng();
        let change: i64 = rng.gen_range(-50..51); // -0.50 to +0.50 change

        // Random walk with drift? No, pure random walk.
        let new_price = (self.current_price as i64 + change).max(1) as u64;
        self.prices.push(new_price);
        self.current_price = new_price;
    }
}

pub struct Trader {
    pub soroban: Soroban,
    pub window_size: usize,
    pub current_sma: u64,
    pub last_signal: Signal,
}

impl Trader {
    pub fn new(window_size: usize) -> Self {
        Self {
            soroban: Soroban::new(),
            window_size,
            current_sma: 0,
            last_signal: Signal::Hold,
        }
    }

    pub fn process(&mut self, market: &Market) {
        let prices = &market.prices;
        let n = prices.len();

        if n < 1 { return; }

        let new_price = prices[n - 1];

        // Update Running Sum on Soroban
        self.soroban.add(new_price);

        if n > self.window_size {
            let old_price = prices[n - 1 - self.window_size];
            self.soroban.sub(old_price);
        }

        // Calculate SMA from Soroban value
        let sum = self.soroban.value();
        // Avoid division by zero
        let divisor = self.window_size.min(n) as u64;
        if divisor > 0 {
            self.current_sma = sum / divisor;
        }

        // Generate Signal
        if new_price > self.current_sma + 10 {
            self.last_signal = Signal::Buy;
        } else if new_price < self.current_sma.saturating_sub(10) {
            self.last_signal = Signal::Sell;
        } else {
            self.last_signal = Signal::Hold;
        }
    }
}
