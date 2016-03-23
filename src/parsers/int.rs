use std::hash::{SipHasher, Hash, Hasher};

use parsers::{Parser, ObjectSafeHash, SetParser, ParseResult, HasLengthConstraint};

#[derive(Clone, Debug, Hash)]
pub struct IntParser {
    delegate: SetParser,
}

impl IntParser {
    pub fn with_name<S: Into<String>>(name: S) -> IntParser {
        let delegate = SetParser::with_name(name.into(), "0123456789");
        IntParser { delegate: delegate }
    }

    pub fn new() -> IntParser {
        IntParser::default()
    }
}

impl Parser for IntParser {
    fn parse<'a, 'b>(&'a self, value: &'b str) -> Option<ParseResult<'a, 'b>> {
        self.delegate.parse(value)
    }

    fn name(&self) -> Option<&str> {
        self.delegate.name()
    }

    fn set_name(&mut self, name: Option<String>) {
        self.delegate.set_name(name);
    }

    fn boxed_clone(&self) -> Box<Parser> {
        Box::new(self.clone())
    }
}

impl Default for IntParser {
    fn default() -> Self {
        IntParser { delegate: SetParser::new("0123456789") }
    }
}

impl HasLengthConstraint for IntParser {
    fn min_length(&self) -> Option<usize> {
        self.delegate.min_length()
    }
    fn set_min_length(&mut self, length: Option<usize>) {
        self.delegate.set_min_length(length);
    }
    fn max_length(&self) -> Option<usize> {
        self.delegate.max_length()
    }
    fn set_max_length(&mut self, length: Option<usize>) {
        self.delegate.set_max_length(length);
    }
}

impl ObjectSafeHash for IntParser {
    fn hash_os(&self) -> u64 {
        let mut hasher = SipHasher::new();
        "parser:int".hash(&mut hasher);
        self.hash(&mut hasher);
        hasher.finish()
    }
}

#[cfg(test)]
mod test {
    use parsers::{IntParser, Parser, HasLengthConstraint};

    #[test]
    fn test_given_int_parser_when_the_match_is_empty_then_the_result_isnt_successful() {
        let parser = IntParser::with_name("test_int_parser");
        assert_eq!(parser.parse("").is_none(), true);
        assert_eq!(parser.parse("asdf").is_none(), true);
    }

    #[test]
    fn test_given_matching_string_when_it_is_parsed_then_it_matches() {
        let parser_name = "test_int_parser";
        let parser = IntParser::with_name(parser_name);
        let res = parser.parse("1234asd").unwrap();
        assert_eq!(res.parser().name(), Some(parser_name));
        assert_eq!(res.value(), "1234");
    }

    #[test]
    fn test_given_matching_string_which_is_longer_than_the_max_match_length_when_it_is_parsed_then_it_does_not_match
        () {
        let parser_name = "test_int_parser";
        let mut parser = IntParser::with_name(parser_name);
        parser.set_max_length(Some(3));
        assert_eq!(parser.parse("1234asd").is_none(), true);
    }
}
