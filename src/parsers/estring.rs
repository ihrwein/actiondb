use std::hash::{SipHasher, Hash, Hasher};
use super::{ParserBase, Parser, ObjectSafeHash};

#[derive(Debug, Hash)]
pub struct EStringParser {
    base: ParserBase,
    end_string: Option<String>
}

impl EStringParser {
    pub fn new(name: String) -> EStringParser {
        EStringParser{ base: ParserBase::new(name),
                       end_string: None }
    }

    pub fn from_str(name: &str, end_string: &str) -> EStringParser {
        let mut parser = EStringParser::new(name.to_string());
        parser.set_end_string(end_string.to_string());
        parser
    }

    pub fn set_end_string(&mut self, end_string: String) {
        self.end_string = Some(end_string);
    }
}

impl ObjectSafeHash for EStringParser {
    fn hash_os(&self) -> u64 {
        let mut hasher = SipHasher::new();
        "parser:estring".hash(&mut hasher);
        self.hash(&mut hasher);
        hasher.finish()
    }
}

impl Parser for EStringParser {
    fn parse<'a, 'b>(&'a self, value: &'b str) -> Option<(&'a str, &'b str)> {
        if self.end_string.is_none() {
            return Some((self.name(), &value[..]))
        }

        if let Some(pos) = value.find(&self.end_string.as_ref().unwrap()[..]) {
            Some((self.name(), &value[..pos]))
        } else {
            None
        }
    }

    fn name(&self) -> &str {
        self.base.name()
    }
}

#[cfg(test)]
mod test {
    use parsers::{EStringParser, Parser};

    #[test]
    fn test_given_estring_parser_when_the_end_string_is_not_found_in_the_value_then_the_parser_doesnt_match() {
        let parser = EStringParser::from_str("name", "foo");
        assert_eq!(parser.parse("qux baz bar"), None);
    }

    #[test]
    fn test_given_estring_parser_when_the_end_string_is_found_in_the_value_then_the_parser_matches() {
        let parser = EStringParser::from_str("name", "foo");
        assert_eq!(parser.parse("qux foo bar"), Some(("name", "qux ")));
    }
}
