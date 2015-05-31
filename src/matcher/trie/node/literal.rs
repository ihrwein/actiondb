use std::cmp::{Ord, Ordering};
use std::borrow::Borrow;
use utils::common_prefix::CommonPrefix;

use matcher::trie::node::Node;

pub struct LiteralNode <'a, 'b> {
    literal: String,
    node: Option<Box<Node<'a, 'b>>>,
}

impl <'a, 'b> LiteralNode<'a, 'b> {
    pub fn new(literal: String) -> LiteralNode<'a, 'b> {
        LiteralNode{ literal: literal,
                     node: None}
    }

    pub fn from_str(literal: &str) -> LiteralNode<'a, 'b> {
        LiteralNode{ literal: literal.to_string(),
                     node: None}
    }

    pub fn literal(&self) -> &str {
        &self.literal[..]
    }

    pub fn set_node(&mut self, node: Option<Box<Node<'a, 'b>>>) {
        self.node = node;
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
                 literal: &str) -> LiteralNode<'a, 'b> {
        let LiteralNode{ literal: self_literal, node: self_node} = self;

        let common_prefix = literal.rtrunc(literal.len() - common_prefix_len);
        let left_branch = literal.ltrunc(common_prefix_len);
        let right_branch = literal.ltrunc(common_prefix_len);

        let mut node_to_return = LiteralNode::from_str(common_prefix);

        let mut new_node = Box::new(Node::new());
        let mut left_node = LiteralNode::from_str(left_branch);
        let mut right_node = LiteralNode::from_str(right_branch);

        right_node.set_node(self_node);

        new_node.add_literal_node(left_node);
        new_node.add_literal_node(right_node);
        node_to_return.set_node(Some(new_node));
        node_to_return
    }

    fn compare_first_chars(&self, other : &LiteralNode) -> Ordering {
        self.cmp_str(other.literal())
    }
}


impl <'a, 'b> Eq for LiteralNode<'a, 'b> {}

impl <'a, 'b> PartialEq for LiteralNode<'a, 'b> {
    fn eq(&self, other: &Self) -> bool {
        self.compare_first_chars(other) == Ordering::Equal
    }

    fn ne(&self, other: &Self) -> bool {
        self.compare_first_chars(other) != Ordering::Equal
    }
}

impl <'a, 'b> Ord for LiteralNode<'a, 'b> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.compare_first_chars(other)
    }
}

impl <'a, 'b> PartialOrd for LiteralNode<'a, 'b> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.compare_first_chars(other))
    }
}

#[cfg(test)]
mod test {
    use matcher::trie::node::LiteralNode;
    use std::cmp::Ordering;

    #[test]
    fn given_literal_node_when_it_is_compared_to_an_other_literal_node_then_only_their_first_chars_are_checked() {
        let alpha = LiteralNode::new("alpha".to_string());
        let beta = LiteralNode::new("beta".to_string());
        let aleph = LiteralNode::from_str("aleph");
        let empty = LiteralNode::from_str("");

        assert_eq!(alpha.cmp(&beta), Ordering::Less);
        assert_eq!(alpha.cmp(&aleph), Ordering::Equal);
        assert_eq!(beta.cmp(&alpha), Ordering::Greater);
        assert_eq!(alpha.cmp(&empty), Ordering::Greater);
        assert_eq!(empty.cmp(&alpha), Ordering::Less);
    }
}
