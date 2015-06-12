use std::collections::BTreeSet;
use std::iter::FromIterator;
use std::hash::{SipHasher, Hash, Hasher};

use parsers::{Parser, ObjectSafeHash};

#[derive(Debug)]
pub struct SetParser {
    character_set: BTreeSet<u8>,
    name: String,
    min_length: Option<usize>,
    max_length: Option<usize>
}

impl SetParser {
    pub fn new(name: &str, set: &str) -> SetParser {
        SetParser{ character_set: SetParser::create_set_from_str(set),
                        name: name.to_string(),
                        min_length: None,
                        max_length: None}
    }

    pub fn set_min_length(&mut self, length: usize) {
        self.min_length = Some(length);
    }

    pub fn set_max_length(&mut self, length: usize) {
        self.max_length = Some(length);
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    fn create_set_from_str(set: &str) -> BTreeSet<u8> {
        let vset: Vec<u8> = set.bytes().collect();
        BTreeSet::from_iter(vset)
    }

    fn is_match_length_ok(&self, match_length: usize) -> bool {
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

        if self.is_match_length_ok(match_len) {
            Some((&self.name, &value[..match_len]))
        } else {
            None
        }
    }

    fn name(&self) -> &str {
        &self.name
    }
}

impl ObjectSafeHash for SetParser {
    fn hash_os(&self) -> u64 {
        let mut hasher = SipHasher::new();
        "parser:set".hash(&mut hasher);
        self.character_set.hash(&mut hasher);
        self.min_length.hash(&mut hasher);
        self.max_length.hash(&mut hasher);
        hasher.finish()
    }
}

#[cfg(test)]
mod test {
    use parsers::{Parser, SetParser};

    #[test]
    fn test_given_empty_string_when_parsed_it_wont_match() {
        let p = SetParser::new("test", "");
        assert_eq!(p.parse("almafa"),
                   None);
    }

    #[test]
    fn test_given_not_matching_string_when_parsed_it_wont_match() {
        let p = SetParser::new("test", "123");
        assert_eq!(p.parse("almafa"),
                   None);
    }

    #[test]
    fn test_given_matching_string_when_parsed_it_matches() {
        let mut p = SetParser::new("name", "0123");
        assert_eq!(p.parse("11230almafa"),
                   Some(("name", "11230")));
    }

    #[test]
    fn test_given_minimum_match_length_when_a_match_is_shorter_it_doesnt_count_as_a_match() {
        let mut p = SetParser::new("test", "0123");
        p.set_min_length(7);
        assert_eq!(p.parse("11230almafa"),
                   None);
    }

    #[test]
    fn test_given_maximum_match_length_when_a_match_is_longer_it_doesnt_count_as_a_match() {
        let mut p = SetParser::new("name", "0123");
        p.set_max_length(3);
        assert_eq!(p.parse("11230almafa"),
                   None);
    }

    #[test]
    fn test_given_minimum_and_maximum_match_length_when_a_proper_length_match_occures_it_counts_as_a_match() {
        let mut p = SetParser::new("testname", "0123");
        p.set_min_length(3);
        p.set_max_length(7);
        assert_eq!(p.parse("11230almafa"),
                   Some(("testname", "11230")));
    }

    use parsers::ObjectSafeHash;

    #[test]
    fn test_given_set_parser_and_when_differently_parametrized_instances_are_hashed_then_the_hashes_are_different() {
        let p1 = SetParser::new("test", "0123");
        let p2 = SetParser::new("test", "01234");
        assert_eq!(p1.hash_os() == p2.hash_os(), false);
    }
}
