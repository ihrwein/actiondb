use parsers::{HasOptionalParameter, OptionalParameter, ParserBase};

#[derive(Hash, Debug)]
pub struct LengthConstrainedParserBase {
    base: ParserBase,
    min_length: Option<usize>,
    max_length: Option<usize>
}

impl LengthConstrainedParserBase {
    pub fn new(name: String) -> LengthConstrainedParserBase {
        LengthConstrainedParserBase { base: ParserBase::new(name),
                     min_length: None,
                     max_length: None }
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

    pub fn name(&self) -> &str {
        self.base.name()
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

impl HasOptionalParameter for LengthConstrainedParserBase {
    fn set_optional_params<'a>(&mut self, params: &Vec<OptionalParameter<'a>>) -> bool {
        for i in params {
            match i {
                &OptionalParameter::Int(key, value) => {
                    match key {
                        "min_len" => self.set_min_length(value),
                        "max_len" => self.set_max_length(value),
                        _ => return false
                    }
                }
            }
        }

        true
    }
}
