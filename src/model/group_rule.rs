use std::fmt::Debug;

use super::{quotation::Quotation, validator::Validator};
pub trait GroupRule: Debug + Clone {
    fn apply(&self, quotation: &Quotation) -> Validator;
}

#[derive(Debug, Clone)]
pub struct MinProductNumber {
    value: u32,
}

impl From<u32> for MinProductNumber {
    fn from(value: u32) -> Self {
        Self { value }
    }
}

impl GroupRule for MinProductNumber {
    fn apply(&self, quotation: &Quotation) -> Validator {
        if quotation.purchasables.len() < self.value as usize {
            return Validator::from(format!(
                "MinProductNumber[{}]: Expect({}), Got({})",
                quotation.id,
                self.value,
                quotation.purchasables.len()
            ));
        }
        Validator::new()
    }
}

#[derive(Debug, Clone)]
pub struct MaxProductNumber {
    value: u32,
}

impl From<u32> for MaxProductNumber {
    fn from(value: u32) -> Self {
        Self { value }
    }
}

impl GroupRule for MaxProductNumber {
    fn apply(&self, quotation: &Quotation) -> Validator {
        if quotation.purchasables.len() > self.value as usize {
            return Validator::from(format!(
                "MaxProductNumber[{}]: Expect({}), Got({})",
                quotation.id,
                self.value,
                quotation.purchasables.len()
            ));
        }
        Validator::new()
    }
}

#[derive(Debug, Clone)]
pub enum QuotationRule {
    MinProductNumber(MinProductNumber),
    MaxProductNumber(MaxProductNumber),
}

impl GroupRule for QuotationRule {
    fn apply(&self, quotation: &Quotation) -> Validator {
        match self {
            QuotationRule::MinProductNumber(r) => r.apply(quotation),
            QuotationRule::MaxProductNumber(r) => r.apply(quotation),
        }
    }
}
