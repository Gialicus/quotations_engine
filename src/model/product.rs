use super::unit::UnitOfMeasure;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub package_quantity: f64,
    pub um: UnitOfMeasure,
    pub price: f64,
}

impl Product {
    pub fn new(id: &str, name: &str, package_quantity: f64, um: UnitOfMeasure, price: f64) -> Self {
        Product {
            id: id.into(),
            name: name.into(),
            package_quantity,
            um,
            price,
        }
    }
    pub fn unit_price(&self) -> f64 {
        self.price / self.package_quantity
    }
}

#[test]
fn create_product() {
    use crate::model::unit::Lenght;
    use crate::model::unit::LenghtType;

    let p: Product = Product::new(
        "0".into(),
        "cavi usb rame x5 Gialix".into(),
        5.0,
        UnitOfMeasure::Lenght(Lenght {
            lenght: 30.0,
            uom: LenghtType::Centimeter,
        }),
        10.0,
    );
    assert_eq!("0", p.id);
    assert_eq!(2.0, p.unit_price())
}
