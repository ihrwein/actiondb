use super::interface::{
    SuffixArray,
    Entry,
    LiteralEntry,
    ParserEntry
};

use parsers::{Parser, ParseResult};
use matcher::{
    Matcher,
    Pattern
};
use matcher::compiled_pattern::TokenType;
use matcher::result::MatchResult;
use utils::CommonPrefix;

use std::borrow::Borrow;

#[derive(Debug, Clone)]
pub struct SuffixTable {
    literal_entries: Vec<LiteralE>,
    parser_entries: Vec<ParserE>,
}

impl SuffixTable {
    fn longest_common_prefix_between_consecutive_entries(&self, value: &str, pos: usize) -> Option<&LiteralE> {
        let first_opt = self.literal_entries.get(pos);
        let second_opt = self.literal_entries.get(pos + 1);

        first_opt.map_or(second_opt, |first| {
            second_opt.map_or(first_opt, |second| {
                if first.literal().common_prefix_len(value) >= second.literal().common_prefix_len(value) {
                    first_opt
                } else {
                    second_opt
                }
            })
        })
    }

    fn longest_common_prefix_around_pos(&self, value: &str, pos: usize) -> Option<&LiteralE> {
        if pos == 0 {
            self.literal_entries.get(pos)
        } else {
            self.longest_common_prefix_between_consecutive_entries(value, pos - 1)
        }
    }

    fn insert_literal(&mut self, literal: String) -> &mut Entry<SA=SuffixTable> {
        let result = self.literal_entries.binary_search_by(|probe| probe.literal().cmp(&literal));
        match result {
            Ok(pos) => {
                self.literal_entries.get_mut(pos).expect("Literal entry found, but failed to remove")
            },
            Err(pos) => {
                let entry = LiteralE::new(literal);
                self.literal_entries.insert(pos, entry);
                self.literal_entries.get_mut(pos).expect("Literal entry inserted, but failed to remove")
            }
        }
    }

    fn parse_with_parsers<'a, 'b>(&'a self, value: &'b str) -> Option<MatchResult<'a, 'b>> {
        for parser in &self.parser_entries {
            if let Some(result) = parser.parse(value) {
                return Some(result);
            }
        }
        None
    }

    fn insert_parser(&mut self, parser: Box<Parser>) -> &mut Entry<SA=SuffixTable> {
        let pos = self.parser_entries.iter().position(|x| {
            x.parser.hash_os() == parser.hash_os()
        });
        if let Some(pos) = pos {
            self.parser_entries.get_mut(pos).expect("Parser entry found, but failed to remove")
        } else {
            let parser = ParserE::new(parser);
            self.parser_entries.push(parser);
            self.parser_entries.last_mut().expect("Parser entry inserted, but failed to remove")
        }
    }

    pub fn longest_common_prefix<'a, 'b>(&'a self, value: &'b str) -> Option<&'a LiteralE> {
        let result = self.literal_entries.binary_search_by(|probe| {
            let s: &str = probe.literal().borrow();
            s.cmp(value)
        });
        match result {
            Ok(pos) => self.literal_entries.get(pos),
            Err(pos) =>self.longest_common_prefix_around_pos(value, pos)
        }
    }
}

impl Default for SuffixTable {
    fn default() -> Self {
        SuffixTable {
            literal_entries: Vec::new(),
            parser_entries: Vec::new()
        }
    }
}

impl SuffixArray for SuffixTable {
    fn new() -> SuffixTable {
        SuffixTable::default()
    }

    fn insert(&mut self, mut pattern: Pattern) {
        if let Some(token) = pattern.pop_first_token() {
            let mut entry: &mut Entry<SA=SuffixTable> = match token {
                TokenType::Literal(literal) => {
                    self.insert_literal(literal)
                },
                TokenType::Parser(parser) => {
                    self.insert_parser(parser)
                }
            };
            entry.insert(pattern);
        }
    }
}

#[derive(Debug)]
pub struct ParserE {
    pattern: Option<Pattern>,
    parser: Box<Parser>,
    child: Option<SuffixTable>
}

