use serde::{Deserialize, Serialize};

use super::{purchasable::Purchasable, rule::Rule};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Package {
    pub kind: String,
    pub rules: Vec<Rule>,
    pub purchasables: Vec<Purchasable>,
}

impl Package {
    pub fn new(kind: String, rules: Vec<Rule>, purchasables: Vec<Purchasable>) -> Self {
        Self {
            kind,
            rules,
            purchasables,
        }
    }
    pub fn add_purchasable(&mut self, purchasable: Purchasable) {
        self.purchasables.push(purchasable)
    }
}
