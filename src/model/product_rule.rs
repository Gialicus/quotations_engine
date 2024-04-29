use super::{purchasable::Purchasable, validator::Validator};

pub trait Rule: std::fmt::Debug + Clone {
    fn apply(&self, purchasable: &Purchasable) -> Validator;
}

#[derive(Debug, Clone)]
pub struct MaxPrice {
    pub value: f64,
}

impl From<f64> for MaxPrice {
    fn from(value: f64) -> Self {
        Self { value }
    }
}

impl Rule for MaxPrice {
    fn apply(&self, purchasable: &Purchasable) -> Validator {
        if purchasable.product.price.price > self.value {
            return Validator::from(format!(
                "MaxPrice: Expect({}), Got({})",
                self.value, purchasable.product.price.price
            ));
        }
        Validator::new()
    }
}

#[derive(Debug, Clone)]
pub struct MinPrice {
    pub value: f64,
}

impl From<f64> for MinPrice {
    fn from(value: f64) -> Self {
        Self { value }
    }
}

impl Rule for MinPrice {
    fn apply(&self, purchasable: &Purchasable) -> Validator {
        if purchasable.product.price.price < self.value {
            return Validator::from(format!(
                "MinPrice: Expect({}), Got({})",
                self.value, purchasable.product.price.price
            ));
        }
        Validator::new()
    }
}

#[derive(Debug, Clone)]
pub struct MaxQuantity {
    pub value: u32,
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
                "MaxQuantity: Expect({}), Got({})",
                self.value, purchasable.required_amount
            ));
        }
        Validator::new()
    }
}

#[derive(Debug, Clone)]
pub struct MinQuantity {
    pub value: u32,
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
                "MinQuantity: Expect({}), Got({})",
                self.value, purchasable.required_amount
            ));
        }
        Validator::new()
    }
}

#[derive(Debug, Clone)]
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
