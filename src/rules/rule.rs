use serde::{Deserialize, Serialize};

use crate::model::quotation::Quotation;

use super::{
    max_price::MaxPrice, max_quantity::MaxQuantity, min_price::MinPrice, min_quantity::MinQuantity,
    validators::Validators,
};

pub trait QuotationRule {
    fn apply(&self, quotation: &Quotation) -> Result<(), Validators>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QuotationRuleType {
    MaxPrice(MaxPrice),
    MaxQuantity(MaxQuantity),
    MinPrice(MinPrice),
    MinQuantity(MinQuantity),
}

impl QuotationRule for QuotationRuleType {
    fn apply(&self, quotation: &Quotation) -> Result<(), Validators> {
        match self {
            QuotationRuleType::MaxPrice(rule) => rule.apply(quotation),
            QuotationRuleType::MaxQuantity(rule) => rule.apply(quotation),
            QuotationRuleType::MinPrice(rule) => rule.apply(quotation),
            QuotationRuleType::MinQuantity(rule) => rule.apply(quotation),
        }
    }
}
