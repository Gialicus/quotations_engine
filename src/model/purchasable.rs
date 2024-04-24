use serde::{Deserialize, Serialize};

use super::{price::Price, product::Product};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Purchasable {
    pub product: Product,
    pub price: Price,
    pub required_amount: u32,
}

impl Purchasable {
    pub fn new(product: Product, price: Price, required_amount: u32) -> Self {
        Purchasable {
            product,
            price,
            required_amount,
        }
    }
    ///required_amount * product.package_quantity
    pub fn total_package_quantity(&self) -> f64 {
        self.required_amount as f64 * self.product.package_quantity
    }

    pub fn total_price(&self) -> f64 {
        self.required_amount as f64 * self.price.price
    }

    pub fn total_discounted(&self) -> f64 {
        let total_price = self.total_price();
        total_price * (1.0 - self.price.discount)
    }
}

#[cfg(test)]
mod purchasable_test {
    use crate::{model::purchasable::Purchasable, utils::mock::mock_purchasable};

    #[test]
    fn test_totals() {
        let pu: Purchasable = mock_purchasable();
        assert_eq!(pu.total_package_quantity(), 10.0);
        assert_eq!(pu.total_price(), 20.0);
        assert_eq!(pu.total_discounted(), 10.0);
    }
}
