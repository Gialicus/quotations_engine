use super::unit::UnitOfMeasure;

#[derive(Debug, Clone)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub package_quantity: f64,
    pub um: UnitOfMeasure,
}

impl Product {
    pub fn new(id: &str, name: &str, package_quantity: f64, um: UnitOfMeasure) -> Self {
        Product {
            id: id.into(),
            name: name.into(),
            package_quantity,
            um,
        }
    }
}

#[test]
fn create_product() {
    use crate::utils::mock::mock_product;
    let p: Product = mock_product();
    assert_eq!("0", p.id);
}
