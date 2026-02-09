use rand::prelude::*;
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AgentId(pub u64);

#[derive(Debug, Clone)]
pub struct MemoryBlock {
    pub id: BlockId,
    pub owner: Option<AgentId>,
    pub price: f64, // Last transaction price or initial valuation
    pub value: f64, // Intrinsic value / utility for the agent
}

#[derive(Debug, Clone, PartialEq)]
pub enum Strategy {
    Random,
    Hoarder,
    Flipper,
    Desperate,
}

#[derive(Debug, Clone)]
pub struct Agent {
    pub id: AgentId,
    pub strategy: Strategy,
    pub balance: f64,
    pub owned_blocks: Vec<BlockId>,
    pub memory_need: usize, // Target number of blocks
}

#[derive(Debug, Clone)]
pub struct Order {
    pub agent_id: AgentId,
    pub price: f64,
    pub block_id: Option<BlockId>, // None for buy (any block), Some for sell (specific block)
    pub is_buy: bool,
}

#[derive(Debug, Clone)]
pub struct Transaction {
    pub buyer: AgentId,
    pub seller: Option<AgentId>, // None if sold by system
    pub block_id: BlockId,
    pub price: f64,
    pub tick: u64,
}

#[derive(Debug, Default, Clone)]
pub struct Market {
    pub bids: Vec<Order>,
    pub asks: Vec<Order>,
    pub history: VecDeque<Transaction>,
}

#[derive(Debug)]
pub struct System {
    pub memory: Vec<MemoryBlock>,
    pub agents: Vec<Agent>,
    pub market: Market,
    pub tick: u64,
    pub next_block_id: u64,
    pub next_agent_id: u64,
}

impl System {
    pub fn new(memory_size: usize, agent_count: usize) -> Self {
        let mut memory = Vec::with_capacity(memory_size);
        for i in 0..memory_size {
            memory.push(MemoryBlock {
                id: BlockId(i as u64),
                owner: None,
                price: 10.0,
                value: 10.0,
            });
        }

        let mut agents = Vec::with_capacity(agent_count);
        let mut rng = rand::thread_rng();
        for i in 0..agent_count {
            let strategy = match rng.gen_range(0..4) {
                0 => Strategy::Random,
                1 => Strategy::Hoarder,
                2 => Strategy::Flipper,
                _ => Strategy::Desperate,
            };
            agents.push(Agent {
                id: AgentId(i as u64),
                strategy,
                balance: 1000.0,
                owned_blocks: Vec::new(),
                memory_need: rng.gen_range(5..20),
            });
        }

        System {
            memory,
            agents,
            market: Market::default(),
            tick: 0,
            next_block_id: memory_size as u64,
            next_agent_id: agent_count as u64,
        }
    }

    pub fn tick(&mut self) {
        self.tick += 1;
        let mut rng = rand::thread_rng();

        // 1. Agent Actions
        // Clone agents to avoid borrow checker issues during iteration, update them later
        // Actually, better to separate decision making from state mutation.
        // We will collect orders first.
        let mut new_orders = Vec::new();

        for agent in &mut self.agents {
            // Income
            let income = agent.owned_blocks.len() as f64 * 1.0; // Production
            agent.balance += income;

            // Rent / Maintenance
            let rent = agent.owned_blocks.len() as f64 * 0.1;
            agent.balance -= rent;

            // Strategy
            match agent.strategy {
                Strategy::Random => {
                    if rng.gen_bool(0.1) {
                        let price = rng.gen_range(5.0..15.0);
                        if agent.balance > price && agent.owned_blocks.len() < agent.memory_need {
                            new_orders.push(Order {
                                agent_id: agent.id,
                                price,
                                block_id: None,
                                is_buy: true,
                            });
                        }
                    }
                    if rng.gen_bool(0.1) && !agent.owned_blocks.is_empty() {
                        let block_idx = rng.gen_range(0..agent.owned_blocks.len());
                        let block_id = agent.owned_blocks[block_idx];
                        let price = rng.gen_range(10.0..20.0);
                        new_orders.push(Order {
                            agent_id: agent.id,
                            price,
                            block_id: Some(block_id),
                            is_buy: false,
                        });
                    }
                }
                Strategy::Hoarder => {
                    // Buys if price is low, rarely sells
                    if agent.balance > 50.0 && agent.owned_blocks.len() < agent.memory_need * 2 {
                        let bid_price = 8.0;
                        new_orders.push(Order {
                            agent_id: agent.id,
                            price: bid_price,
                            block_id: None,
                            is_buy: true,
                        });
                    }
                }
                Strategy::Desperate => {
                    // Must reach memory_need
                    if agent.owned_blocks.len() < agent.memory_need {
                        let bid_price = agent.balance.min(50.0); // All in if needed
                        new_orders.push(Order {
                            agent_id: agent.id,
                            price: bid_price,
                            block_id: None,
                            is_buy: true,
                        });
                    } else if agent.balance < 50.0 && !agent.owned_blocks.is_empty() {
                        // Sell to survive
                        let block_idx = rng.gen_range(0..agent.owned_blocks.len());
                        let block_id = agent.owned_blocks[block_idx];
                        new_orders.push(Order {
                            agent_id: agent.id,
                            price: 5.0,
                            block_id: Some(block_id),
                            is_buy: false,
                        });
                    }
                }
                Strategy::Flipper => {
                    // Buy low, sell high
                    if agent.balance > 20.0 {
                        new_orders.push(Order {
                            agent_id: agent.id,
                            price: 9.0,
                            block_id: None,
                            is_buy: true,
                        });
                    }
                    for &block_id in &agent.owned_blocks {
                        // Find block current price? We store it in memory.
                        // For simplicity, just sell randomly for profit
                        if rng.gen_bool(0.2) {
                            new_orders.push(Order {
                                agent_id: agent.id,
                                price: 15.0,
                                block_id: Some(block_id),
                                is_buy: false,
                            });
                        }
                    }
                }
            }
        }

        // System also sells unowned blocks
        for block in &self.memory {
            if block.owner.is_none() {
                // System asks
                self.market.asks.push(Order {
                    agent_id: AgentId(u64::MAX), // System ID
                    price: 10.0,                 // Reserve price
                    block_id: Some(block.id),
                    is_buy: false,
                });
            }
        }

        // Add agent orders
        for order in new_orders {
            if order.is_buy {
                self.market.bids.push(order);
            } else {
                self.market.asks.push(order);
            }
        }

        // 2. Market Clearing
        self.market
            .bids
            .sort_by(|a, b| b.price.partial_cmp(&a.price).unwrap()); // Descending
        self.market
            .asks
            .sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap()); // Ascending

