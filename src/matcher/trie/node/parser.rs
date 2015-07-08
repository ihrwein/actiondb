use matcher::trie::node::{Node, LiteralNode};
use matcher::trie::TrieOperations;
use matcher::Pattern;
use parsers::Parser;
use utils::CommonPrefix;

use std::rc::Rc;

#[derive(Debug)]
pub struct ParserNode {
    parser: Box<Parser>,
    pattern: Option<Rc<Pattern>>,
    node: Option<Box<Node>>,
}

impl ParserNode {
    pub fn new(parser: Box<Parser>) -> ParserNode {
        ParserNode{ parser: parser,
                    pattern: None,
                    node: None}
    }

    pub fn parser(&self) -> &Parser {
        &*self.parser
    }

    pub fn is_leaf(&self) -> bool {
        self.node.is_none()
    }

    pub fn node(&self) -> Option<&Node> {
        match self.node {
            Some(ref boxed_node) => {
                Some(boxed_node)
            },
            None => None
        }
    }

    pub fn parse<'a, 'b>(&'a self, text: &'b str) -> Option<Vec<(&'a str, &'b str)>> {
        if let Some(parsed_kwpair) = self.parser.parse(text) {
            trace!("parse(): parsed_kwpair = {:?}", &parsed_kwpair);
            let text = text.ltrunc(parsed_kwpair.1.len());

            return match self.node() {
                Some(node) => {
                    node.parse_then_push_kvpair(text, parsed_kwpair)
                },
                None => {
                    self.push_last_kvpair(text, parsed_kwpair)
                }
            }
        }
        None
    }

    fn push_last_kvpair<'a, 'b>(&'a self, text: &'b str, kvpair: (&'a str, &'b str)) -> Option<Vec<(&'a str, &'b str)>> {
        if text.is_empty() {
            Some(vec!(kvpair))
        } else {
            None
        }
    }
}

impl TrieOperations for ParserNode {
    fn insert_literal(&mut self, literal: &str) -> &mut LiteralNode {
        if self.is_leaf() {
            self.node = Some(Box::new(Node::new()));
        }

        self.node.as_mut().unwrap().insert_literal(literal)
    }

    fn insert_parser(&mut self, parser: Box<Parser>) -> &mut ParserNode {
        if self.is_leaf() {
            self.node = Some(Box::new(Node::new()));
        }

        self.node.as_mut().unwrap().insert_parser(parser)
    }

    fn set_pattern(&mut self, pattern: Rc<Pattern>) {
        self.pattern = Some(pattern);
    }
}

impl Clone for ParserNode {
    fn clone(&self) -> ParserNode {
        ParserNode{
            parser: self.parser.boxed_clone(),
            pattern: self.pattern.clone(),
            node: self.node.clone()
        }
    }
}
