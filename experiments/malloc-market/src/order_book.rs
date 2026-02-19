use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderType {
    Bid, // Buy
    Ask, // Sell
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Order {
    pub id: u64,
    pub agent_id: usize,
    pub price: f32,
    pub quantity: usize, // Number of cells
    pub order_type: OrderType,
}

#[derive(Debug, Clone)]
pub struct Transaction {
    pub buyer_id: usize,
    pub seller_id: usize,
    pub price: f32,
    pub quantity: usize,
}

pub struct OrderBook {
    pub bids: Vec<Order>, // Sorted high to low
    pub asks: Vec<Order>, // Sorted low to high
    next_order_id: u64,
}

impl OrderBook {
    pub fn new() -> Self {
        Self {
            bids: Vec::new(),
            asks: Vec::new(),
            next_order_id: 0,
        }
    }

    pub fn add_order(&mut self, agent_id: usize, price: f32, quantity: usize, order_type: OrderType) {
        let order = Order {
            id: self.next_order_id,
            agent_id,
            price,
            quantity,
            order_type,
        };
        self.next_order_id += 1;

        match order_type {
            OrderType::Bid => {
                self.bids.push(order);
                // Sort bids: Highest price first
                self.bids.sort_by(|a, b| b.price.partial_cmp(&a.price).unwrap_or(Ordering::Equal));
            }
            OrderType::Ask => {
                self.asks.push(order);
                // Sort asks: Lowest price first
                self.asks.sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap_or(Ordering::Equal));
            }
        }
    }

    pub fn match_orders(&mut self) -> Vec<Transaction> {
        let mut transactions = Vec::new();

        while !self.bids.is_empty() && !self.asks.is_empty() {
            let best_bid = &self.bids[0];
            let best_ask = &self.asks[0];

            if best_bid.price >= best_ask.price {
                // Match!
                let price = (best_bid.price + best_ask.price) / 2.0; // Mid-point clearing price
                let quantity = best_bid.quantity.min(best_ask.quantity);

                transactions.push(Transaction {
                    buyer_id: best_bid.agent_id,
                    seller_id: best_ask.agent_id,
                    price,
                    quantity,
                });

                // Update quantities
                let bid_filled = best_bid.quantity == quantity;
                let ask_filled = best_ask.quantity == quantity;

                if bid_filled {
                    self.bids.remove(0);
                } else {
                    self.bids[0].quantity -= quantity;
                }

                if ask_filled {
                    self.asks.remove(0);
                } else {
                    self.asks[0].quantity -= quantity;
                }
            } else {
                break; // No more matches possible
            }
        }

        transactions
    }

    pub fn clear(&mut self) {
        // Optionally clear unmatched orders every tick if they expire?
        // For now, let's keep them.
        // Or maybe simulate "Fill or Kill" by clearing everything?
        // Let's clear for this simple simulation to avoid stale orders piling up.
        self.bids.clear();
        self.asks.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matching() {
        let mut book = OrderBook::new();

        // Bid: Agent 1 wants 10 @ $5.0
        book.add_order(1, 5.0, 10, OrderType::Bid);
        // Ask: Agent 2 sells 5 @ $4.0
        book.add_order(2, 4.0, 5, OrderType::Ask);

        let txs = book.match_orders();

        assert_eq!(txs.len(), 1);
        assert_eq!(txs[0].buyer_id, 1);
        assert_eq!(txs[0].seller_id, 2);
        assert_eq!(txs[0].quantity, 5);
        assert_eq!(txs[0].price, 4.5); // (5+4)/2

        // Remaining Bid: Agent 1 wants 5 @ $5.0
        assert_eq!(book.bids.len(), 1);
        assert_eq!(book.bids[0].quantity, 5);
        assert_eq!(book.asks.len(), 0);
    }
}
