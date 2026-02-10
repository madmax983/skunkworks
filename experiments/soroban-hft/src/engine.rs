use crate::beads::Soroban;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Bid,
    Ask,
}

#[derive(Debug, Clone)]
pub struct Order {
    pub id: u64,
    pub side: Side,
    pub price: Soroban,
    pub quantity: Soroban,
}

#[derive(Debug, Clone)]
pub struct Trade {
    pub maker_id: u64, // The order that was already in the book
    pub taker_id: u64, // The incoming order (or matched against)
    pub price: Soroban,
    pub quantity: Soroban,
}

pub struct OrderBook {
    pub bids: Vec<Order>, // Highest price first
    pub asks: Vec<Order>, // Lowest price first
    pub next_id: u64,
}

impl Default for OrderBook {
    fn default() -> Self {
        Self::new()
    }
}

impl OrderBook {
    pub fn new() -> Self {
        Self {
            bids: Vec::new(),
            asks: Vec::new(),
            next_id: 1,
        }
    }

    pub fn place_limit_order(&mut self, side: Side, price: Soroban, quantity: Soroban) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        let order = Order {
            id,
            side,
            price,
            quantity,
        };

        match side {
            Side::Bid => {
                // Bids sorted DESC.
                let idx = self.bids.partition_point(|o| o.price >= order.price);
                self.bids.insert(idx, order);
            }
            Side::Ask => {
                // Asks sorted ASC.
                let idx = self.asks.partition_point(|o| o.price <= order.price);
                self.asks.insert(idx, order);
            }
        }
        id
    }

    // Match orders and return executed trades.
    // In a real engine, this happens immediately on placement.
    // Here we can call it explicitly for the simulation steps.
    pub fn match_orders(&mut self) -> Vec<Trade> {
        let mut trades = Vec::new();

        loop {
            // Check if match possible
            if self.bids.is_empty() || self.asks.is_empty() {
                break;
            }

            let best_bid = &self.bids[0];
            let best_ask = &self.asks[0];

            if best_bid.price < best_ask.price {
                break;
            }

            // Match!
            // Price is determined by... usually the resting order.
            // But here we just say they match at the Ask price (taker buys at ask) or Bid (taker sells at bid).
            // Let's assume the older order sets the price.
            // We don't track timestamp here perfectly, but let's just use the Bid price for now or average?
            // "Ancient" system: maybe the geometric mean? No, bead arithmetic only.
            // Let's use the Ask price.
            let match_price = best_ask.price.clone();

            // Quantity is min(bid.qty, ask.qty)
            let bid_qty = &best_bid.quantity;
            let ask_qty = &best_ask.quantity;

            let trade_qty = if bid_qty < ask_qty {
                bid_qty.clone()
            } else {
                ask_qty.clone()
            };

            // Execute trade
            let maker_id = best_ask.id; // Arbitrary assignment for simulation
            let taker_id = best_bid.id;

            trades.push(Trade {
                maker_id,
                taker_id,
                price: match_price,
                quantity: trade_qty.clone(),
            });

            // Update quantities
            // We need to mutate the orders.
            // We can't mutate while holding references.

            // We know the indices are 0.
            // Update bid
            self.bids[0].quantity = self.bids[0].quantity.clone() - trade_qty.clone();
            // Update ask
            self.asks[0].quantity = self.asks[0].quantity.clone() - trade_qty.clone();

            // Remove filled orders
            // Check if quantity is zero.
            if self.bids[0].quantity.to_u64() == 0 {
                self.bids.remove(0);
            }
            if !self.asks.is_empty() && self.asks[0].quantity.to_u64() == 0 {
                self.asks.remove(0);
            }
        }

        trades
    }
}
