use super::{price::Price, unit::UnitOfMeasure};

#[derive(Debug, Clone)]
pub struct Product {
    pub name: String,
    pub description: String,
    pub package_quantity: f64,
    pub um: UnitOfMeasure,
    pub price: Price,
}

impl Product {
    pub fn new(
        name: &str,
        description: &str,
        package_quantity: f64,
        um: UnitOfMeasure,
        price: Price,
    ) -> Self {
        Product {
            name: name.into(),
            description: description.into(),
            package_quantity,
            um,
            price,
        }
    }
}
