use super::{quotation::Quotation, validators::Validators};

pub trait Rule {
    fn apply(&self, quotation: &Quotation) -> Result<(), Validators>;
}
