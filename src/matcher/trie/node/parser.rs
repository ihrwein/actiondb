use matcher::trie::node::SuffixTree;
use matcher::trie::node::interface::{Entry, ParserEntry};
use matcher::result::MatchResult;
use matcher::Pattern;
use parsers::{Parser, ParseResult};
use utils::CommonPrefix;

#[derive(Debug)]
pub struct ParserNode {
    parser: Box<Parser>,
    pattern: Option<Pattern>,
    node: Option<SuffixTree>,
}

impl ParserNode {
    pub fn new(parser: Box<Parser>) -> ParserNode {
        ParserNode {
            parser: parser,
            pattern: None,
            node: None,
        }
    }

    pub fn parser(&self) -> &Parser {
        &*self.parser
    }

    pub fn is_leaf(&self) -> bool {
        self.node.is_none()
    }

    pub fn node(&self) -> Option<&SuffixTree> {
        self.node.as_ref()
    }

    pub fn parse<'a, 'b>(&'a self, text: &'b str) -> Option<MatchResult<'a, 'b>> {
        if let Some(parsed_kwpair) = self.parser.parse(text) {
            trace!("parse(): parsed_kwpair = {:?}", &parsed_kwpair);
            let text = text.ltrunc(parsed_kwpair.value().len());

            return match self.node() {
                Some(node) => {
                    node.parse_then_push_kvpair(text, parsed_kwpair)
                }
                None => {
                    self.push_last_kvpair(text, parsed_kwpair)
                }
            };
        }
        None
    }

    fn push_last_kvpair<'a, 'b>(&'a self,
                                text: &'b str,
                                kvpair: ParseResult<'a, 'b>)
                                -> Option<MatchResult<'a, 'b>> {
        if text.is_empty() {
            let mut result = MatchResult::new(self.pattern().unwrap());
            result.insert(kvpair);
            Some(result)
        } else {
            None
        }
    }
}

impl Entry for ParserNode {
    type ST = SuffixTree;
    fn pattern(&self) -> Option<&Pattern> {
        self.pattern.as_ref()
    }
    fn set_pattern(&mut self, pattern: Option<Pattern>) {
        self.pattern = pattern;
    }
    fn child(&self) -> Option<&SuffixTree> {
        self.node.as_ref()
    }
    fn child_mut(&mut self) -> Option<&mut SuffixTree> {
        self.node.as_mut()
    }
    fn set_child(&mut self, child: Option<Self::ST>) {
        self.node = child;
    }
}

impl ParserEntry for ParserNode {
    fn parse<'a, 'b>(&'a self, value: &'b str) -> Option<MatchResult<'a, 'b>> {
        if let Some(parsed_kwpair) = self.parser.parse(value) {
            trace!("parse(): parsed_kwpair = {:?}", &parsed_kwpair);
            let text = value.ltrunc(parsed_kwpair.value().len());

            return match self.node() {
                Some(node) => {
                    node.parse_then_push_kvpair(text, parsed_kwpair)
                }
                None => {
                    self.push_last_kvpair(text, parsed_kwpair)
                }
            };
        }
        None
    }
    fn parser(&self) -> &Box<Parser> {
        &self.parser
    }
}

impl Clone for ParserNode {
    fn clone(&self) -> ParserNode {
        ParserNode {
            parser: self.parser.boxed_clone(),
            pattern: self.pattern.clone(),
            node: self.node.clone(),
        }
    }
}
