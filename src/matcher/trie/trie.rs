use matcher::trie::node::Node;
use matcher::compiled_pattern::TokenType;
use matcher::trie::TrieElement;
use matcher::result::MatchResult;
use matcher::Pattern;

#[derive(Debug, Clone)]
pub struct TrieMatcher {
    root: Node,
}

impl TrieMatcher {
    pub fn new() -> TrieMatcher {
        TrieMatcher { root: Node::new() }
    }

    pub fn insert(&mut self, pattern: Pattern) {
        TrieMatcher::insert_pattern(&mut self.root, pattern)
    }

    pub fn parse<'a, 'b>(&'a self, text: &'b str) -> Option<MatchResult<'a, 'b>> {
        self.root.parse(text)
    }

    fn insert_pattern<T>(node: &mut T, mut pattern: Pattern)
        where T: TrieElement
    {
        if let Some(token) = pattern.pop_first_token() {
            match token {
                TokenType::Literal(literal) => {
                    let mut node = node.insert_literal(&literal);
                    TrieMatcher::insert_pattern(node, pattern)
                }
                TokenType::Parser(parser) => {
                    let mut node = node.insert_parser(parser);
                    TrieMatcher::insert_pattern(node, pattern)
                }
            }
        } else {
            node.set_pattern(pattern);
        }
    }
}

#[cfg(test)]
mod test {
    use matcher::compiled_pattern::CompiledPatternBuilder;
    use matcher::trie::TrieMatcher;
    use parsers::{SetParser, IntParser, GreedyParser};
    use matcher::pattern::Pattern;

    #[test]
    fn test_given_patterns_when_inserted_into_the_prefix_tree_then_the_proper_tree_is_built() {
        let mut trie = TrieMatcher::new();
        let cp1 = CompiledPatternBuilder::new()
                      .literal("app")
                      .parser(Box::new(SetParser::from_str("test", "01234")))
                      .literal("le")
                      .build();
        {
            let mut pattern = Pattern::with_random_uuid();
            pattern.set_pattern(cp1);
            trie.insert(pattern);
            println!("{:?}", &trie);
        }
        {
            let mut pattern = Pattern::with_random_uuid();
            let cp2 = CompiledPatternBuilder::new()
                          .literal("appletree")
                          .build();
            pattern.set_pattern(cp2);
            trie.insert(pattern);
        }
    }

    #[test]
    fn test_given_pattern_when_inserted_into_the_parser_tree_then_the_pattern_is_stored_in_the_leaf
        () {
        let mut trie = TrieMatcher::new();
        let cp1 = CompiledPatternBuilder::new()
                      .literal("app")
                      .parser(Box::new(SetParser::from_str("test", "01234")))
                      .literal("le")
                      .build();
        let mut pattern = Pattern::with_random_uuid();
        pattern.set_pattern(cp1);

        trie.insert(pattern);
        println!("{:?}", &trie);

        match trie.parse("app23le") {
            Some(res) => {
                println!("{:?}", res);
                assert_eq!(res.values(), &btreemap!["test" => "23"]);
            }
            None => unreachable!(),
        }
    }

    #[test]
    fn test_given_pattern_with_two_neighbouring_parser_when_the_pattern_is_inserted_into_the_trie_then_everything_is_ok
        () {
        let mut trie = TrieMatcher::new();
        let expected = btreemap!["test" => "ccc", "test2" => "12", "test3" => "le"];
        let cp1 = CompiledPatternBuilder::new()
                      .literal("app")
                      .parser(Box::new(SetParser::from_str("test", "abcd")))
                      .parser(Box::new(IntParser::from_str("test2")))
                      .parser(Box::new(GreedyParser::with_name("test3".to_string())))
                      .build();
        let mut pattern = Pattern::with_random_uuid();
        pattern.set_pattern(cp1);

        trie.insert(pattern);
        println!("{:?}", &trie);

        match trie.parse("appccc12le") {
            Some(res) => {
                let got = res.values().clone();
                println!("{:?}", res);
                assert_eq!(expected, got);
            }
            None => unreachable!(),
        }
    }
}
