#[cfg(feature = "nova")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "nova")]
use std::collections::VecDeque;

#[cfg(feature = "nova")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: u64,
    pub trader_id: usize,
    pub item: String,
    pub price: i64,
    pub is_sell: bool, // true = Ask (Sell), false = Bid (Buy)
}

#[cfg(feature = "nova")]
#[derive(Debug, Clone, Default)]
pub struct MarketState {
    pub asks: Vec<Order>, // Sell orders
    pub bids: Vec<Order>, // Buy orders (future use)
    pub wallets: Vec<i64>, // Credits per strand
    pub history: VecDeque<(String, i64)>, // (Item, Price)
    pub next_order_id: u64,
}

#[cfg(feature = "nova")]
impl MarketState {
    pub fn new() -> Self {
        Self {
            asks: Vec::new(),
            bids: Vec::new(),
            wallets: Vec::new(),
            history: VecDeque::new(),
            next_order_id: 0,
        }
    }

    pub fn clear(&mut self) {
        self.asks.clear();
        self.bids.clear();
        self.wallets.clear();
        self.history.clear();
        self.next_order_id = 0;
    }

    pub fn ensure_wallet(&mut self, trader_id: usize) {
        if trader_id >= self.wallets.len() {
            self.wallets.resize(trader_id + 1, 0);
        }
    }

    pub fn get_balance(&mut self, trader_id: usize) -> i64 {
        self.ensure_wallet(trader_id);
        self.wallets[trader_id]
    }

    pub fn credit(&mut self, trader_id: usize, amount: i64) {
        self.ensure_wallet(trader_id);
        self.wallets[trader_id] = self.wallets[trader_id].saturating_add(amount);
    }

    pub fn debit(&mut self, trader_id: usize, amount: i64) -> bool {
        self.ensure_wallet(trader_id);
        if self.wallets[trader_id] >= amount {
            self.wallets[trader_id] -= amount;
            true
        } else {
            false
        }
    }

    pub fn place_ask(&mut self, trader_id: usize, item: String, price: i64) -> u64 {
        let id = self.next_order_id;
        self.next_order_id += 1;
        self.asks.push(Order {
            id,
            trader_id,
            item,
            price,
            is_sell: true,
        });
        id
    }

    pub fn match_buy(&mut self, buyer_id: usize, query: String, max_price: i64) -> Option<(String, i64)> {
        // Sort asks by price ascending (cheapest first)
        self.asks.sort_by_key(|o| o.price);

        let mut match_idx = None;
        for (i, ask) in self.asks.iter().enumerate() {
            // Check price and query match
            if ask.price <= max_price && (ask.item == query || query == "*") {
                // Prevent self-trading
                if ask.trader_id != buyer_id {
                    match_idx = Some(i);
                    break;
                }
            }
        }

        if let Some(i) = match_idx {
            let ask = self.asks.remove(i);
            let cost = ask.price;

            // Check if buyer has funds
            if self.debit(buyer_id, cost) {
                // Transfer to seller
                self.credit(ask.trader_id, cost);

                // Record history
                self.history.push_back((ask.item.clone(), cost));
                if self.history.len() > 50 {
                    self.history.pop_front();
                }

                return Some((ask.item, cost));
            } else {
                // Buyer broke, put ask back
                self.asks.insert(i, ask);
                return None;
            }
        }

        None
    }
}
