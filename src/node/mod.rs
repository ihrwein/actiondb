mod literal;
mod parser;

use std::collections::BTreeMap;
use std::borrow::ToOwned;

use parsers::{Parser, SetParser};
use utils::SortedVec;
use self::literal::LiteralNode;
use self::parser::ParserNode;

type MatchResult<'a, 'b> = Option<BTreeMap<&'a str, &'b str>>;
type CompiledPattern<'a, 'b> = Vec<NodeType<'a, 'b>>;

enum NodeType<'a, 'b> {
    Parser(Box<Parser<'a, 'b>>),
    Literal(String)
}

struct Node<'a, 'b> {
    literal_children: SortedVec<Box<LiteralNode<'a, 'b, String>>>,
    parser_children: Vec<Box<ParserNode<'a, 'b>>>
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
            let literal_node = Box::new(LiteralNode::new(literal));
            parent.literal_children.push(literal_node);
        } else {
            let literal_without_common_prefix = &literal[matching_prefix_length..];
            let literal_node = Box::new(LiteralNode::new(literal_without_common_prefix.to_owned()));
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

#[cfg(test)]
mod test {
    use node::{CompiledPattern, NodeType, Node};
    use parsers::{Parser, SetParser};

    #[test]
    fn test_given_patterns_when_inserted_into_the_prefix_tree_then_the_proper_tree_is_built() {
        let mut root = Node::new();
        let mut cp1 = CompiledPattern::new();
        cp1.push(NodeType::Literal("app".to_owned()));
        cp1.push(NodeType::Parser(Box::new(SetParser::new("01234"))));
        cp1.push(NodeType::Literal("le".to_owned()));

        let mut cp2 = CompiledPattern::new();
        cp2.push(NodeType::Literal("applause".to_owned()));

        root.add_pattern(cp1);
        root.add_pattern(cp2);
    }
}
