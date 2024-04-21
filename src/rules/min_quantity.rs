use crate::model::{quotation::Quotation, rule::Rule, validators::Validators};

pub struct MinQuantity {
    pub quantity: u32,
}

impl Rule for MinQuantity {
    fn apply(&self, quotation: &Quotation) -> Result<(), Validators> {
        let mut validators = Validators::new();
        for purchasable in &quotation.purchasables {
            if purchasable.required_amount <= self.quantity {
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
