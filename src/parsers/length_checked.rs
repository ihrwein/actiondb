use parsers::{HasOptionalParameter, OptionalParameter, ParserBase};

#[derive(Clone, Hash, Debug)]
pub struct LengthCheckedParserBase {
    base: ParserBase,
    min_length: Option<usize>,
    max_length: Option<usize>
}

impl LengthCheckedParserBase {
    pub fn new(name: String) -> LengthCheckedParserBase {
        LengthCheckedParserBase { base: ParserBase::new(name),
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

impl HasOptionalParameter for LengthCheckedParserBase {
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

#[cfg(test)]
mod test {
    use super::LengthCheckedParserBase;

    #[test]
    fn test_given_parser_when_the_match_length_is_not_constrained_then_the_match_length_is_ok_in_every_case() {
        let base = LengthCheckedParserBase::new("name".to_string());
        assert_eq!(base.is_match_length_ok(42), true);
        assert_eq!(base.is_match_length_ok(1), true);
    }

    #[test]
    fn test_given_parser_when_the_minimum_match_length_is_set_then_the_shorter_matches_are_discarded() {
        let mut base = LengthCheckedParserBase::new("name".to_string());
        base.set_min_length(10);
        assert_eq!(base.is_match_length_ok(42), true);
        assert_eq!(base.is_match_length_ok(1), false);
        assert_eq!(base.is_match_length_ok(9), false);
        assert_eq!(base.is_match_length_ok(10), true);
    }

    #[test]
    fn test_given_parser_when_the_maximum_match_length_is_set_then_the_longer_matches_are_discarded() {
        let mut base = LengthCheckedParserBase::new("name".to_string());
        base.set_max_length(10);
        assert_eq!(base.is_match_length_ok(42), false);
        assert_eq!(base.is_match_length_ok(1), true);
        assert_eq!(base.is_match_length_ok(9), true);
        assert_eq!(base.is_match_length_ok(10), true);
    }
}
