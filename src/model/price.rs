use serde::{Deserialize, Serialize};

use crate::rules::rule::ProductRuleType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Price {
    pub price: f64,
    pub discount: f64,
    pub rules: Vec<ProductRuleType>,
}
