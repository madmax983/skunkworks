use rand::prelude::*;
use rayon::prelude::*;
use std::collections::VecDeque;

#[derive(Clone, Debug)]
pub struct Block {
    pub start: usize,
    pub size: usize,
    pub owner: Option<usize>, // Agent ID
    pub price: f64,
    pub ticks_remaining: usize,
}

#[derive(Clone, Debug)]
pub struct Heap {
    pub blocks: Vec<Block>,
    pub capacity: usize,
}

impl Heap {
    pub fn new(capacity: usize) -> Self {
        Self {
            blocks: vec![Block {
                start: 0,
                size: capacity,
                owner: None,
                price: 0.0,
                ticks_remaining: 0,
            }],
            capacity,
        }
    }

    pub fn defrag(&mut self) {
        let mut new_blocks = Vec::new();
        if self.blocks.is_empty() {
            return;
        }

        let mut current = self.blocks[0].clone();

        for next in self.blocks.iter().skip(1) {
            if current.owner.is_none() && next.owner.is_none() {
                // Merge free blocks
                current.size += next.size;
            } else {
                new_blocks.push(current);
                current = next.clone();
            }
        }
        new_blocks.push(current);
        self.blocks = new_blocks;
    }

    pub fn allocate(&mut self, size: usize, agent_id: usize, price: f64, duration: usize) -> bool {
        // Find first fit
        if let Some(index) = self.blocks.iter().position(|b| b.owner.is_none() && b.size >= size) {
            let block = &mut self.blocks[index];

            if block.size == size {
                block.owner = Some(agent_id);
                block.price = price;
                block.ticks_remaining = duration;
            } else {
                // Split
                let remaining_size = block.size - size;
                let new_block = Block {
                    start: block.start + size,
                    size: remaining_size,
                    owner: None,
                    price: 0.0,
                    ticks_remaining: 0,
                };

                block.size = size;
                block.owner = Some(agent_id);
                block.price = price;
                block.ticks_remaining = duration;

                self.blocks.insert(index + 1, new_block);
            }
            return true;
        }
        false
    }

    pub fn free_expired(&mut self) -> usize {
        let mut freed_count = 0;
        for block in &mut self.blocks {
            if block.owner.is_some() {
                if block.ticks_remaining > 0 {
                    block.ticks_remaining -= 1;
                }
                if block.ticks_remaining == 0 {
                    block.owner = None;
                    block.price = 0.0; // Reset price to 0.0 for free blocks
                    freed_count += 1;
                }
            }
        }
        // Defrag is called separately in tick() to avoid double borrow
        freed_count
    }

    pub fn fragmentation(&self) -> f64 {
        let free_blocks = self.blocks.iter().filter(|b| b.owner.is_none()).count();
        if free_blocks <= 1 {
            return 0.0;
        }
        // Simple metric: number of free blocks / total capacity (normalized roughly)
        // Or just return count.
        free_blocks as f64
    }
}

#[derive(Clone, Debug)]
pub enum Strategy {
    Hoarder,   // Buys big, holds long
    Flipper,   // Buys small, holds short (high frequency)
    JustInTime, // Buys what it needs immediately
    Panic,     // Buys anything at any price if capacity is low
}

#[derive(Clone, Debug)]
pub struct Agent {
    pub id: usize,
    pub budget: f64,
    pub strategy: Strategy,
    pub owned_blocks: usize,
}

impl Agent {
    pub fn new(id: usize, strategy: Strategy) -> Self {
        Self {
            id,
            budget: 1000.0, // Initial budget
            strategy,
            owned_blocks: 0,
        }
    }

    pub fn decide(&self, market_price: f64, fragmentation: f64) -> Option<Order> {
        let mut rng = rand::thread_rng();

        // Simple logic for now
        match self.strategy {
            Strategy::Hoarder => {
                if rng.gen_bool(0.1) && self.budget > market_price * 100.0 {
                    return Some(Order::Bid {
                        agent_id: self.id,
                        size: rng.gen_range(50..200),
                        price: market_price * 1.1,
                        duration: rng.gen_range(100..500),
                    });
                }
            },
            Strategy::Flipper => {
                 if rng.gen_bool(0.3) && self.budget > market_price * 10.0 {
                    return Some(Order::Bid {
                        agent_id: self.id,
                        size: rng.gen_range(5..20),
                        price: market_price * 0.9,
                        duration: rng.gen_range(5..20),
                    });
                }
            },
            Strategy::JustInTime => {
                if rng.gen_bool(0.05) && self.budget > market_price * 50.0 {
                     return Some(Order::Bid {
                        agent_id: self.id,
                        size: rng.gen_range(10..50),
                        price: market_price * 1.05,
                        duration: rng.gen_range(20..50),
                    });
                }
            },
            Strategy::Panic => {
                if fragmentation > 10.0 && self.budget > 0.0 {
                     return Some(Order::Bid {
                        agent_id: self.id,
                        size: rng.gen_range(1..10),
                        price: market_price * 2.0,
                        duration: rng.gen_range(50..100),
                    });
                }
            }
        }
        None
    }
}

