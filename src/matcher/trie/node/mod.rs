mod literal;
mod parser;
mod node;

use std::borrow::ToOwned;

use parsers::{Parser, SetParser};
pub use self::node::MatchResult;
pub use self::node::CompiledPattern;
pub use self::node::NodeType;
pub use self::node::Node;
pub use self::literal::LiteralNode;
pub use self::parser::ParserNode;


#[cfg(test)]
mod test {
    use matcher::trie::node::{CompiledPattern, NodeType, Node};
    use parsers::{Parser, SetParser};

    #[test]
    fn test_given_patterns_when_inserted_into_the_prefix_tree_then_the_proper_tree_is_built() {
        let mut root = Node::new();
        let mut cp1 = CompiledPattern::new();
        cp1.push(NodeType::Literal("app"));
        cp1.push(NodeType::Parser(Box::new(SetParser::new("01234"))));
        cp1.push(NodeType::Literal("le"));

        let mut cp2 = CompiledPattern::new();
        cp2.push(NodeType::Literal("applause"));

        root.insert(&cp1);
        root.insert(&cp2);
    }
}
