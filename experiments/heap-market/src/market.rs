use crate::model::*;

pub struct Market {
    pub bids: Vec<Order>,
    pub asks: Vec<Order>,
    pub history: Vec<u64>,
}

impl Default for Market {
    fn default() -> Self {
        Self::new()
    }
}

impl Market {
    pub fn new() -> Self {
        Self {
            bids: Vec::new(),
            asks: Vec::new(),
            history: Vec::new(),
        }
    }

    pub fn add_order(&mut self, order: Order) {
        match order.order_type {
            OrderType::Bid => self.bids.push(order),
            OrderType::Ask => self.asks.push(order),
        }
    }

    pub fn match_orders(&mut self) -> Vec<Transaction> {
        // Sort bids high to low
        self.bids.sort_by(|a, b| b.price.cmp(&a.price));
        // Sort asks low to high
        self.asks.sort_by(|a, b| a.price.cmp(&b.price));

        let mut transactions = Vec::new();
        let mut bid_idx = 0;
        let mut ask_idx = 0;

        while bid_idx < self.bids.len() && ask_idx < self.asks.len() {
            // Check if match is possible
            if self.bids[bid_idx].price >= self.asks[ask_idx].price {
                let trade_qty = std::cmp::min(self.bids[bid_idx].quantity, self.asks[ask_idx].quantity);

                if trade_qty > 0 {
                    // Execute trade
                    let clearing_price = (self.bids[bid_idx].price + self.asks[ask_idx].price) / 2;
                    self.history.push(clearing_price);

                    transactions.push(Transaction {
                        buyer_id: self.bids[bid_idx].agent_id,
                        seller_id: self.asks[ask_idx].agent_id,
                        price: clearing_price,
                        quantity: trade_qty,
                    });

                    self.bids[bid_idx].quantity -= trade_qty;
                    self.asks[ask_idx].quantity -= trade_qty;
                }

                // Advance indices if orders are filled
                if self.bids[bid_idx].quantity == 0 {
                    bid_idx += 1;
                }
                if self.asks[ask_idx].quantity == 0 {
                    ask_idx += 1;
                }
            } else {
                break; // No more overlapping prices
            }
        }

        // Cleanup filled orders
        self.bids.retain(|o| o.quantity > 0);
        self.asks.retain(|o| o.quantity > 0);

        transactions
    }

    pub fn clearing_price(&self) -> Option<u64> {
        self.history.last().copied()
    }
}
