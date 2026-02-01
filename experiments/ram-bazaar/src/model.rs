use ratatui::style::Color;

pub type AgentId = usize;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Strategy {
    Greedy,
    Saver,
    Hoarder,
    Panic,
}

impl Strategy {
    pub fn color(&self) -> Color {
        match self {
            Strategy::Greedy => Color::Red,
            Strategy::Saver => Color::Green,
            Strategy::Hoarder => Color::Yellow,
            Strategy::Panic => Color::Magenta,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Agent {
    pub id: AgentId,
    pub strategy: Strategy,
    pub budget: f64,
    pub max_budget: f64,
    pub desired_memory: usize,
    pub owned_pages: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct Bid {
    pub agent_id: AgentId,
    pub price: f64,
}

impl Agent {
    pub fn new(id: AgentId, strategy: Strategy, budget: f64, desired_memory: usize) -> Self {
        Self {
            id,
            strategy,
            budget,
            max_budget: budget,
            desired_memory,
            owned_pages: 0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Page {
    pub owner: Option<AgentId>,
    pub rent: f64,
}

impl Default for Page {
    fn default() -> Self {
        Self {
            owner: None,
            rent: 0.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MarketState {
    pub pages: Vec<Page>,
    pub width: usize,
    pub height: usize,
    pub current_price: f64,
    pub price_history: Vec<f64>,
}

impl MarketState {
    pub fn new(width: usize, height: usize) -> Self {
        let pages = vec![Page::default(); width * height];
        Self {
            pages,
            width,
            height,
            current_price: 10.0, // Base price
            price_history: Vec::new(),
        }
    }

    pub fn total_pages(&self) -> usize {
        self.width * self.height
    }
}
