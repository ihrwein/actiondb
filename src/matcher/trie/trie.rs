use matcher::trie::node::{Node, NodeType, LiteralNode};
use matcher::trie::node::CompiledPattern;
use matcher::trie::TrieOperations;

#[derive(Debug)]
pub struct PatternTrie<'a> {
    root: Node<'a>,
}

impl <'a, 'b> PatternTrie<'a> {
    pub fn new() -> PatternTrie<'a> {
        PatternTrie{ root: Node::new() }
    }

    pub fn insert(&'a mut self, mut pattern: CompiledPattern<'a, 'b>) -> & mut TrieOperations<'a> {
        PatternTrie::insert_recurse(&mut self.root, pattern)
    }

    fn insert_not_empty_pattern(node: &'a mut TrieOperations<'a>, mut pattern: CompiledPattern<'a, 'b>) -> &'a mut TrieOperations<'a> {
        let item = pattern.remove(0);
        match item {
            NodeType::Literal(literal) => {
                PatternTrie::insert_recurse(node.insert_literal(literal), pattern)
            },
            NodeType::Parser(parser) => {
                PatternTrie::insert_recurse(node.insert_parser(parser), pattern)
            }
        }
    }

    fn insert_recurse(node: &'a mut TrieOperations<'a>, pattern: CompiledPattern<'a, 'b>) -> &'a mut TrieOperations<'a> {
        if pattern.is_empty() {
            node
        } else {
            PatternTrie::insert_not_empty_pattern(node, pattern)
        }
    }
}

#[cfg(test)]
mod test {
    use matcher::trie::node::{CompiledPattern, NodeType, Node};
    use matcher::trie::PatternTrie;
    use parsers::{Parser, SetParser};

    #[test]
    fn test_given_patterns_when_inserted_into_the_prefix_tree_then_the_proper_tree_is_built() {
        let mut trie = PatternTrie::new();
        let mut cp1 = CompiledPattern::new();
        cp1.push(NodeType::Literal("app"));
        cp1.push(NodeType::Parser(Box::new(SetParser::new("01234"))));
        cp1.push(NodeType::Literal("le"));

        let _ = trie.insert(cp1);
        //println!("{:?}", &trie);
    }
}
