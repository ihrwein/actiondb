use parsers::{HasOptionalParameter, OptionalParameter};

#[derive(Hash, Debug)]
pub struct ParserBase {
    name: String,
    min_length: Option<usize>,
    max_length: Option<usize>
}

impl ParserBase {
    pub fn with_name_str(name: &str) -> ParserBase {
        ParserBase::with_name(name.to_string())
    }

    pub fn with_name(name: String) -> ParserBase {
        let mut base = ParserBase::new();
        base.set_name(name);
        base
    }

    pub fn new() -> ParserBase {
        ParserBase { name: "".to_string(),
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

impl HasOptionalParameter for ParserBase {
    fn set_optional_params(&mut self, params: &Vec<OptionalParameter>) -> bool {
        for i in params {
            match i {
                OptionalParameter::Int(key, value) => {
                    match key {
                        "min_len" => self.set_min_length(value),
                        "max_len" => self.set_max_length(value)
                    }
                }
                _ => return false
            }
        }

        true
    }
}
