#[derive(Hash, Debug)]
pub struct ParserBase {
    name: String,
    min_length: Option<usize>,
    max_length: Option<usize>
}

impl ParserBase {
    pub fn from_str(name: &str) -> ParserBase {
        ParserBase::new(name.to_string())
    }

    pub fn new(name: String) -> ParserBase {
        ParserBase { name: name,
                     min_length: None,
                     max_length: None }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_min_length(&mut self, length: usize) {
        self.min_length = Some(length);
    }

    pub fn set_max_length(&mut self, length: usize) {
        self.max_length = Some(length);
    }

    pub fn is_match_length_ok(&self, match_length: usize) -> bool {
        match_length > 0 &&
            self.is_min_length_ok(match_length) &&
            self.is_max_length_ok(match_length)
    }

    fn is_min_length_ok(&self, match_length: usize) -> bool {
        match self.min_length {
            Some(x) => match_length >= x,
            None => true
        }
    }

    fn is_max_length_ok(&self, match_length: usize) -> bool {
        match self.max_length {
            Some(x) => match_length <= x,
            None => true
        }
    }
}
