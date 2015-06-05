mod literal;
mod parser;
mod node;

use std::borrow::ToOwned;

use parsers::{Parser, SetParser};
pub use self::node::MatchResult;
pub use self::node::CompiledPattern;
pub use self::node::NodeType;
pub use self::node::Node;
pub use self::literal::{LiteralNode};
pub use self::parser::ParserNode;

#[cfg(test)]
mod test {
    use matcher::trie::node::Node;

    #[test]
    fn test_given_node_when_literals_are_inserted_in_chains_then_they_can_be_looked_up() {
        let mut node = Node::new();

        let _ = node.insert_literal("appl").insert_literal("et").insert_literal("ree");
        println!("{:?}", &node);
        assert_eq!(node.lookup_literal("applet").is_ok(), true);
        assert_eq!(node.lookup_literal("appletree").is_ok(), true);
        assert_eq!(node.lookup_literal("appletre").is_ok(), false);
    }
}
