use serde::{Deserialize, Serialize};

use super::{product::Product, rule::Rule};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Purchasable {
    pub product: Product,
    pub lead_time: u32,
    pub required_amount: u32,
    pub rules: Vec<Rule>,
    pub discount: Option<f64>,
}

impl Purchasable {
    pub fn new(
        product: Product,
        lead_time: u32,
        required_amount: u32,
        discount: Option<f64>,
        rules: Option<Vec<Rule>>,
    ) -> Self {
        Purchasable {
            product,
            lead_time,
            required_amount,
            discount,
            rules: rules.unwrap_or_default(),
        }
    }
    ///required_amount * product.package_quantity
    pub fn total_package_quantity(&self) -> f64 {
        self.required_amount as f64 * self.product.package_quantity
    }

    pub fn total_price(&self) -> f64 {
        self.required_amount as f64 * self.product.package_price()
    }

    pub fn total_discounted(&self) -> f64 {
        let total_price = self.total_price();
        match self.discount {
            Some(d) => total_price * (1.0 - d),
            None => total_price,
        }
    }
    pub fn apply_rule(&mut self, rule: Rule) {
        self.rules.push(rule)
    }
    pub fn validate(&self) -> Result<(), String> {
        let mut validators = Vec::new();
        for rule in self.rules.clone() {
            match rule {
                Rule::MinPrice(price) => {
                    if self.product.package_price() <= price {
                        validators.push("MinPrice not exausted".to_string())
                    }
                }
                Rule::MaxPrice(price) => {
                    if self.product.package_price() >= price {
                        validators.push("MaxPrice not exausted".to_string())
                    }
                }
                Rule::MinQuantity(q) => {
                    if self.total_package_quantity() <= q as f64 {
                        validators.push("MinQuantity not exausted".to_string())
                    }
                }
                Rule::MaxQuantity(q) => {
                    if self.total_package_quantity() >= q as f64 {
                        validators.push("MaxQuantity not exausted".to_string())
                    }
                }
            }
        }
        if validators.len() == 0 {
            Ok(())
        } else {
            Err(validators.join("\n"))
        }
    }
}

#[cfg(test)]
mod purchasable_test {
    use crate::model::{product::Product, purchasable::Purchasable, rule::Rule};

    fn mock_purchasable() -> Purchasable {
        use crate::model::product::LenghtType;
        use crate::model::product::Um;
        let p: Product = Product::new(
            "0".into(),
            "cavi usb rame x5 Gialix".into(),
            5.0,
            Um::Lenght((LenghtType::Meter, 3.0)),
            10.0,
        );
        let pu: Purchasable = Purchasable::new(p, 5, 2, Some(0.2), None);
        pu
    }

    #[test]
    fn test_totals() {
        let pu: Purchasable = mock_purchasable();
        assert_eq!(pu.total_package_quantity(), 10.0);
        assert_eq!(pu.total_price(), 100.0);
        assert_eq!(pu.total_discounted(), 80.0);
    }

    #[test]
    fn test_min_rule() {
        let mut pu: Purchasable = mock_purchasable();
        pu.apply_rule(Rule::MinPrice(10.0));
        assert_eq!(pu.validate(), Ok(()))
    }
}
