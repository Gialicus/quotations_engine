pub struct Validators {
    pub stack: Vec<String>,
}

impl Validators {
    pub fn new() -> Self {
        Validators { stack: Vec::new() }
    }
    pub fn add(&mut self, msg: String) {
        self.stack.push(msg)
    }
}
