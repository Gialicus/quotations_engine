use serde::{Deserialize, Serialize};

use crate::model::quotation::Quotation;

use super::{rule::QuotationRule, validators::Validators};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinQuantity {
    pub quantity: u32,
}

impl QuotationRule for MinQuantity {
    fn apply(&self, quotation: &Quotation) -> Result<(), Validators> {
        let mut validators = Validators::new();
        for purchasable in &quotation.purchasables {
            if purchasable.required_amount < self.quantity {
                validators.add(format!(
                    "{}:{} break MinQuantity",
                    purchasable.product.id, purchasable.product.name
                ))
            }
        }
        if validators.stack.len() > 0 {
            return Err(validators);
        }
        Ok(())
    }
}

#[cfg(test)]
mod test_min_quantity {
    use crate::{
        model::engine::Engine, rules::rule::QuotationRuleType, utils::mock::mock_quotation,
    };

    use super::MinQuantity;

    #[test]
    fn test_min_quantity_success() {
        let q = mock_quotation();
        let rules: Vec<QuotationRuleType> =
            vec![QuotationRuleType::MinQuantity(MinQuantity { quantity: 1 })];
        let e = Engine::new(q, rules);
        let result = e.validate();
        assert_eq!(result.is_ok(), true)
    }

    #[test]
    fn test_min_quantity_fail() {
        let q = mock_quotation();
        let rules: Vec<QuotationRuleType> =
            vec![QuotationRuleType::MinQuantity(MinQuantity { quantity: 5 })];
        let e = Engine::new(q, rules);
        let result = e.validate();
        match result {
            Ok(_) => panic!("Validators sould not be empty"),
            Err(validator) => {
                assert_eq!(validator.stack.len(), 3)
            }
        }
    }
}
