use serde::{Deserialize, Serialize};

use super::purchasable::Purchasable;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quotation {
    pub id: String,
    pub products: Vec<Purchasable>,
}

impl Quotation {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            products: Vec::new(),
        }
    }
    pub fn add_item(&mut self, p: Purchasable) {
        self.products.push(p)
    }
    pub fn total_price(&self) -> f32 {
        self.products
            .iter()
            .map(|pur: &Purchasable| pur.total_price())
            .sum()
    }
    pub fn total_discounted(&self) -> f32 {
        self.products
            .iter()
            .map(|pur: &Purchasable| pur.total_discounted())
            .sum()
    }
    pub fn apply_final_discount(&self, discount: Option<f32>) -> f32 {
        let total: f32 = self
            .products
            .iter()
            .map(|pur: &Purchasable| pur.total_discounted())
            .sum();
        match discount {
            Some(v) => total * (1.0 - v),
            None => total,
        }
    }
    pub fn total_quantity(&self) -> f32 {
        self.products
            .iter()
            .map(|pur| pur.total_package_quantity())
            .sum()
    }
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or("{}".to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::model::product::LenghtType;
    use crate::model::product::Product;
    use crate::model::product::Um;
    use crate::model::purchasable::Purchasable;

    use super::Quotation;

    fn mock_quotations() -> Quotation {
        // 20
        let p = Product::new(
            "0",
            "piastrelle di ceramica",
            10.0,
            Um::Lenght((LenghtType::Centimeter, 10.0)),
            2.0,
        );
        // 100
        let p2 = Product::new(
            "1",
            "box doccia",
            1.0,
            Um::Area((LenghtType::Centimeter, (90.0, 90.0))),
            100.0,
        );
        // 100
        let p3 = Product::new(
            "2",
            "rubinetti vintage",
            4.0,
            Um::Footprint((LenghtType::Centimeter, (10.0, 10.0, 10.0))),
            25.0,
        );
        //32 = 40 * 0.8
        let pu: Purchasable = Purchasable::new(p, 5, 2, Some(0.2));
        //80 = 100 * 0.8
        let pu2: Purchasable = Purchasable::new(p2, 5, 1, Some(0.2));
        //160 = 200 * 0.8
        let pu3: Purchasable = Purchasable::new(p3, 5, 2, Some(0.2));
        let mut quo: Quotation = Quotation::new("0");
        quo.add_item(pu);
        quo.add_item(pu2);
        quo.add_item(pu3);
        quo
    }

    #[test]
    fn quotation_total_price() {
        let q = mock_quotations();
        assert_eq!(q.total_price(), 340.0);
    }
    #[test]
    fn quotation_total_discounted() {
        let q = mock_quotations();
        assert_eq!(q.total_discounted(), 272.0);
    }
    #[test]
    fn quotation_final_total_discount() {
        let q = mock_quotations();
        assert_eq!(q.apply_final_discount(Some(0.5)), 136.0);
    }
    #[test]
    fn quotation_total_quantity() {
        let q = mock_quotations();
        assert_eq!(q.total_quantity(), 29.0);
    }
    #[test]
    fn test_to_json() {
        let q = mock_quotations();
        assert_eq!(
            q.to_json(),
            "{\"id\":\"0\",\"products\":[{\"product\":{\"id\":\"0\",\"name\":\"piastrelle di ceramica\",\"package_quantity\":10.0,\"um\":{\"Lenght\":[\"Centimeter\",10.0]},\"unit_price\":2.0},\"lead_time\":5,\"required_amount\":2,\"discount\":0.2},{\"product\":{\"id\":\"1\",\"name\":\"box doccia\",\"package_quantity\":1.0,\"um\":{\"Area\":[\"Centimeter\",[90.0,90.0]]},\"unit_price\":100.0},\"lead_time\":5,\"required_amount\":1,\"discount\":0.2},{\"product\":{\"id\":\"2\",\"name\":\"rubinetti vintage\",\"package_quantity\":4.0,\"um\":{\"Footprint\":[\"Centimeter\",[10.0,10.0,10.0]]},\"unit_price\":25.0},\"lead_time\":5,\"required_amount\":2,\"discount\":0.2}]}"
        );
    }
}
