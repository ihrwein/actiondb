use std::collections::BTreeSet;
use std::iter::FromIterator;

enum MatchResult {
    Matched(usize),
    NotMatched
}

pub trait ParserNode {
    fn parse(&self, value: &str) -> MatchResult;
}

pub struct SetParserNode {
    character_set: BTreeSet<u8>,
    min_length: Option<i32>,
    max_length: Option<i32>
}

impl SetParserNode {
    fn create_set_from_str(set: &str) -> BTreeSet<u8> {
        let vset: Vec<u8> = set.bytes().collect();
        BTreeSet::from_iter(vset)
    }

    pub fn new(set: &str) -> SetParserNode {
        SetParserNode{ character_set: SetParserNode::create_set_from_str(set),
                        min_length: None,
                        max_length: None}
    }
}

impl ParserNode for SetParserNode {
    fn parse(&self, value: &str) -> MatchResult {
        let mut match_len = 0;

        for c in value.bytes() {
            if self.character_set.contains(&c) {
                match_len += 1;
            } else {
                break;
            }
        }

        if match_len > 0 {
            return MatchResult::Matched(match_len)
        } else {
            return MatchResult::NotMatched;
        }
    }
}
