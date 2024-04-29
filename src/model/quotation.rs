use super::{
    group_rule::{GroupRule, QuotationRule},
    purchasable::Purchasable,
    validator::Validator,
};

#[derive(Debug, Clone)]
pub struct Quotation {
    pub purchasables: Vec<Purchasable>,
    pub rules: Vec<QuotationRule>,
}

impl Quotation {
    pub fn new() -> Self {
        Self {
            purchasables: Vec::new(),
            rules: Vec::new(),
        }
    }

    pub fn add_item(&mut self, p: Purchasable) {
        self.purchasables.push(p)
    }

    pub fn total_price(&self) -> f64 {
        self.purchasables
            .iter()
            .map(|pur: &Purchasable| pur.total_price())
            .sum()
    }

    pub fn total_discounted(&self) -> f64 {
        self.purchasables
            .iter()
            .map(|pur: &Purchasable| pur.total_discounted())
            .sum()
    }

    pub fn apply_final_discount(&self, discount: Option<f64>) -> f64 {
        let total = self
            .purchasables
            .iter()
            .map(|pur: &Purchasable| pur.total_discounted())
            .sum();
        match discount {
            Some(v) => total * (1.0 - v),
            None => total,
        }
    }

    pub fn total_quantity(&self) -> f64 {
        self.purchasables
            .iter()
            .map(|pur| pur.total_package_quantity())
            .sum()
    }

    pub fn add_rule(&mut self, rule: &QuotationRule) {
        self.rules.push(rule.clone())
    }

    pub fn validate(&self) -> Validator {
        let mut v = Validator::new();
        self.purchasables.iter().for_each(|pur| {
            let inner = pur.validate();
            v.concat(&inner)
        });
        for rule in &self.rules {
            let inner = rule.apply(self);
            v.concat(&inner)
        }
        v
    }
}

#[cfg(test)]
mod quotation_test {
    use crate::{
        model::{
            group_rule::{MaxProductNumber, MinProductNumber, QuotationRule},
            product_rule::{MinPrice, ProductRule},
        },
        utils::mock::mock_quotation,
    };

    #[test]
    fn quotation_total_price() {
        let q = mock_quotation();
        assert_eq!(q.total_price(), 440.0);
    }
    #[test]
    fn quotation_total_discounted() {
        let q = mock_quotation();
        assert_eq!(q.total_discounted(), 220.0);
    }
    #[test]
    fn quotation_final_total_discount() {
        let q = mock_quotation();
        assert_eq!(q.apply_final_discount(Some(0.5)), 110.0);
    }
    #[test]
    fn quotation_total_quantity() {
        let q = mock_quotation();
        assert_eq!(q.total_quantity(), 37.0);
    }

    #[test]
    fn quotation_validate_success() {
        let mut q = mock_quotation();
        q.add_rule(&QuotationRule::MinProductNumber(MinProductNumber::from(2)));
        q.add_rule(&QuotationRule::MaxProductNumber(MaxProductNumber::from(3)));
        let v = q.validate();
        assert_eq!(v.is_valid(), true);
    }

    #[test]
    fn quotation_validate_fail() {
        let mut q = mock_quotation();
        q.add_rule(&QuotationRule::MinProductNumber(MinProductNumber::from(4)));
        q.add_rule(&QuotationRule::MaxProductNumber(MaxProductNumber::from(1)));
        let v = q.validate();
        assert_eq!(v.is_valid(), false);
    }

    #[test]
    fn quotation_validate_group_success() {
        let mut q = mock_quotation();
        q.add_rule(&QuotationRule::MinProductNumber(MinProductNumber::from(1)));
        q.add_rule(&QuotationRule::MaxProductNumber(MaxProductNumber::from(3)));
        q.purchasables
            .iter_mut()
            .for_each(|p| p.add_rule(&ProductRule::MinPrice(MinPrice::from(10.0))));
        let v = q.validate();
        assert_eq!(v.is_valid(), true);
    }
    #[test]
    fn quotation_validate_group_fail() {
        let mut q = mock_quotation();
        q.add_rule(&QuotationRule::MinProductNumber(MinProductNumber::from(5)));
        q.add_rule(&QuotationRule::MaxProductNumber(MaxProductNumber::from(1)));
        q.purchasables
            .iter_mut()
            .for_each(|p| p.add_rule(&ProductRule::MinPrice(MinPrice::from(1000.0))));
        let v = q.validate();
        assert_eq!(v.is_valid(), false);
    }
}