        let mut transactions = Vec::new();

        // Simple matching: iterate through bids and try to find a matching ask
        // Note: Asks are for specific blocks (usually). Bids are for "any" block.
        // If a bid matches an ask price, we execute.

        let mut executed_bids = Vec::new();
        let mut executed_asks = Vec::new();

        // We need to be careful with indices if we remove items.
        // Let's just iterate and mark matched indices.
        let mut bid_idx = 0;
        let mut ask_idx = 0;

        // This is a simplification. A real double auction is more complex with "any" vs "specific".
        // Here, buyers want *a* block. Sellers sell *a specific* block.
        // So we match the highest bidder with the lowest asker.

        while bid_idx < self.market.bids.len() && ask_idx < self.market.asks.len() {
            let bid = &self.market.bids[bid_idx];
            let ask = &self.market.asks[ask_idx];

            if bid.price >= ask.price {
                // Match!
                let price = (bid.price + ask.price) / 2.0;
                let block_id = ask.block_id.unwrap(); // Asks must have a block_id

                // Record transaction
                transactions.push(Transaction {
                    buyer: bid.agent_id,
                    seller: if ask.agent_id == AgentId(u64::MAX) {
                        None
                    } else {
                        Some(ask.agent_id)
                    },
                    block_id,
                    price,
                    tick: self.tick,
                });

                executed_bids.push(bid_idx);
                executed_asks.push(ask_idx);

                bid_idx += 1;
                ask_idx += 1;
            } else {
                break; // No more matches possible (sorted)
            }
        }

        // Apply transactions
        for tx in transactions.clone() {
            // 1. Transfer money
            if let Some(seller_id) = tx.seller {
                if let Some(seller) = self.agents.iter_mut().find(|a| a.id == seller_id) {
                    seller.balance += tx.price;
                    seller.owned_blocks.retain(|&b| b != tx.block_id);
                }
            }

            if let Some(buyer) = self.agents.iter_mut().find(|a| a.id == tx.buyer) {
                buyer.balance -= tx.price;
                buyer.owned_blocks.push(tx.block_id);
            }

            // 2. Update block ownership and price
            if let Some(block) = self.memory.iter_mut().find(|b| b.id == tx.block_id) {
                block.owner = Some(tx.buyer);
                block.price = tx.price;
            }
        }

        // Add to history
        for tx in transactions {
            self.market.history.push_back(tx);
            if self.market.history.len() > 100 {
                self.market.history.pop_front();
            }
        }

        // Clear orders (continuous auction, unfulfilled orders expire for simplicity in this tick-based sim)
        self.market.bids.clear();
        self.market.asks.clear();

        // 3. GC (Bankruptcy)
        let bankrupt_ids: Vec<AgentId> = self
            .agents
            .iter()
            .filter(|a| a.balance < 0.0)
            .map(|a| a.id)
            .collect();

        // Return blocks to system
        for agent in &self.agents {
            if bankrupt_ids.contains(&agent.id) {
                for &block_id in &agent.owned_blocks {
                    if let Some(block) = self.memory.iter_mut().find(|b| b.id == block_id) {
                        block.owner = None;
                    }
                }
            }
        }

        // Remove bankrupt agents
        self.agents.retain(|a| !bankrupt_ids.contains(&a.id));

        // Respawn
        for _ in 0..bankrupt_ids.len() {
            let strategy = match rng.gen_range(0..4) {
                0 => Strategy::Random,
                1 => Strategy::Hoarder,
                2 => Strategy::Flipper,
                _ => Strategy::Desperate,
            };
            self.agents.push(Agent {
                id: AgentId(self.next_agent_id),
                strategy,
                balance: 1000.0,
                owned_blocks: Vec::new(),
                memory_need: rng.gen_range(5..20),
            });
            self.next_agent_id += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_init() {
        let system = System::new(100, 10);
        assert_eq!(system.memory.len(), 100);
        assert_eq!(system.agents.len(), 10);
    }

    #[test]
    fn test_bankruptcy() {
        let mut system = System::new(10, 1);
        system.agents[0].balance = -10.0;
        let old_id = system.agents[0].id;

        system.tick();

        assert_ne!(system.agents[0].id, old_id); // Should be replaced
        assert_eq!(system.agents.len(), 1); // Maintain population
    }
}
