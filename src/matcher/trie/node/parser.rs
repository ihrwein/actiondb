use matcher::trie::node::{Node, LiteralNode};
use matcher::trie::TrieElement;
use matcher::result::MatchResult;
use matcher::Pattern;
use parsers::{Parser, ParseResult};
use utils::CommonPrefix;

#[derive(Debug)]
pub struct ParserNode {
    parser: Box<Parser>,
    pattern: Option<Pattern>,
    node: Option<Node>,
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

    pub fn node(&self) -> Option<&Node> {
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

impl TrieElement for ParserNode {
    fn insert_literal(&mut self, literal: &str) -> &mut LiteralNode {
        if self.is_leaf() {
            self.node = Some(Node::new());
        }

        self.node.as_mut().unwrap().insert_literal(literal)
    }

    fn insert_parser(&mut self, parser: Box<Parser>) -> &mut ParserNode {
        if self.is_leaf() {
            self.node = Some(Node::new());
        }

        self.node.as_mut().unwrap().insert_parser(parser)
    }

    fn set_pattern(&mut self, pattern: Pattern) {
        self.pattern = Some(pattern);
    }

    fn pattern(&self) -> Option<&Pattern> {
        self.pattern.as_ref()
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
