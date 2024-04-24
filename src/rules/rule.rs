use serde::{Deserialize, Serialize};

use crate::model::{product::Product, quotation::Quotation};

use super::{
    max_price::MaxPrice, max_quantity::MaxQuantity, min_price::MinPrice, min_quantity::MinQuantity,
    validators::Validators,
};

pub trait QuotationRule {
    fn apply_quotation_rule(&self, quotation: &Quotation) -> Result<(), Validators>;
}
pub trait ProductRule {
    fn apply_product_rule(&self, quotation: &Product) -> Result<(), Validators>;
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QuotationRuleType {
    MaxPrice(MaxPrice),
    MaxQuantity(MaxQuantity),
    MinPrice(MinPrice),
    MinQuantity(MinQuantity),
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProductRuleType {
    MaxPrice(MaxPrice),
    MaxQuantity(MaxQuantity),
    MinPrice(MinPrice),
    MinQuantity(MinQuantity),
}
impl QuotationRule for QuotationRuleType {
    fn apply_quotation_rule(&self, quotation: &Quotation) -> Result<(), Validators> {
        match self {
            QuotationRuleType::MaxPrice(rule) => rule.apply_quotation_rule(quotation),
            QuotationRuleType::MaxQuantity(rule) => rule.apply_quotation_rule(quotation),
            QuotationRuleType::MinPrice(rule) => rule.apply_quotation_rule(quotation),
            QuotationRuleType::MinQuantity(rule) => rule.apply_quotation_rule(quotation),
        }
    }
}
