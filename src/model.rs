use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct PearlCategory {
    pub name: String,
    pub page_count: u64,
    pub pearls: Vec<String>
}