use super::{
    product::{LenghtType, Product, Um},
    purchasable::Purchasable,
};

#[derive(Debug, Clone)]
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
        self.products.push(p.clone())
    }
    pub fn total(&self) -> f32 {
        self.products.iter().map(|pur| pur.total_price()).sum()
    }
    pub fn total_discounted(&self) -> f32 {
        self.products.iter().map(|pur| pur.total_discounted()).sum()
    }
    pub fn apply_final_discount(&self, discount: Option<f32>) -> f32 {
        let total = self.products.iter().map(|pur| pur.total_discounted()).sum();
        match discount {
            Some(v) => total * (1.0 - v),
            None => total,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*; // Importa le funzioni e le strutture dalla mod principale

    fn mock_quotations() -> Quotation {
        let p = Product::new(
            "0",
            "piastrelle di ceramica",
            10.0,
            Um::Lenght((LenghtType::Centimeter, 10.0)),
            2.0,
        );
        let pu: Purchasable = Purchasable::new(p, 5, 2, Some(0.2));
        let mut quo: Quotation = Quotation::new("0");
        quo.add_item(pu.clone());
        quo.add_item(pu.clone());
        quo.add_item(pu.clone());
        quo
    }

    #[test]
    fn quotation_total() {
        let q = mock_quotations();
        assert_eq!(q.total(), 1200.0);
        assert_eq!(q.total_discounted(), 960.0);
    }
}
