#[derive(Clone, Hash, Debug)]
pub struct ParserBase {
    name: String
}

impl ParserBase {
    pub fn new(name: String) -> ParserBase {
        ParserBase { name: name }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
}
