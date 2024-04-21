use super::{quotation::Quotation, rule::Rule, validators::Validators};

pub struct Engine {
    pub quotation: Quotation,
    pub rules: Vec<Box<dyn Rule>>,
}

impl Engine {
    pub fn new(quotation: Quotation, rules: Vec<Box<dyn Rule>>) -> Self {
        Self { quotation, rules }
    }
    pub fn validate(&self) -> Result<(), Validators> {
        let mut validators = Validators::new();
        for rule in &self.rules {
            let rule_result = rule.apply(&self.quotation);
            match rule_result {
                Ok(()) => continue,
                Err(inner_validators) => validators.stack.extend(inner_validators.stack),
            }
        }
        if validators.stack.len() > 0 {
            return Err(validators);
        }
        Ok(())
    }
}

#[cfg(test)]
mod engine_tests {
    use crate::utils::mock::mock_engine;

    #[test]
    fn test_validate() {
        let engine = mock_engine();
        let validators = engine.validate();
        match validators {
            Ok(_) => panic!("Validators should not be empty"),
            Err(val) => {
                assert_eq!(
                    val.stack.join("\n"),
                    "0:piastrelle di ceramica break MinQuantity\n1:box doccia break MinQuantity"
                )
            }
        }
    }
}
