use matcher::trie::node::{Node, LiteralNode};
use matcher::trie::TrieOperations;
use parsers::Parser;

#[derive(Debug)]
pub struct ParserNode<'a> {
    parser: Box<Parser<'a>>,
    node: Option<Box<Node<'a>>>,
}

impl <'a> ParserNode<'a> {
    pub fn new(parser: Box<Parser<'a>>) -> ParserNode<'a> {
        ParserNode{ parser: parser,
                    node: None}
    }

    pub fn parser(&self) -> &Parser<'a> {
        &*self.parser
    }

    pub fn is_leaf(&self) -> bool {
        self.node.is_none()
    }
}

impl <'a> TrieOperations<'a> for ParserNode<'a> {
    fn insert_literal(&mut self, literal: &str) -> &mut LiteralNode<'a> {
        if self.is_leaf() {
            self.node = Some(Box::new(Node::new()));
        }

        self.node.as_mut().unwrap().insert_literal(literal)
    }

    fn insert_parser(&mut self, parser: Box<Parser<'a>>) -> &mut ParserNode<'a> {
        if self.is_leaf() {
            self.node = Some(Box::new(Node::new()));
        }

        self.node.as_mut().unwrap().insert_parser(parser)
    }
}
