use serde::{Deserialize, Serialize};

use super::product::Product;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Purchasable {
    pub product: Product,
    pub lead_time: u32,
    pub required_amount: u32,
    pub discount: Option<f64>,
}

impl Purchasable {
    pub fn new(
        product: Product,
        lead_time: u32,
        required_amount: u32,
        discount: Option<f64>,
    ) -> Self {
        Purchasable {
            product,
            lead_time,
            required_amount,
            discount,
        }
    }
    ///required_amount * product.package_quantity
    pub fn total_package_quantity(&self) -> f64 {
        self.required_amount as f64 * self.product.package_quantity
    }

    pub fn total_price(&self) -> f64 {
        self.required_amount as f64 * self.product.price
    }

    pub fn total_discounted(&self) -> f64 {
        let total_price = self.total_price();
        match self.discount {
            Some(d) => total_price * (1.0 - d),
            None => total_price,
        }
    }
}

#[cfg(test)]
mod purchasable_test {
    use crate::model::{
        product::Product,
        purchasable::Purchasable,
        unit::{Lenght, LenghtType, UnitOfMeasure},
    };

    fn mock_purchasable() -> Purchasable {
        let p: Product = Product::new(
            "0".into(),
            "cavi usb rame x5 Gialix".into(),
            5.0,
            UnitOfMeasure::Lenght(Lenght::new(LenghtType::Centimeter, 15.0)),
            10.0,
        );
        let pu: Purchasable = Purchasable::new(p, 5, 2, Some(0.2));
        pu
    }

    #[test]
    fn test_totals() {
        let pu: Purchasable = mock_purchasable();
        assert_eq!(pu.total_package_quantity(), 10.0);
        assert_eq!(pu.total_price(), 20.0);
        assert_eq!(pu.total_discounted(), 16.0);
    }
}
