use serde::{Deserialize, Serialize};

use super::{purchasable::Purchasable, validator::Validator};

pub trait Rule: std::fmt::Debug + Clone {
    fn apply(&self, purchasable: &Purchasable) -> Validator;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaxPrice {
    value: f64,
}

impl From<f64> for MaxPrice {
    fn from(value: f64) -> Self {
        Self { value }
    }
}

impl Rule for MaxPrice {
    fn apply(&self, purchasable: &Purchasable) -> Validator {
        if purchasable.price.price > self.value {
            return Validator::from(format!(
                "MaxPrice[{}]: Expect({}), Got({})",
                purchasable.product.id, self.value, purchasable.price.price
            ));
        }
        Validator::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinPrice {
    value: f64,
}

impl From<f64> for MinPrice {
    fn from(value: f64) -> Self {
        Self { value }
    }
}

impl Rule for MinPrice {
    fn apply(&self, purchasable: &Purchasable) -> Validator {
        if purchasable.price.price < self.value {
            return Validator::from(format!(
                "MinPrice[{}]: Expect({}), Got({})",
                purchasable.product.id, self.value, purchasable.price.price
            ));
        }
        Validator::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaxQuantity {
    value: u32,
}

impl From<u32> for MaxQuantity {
    fn from(value: u32) -> Self {
        Self { value }
    }
}

impl Rule for MaxQuantity {
    fn apply(&self, purchasable: &Purchasable) -> Validator {
        if purchasable.required_amount > self.value {
            return Validator::from(format!(
                "MaxQuantity[{}]: Expect({}), Got({})",
                purchasable.product.id, self.value, purchasable.required_amount
            ));
        }
        Validator::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinQuantity {
    value: u32,
}

impl From<u32> for MinQuantity {
    fn from(value: u32) -> Self {
        Self { value }
    }
}

impl Rule for MinQuantity {
    fn apply(&self, purchasable: &Purchasable) -> Validator {
        if purchasable.required_amount < self.value {
            return Validator::from(format!(
                "MinQuantity[{}]: Expect({}), Got({})",
                purchasable.product.id, self.value, purchasable.required_amount
            ));
        }
        Validator::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProductRule {
    MaxPrice(MaxPrice),
    MaxQuantity(MaxQuantity),
    MinPrice(MinPrice),
    MinQuantity(MinQuantity),
}

impl Rule for ProductRule {
    fn apply(&self, purchasable: &Purchasable) -> Validator {
        match self {
            ProductRule::MaxPrice(rule) => rule.apply(purchasable),
            ProductRule::MaxQuantity(rule) => rule.apply(purchasable),
            ProductRule::MinPrice(rule) => rule.apply(purchasable),
            ProductRule::MinQuantity(rule) => rule.apply(purchasable),
        }
    }
}
