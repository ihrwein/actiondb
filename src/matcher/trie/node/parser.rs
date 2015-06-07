use matcher::trie::node::{Node, LiteralNode};
use matcher::trie::TrieOperations;
use parsers::Parser;

#[derive(Debug)]
pub struct ParserNode {
    parser: Box<Parser>,
    node: Option<Box<Node>>,
}

impl ParserNode {
    pub fn new(parser: Box<Parser>) -> ParserNode {
        ParserNode{ parser: parser,
                    node: None}
    }

    pub fn parser(&self) -> &Parser {
        &*self.parser
    }

    pub fn is_leaf(&self) -> bool {
        self.node.is_none()
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
}
