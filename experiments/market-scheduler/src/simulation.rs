use crate::agent::{Agent, Strategy};
use crate::market::OrderBook;
use crate::types::{Order, Side, Transaction};
use rand::Rng;
use uuid::Uuid;

pub struct Simulation {
    pub agents: Vec<Agent>,
    pub market: OrderBook,
    pub tick_count: u64,
    pub transactions: Vec<Transaction>,
}

impl Simulation {
    pub fn new(agent_count: usize) -> Self {
        let mut agents = Vec::new();
        let mut rng = rand::thread_rng();

        for i in 0..agent_count {
            let strategy = match rng.gen_range(0..3) {
                0 => Strategy::Desperate,
                1 => Strategy::Saver,
                _ => Strategy::Market,
            };
            agents.push(Agent::new(format!("Proc-{:03}", i), strategy));
        }

        Self {
            agents,
            market: OrderBook::new(),
            tick_count: 0,
            transactions: Vec::new(),
        }
    }

    pub fn update(&mut self) {
        self.tick_count += 1;

        // 1. Agents Tick & Decide
        let mut new_orders = Vec::new();
        for agent in &mut self.agents {
            agent.tick();
            if let Some(order) = agent.decide(self.market.last_price, self.tick_count) {
                new_orders.push(order);
            }
        }

        // 2. The Kernel (Provider of CPU)
        // Always provides some base capacity
        let kernel_id = Uuid::nil();

        // Kernel Asks (Selling CPU)
        // Price adjusts slightly around last price to provide anchor
        let anchor_price = self.market.last_price;
        new_orders.push(Order::new(Side::Ask, anchor_price * 1.1, 20, kernel_id, self.tick_count));
        new_orders.push(Order::new(Side::Ask, anchor_price * 1.2, 20, kernel_id, self.tick_count));

        // Kernel Bids (Buying Back / Sink) - rare, maybe for "GC"?
        // No, Kernel only sells in this model.

        // 3. Submit Orders
        for order in new_orders {
            self.market.add_order(order);
        }

        // 4. Match
        let trades = self.market.match_orders();

        // 5. Settlement
        for trade in &trades {
            // Buyer
            if let Some(buyer) = self.agents.iter_mut().find(|a| a.id == trade.buyer_id) {
                buyer.on_trade(trade.quantity, trade.price, Side::Bid);
            }
            // Seller
            if let Some(seller) = self.agents.iter_mut().find(|a| a.id == trade.seller_id) {
                seller.on_trade(trade.quantity, trade.price, Side::Ask);
            }
        }

        // Keep history
        self.transactions.extend(trades);
        if self.transactions.len() > 100 {
            let overflow = self.transactions.len() - 100;
            self.transactions.drain(0..overflow);
        }
    }
}
