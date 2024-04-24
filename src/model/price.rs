use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Price {
    pub price: f64,
    pub discount: f64,
    pub lead_time: u32,
}
impl Price {
    pub fn new(price: f64, discount: f64, lead_time: u32) -> Self {
        Self {
            price,
            discount,
            lead_time,
        }
    }
}
