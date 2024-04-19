use super::product::Product;
#[derive(Debug, Clone)]
pub struct Purchasable {
    pub product: Product,
    pub lead_time: u32,
    pub quantity: u32,
    pub discount: Option<f32>,
}

impl Purchasable {
    pub fn new(product: Product, lead_time: u32, quantity: u32, discount: Option<f32>) -> Self {
        Purchasable {
            product,
            lead_time,
            quantity,
            discount,
        }
    }
    /// quantity * product.packaged_quantity
    ///
    /// Multiply needed quantity for packaged quantity
    pub fn processed_quantity(&self) -> f32 {
        self.quantity as f32 * self.product.package_quantity
    }
    /// quantity * product.packaged_quantity * product.price_list()
    ///
    /// Multiply needed quantity for packaged quantity and product.price_list()
    pub fn total_price(&self) -> f32 {
        self.processed_quantity() * self.product.price_list()
    }
    /// total_price() * 1 - discount
    ///
    /// Multiply total_price() for (1 - discount)
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
    assert_eq!(50.0, p.price_list());
    let pu: Purchasable = Purchasable::new(p, 5, 2, Some(0.2));
    assert_eq!(pu.processed_quantity(), 10.0);
    assert_eq!(pu.total_price(), 500.0);
    assert_eq!(pu.total_discounted(), 400.0);
}
