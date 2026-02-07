use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Convoy {
    pub id: String,
    pub name: String,
    pub tasks: Vec<String>, // Bead IDs
}
