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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum TimeType {
    Second,
    Minute,
    Hour,
    Day,
    Month,
    Year,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Um {
    Lenght((LenghtType, f64)),
    Area((LenghtType, (f64, f64))),
    Footprint((LenghtType, (f64, f64, f64))),
    Volume((VolumeType, (f64, f64, f64))),
    Mass((MassType, f64)),
    Time(TimeType, f64),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub package_quantity: f64,
    pub um: Um,
    pub unit_price: f64,
}

impl Product {
    pub fn new(id: &str, name: &str, package_quantity: f64, um: Um, unit_price: f64) -> Self {
        Product {
            id: id.into(),
            name: name.into(),
            package_quantity,
            um,
            unit_price,
        }
    }
    pub fn package_price(&self) -> f64 {
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
