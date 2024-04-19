use serde::{Deserialize, Serialize};

use super::product::Product;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Purchasable {
    pub product: Product,
    pub lead_time: u32,
    pub required_amount: u32,
    pub discount: Option<f32>,
}

impl Purchasable {
    pub fn new(
        product: Product,
        lead_time: u32,
        required_amount: u32,
        discount: Option<f32>,
    ) -> Self {
        Purchasable {
            product,
            lead_time,
            required_amount,
            discount,
        }
    }
    ///required_amount * product.package_quantity
    pub fn total_package_quantity(&self) -> f32 {
        self.required_amount as f32 * self.product.package_quantity
    }

    pub fn total_price(&self) -> f32 {
        self.required_amount as f32 * self.product.package_price()
    }

    pub fn total_discounted(&self) -> f32 {
        let total_price = self.total_price();
        match self.discount {
            Some(d) => total_price * (1.0 - d),
            None => total_price,
        }
    }
}

#[test]
fn create_purchasable() {
    use crate::model::product::LenghtType;
    use crate::model::product::Um;

    let p: Product = Product::new(
        "0".into(),
        "cavi usb rame x5 Gialix".into(),
        5.0,
        Um::Lenght((LenghtType::Meter, 3.0)),
        10.0,
    );
    assert_eq!(50.0, p.package_price());
    let pu: Purchasable = Purchasable::new(p, 5, 2, Some(0.2));
    assert_eq!(pu.total_package_quantity(), 10.0);
    assert_eq!(pu.total_price(), 100.0);
    assert_eq!(pu.total_discounted(), 80.0);
}
