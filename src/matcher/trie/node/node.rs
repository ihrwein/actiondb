use std::collections::BTreeMap;
use parsers::{Parser, SetParser};
use utils::SortedVec;
use matcher::trie::node::LiteralNode;
use matcher::trie::node::ParserNode;

pub type MatchResult<'a, 'b> = Option<BTreeMap<&'a str, &'b str>>;
pub type CompiledPattern<'a, 'b> = Vec<NodeType<'a, 'b>>;

pub enum NodeType<'a, 'b> {
    Parser(Box<Parser<'a, 'b>>),
    Literal(String)
}

pub struct Node<'a, 'b> {
    literal_children: SortedVec<LiteralNode<'a, 'b, String>>,
    parser_children: Vec<ParserNode<'a, 'b>>
}

impl <'a, 'b, 'c> Node<'a, 'b> {
    pub fn new() -> Node<'a, 'b> {
        Node{ literal_children: SortedVec::new(),
              parser_children: Vec::new() }
    }

    pub fn parse(&mut self, value: &'b str) -> MatchResult<'c, 'b> {
        None
    }

    fn find(&mut self, literal: &str) -> (&mut Node<'a, 'b>, usize) {
        if self.literal_children.len() == 0 {
            return (self, 0);
        }
        (self, 0)
    }

    fn add_literal(&mut self, literal: String) {
        let (parent, matching_prefix_length) = self.find(&literal);

        if matching_prefix_length == 0 {
            let literal_node = LiteralNode::new(literal);
            parent.literal_children.push(literal_node);
        } else {
            let literal_without_common_prefix = &literal[matching_prefix_length..];
            let literal_node = LiteralNode::new(literal_without_common_prefix.to_owned());
            parent.literal_children.push(literal_node);
        }
    }

    fn add_parser(&mut self, boxed_parser: Box<Parser<'c, 'b>>) {

    }

    pub fn add_pattern(&mut self, pattern: CompiledPattern<'c, 'b>)  {
        for i in pattern.into_iter() {
            match i {
                NodeType::Literal(literal) => {
                    self.add_literal(literal);
                },
                NodeType::Parser(boxed_parser) => {
                    self.add_parser(boxed_parser);
                }
            }
        }
    }
}
