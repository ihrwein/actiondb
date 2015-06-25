use matcher::trie::node::{Node, TokenType};
use matcher::trie::node::{CompiledPattern};
use matcher::trie::TrieOperations;

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
            TokenType::Literal(literal) => {
                ParserTrie::insert_recurse(node.insert_literal(&literal), pattern)
            },
            TokenType::Parser(parser) => {
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
    use matcher::trie::node::{CompiledPattern, TokenType};
    use matcher::trie::ParserTrie;
    use parsers::{SetParser};

    #[test]
    fn test_given_patterns_when_inserted_into_the_prefix_tree_then_the_proper_tree_is_built() {
        let mut trie = ParserTrie::new();
        let mut cp1 = CompiledPattern::new();
        cp1.push(TokenType::Literal("app".to_string()));
        cp1.push(TokenType::Parser(Box::new(SetParser::from_str("test", "01234"))));
        cp1.push(TokenType::Literal("le".to_string()));

        trie.insert(cp1);
        println!("{:?}", &trie);

        let mut cp2 = CompiledPattern::new();
        cp2.push(TokenType::Literal("appletree".to_string()));
        trie.insert(cp2);
        println!("{:?}", &trie);
    }
}
