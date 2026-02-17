use rand::Rng;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum MarketPhase {
    Innovation,
    Peak,
    Trough,
    Slope,
    Plateau,
}

#[derive(Debug, Clone)]
pub struct Topic {
    pub name: String,
    pub intrinsic_value: f64,
    pub market_price: f64,
    pub history: Vec<f64>,
    pub phase: MarketPhase,
}

impl Topic {
    pub fn new(name: &str, intrinsic_value: f64) -> Self {
        Self {
            name: name.to_string(),
            intrinsic_value,
            market_price: 1.0,
            history: vec![1.0],
            phase: MarketPhase::Innovation,
        }
    }

    pub fn update_price(&mut self, total_attention: f64) {
        // Price Model: Base + Attention * Multiplier
        // We add some noise to make it organic
        let noise = rand::thread_rng().gen_range(-0.5..0.5);
        self.market_price = (1.0 + total_attention * 0.1 + noise).max(0.1);
        self.history.push(self.market_price);

        if self.history.len() > 100 {
            self.history.remove(0);
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Strategy {
    TrendFollower,
    ValueInvestor,
    HypeBeast,
    Contrarian,
    Random,
}

#[derive(Debug, Clone)]
pub struct Agent {
    pub name: String,
    pub portfolio: HashMap<String, f64>,
    pub cash: f64,
    pub strategy: Strategy,
}

impl Agent {
    pub fn new(name: &str, strategy: Strategy, cash: f64) -> Self {
        Self {
            name: name.to_string(),
            portfolio: HashMap::new(),
            cash,
            strategy,
        }
    }

    pub fn allocate(&mut self, topic_name: &str, amount: f64) {
        if self.cash >= amount {
            self.cash -= amount;
            *self.portfolio.entry(topic_name.to_string()).or_insert(0.0) += amount;
        }
    }

    pub fn withdraw(&mut self, topic_name: &str, amount: f64) {
        if let Some(allocation) = self.portfolio.get_mut(topic_name) {
            if *allocation >= amount {
                *allocation -= amount;
                self.cash += amount;
            } else {
                self.cash += *allocation;
                *allocation = 0.0;
            }
        }
    }

    pub fn decide(&mut self, topics: &[Topic]) {
        let mut rng = rand::thread_rng();

        for topic in topics {
            let current_allocation = *self.portfolio.get(&topic.name).unwrap_or(&0.0);

            match self.strategy {
                Strategy::Random => {
                    if rng.gen_bool(0.1) {
                        if rng.gen_bool(0.5) {
                            self.allocate(&topic.name, 10.0);
                        } else {
                            self.withdraw(&topic.name, 10.0);
                        }
                    }
                }
                Strategy::ValueInvestor => {
                    // Buy if Price < Intrinsic. Sell if Price > Intrinsic.
                    if topic.market_price < topic.intrinsic_value * 0.8 {
                        // Undervalued -> Buy
                        self.allocate(&topic.name, self.cash * 0.1);
                    } else if topic.market_price > topic.intrinsic_value * 1.2 {
                        // Overvalued -> Sell
                        self.withdraw(&topic.name, current_allocation * 0.5);
                    }
                }
                Strategy::TrendFollower => {
                    // Look at last 2 ticks
                    if topic.history.len() >= 2 {
                        let last = topic.history[topic.history.len() - 1];
                        let prev = topic.history[topic.history.len() - 2];
                        if last > prev {
                            // Trend UP -> Buy
                            self.allocate(&topic.name, self.cash * 0.1);
                        } else {
                            // Trend DOWN -> Sell
                            self.withdraw(&topic.name, current_allocation * 0.1);
                        }
                    }
                }
                Strategy::HypeBeast => {
                    // Buy if price is high (Momentum)
                    if topic.market_price > 50.0 {
                        self.allocate(&topic.name, self.cash * 0.2);
                    } else {
                        self.withdraw(&topic.name, current_allocation * 0.05);
                    }
                }
                Strategy::Contrarian => {
                    // Inverse of Trend
                    if topic.history.len() >= 2 {
                        let last = topic.history[topic.history.len() - 1];
                        let prev = topic.history[topic.history.len() - 2];
                        if last > prev {
                            // Trend UP -> Sell
                            self.withdraw(&topic.name, current_allocation * 0.2);
                        } else {
                            // Trend DOWN -> Buy
                            self.allocate(&topic.name, self.cash * 0.1);
                        }
                    }
                }
            }
        }
    }
}

pub struct Market {
    pub topics: Vec<Topic>,
    pub agents: Vec<Agent>,
    pub tick: u64,
}

impl Market {
    pub fn new(topics: Vec<Topic>, agents: Vec<Agent>) -> Self {
        Self {
            topics,
            agents,
            tick: 0,
        }
    }

    pub fn update(&mut self) {
        self.tick += 1;

        // 1. Agents Decide (Clone topics to avoid borrow checker hell,
        //    or just iterate indices. Since structs are small, cloning topics for observation is okay for now).
        //    Actually, we can pass a slice if we iterate agents separately.
        let topics_snapshot = self.topics.clone();

        // Use rayon later if performance is an issue, for now sequential is fine for <1000 agents
        for agent in &mut self.agents {
            agent.decide(&topics_snapshot);
        }

        // 2. Calculate Total Attention
        let mut topic_attention: HashMap<String, f64> = HashMap::new();
        for agent in &self.agents {
            for (topic, amount) in &agent.portfolio {
                *topic_attention.entry(topic.clone()).or_insert(0.0) += *amount;
            }
        }

        // 3. Update Prices
        for topic in &mut self.topics {
            let attention = topic_attention.get(&topic.name).cloned().unwrap_or(0.0);
            topic.update_price(attention);

            // Random Walk Intrinsic Value (Concept drift)
            let drift = rand::thread_rng().gen_range(-1.0..1.0);
            topic.intrinsic_value = (topic.intrinsic_value + drift).max(1.0);
        }
    }

    pub fn get_topic(&self, name: &str) -> Option<&Topic> {
        self.topics.iter().find(|t| t.name == name)
    }
}
