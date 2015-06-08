use matcher::trie::node::{Node, NodeType};
use matcher::trie::node::{CompiledPattern};
use matcher::trie::TrieOperations;
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct ParserTrie {
    root: Node,
}

impl ParserTrie {
    pub fn new() -> ParserTrie {
        ParserTrie{ root: Node::new() }
    }

    pub fn insert(&mut self, pattern: CompiledPattern) -> &mut TrieOperations {
        ParserTrie::insert_recurse(&mut self.root, pattern)
    }

    pub fn parse<'a, 'b>(&'a self, text: &'b str) -> Option<Vec<(&'a str, &'b str)>> {
        self.root.parse(text)
    }

    fn insert_not_empty_pattern<'a>(node: &'a mut TrieOperations, mut pattern: CompiledPattern) -> &'a mut TrieOperations {
        let item = pattern.remove(0);
        match item {
            NodeType::Literal(literal) => {
                ParserTrie::insert_recurse(node.insert_literal(literal), pattern)
            },
            NodeType::Parser(parser) => {
                ParserTrie::insert_recurse(node.insert_parser(parser), pattern)
            }
        }
    }

    fn insert_recurse<'a>(node: &'a mut TrieOperations, pattern: CompiledPattern) -> &'a mut TrieOperations {
        if pattern.is_empty() {
            node
        } else {
            ParserTrie::insert_not_empty_pattern(node, pattern)
        }
    }
}

#[cfg(test)]
mod test {
    use matcher::trie::node::{CompiledPattern, NodeType};
    use matcher::trie::ParserTrie;
    use parsers::{SetParser};

    #[test]
    fn test_given_patterns_when_inserted_into_the_prefix_tree_then_the_proper_tree_is_built() {
        let mut trie = ParserTrie::new();
        let mut cp1 = CompiledPattern::new();
        cp1.push(NodeType::Literal("app"));
        cp1.push(NodeType::Parser(Box::new(SetParser::new("test", "01234"))));
        cp1.push(NodeType::Literal("le"));

        trie.insert(cp1);
        println!("{:?}", &trie);

        let mut cp2 = CompiledPattern::new();
        cp2.push(NodeType::Literal("appletree"));
        trie.insert(cp2);
        println!("{:?}", &trie);
    }
}
