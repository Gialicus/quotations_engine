pub struct Validator {
    pub stack: Vec<String>,
}
impl Validator {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }
    pub fn push(&mut self, value: &str) {
        self.stack.push(value.into())
    }
    pub fn concat(&mut self, validator: &Validator) {
        self.stack.extend(validator.stack.clone())
    }
    pub fn pretty(&self) -> String {
        self.stack.join("\n")
    }
}

impl From<&str> for Validator {
    fn from(value: &str) -> Self {
        let mut v = Validator::new();
        v.push(value);
        v
    }
}
impl From<String> for Validator {
    fn from(value: String) -> Self {
        let mut v = Validator::new();
        v.push(value.as_str());
        v
    }
}
