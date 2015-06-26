use std::collections::BTreeSet;
use std::iter::FromIterator;
use std::hash::{SipHasher, Hash, Hasher};

use parsers::{Parser, ObjectSafeHash, ParserBase, HasOptionalParameter, OptionalParameter};

#[derive(Debug, Hash)]
pub struct SetParser {
    character_set: BTreeSet<u8>,
    base: ParserBase
}

impl SetParser {
    pub fn new(name: String, set: &str) -> SetParser {
        SetParser{ character_set: SetParser::create_set_from_str(set),
                   base: ParserBase::new(name)}
    }

    pub fn from_str(name: &str, set: &str) -> SetParser {
        SetParser::new(name.to_string(), set)
    }

    pub fn set_character_set(&mut self, set: &str) {
        self.character_set = SetParser::create_set_from_str(set);
    }

    pub fn set_min_length(&mut self, length: usize) {
        self.base.set_min_length(length)
    }

    pub fn set_max_length(&mut self, length: usize) {
        self.base.set_max_length(length)
    }

    fn create_set_from_str(set: &str) -> BTreeSet<u8> {
        let vset: Vec<u8> = set.bytes().collect();
        BTreeSet::from_iter(vset)
    }

    fn calculate_match_length(&self, value: &str) -> usize {
        let mut match_len = 0;

        for c in value.bytes() {
            if self.character_set.contains(&c) {
                match_len += 1;
            } else {
                break;
            }
        }

        match_len
    }
}

impl Parser for SetParser {
    fn parse<'a, 'b>(&'a self, value: &'b str) -> Option<(&'a str, &'b str)> {
        let match_len = self.calculate_match_length(value);

        if self.base.is_match_length_ok(match_len) {
            Some((&self.name(), &value[..match_len]))
        } else {
            None
        }
    }

    fn name(&self) -> &str {
        self.base.name()
    }
}

impl ObjectSafeHash for SetParser {
    fn hash_os(&self) -> u64 {
        let mut hasher = SipHasher::new();
        "parser:set".hash(&mut hasher);
        self.hash(&mut hasher);
        hasher.finish()
    }
}

impl HasOptionalParameter for SetParser {
    fn set_optional_params<'a>(&mut self, params: &Vec<OptionalParameter<'a>>) -> bool {
        self.base.set_optional_params(params)
    }
}

#[cfg(test)]
mod test {
    use parsers::{Parser, SetParser};

    #[test]
    fn test_given_empty_string_when_parsed_it_wont_match() {
        let p = SetParser::from_str("test", "");
        assert_eq!(p.parse("almafa"),
                   None);
    }

    #[test]
    fn test_given_not_matching_string_when_parsed_it_wont_match() {
        let p = SetParser::from_str("test", "123");
        assert_eq!(p.parse("almafa"),
                   None);
    }

    #[test]
    fn test_given_matching_string_when_parsed_it_matches() {
        let p = SetParser::from_str("name", "0123");
        assert_eq!(p.parse("11230almafa"),
                   Some(("name", "11230")));
    }

    #[test]
    fn test_given_minimum_match_length_when_a_match_is_shorter_it_doesnt_count_as_a_match() {
        let mut p = SetParser::from_str("test", "0123");
        p.set_min_length(7);
        assert_eq!(p.parse("11230almafa"),
                   None);
    }

    #[test]
    fn test_given_maximum_match_length_when_a_match_is_longer_it_doesnt_count_as_a_match() {
        let mut p = SetParser::from_str("name", "0123");
        p.set_max_length(3);
        assert_eq!(p.parse("11230almafa"),
                   None);
    }

    #[test]
    fn test_given_minimum_and_maximum_match_length_when_a_proper_length_match_occures_it_counts_as_a_match() {
        let mut p = SetParser::from_str("testname", "0123");
        p.set_min_length(3);
        p.set_max_length(7);
        assert_eq!(p.parse("11230almafa"),
                   Some(("testname", "11230")));
    }

    use parsers::ObjectSafeHash;

    #[test]
    fn test_given_set_parser_and_when_differently_parametrized_instances_are_hashed_then_the_hashes_are_different() {
        let p1 = SetParser::from_str("test", "0123");
        let p2 = SetParser::from_str("test", "01234");
        assert_eq!(p1.hash_os() == p2.hash_os(), false);
    }
}
