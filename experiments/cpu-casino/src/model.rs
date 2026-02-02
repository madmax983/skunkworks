use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Strategy {
    HighFreq,  // Bids slightly above market, frequent
    Sniper,    // Bids high when deadline is near
    Desperate, // Bids exponentially based on urgency
    Value,     // Bids based on remaining budget vs work
}

impl fmt::Display for Strategy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
pub struct ThreadAgent {
    pub id: usize,
    pub strategy: Strategy,
    pub credits: f64,
    pub work_total: f64,
    pub work_remaining: f64,
    pub deadline: u64, // In ticks
    pub executed_ticks: u64,
    pub state: AgentState,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AgentState {
    Active,
    Running, // Currently executing
    Finished,
    Killed, // Missed deadline
}

#[derive(Debug, Clone)]
pub struct Core {
    pub id: usize,
    pub current_agent_id: Option<usize>,
    pub utilization: f64, // Rolling average
}

#[derive(Debug, Clone)]
pub struct MarketState {
    pub current_tick: u64,
    pub current_price: f64,
    pub price_history: Vec<f64>,
    pub active_thread_count: usize,
    pub killed_count: usize,
    pub finished_count: usize,
}

impl Default for MarketState {
    fn default() -> Self {
        Self::new()
    }
}

impl MarketState {
    pub fn new() -> Self {
        Self {
            current_tick: 0,
            current_price: 1.0,
            price_history: Vec::new(),
            active_thread_count: 0,
            killed_count: 0,
            finished_count: 0,
        }
    }
}
