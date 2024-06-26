use super::{
    product::Product,
    product_rule::{ProductRule, Rule},
    validator::Validator,
};
#[derive(Debug, Clone)]
pub struct Purchasable {
    pub product: Product,
    pub required_amount: u32,
    pub rules: Vec<ProductRule>,
}

impl Purchasable {
    pub fn new(product: Product, required_amount: u32, rules: Vec<ProductRule>) -> Self {
        Purchasable {
            product,
            required_amount,
            rules,
        }
    }
    ///add validation rule to product
    pub fn add_rule(&mut self, rule: &ProductRule) {
        self.rules.push(rule.clone())
    }
    ///required_amount * product.package_quantity
    pub fn total_package_quantity(&self) -> f64 {
        self.required_amount as f64 * self.product.package_quantity
    }
    ///required_amount * price.price
    pub fn total_price(&self) -> f64 {
        self.required_amount as f64 * self.product.price.price
    }
    ///total_price() * (1.0 - self.price.discount)
    pub fn total_discounted(&self) -> f64 {
        let total_price = self.total_price();
        total_price * (1.0 - self.product.price.discount)
    }
    pub fn validate(&self) -> Validator {
        let mut base = Validator::new();
        for rule in &self.rules {
            let validator = rule.apply(&self);
            base.concat(&validator)
        }
        base
    }
}

impl From<Product> for Purchasable {
    fn from(value: Product) -> Self {
        Self {
            product: value,
            required_amount: 1,
            rules: Vec::new(),
        }
    }
}
impl From<(Product, u32)> for Purchasable {
    fn from(value: (Product, u32)) -> Self {
        Self {
            product: value.0,
            required_amount: value.1,
            rules: Vec::new(),
        }
    }
}
#[cfg(test)]
mod purchasable_test {
    use crate::{
        model::{
            product_rule::{MaxQuantity, MinQuantity, ProductRule},
            purchasable::Purchasable,
        },
        utils::mock::mock_purchasable,
    };

    #[test]
    fn test_totals() {
        let pu: Purchasable = mock_purchasable();
        assert_eq!(pu.total_package_quantity(), 10.0);
        assert_eq!(pu.total_price(), 20.0);
        assert_eq!(pu.total_discounted(), 10.0);
    }
    #[test]
    fn test_validate_success() {
        let mut pu = mock_purchasable();
        pu.add_rule(&ProductRule::MinQuantity(MinQuantity::from(1)));
        pu.add_rule(&ProductRule::MaxQuantity(MaxQuantity::from(3)));
        let v = pu.validate();
        assert_eq!(v.is_valid(), true)
    }

    #[test]
    fn test_validate_fail() {
        let mut pu = mock_purchasable();
        pu.add_rule(&ProductRule::MinQuantity(MinQuantity::from(3)));
        pu.add_rule(&ProductRule::MaxQuantity(MaxQuantity::from(1)));
        let v = pu.validate();
        assert_eq!(v.is_valid(), false);
    }
}