impl Clone for ParserE {
    fn clone(&self) -> ParserE {
        ParserE {
            pattern: self.pattern.clone(),
            parser: self.parser.boxed_clone(),
            child: self.child.clone()
        }
    }
}

impl ParserE {
    pub fn new(parser: Box<Parser>) -> ParserE {
        ParserE {
            pattern: None,
            parser: parser,
            child: None
        }
    }

    fn create_match_result<'a, 'b>(&'a self, kvpair: ParseResult<'a, 'b>) -> Option<MatchResult<'a, 'b>> {
        if let Some(pattern) = self.pattern() {
            let mut result = MatchResult::new(pattern);
            result.insert(kvpair);
            Some(result)
        } else {
            debug!("Value parsing ended before reaching a leaf. Please create a new, shorter pattern.");
            None
        }
    }
}

impl Entry for ParserE {
    type SA = SuffixTable;
    fn pattern(&self) -> Option<&Pattern> {
        self.pattern.as_ref()
    }
    fn set_pattern(&mut self, pattern: Option<Pattern>) {
        self.pattern = pattern;
    }
    fn child(&self) -> Option<&SuffixTable> {
        self.child.as_ref()
    }
    fn child_mut(&mut self) -> Option<&mut SuffixTable> {
        self.child.as_mut()
    }
    fn set_child(&mut self, child: Option<Self::SA>) {
        self.child = child;
    }
}
impl ParserEntry for ParserE {
    fn parser(&self) -> &Box<Parser> {
        &self.parser
    }
    fn parse<'a, 'b>(&'a self, value: &'b str) -> Option<MatchResult<'a, 'b>> {
        self.parser.parse(value).and_then(|kvpair| {
            let value = value.ltrunc(kvpair.value().len());

            if let Some(child) = self.child() {
                child.parse(value).and_then(|mut result| {
                    result.insert(kvpair);
                    Some(result)
                })
            } else {
                if value.is_empty() {
                    self.create_match_result(kvpair)
                } else {
                    None
                }
            }
        })
    }
}

#[derive(Debug, Clone)]
pub struct LiteralE {
    pattern: Option<Pattern>,
    literal: String,
    child: Option<SuffixTable>
}

impl LiteralE {
    pub fn new(literal: String) -> LiteralE {
        LiteralE {
            literal: literal,
            pattern: None,
            child: None
        }
    }
}

impl Entry for LiteralE {
    type SA = SuffixTable;
    fn pattern(&self) -> Option<&Pattern> {
        self.pattern.as_ref()
    }
    fn set_pattern(&mut self, pattern: Option<Pattern>) {
        self.pattern = pattern;
    }
    fn child(&self) -> Option<&SuffixTable> {
        self.child.as_ref()
    }
    fn child_mut(&mut self) -> Option<&mut SuffixTable> {
        self.child.as_mut()
    }
    fn set_child(&mut self, child: Option<Self::SA>) {
        self.child = child;
    }
}

impl LiteralEntry for LiteralE {
    fn literal(&self) -> &String {
        &self.literal
    }
}

impl Matcher for SuffixTable {
    fn parse<'a, 'b>(&'a self, value: &'b str) -> Option<MatchResult<'a, 'b>> {
        if let Some(child) = self.longest_common_prefix(value) {
            let common_prefix_len = child.literal().common_prefix_len(value);
            if common_prefix_len == value.len() {
                child.pattern().and_then(|pattern| Some(MatchResult::new(pattern)))
            } else if common_prefix_len < value.len() {
                let value = value.ltrunc(common_prefix_len);
                child.child().and_then(|child| child.parse(value))
            } else {
                None
            }
        } else {
            self.parse_with_parsers(value)
        }
    }
    fn add_pattern(&mut self, pattern: Pattern) {
        self.insert(pattern);
    }
    fn boxed_clone(&self) -> Box<Matcher> {
        Box::new(self.clone())
    }
}
