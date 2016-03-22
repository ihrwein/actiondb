use std::borrow::Borrow;

#[derive(Clone, Hash, Debug)]
pub struct ParserBase {
    name: Option<String>,
}

impl ParserBase {
    pub fn with_name(name: String) -> ParserBase {
        ParserBase { name: Some(name) }
    }

    pub fn new() -> ParserBase {
        ParserBase::default()
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_ref().and_then(|x| Some(x.borrow()))
    }

    pub fn set_name(&mut self, name: Option<String>) {
        self.name = name;
    }
}

impl Default for ParserBase {
    fn default() -> Self {
        ParserBase { name: None }
    }
}
