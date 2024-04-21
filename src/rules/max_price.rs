use crate::model::{quotation::Quotation, rule::Rule, validators::Validators};

pub struct MaxPrice {
    pub price: f64,
}

impl Rule for MaxPrice {
    fn apply(&self, quotation: &Quotation) -> Result<(), Validators> {
        let mut validators = Validators::new();
        for purchasable in &quotation.purchasables {
            if purchasable.total_price() > self.price {
                validators.add(format!(
                    "{}:{} break MaxPrice",
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
        model::{engine::Engine, rule::Rule},
        utils::mock::mock_quotation,
    };

    use super::MaxPrice;

    #[test]
    fn test_max_price_success() {
        let q = mock_quotation();
        let rules: Vec<Box<dyn Rule>> = vec![Box::new(MaxPrice { price: 150.0 })];
        let e = Engine::new(q, rules);
        let result = e.validate();
        assert_eq!(result.is_ok(), true)
    }

    #[test]
    fn test_max_price_fail() {
        let q = mock_quotation();
        let rules: Vec<Box<dyn Rule>> = vec![Box::new(MaxPrice { price: 1.0 })];
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