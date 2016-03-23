use std::cmp::{Ord, Ordering};
use utils::common_prefix::CommonPrefix;

use matcher::trie::node::SuffixTree;
use matcher::Pattern;

use matcher::trie::node::interface::{Entry, LiteralEntry};

#[derive(Debug, Clone)]
pub struct LiteralNode {
    literal: String,
    has_value: bool,
    pattern: Option<Pattern>,
    node: Option<SuffixTree>,
}

impl LiteralNode {
    pub fn new<S: Into<String>>(literal: S) -> LiteralNode {
        LiteralNode {
            literal: literal.into(),
            has_value: false,
            pattern: None,
            node: None,
        }
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

    pub fn set_node(&mut self, node: Option<SuffixTree>) {
        self.node = node;
    }

    pub fn node_mut(&mut self) -> Option<&mut SuffixTree> {
        self.node.as_mut()
    }

    pub fn node(&self) -> Option<&SuffixTree> {
        self.node.as_ref()
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

    pub fn split(&mut self, common_prefix_len: usize, literal: &str) {
        let common_prefix = literal.rtrunc(literal.len() - common_prefix_len);
        trace!("split(): common_prefix = {}", common_prefix);
        let mut new_node = SuffixTree::new();

        let mut left_node = {
            let left_branch = literal.ltrunc(common_prefix_len);
            trace!("split(): left_branch = {}", left_branch);
            LiteralNode::new(left_branch)
        };
        let mut right_node = {
            let right_branch = self.literal().ltrunc(common_prefix_len);
            trace!("split(): right_branch = {}", right_branch);
            LiteralNode::new(right_branch)
        };
        left_node.set_has_value(true);

        right_node.set_node(self.node.take());
        right_node.set_has_value(self.has_value);

        if let Some(pattern) = self.pattern.take() {
            right_node.set_pattern(Some(pattern));
        }

        new_node.add_literal_node(left_node);
        new_node.add_literal_node(right_node);
        self.set_node(Some(new_node));
        self.has_value = false;
        self.literal = common_prefix.to_owned();
    }

    pub fn is_leaf(&self) -> bool {
        self.node.is_none()
    }

    fn compare_first_chars(&self, other: &LiteralNode) -> Ordering {
        self.cmp_str(other.literal())
    }
}

impl Entry for LiteralNode {
    type ST = SuffixTree;
    fn pattern(&self) -> Option<&Pattern> {
        self.pattern.as_ref()
    }
    fn set_pattern(&mut self, pattern: Option<Pattern>) {
        self.pattern = pattern;
    }
    fn child(&self) -> Option<&SuffixTree> {
        self.node.as_ref()
    }
    fn child_mut(&mut self) -> Option<&mut SuffixTree> {
        self.node.as_mut()
    }
    fn set_child(&mut self, child: Option<Self::ST>) {
        self.node = child;
    }
}

impl LiteralEntry for LiteralNode {
    fn literal(&self) -> &String {
        &self.literal
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
        let aleph = LiteralNode::new("aleph");

        assert_eq!(aleph.is_leaf(), true);
    }

    #[test]
    fn given_literal_node_when_it_is_compared_to_an_other_literal_node_then_only_their_first_chars_are_checked
        () {
        let alpha = LiteralNode::new("alpha");
        let beta = LiteralNode::new("beta");
        let aleph = LiteralNode::new("aleph");
        let a = LiteralNode::new("a");
        let empty = LiteralNode::new("");

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
