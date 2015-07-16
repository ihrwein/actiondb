use std::cmp::{Ord, Ordering};
use utils::common_prefix::CommonPrefix;

use matcher::trie::node::{Node, ParserNode};
use matcher::trie::{HasPattern, TrieOperations};
use matcher::Pattern;
use parsers::Parser;

#[derive(Debug, Clone)]
pub struct LiteralNode {
    literal: String,
    has_value: bool,
    pattern: Option<Pattern>,
    node: Option<Box<Node>>,
}

impl LiteralNode {
    pub fn new(literal: String) -> LiteralNode {
        LiteralNode{ literal: literal,
                     has_value: false,
                     pattern: None,
                     node: None}
    }

    pub fn from_str(literal: &str) -> LiteralNode {
        LiteralNode::new(literal.to_string())
    }

    pub fn literal(&self) -> &str {
        &self.literal[..]
    }

    pub fn has_value(&self) -> bool {
        self.has_value
    }

    pub fn set_has_value(&mut self, has_value: bool) {
        self.has_value = has_value;
    }

    pub fn set_node(&mut self, node: Option<Box<Node>>) {
        self.node = node;
    }

    pub fn node_mut(&mut self) -> Option<&mut Node> {
        match self.node {
            Some(ref mut boxed_node) => {
                Some(boxed_node)
            },
            None => None
        }
    }

    pub fn node(&self) -> Option<&Node> {
        match self.node {
            Some(ref boxed_node) => {
                Some(boxed_node)
            },
            None => None
        }
    }

    pub fn cmp_str(&self, other: &str) -> Ordering {
        if self.literal.is_empty() && other.is_empty() {
            Ordering::Equal
        } else if self.literal.is_empty() {
            Ordering::Less
        } else if other.is_empty() {
            Ordering::Greater
        } else {
            self.literal[0..1].cmp(&other[0..1])
        }
    }

    pub fn split(self,
                 common_prefix_len: usize,
                 literal: &str) -> LiteralNode {
        let LiteralNode{ literal: self_literal,
                         has_value: self_has_value,
                         pattern: self_pattern,
                         node: self_node} = self;

        let common_prefix = literal.rtrunc(literal.len() - common_prefix_len);
        trace!("split(): common_prefix = {}", common_prefix);
        let left_branch = literal.ltrunc(common_prefix_len);
        let right_branch = self_literal.ltrunc(common_prefix_len);
        trace!("split(): left_branch = {}", left_branch);
        trace!("split(): right_branch = {}", right_branch);

        let mut node_to_return = LiteralNode::from_str(common_prefix);

        let mut new_node = Box::new(Node::new());
        let mut left_node = LiteralNode::from_str(left_branch);
        left_node.set_has_value(true);
        let mut right_node = LiteralNode::from_str(right_branch);

        right_node.set_node(self_node);
        right_node.set_has_value(self_has_value);

        if let Some(pattern) = self_pattern {
            right_node.set_pattern(pattern);
        }

        new_node.add_literal_node(left_node);
        new_node.add_literal_node(right_node);
        node_to_return.set_node(Some(new_node));
        node_to_return
    }

    pub fn is_leaf(&self) -> bool {
        self.node.is_none()
    }

    fn compare_first_chars(&self, other : &LiteralNode) -> Ordering {
        self.cmp_str(other.literal())
    }
}

impl TrieOperations for LiteralNode {
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

impl HasPattern for LiteralNode {
    fn set_pattern(&mut self, pattern: Pattern) {
        self.pattern = Some(pattern);
    }

    fn pattern(&self) -> Option<&Pattern> {
        self.pattern.as_ref()
    }
}


impl Eq for LiteralNode {}

impl PartialEq for LiteralNode {
    fn eq(&self, other: &Self) -> bool {
        self.compare_first_chars(other) == Ordering::Equal
    }

    fn ne(&self, other: &Self) -> bool {
        self.compare_first_chars(other) != Ordering::Equal
    }
}

impl Ord for LiteralNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.compare_first_chars(other)
    }
}

impl PartialOrd for LiteralNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod test {
    use matcher::trie::node::LiteralNode;
    use std::cmp::Ordering;

    #[test]
    fn given_literal_node_when_its_leafness_is_checked_the_right_result_is_returned() {
        let aleph = LiteralNode::from_str("aleph");

        assert_eq!(aleph.is_leaf(), true);
    }

    #[test]
    fn given_literal_node_when_it_is_compared_to_an_other_literal_node_then_only_their_first_chars_are_checked() {
        let alpha = LiteralNode::new("alpha".to_string());
        let beta = LiteralNode::new("beta".to_string());
        let aleph = LiteralNode::from_str("aleph");
        let a = LiteralNode::from_str("a");
        let empty = LiteralNode::from_str("");

        assert_eq!(alpha.cmp(&beta), Ordering::Less);
        assert_eq!(alpha.cmp(&aleph), Ordering::Equal);
        assert_eq!(beta.cmp(&alpha), Ordering::Greater);
        assert_eq!(alpha.cmp(&empty), Ordering::Greater);
        assert_eq!(empty.cmp(&alpha), Ordering::Less);
        assert_eq!(empty.cmp(&a), Ordering::Less);
        assert_eq!(empty.cmp_str("a"), Ordering::Less);
        assert_eq!(a.cmp(&empty), Ordering::Greater);
    }
}
