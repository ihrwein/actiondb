use std::collections::BTreeSet;
use std::iter::FromIterator;

use parsers::{Parser,
            MatchResult};

pub struct SetParser {
    character_set: BTreeSet<u8>,
    min_length: Option<usize>,
    max_length: Option<usize>
}

impl SetParser {
    fn create_set_from_str(set: &str) -> BTreeSet<u8> {
        let vset: Vec<u8> = set.bytes().collect();
        BTreeSet::from_iter(vset)
    }

    pub fn new(set: &str) -> SetParser {
        SetParser{ character_set: SetParser::create_set_from_str(set),
                        min_length: None,
                        max_length: None}
    }

    pub fn set_min_length(&mut self, length: usize) {
        self.min_length = Some(length);
    }

    fn is_min_length_ok(&self, match_length: usize) -> bool {
        match self.min_length {
            Some(x) => match_length >= x,
            None => true
        }
    }
}

impl <'a, 'b> Parser<'a, 'b> for SetParser {
    fn parse(&'a self, value: &'b str) -> MatchResult<'b> {
        let mut match_len = 0;

        for c in value.bytes() {
            if self.character_set.contains(&c) {
                match_len += 1;
            } else {
                break;
            }
        }

        if match_len > 0 && self.is_min_length_ok(match_len) {
            return MatchResult::Matched(&value[..match_len])
        } else {
            return MatchResult::NotMatched;
        }
    }
}

#[test]
fn test_given_empty_string_when_parsed_it_wont_match() {
    let p = SetParser::new("");
    assert_eq!(p.parse("almafa"),
               MatchResult::NotMatched);
}

#[test]
fn test_given_not_matching_string_when_parsed_it_wont_match() {
    let p = SetParser::new("123");
    assert_eq!(p.parse("almafa"),
               MatchResult::NotMatched);
}

#[test]
fn test_given_matching_string_when_parsed_it_matches() {
    let p = SetParser::new("0123");
    assert_eq!(p.parse("11230almafa"),
               MatchResult::Matched("11230"));
}

#[test]
fn test_given_minimum_match_length_when_a_match_is_shorter_it_doesnt_count_as_a_match() {
    let mut p = SetParser::new("0123");
    p.set_min_length(7);
    assert_eq!(p.parse("11230almafa"),
               MatchResult::NotMatched);
}
