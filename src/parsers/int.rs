use std::hash::{SipHasher, Hash, Hasher};

use parsers::{Parser, ObjectSafeHash, ParserBase, SetParser};

#[derive(Debug, Hash)]
pub struct IntParser {
    delegate: SetParser
}

impl IntParser {
    pub fn from_str(name: &str) -> IntParser {
        IntParser::new(name.to_string())
    }

    pub fn new(name: String) -> IntParser {
        let delegate = SetParser::new(name, "0123456789");
        IntParser{ delegate: delegate }
    }
}

impl Parser for IntParser {
    fn parse<'a, 'b>(&'a self, value: &'b str) -> Option<(&'a str, &'b str)> {
        self.delegate.parse(value)
    }

    fn base_mut(&mut self) -> &mut ParserBase {
        self.delegate.base_mut()
    }

    fn name(&self) -> &str {
        self.delegate.name()
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
    use parsers::{IntParser, Parser};

    #[test]
    fn test_given_int_parser_when_the_match_is_empty_then_the_result_isnt_successful() {
        let parser = IntParser::from_str("test_int_parser");
        assert_eq!(parser.parse(""), None);
        assert_eq!(parser.parse("asdf"), None);
    }

    #[test]
    fn test_given_matching_string_when_it_is_parsed_then_it_matches() {
        let parser_name = "test_int_parser";
        let parser = IntParser::from_str(parser_name);
        assert_eq!(parser.parse("1234asd").unwrap(), (parser_name, "1234"));
    }
}
