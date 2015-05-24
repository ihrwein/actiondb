use std::collections::BTreeSet;
use std::iter::FromIterator;

use parsers::{Parser,
            MatchResult};

pub struct SetParser {
    character_set: BTreeSet<u8>,
    min_length: Option<i32>,
    max_length: Option<i32>
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

        if match_len > 0 {
            return MatchResult::Matched(&value[..match_len])
        } else {
            return MatchResult::NotMatched;
        }
    }
}
