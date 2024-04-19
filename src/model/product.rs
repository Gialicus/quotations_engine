use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum LenghtType {
    Meter,
    Centimeter,
    Millimeter,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum VolumeType {
    Liter,
    Centiliter,
    Milliliter,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum MassType {
    Kilogram,
    Gram,
    Centigram,
    Milligram,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Um {
    Lenght((LenghtType, f32)),
    Area((LenghtType, (f32, f32))),
    Footprint((LenghtType, (f32, f32, f32))),
    Volume((VolumeType, (f32, f32, f32))),
    Mass((MassType, f32)),
    Time,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub package_quantity: f32,
    pub um: Um,
    pub unit_price: f32,
}

impl Product {
    pub fn new(id: &str, name: &str, package_quantity: f32, um: Um, unit_price: f32) -> Self {
        Product {
            id: id.into(),
            name: name.into(),
            package_quantity,
            um,
            unit_price,
        }
    }
    pub fn package_price(&self) -> f32 {
        self.unit_price * self.package_quantity
    }
}

#[test]
fn create_product() {
    let p: Product = Product::new(
        "0".into(),
        "cavi usb rame x5 Gialix".into(),
        5.0,
        Um::Lenght((LenghtType::Meter, 3.0)),
        10.0,
    );
    assert_eq!("0", p.id);
    assert_eq!(50.0, p.package_price())
}