#[derive(Clone, Debug)]
pub enum Order {
    Bid {
        agent_id: usize,
        size: usize,
        price: f64,
        duration: usize,
    },
    // Ask logic is implicit: Agents free memory when ticks expire.
    // Explicit selling could be added later.
}

pub struct Market {
    pub heap: Heap,
    pub agents: Vec<Agent>,
    pub history: VecDeque<f64>,
    pub current_price: f64,
    pub tick_count: usize,
    pub last_volume: usize,
    pub last_turnover: f64,
}

impl Market {
    pub fn new(capacity: usize, agent_count: usize) -> Self {
        let mut agents = Vec::new();
        let mut rng = rand::thread_rng();

        for i in 0..agent_count {
            let strategy = match rng.gen_range(0..4) {
                0 => Strategy::Hoarder,
                1 => Strategy::Flipper,
                2 => Strategy::JustInTime,
                _ => Strategy::Panic,
            };
            agents.push(Agent::new(i, strategy));
        }

        Self {
            heap: Heap::new(capacity),
            agents,
            history: VecDeque::from(vec![1.0; 100]), // Initial history
            current_price: 1.0,
            tick_count: 0,
            last_volume: 0,
            last_turnover: 0.0,
        }
    }

    pub fn tick(&mut self) {
        self.tick_count += 1;

        // 1. Free expired blocks
        self.heap.free_expired();
        self.heap.defrag();

        // 2. Agents decide (Parallel)
        let fragmentation = self.heap.fragmentation();
        let current_price = self.current_price;

        let orders: Vec<Order> = self.agents.par_iter()
            .filter_map(|a| a.decide(current_price, fragmentation))
            .collect();

        // 3. Process Orders (Sequential)
        // Sort by price (highest bid first)
        let mut sorted_orders = orders;
        sorted_orders.sort_by(|a, b| {
            match (a, b) {
                (Order::Bid { price: p1, .. }, Order::Bid { price: p2, .. }) => p2.partial_cmp(p1).unwrap_or(std::cmp::Ordering::Equal),
            }
        });

        let mut successful_transactions = 0;
        let mut total_transaction_value = 0.0;

        for order in sorted_orders {
            match order {
                Order::Bid { agent_id, size, price, duration } => {
                    if self.heap.allocate(size, agent_id, price, duration) {
                        if let Some(agent) = self.agents.get_mut(agent_id) {
                            agent.budget -= price * size as f64; // Price is per unit? Or total? Let's say price is per unit.
                            agent.owned_blocks += 1;
                        }
                        successful_transactions += 1;
                        total_transaction_value += price;
                    }
                }
            }
        }

        self.last_volume = successful_transactions;
        self.last_turnover = total_transaction_value;

        // 4. Update Market Price
        // Simple supply/demand logic
        // If demand (transactions) is high, price goes up.
        // If demand is low (lots of failed bids due to price? or just no bids?), price goes down.
        // Or based on free space.

        let utilization = 1.0 - (self.heap.blocks.iter().filter(|b| b.owner.is_none()).map(|b| b.size).sum::<usize>() as f64 / self.heap.capacity as f64);

        // Price adjustment
        if utilization > 0.9 {
            self.current_price *= 1.05;
        } else if utilization < 0.5 {
            self.current_price *= 0.95;
        }

        // Clamp price
        if self.current_price < 0.1 { self.current_price = 0.1; }
        if self.current_price > 100.0 { self.current_price = 100.0; }

        self.history.push_back(self.current_price);
        if self.history.len() > 100 {
            self.history.pop_front();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allocation() {
        let mut heap = Heap::new(100);
        assert!(heap.allocate(50, 1, 10.0, 10));
        assert_eq!(heap.blocks.len(), 2); // 1 allocated, 1 free remainder
        assert_eq!(heap.blocks[0].size, 50);
        assert_eq!(heap.blocks[0].owner, Some(1));
        assert_eq!(heap.blocks[1].size, 50);
        assert_eq!(heap.blocks[1].owner, None);

        assert!(heap.allocate(50, 2, 10.0, 10));
        assert_eq!(heap.blocks.len(), 2); // Both allocated
        assert_eq!(heap.blocks[1].owner, Some(2));

        assert!(!heap.allocate(10, 3, 10.0, 10)); // Full
    }

    #[test]
    fn test_defrag() {
        let mut heap = Heap::new(100);
        heap.allocate(30, 1, 10.0, 10);
        heap.allocate(30, 2, 10.0, 10);
        heap.allocate(30, 3, 10.0, 10);
        // [30, 30, 30, 10]

        // Free middle
        heap.blocks[1].owner = None;
        heap.defrag();
        assert_eq!(heap.blocks.len(), 4); // No merge possible yet

        // Free last allocated
        heap.blocks[2].owner = None;
        heap.defrag();
        // Should merge 2nd (30), 3rd (30), and 4th (10) -> 70
        // Blocks: [30 (owned), 70 (free)]
        assert_eq!(heap.blocks.len(), 2);
        assert_eq!(heap.blocks[1].size, 70);
    }

    #[test]
    fn test_market_tick() {
        let mut market = Market::new(100, 5);
        market.tick();
        // Just ensure it doesn't panic
        assert_eq!(market.tick_count, 1);
    }
}
