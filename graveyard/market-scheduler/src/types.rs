use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Side {
    Bid, // Buyer
    Ask, // Seller
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Order {
    pub id: Uuid,
    pub side: Side,
    pub price: f64,
    pub quantity: u32, // Number of ticks/cycles
    pub owner_id: Uuid,
    pub timestamp: u64, // Logic tick
}

#[derive(Debug, Clone, Copy)]
pub struct Transaction {
    pub price: f64,
    pub quantity: u32,
    pub buyer_id: Uuid,
    pub seller_id: Uuid,
    pub timestamp: u64,
}

impl Order {
    pub fn new(side: Side, price: f64, quantity: u32, owner_id: Uuid, timestamp: u64) -> Self {
        Self {
            id: Uuid::new_v4(),
            side,
            price,
            quantity,
            owner_id,
            timestamp,
        }
    }
}
