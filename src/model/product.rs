use super::unit::UnitOfMeasure;

#[derive(Debug, Clone)]
pub struct Product {
    pub name: String,
    pub description: String,
    pub package_quantity: f64,
    pub um: UnitOfMeasure,
}

impl Product {
    pub fn new(name: &str, description: &str, package_quantity: f64, um: UnitOfMeasure) -> Self {
        Product {
            name: name.into(),
            description: description.into(),
            package_quantity,
            um,
        }
    }
}
