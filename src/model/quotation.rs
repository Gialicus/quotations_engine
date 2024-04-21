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
    use crate::model::product::Product;
    use crate::model::purchasable::Purchasable;
    use crate::model::unit::{Area, Footprint, LenghtType, UnitOfMeasure};

    use super::Quotation;

    fn mock_quotations() -> Quotation {
        // 4
        let p = Product::new(
            "0",
            "piastrelle di ceramica",
            10.0,
            UnitOfMeasure::Area(Area {uom:LenghtType::Centimeter, x: 10.0 , y: 10.0 }),
            2.0,
        );
        // 100
        let p2 = Product::new(
            "1",
            "box doccia",
            1.0,
            UnitOfMeasure::Area(Area {uom:LenghtType::Centimeter, x: 100.0 , y: 100.0 }),
            100.0,
        );
        // 100
        let p3 = Product::new(
            "2",
            "rubinetti vintage",
            4.0,
            UnitOfMeasure::Footprint(Footprint {uom:LenghtType::Centimeter, x: 10.0, y: 25.0, z:5.0}),
            25.0,
        );
        //3.2 = 4.0 * 0.8
        let pu: Purchasable = Purchasable::new(p, 5, 2, Some(0.2), None);
        //80 = 100 * 0.8
        let pu2: Purchasable = Purchasable::new(p2, 5, 1, Some(0.2), None);
        //80 = 100 * 0.8
        let pu3: Purchasable = Purchasable::new(p3, 5, 4, Some(0.2), None);
        let mut quo: Quotation = Quotation::new("0");
        quo.add_item(pu);
        quo.add_item(pu2);
        quo.add_item(pu3);
        quo
    }

    #[test]
    fn quotation_total_price() {
        let q = mock_quotations();
        assert_eq!(q.total_price(), 204.0);
    }
    #[test]
    fn quotation_total_discounted() {
        let q = mock_quotations();
        assert_eq!(q.total_discounted(), 163.2);
    }
    #[test]
    fn quotation_final_total_discount() {
        let q = mock_quotations();
        assert_eq!(q.apply_final_discount(Some(0.5)), 81.6);
    }
    #[test]
    fn quotation_total_quantity() {
        let q = mock_quotations();
        assert_eq!(q.total_quantity(), 37.0);
    }
    #[test]
    fn test_to_json() {
        let q = mock_quotations();
        let json = 
        "{\"id\":\"0\",\"purchasables\":[{\"product\":{\"id\":\"0\",\"name\":\"piastrelle di ceramica\",\"package_quantity\":10.0,\"um\":{\"Area\":{\"uom\":\"Centimeter\",\"x\":10.0,\"y\":10.0}},\"price\":2.0},\"lead_time\":5,\"required_amount\":2,\"rules\":[],\"discount\":0.2},{\"product\":{\"id\":\"1\",\"name\":\"box doccia\",\"package_quantity\":1.0,\"um\":{\"Area\":{\"uom\":\"Centimeter\",\"x\":100.0,\"y\":100.0}},\"price\":100.0},\"lead_time\":5,\"required_amount\":1,\"rules\":[],\"discount\":0.2},{\"product\":{\"id\":\"2\",\"name\":\"rubinetti vintage\",\"package_quantity\":4.0,\"um\":{\"Footprint\":{\"uom\":\"Centimeter\",\"x\":10.0,\"y\":25.0,\"z\":5.0}},\"price\":25.0},\"lead_time\":5,\"required_amount\":4,\"rules\":[],\"discount\":0.2}]}";
        assert_eq!(
            q.to_json(),
            json
        );
    }
}
