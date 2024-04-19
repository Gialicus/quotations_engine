use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Rule {
    MinPrice(f64),
    MaxPrice(f64),
    MinQuantity(u32),
    MaxQuantity(u32),
}
