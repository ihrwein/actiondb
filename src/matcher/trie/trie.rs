use matcher::trie::node::Node;

pub struct PatternTrie<'a, 'b> {
    root: Node<'a, 'b>
}
