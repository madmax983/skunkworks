#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AgentId(pub usize);

#[derive(Debug, Clone, PartialEq)]
pub enum OrderType {
    Bid, // Buy
    Ask, // Sell
}

#[derive(Debug, Clone, PartialEq)]
pub struct Order {
    pub agent_id: AgentId,
    pub order_type: OrderType,
    pub price: u64,
    pub quantity: usize,
}

#[derive(Debug, Clone)]
pub struct Agent {
    pub id: AgentId,
    pub budget: u64,
    pub strategy: Strategy,
}

#[derive(Debug, Clone)]
pub enum Strategy {
    Saver,
    Spender,
    Hoarder,
    Whale,      // Buys everything
    PanicSeller, // Sells everything
}

#[derive(Debug, Clone)]
pub struct MemoryBlock {
    pub id: usize,
    pub size: usize,
    pub owner: Option<AgentId>,
}

#[derive(Debug, Clone)]
pub struct Transaction {
    pub buyer_id: AgentId,
    pub seller_id: AgentId,
    pub price: u64,
    pub quantity: usize,
}
