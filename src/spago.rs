#[derive(Debug)]
pub struct Spago {
    module: String,
}

impl Spago {
    pub fn new(module: &str) -> Self {
        Spago {
            module: String::from(module),
        }
    }
}
