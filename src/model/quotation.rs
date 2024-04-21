use serde::{Deserialize, Serialize};

use super::purchasable::Purchasable;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quotation {
    pub id: String,
    pub purchasables: Vec<Purchasable>,
}

impl Quotation {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            purchasables: Vec::new(),
        }
    }
    pub fn add_item(&mut self, p: Purchasable) {
        self.purchasables.push(p)
    }
    pub fn total_price(&self) -> f64 {
        self.purchasables
            .iter()
            .map(|pur: &Purchasable| pur.total_price())
            .sum()
    }
    pub fn total_discounted(&self) -> f64 {
        self.purchasables
            .iter()
            .map(|pur: &Purchasable| pur.total_discounted())
            .sum()
    }
    pub fn apply_final_discount(&self, discount: Option<f64>) -> f64 {
        let total = self
            .purchasables
            .iter()
            .map(|pur: &Purchasable| pur.total_discounted())
            .sum();
        match discount {
            Some(v) => total * (1.0 - v),
            None => total,
        }
    }
    pub fn total_quantity(&self) -> f64 {
        self.purchasables
            .iter()
            .map(|pur| pur.total_package_quantity())
            .sum()
    }
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or("{}".to_string())
    }
}

#[cfg(test)]
mod quotation_test {
    use crate::utils::mock::mock_quotation;

    #[test]
    fn quotation_total_price() {
        let q = mock_quotation();
        assert_eq!(q.total_price(), 204.0);
    }
    #[test]
    fn quotation_total_discounted() {
        let q = mock_quotation();
        assert_eq!(q.total_discounted(), 163.2);
    }
    #[test]
    fn quotation_final_total_discount() {
        let q = mock_quotation();
        assert_eq!(q.apply_final_discount(Some(0.5)), 81.6);
    }
    #[test]
    fn quotation_total_quantity() {
        let q = mock_quotation();
        assert_eq!(q.total_quantity(), 37.0);
    }
}
