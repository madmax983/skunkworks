use crate::types::{Order, Side, Transaction};
use std::cmp::Ordering;

pub struct OrderBook {
    pub bids: Vec<Order>,
    pub asks: Vec<Order>,
    pub last_price: f64,
}

impl OrderBook {
    pub fn new() -> Self {
        Self {
            bids: Vec::new(),
            asks: Vec::new(),
            last_price: 10.0,
        }
    }

    pub fn add_order(&mut self, order: Order) {
        match order.side {
            Side::Bid => self.bids.push(order),
            Side::Ask => self.asks.push(order),
        }
    }

    pub fn match_orders(&mut self) -> Vec<Transaction> {
        // Bids: Highest Price first
        self.bids.sort_by(|a, b| {
            b.price
                .partial_cmp(&a.price)
                .unwrap_or(Ordering::Equal)
                .then(a.timestamp.cmp(&b.timestamp))
        });

        // Asks: Lowest Price first
        self.asks.sort_by(|a, b| {
            a.price
                .partial_cmp(&b.price)
                .unwrap_or(Ordering::Equal)
                .then(a.timestamp.cmp(&b.timestamp))
        });

        let mut transactions = Vec::new();
        let mut bid_idx = 0;
        let mut ask_idx = 0;

        // Since we are modifying quantities, we can't just iterate easily.
        // Let's loop while we have potential matches.

        while bid_idx < self.bids.len() && ask_idx < self.asks.len() {
            // We need to re-check this inside the loop because indices might be invalidated if we removed?
            // No, let's process and then remove empty ones later.

            let bid = &mut self.bids[bid_idx];
            let ask = &mut self.asks[ask_idx];

            if bid.price >= ask.price {
                let trade_price = (bid.price + ask.price) / 2.0;
                let trade_qty = bid.quantity.min(ask.quantity);

                transactions.push(Transaction {
                    price: trade_price,
                    quantity: trade_qty,
                    buyer_id: bid.owner_id,
                    seller_id: ask.owner_id,
                    timestamp: bid.timestamp,
                });

                self.last_price = trade_price;

                bid.quantity -= trade_qty;
                ask.quantity -= trade_qty;

                if bid.quantity == 0 {
                    bid_idx += 1;
                }
                if ask.quantity == 0 {
                    ask_idx += 1;
                }
            } else {
                break; // Spread not crossed
            }
        }

        // Remove filled orders
        // Drain from the front?
        // Or just keep the remaining orders.
        if bid_idx > 0 {
            self.bids.drain(0..bid_idx);
        }
        if ask_idx > 0 {
            self.asks.drain(0..ask_idx);
        }

        transactions
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_matching() {
        let mut book = OrderBook::new();
        let buyer = Uuid::new_v4();
        let seller = Uuid::new_v4();

        book.add_order(Order::new(Side::Bid, 10.0, 10, buyer, 1));
        book.add_order(Order::new(Side::Ask, 9.0, 10, seller, 2));

        let txs = book.match_orders();
        assert_eq!(txs.len(), 1);
        assert_eq!(txs[0].price, 9.5);
        assert_eq!(txs[0].quantity, 10);
        assert_eq!(txs[0].buyer_id, buyer);
        assert_eq!(txs[0].seller_id, seller);

        assert!(book.bids.is_empty());
        assert!(book.asks.is_empty());
    }

    #[test]
    fn test_partial_fill() {
        let mut book = OrderBook::new();
        let buyer = Uuid::new_v4();
        let seller = Uuid::new_v4();

        book.add_order(Order::new(Side::Bid, 10.0, 15, buyer, 1)); // Wants 15
        book.add_order(Order::new(Side::Ask, 10.0, 10, seller, 2)); // Has 10

        let txs = book.match_orders();
        assert_eq!(txs.len(), 1);
        assert_eq!(txs[0].quantity, 10); // Filled 10

        assert_eq!(book.bids.len(), 1);
        assert_eq!(book.bids[0].quantity, 5); // 5 Remaining
        assert!(book.asks.is_empty());
    }
}
