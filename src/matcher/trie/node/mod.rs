use parsers::{Parser, ParseResult};
use utils::{SortedVec, CommonPrefix};
use matcher::pattern::Pattern;
use matcher::result::MatchResult;
use matcher::compiled_pattern::TokenType;

mod literal;
mod parser;
pub mod interface;

use self::interface::Entry;

pub use self::literal::LiteralNode;
pub use self::parser::ParserNode;

#[derive(Debug, Clone)]
pub struct SuffixTree {
    literal_children: SortedVec<LiteralNode>,
    parser_children: Vec<ParserNode>,
}

enum LiteralLookupResult<'a> {
    Found(usize),
    NotFound,
    GoDown(usize, &'a str),
}

impl SuffixTree {
    pub fn new() -> SuffixTree {
        SuffixTree::default()
    }

    pub fn add_literal_node(&mut self, lnode: LiteralNode) {
        self.literal_children.push(lnode);
    }

    pub fn is_leaf(&self) -> bool {
        self.literal_children.is_empty() && self.parser_children.is_empty()
    }


    // If a literal isn't found the last SuffixTree instance and the remaining
    // length of
    // the literal will be returned
    // if the literal is in the trie, we return the last SuffixTree instance and the
    // index of the LiteralNode which contains the literal
    pub fn lookup_literal_mut(&mut self,
                              literal: &str)
                              -> Result<(&mut SuffixTree, usize), (&mut SuffixTree, usize)> {
        match self.search(literal) {
            LiteralLookupResult::Found(pos) => {
                Ok((self, pos))
            }
            LiteralLookupResult::NotFound => {
                Err((self, literal.len()))
            }
            LiteralLookupResult::GoDown(pos, truncated_literal) => {
                self.literal_children
                    .get_mut(pos)
                    .unwrap()
                    .node_mut()
                    .unwrap()
                    .lookup_literal_mut(truncated_literal)
            }
        }
    }

    // It's the same as lookup_literal_mut() without the muts
    pub fn lookup_literal(&self,
                          literal: &str)
                          -> Result<(&SuffixTree, usize), (&SuffixTree, usize)> {
        match self.search(literal) {
            LiteralLookupResult::Found(pos) => {
                Ok((self, pos))
            }
            LiteralLookupResult::NotFound => {
                Err((self, literal.len()))
            }
            LiteralLookupResult::GoDown(pos, truncated_literal) => {
                self.literal_children
                    .get(pos)
                    .unwrap()
                    .node()
                    .unwrap()
                    .lookup_literal(truncated_literal)
            }
        }
    }

    fn search<'a, 'b>(&'a self, literal: &'b str) -> LiteralLookupResult<'b> {
        trace!("search(): stepped in");
        trace!("search(): #children = {}", self.literal_children.len());
        trace!("search(): #pchildren = {}", self.parser_children.len());
        let cmp_str = |probe: &LiteralNode| probe.cmp_str(literal);

        match self.literal_children.binary_search_by(&cmp_str) {
            Ok(pos) => {
                self.search_prefix_is_found(literal, pos)
            }
            Err(_) => {
                trace!("search(): there is no common prefix with this literal");
                trace!("search(): literal = {}", literal);
                trace!("search(): #children = {}", self.literal_children.len());
                trace!("search(): #pchildren = {}", self.parser_children.len());
                LiteralLookupResult::NotFound
            }
        }
    }

    fn search_prefix_is_found<'a, 'b>(&'a self,
                                      literal: &'b str,
                                      pos: usize)
                                      -> LiteralLookupResult<'b> {
        if self.literal_children.get(pos).unwrap().is_leaf() {
            self.search_prefix_is_found_and_node_is_leaf(literal, pos)
        } else {
            self.search_prefix_is_found_and_node_is_not_leaf(literal, pos)
        }
    }

    fn search_prefix_is_found_and_node_is_leaf<'a, 'b>(&'a self,
                                                       literal: &'b str,
                                                       pos: usize)
                                                       -> LiteralLookupResult<'b> {
        trace!("search(): we found a prefix, but it's a leaf");
        if self.literal_children.get(pos).unwrap().literal() == literal {
            trace!("search(): we got it");
            LiteralLookupResult::Found(pos)
        } else {
            trace!("search(): we didn't get it");
            LiteralLookupResult::NotFound
        }
    }

    fn search_prefix_is_found_and_node_is_not_leaf<'a, 'b>(&'a self,
                                                           literal: &'b str,
                                                           pos: usize)
                                                           -> LiteralLookupResult<'b> {
        let literal_node = self.literal_children.get(pos).unwrap();
        let common_prefix_len = literal_node.literal().common_prefix_len(literal);

        if common_prefix_len < literal_node.literal().len() {
            return LiteralLookupResult::NotFound;
        }

        if literal_node.has_value() && (literal.is_empty() || common_prefix_len == literal.len()) {
            trace!("search(): we got it");
            return LiteralLookupResult::Found(pos);
        }

        if let Some(_) = literal_node.node() {
            trace!("search(): literal len = {}", literal.len());
            trace!("search(): common_prefix_len = {}", common_prefix_len);
            trace!("search(): going deeper");
            return LiteralLookupResult::GoDown(pos, literal.ltrunc(common_prefix_len));
        } else {
            unreachable!();
        }
    }

    pub fn parse<'a, 'b>(&'a self, text: &'b str) -> Option<MatchResult<'a, 'b>> {
        trace!("parse(): text = {}", text);
        match self.lookup_literal(text) {
            Ok((node, pos)) => {
                trace!("{:?}", node);
                let child = node.literal_children
                                .get(pos)
                                .expect("Failed to get a looked up child");
                SuffixTree::create_match_result_if_child_is_leaf(child)
            }
            Err((node, remaining_len)) => {
                let text = text.ltrunc(text.len() - remaining_len);
                trace!("parse(): text = {}", text);
                trace!("parse(): #parser_children = {}", node.parser_children.len());
                node.parse_with_parsers(text)
            }
        }
    }

    fn create_match_result_if_child_is_leaf<'a, 'b>(child: &'a LiteralNode)
                                                    -> Option<MatchResult<'a, 'b>> {
        if let Some(pattern) = child.pattern() {
            let result = MatchResult::new(pattern);
            Some(result)
        } else {
            info!("Early matching message: the message was too short to reach a leaf");
            None
        }
    }

    fn parse_with_parsers<'a, 'b>(&'a self, text: &'b str) -> Option<MatchResult<'a, 'b>> {
        for i in &self.parser_children {
            trace!("parse(): testing parser");

            if let Some(result) = i.parse(text) {
                return Some(result);
            }
        }
        None
    }

    pub fn parse_then_push_kvpair<'a, 'b>(&'a self,
                                          text: &'b str,
                                          kvpair: ParseResult<'a, 'b>)
                                          -> Option<MatchResult<'a, 'b>> {
        if let Some(mut result) = self.parse(text) {
            result.insert(kvpair);
            Some(result)
        } else {
            None
        }
    }

    fn lookup_parser(&mut self, parser: &Parser) -> Option<usize> {
        self.parser_children.iter().position(|ref x| x.parser().hash_os() == parser.hash_os())
    }

    fn insert_literal_tail(&mut self, tail: &str) -> &mut LiteralNode {
        trace!("insert_literal_tail(): tail = {}", tail);
        let cmp_str = |probe: &LiteralNode| probe.cmp_str(tail);

        match self.literal_children.binary_search_by(&cmp_str) {
            Ok(pos) => {
                if let Some(common_prefix_len) = self.literal_children
                                                     .get(pos)
                                                     .unwrap()
                                                     .literal()
                                                     .has_common_prefix(tail) {
                    trace!("insert_literal_tail(): common_prefix_len = {}",
                           common_prefix_len);
                    let hit = self.literal_children
                                  .get_mut(pos)
                                  .expect("Failed to remove a literal child");
                    trace!("insert_literal_tail(): to_be_split = {}", hit.literal());
                    trace!("insert_literal_tail(): tail = {}", tail);
                    hit.split(common_prefix_len, tail);
                    let suffix = tail.ltrunc(common_prefix_len);
                    hit.node_mut().expect("Failed to get").lookup_freshly_inserted_literal(suffix)
                } else {
                    unreachable!()
                }
            }
            Err(pos) => {
                trace!("insert_literal_tail(): creating new literal node from tail = {}",
                       tail);
                let mut new_node = LiteralNode::new(tail);
                new_node.set_has_value(true);
                self.add_literal_node(new_node);
                self.literal_children.get_mut(pos).unwrap()
            }
        }
    }

    fn lookup_freshly_inserted_literal(&mut self, literal: &str) -> &mut LiteralNode {
        let (node, pos) = self.lookup_literal_mut(literal)
                              .ok()
                              .unwrap();
        node.literal_children.get_mut(pos).unwrap()
    }

    pub fn insert_literal(&mut self, literal: &str) -> &mut LiteralNode {
        trace!("inserting literal: '{}'", literal);

        match self.lookup_literal_mut(literal) {
            Ok((node, index)) => {
                trace!("insert_literal(): it was already inserted");
                node.literal_children.get_mut(index).unwrap()
            }
            Err((node, rem_len)) => {
                trace!("INSERTING({}), remaining len: {}", literal, rem_len);
                let tail = literal.ltrunc(literal.len() - rem_len);
                node.insert_literal_tail(tail)
            }
        }
    }

    pub fn insert_parser(&mut self, parser: Box<Parser>) -> &mut ParserNode {
        if let Some(item) = self.lookup_parser(&*parser) {
            self.parser_children.get_mut(item).unwrap()
        } else {
            let pnode = ParserNode::new(parser);
            self.parser_children.push(pnode);
            self.parser_children.last_mut().unwrap()
        }
    }
}

impl Default for SuffixTree {
    fn default() -> Self {
        SuffixTree {
            literal_children: SortedVec::new(),
            parser_children: Vec::new(),
        }
    }
}

impl self::interface::SuffixTree for SuffixTree {
    fn new() -> Self {
        SuffixTree {
            literal_children: SortedVec::new(),
            parser_children: Vec::new(),
        }
    }
    fn insert(&mut self, mut pattern: Pattern) {
        if let Some(token) = pattern.pop_first_token() {
            let mut entry: &mut Entry<ST = Self> = match token {
                TokenType::Literal(literal) => {
                    self.insert_literal(&literal)
                }
                TokenType::Parser(parser) => {
                    self.insert_parser(parser)
                }
            };
            entry.insert(pattern);
        }
    }

}

#[cfg(test)]
mod test {
    use matcher::trie::node::SuffixTree;
    use parsers::{SetParser, IntParser, GreedyParser};
    use matcher::compiled_pattern::CompiledPatternBuilder;
    use matcher::pattern::Pattern;
    use matcher::trie::node::interface::SuffixTree as STree;

    use std::iter::FromIterator;
    use std::collections::BTreeMap;

    #[test]
    fn given_empty_trie_when_literals_are_inserted_then_they_can_be_looked_up() {
        let mut node = SuffixTree::new();

        let _ = node.insert_literal("alma");
        assert_eq!(node.lookup_literal("alma").is_ok(), true);
        assert_eq!(node.lookup_literal("alm").is_err(), true);
        let _ = node.insert_literal("alm");
        assert_eq!(node.lookup_literal("alm").is_ok(), true);
        assert_eq!(node.literal_children.len(), 1);
    }

    #[test]
    fn test_given_empty_trie_when_literals_are_inserted_the_child_counts_are_right() {
        let mut node = SuffixTree::new();

        let _ = node.insert_literal("alma");
        let _ = node.insert_literal("alm");
        assert_eq!(node.literal_children.len(), 1);
        assert_eq!(node.lookup_literal("alma").is_ok(), true);
        assert_eq!(node.lookup_literal("alm").ok().unwrap().0.literal_children.len(),
                   2);
    }

    #[test]
    fn test_given_empty_trie_when_literals_are_inserted_the_nodes_are_split_on_the_right_place() {
        let mut node = SuffixTree::new();

        let _ = node.insert_literal("alm");
        let _ = node.insert_literal("alma");
        let _ = node.insert_literal("ai");
        assert_eq!(node.literal_children.len(), 1);
        assert_eq!(node.lookup_literal("alma").is_ok(), true);
        assert_eq!(node.lookup_literal("alm").ok().unwrap().0.literal_children.len(),
                   2);
        assert_eq!(node.lookup_literal("ai").ok().unwrap().0.literal_children.len(),
                   2);
    }

    #[test]
    fn test_given_trie_when_literals_are_looked_up_then_the_edges_in_the_trie_are_not_counted_as_literals
        () {
        let mut node = SuffixTree::new();

        let _ = node.insert_literal("alm");
        let _ = node.insert_literal("ala");
        assert_eq!(node.lookup_literal("al").is_err(), true);
    }

    #[test]
    fn test_given_node_when_the_same_parsers_are_inserted_then_they_are_merged_into_one_parsernode
        () {
        let mut node = SuffixTree::new();

        let _ = node.insert_parser(Box::new(SetParser::from_str("test", "ab")));
        let _ = node.insert_parser(Box::new(SetParser::from_str("test", "ab")));

        assert_eq!(node.parser_children.len(), 1);
    }

    #[test]
    fn test_given_node_when_different_parsers_are_inserted_then_they_are_not_merged() {
        let mut node = SuffixTree::new();

        let _ = node.insert_parser(Box::new(SetParser::from_str("test", "ab")));
        let _ = node.insert_parser(Box::new(SetParser::from_str("test", "a")));

        assert_eq!(node.parser_children.len(), 2);
    }

    fn create_parser_trie() -> SuffixTree {
        let mut root = SuffixTree::new();
        let cp_1 = CompiledPatternBuilder::new()
                      .literal("app")
                      .parser(Box::new(SetParser::from_str("test", "01234")))
                      .literal("le")
                      .build();
        let cp_2 = CompiledPatternBuilder::new()
                      .literal("appletree")
                      .build();
        let cp_3 = CompiledPatternBuilder::new()
                      .literal("apple")
                      .build();
        let mut pattern_1 = Pattern::with_random_uuid();
        pattern_1.set_pattern(cp_1);
        let mut pattern_2 = Pattern::with_random_uuid();
        pattern_2.set_pattern(cp_2);
        let mut pattern_3 = Pattern::with_random_uuid();
        pattern_3.set_pattern(cp_3);

        root.insert(pattern_1);
        root.insert(pattern_2);
        root.insert(pattern_3);

        root
    }

    #[test]
    fn test_given_parser_trie_when_some_patterns_are_inserted_then_texts_can_be_parsed() {
        let root = create_parser_trie();

        println!("root: {:?}", &root);
        {
            let result = root.parse("bamboo");
            assert_eq!(result.is_none(), true);
        }
        {
            let result = root.parse("app42le");
            assert_eq!(result.is_some(), true);
        }
    }

    #[test]
    fn test_given_parser_trie_when_some_patterns_are_inserted_then_fully_matching_literals_are_returned_as_empty_vectors
        () {
        let root = create_parser_trie();
        println!("root: {:?}", &root);
        {
            let result = root.parse("appletree");
            assert_eq!(result.unwrap().values().is_empty(), true);
        }
    }

    #[test]
    fn test_given_parser_trie_when_some_patterns_are_inserted_then_literal_matches_have_precedence_over_parser_matches
        () {
        let root = create_parser_trie();
        println!("root: {:?}", &root);
        {
            let result = root.parse("apple");
            assert_eq!(result.unwrap().values().is_empty(), true);
        }
    }

    fn create_complex_parser_trie() -> SuffixTree {
        let mut root = SuffixTree::new();
        let cp_1 = CompiledPatternBuilder::new()
                      .literal("app")
                      .parser(Box::new(SetParser::from_str("middle", "01234")))
                      .literal("letree")
                      .parser(Box::new(SetParser::from_str("end", "012")))
                      .build();
        let cp_2 = CompiledPatternBuilder::new()
                      .literal("app")
                      .parser(Box::new(SetParser::from_str("middle", "01234")))
                      .literal("letree")
                      .parser(Box::new(SetParser::from_str("end", "0123")))
                      .build();
        let cp_3 = CompiledPatternBuilder::new()
                      .literal("bamboo")
                      .build();
        let cp_4 = CompiledPatternBuilder::new()
                      .literal("bamba")
                      .build();

        let mut pattern_1 = Pattern::with_random_uuid();
        pattern_1.set_pattern(cp_1);
        let mut pattern2 = Pattern::with_random_uuid();
        pattern2.set_pattern(cp_2);
        let mut pattern_3 = Pattern::with_random_uuid();
        pattern_3.set_pattern(cp_3);
        let mut pattern_4 = Pattern::with_random_uuid();
        pattern_4.set_pattern(cp_4);

        root.insert(pattern_1);
        root.insert(pattern2);
        root.insert(pattern_3);
        root.insert(pattern_4);

        root
    }

    #[test]
    fn test_given_parser_trie_when_a_parser_is_not_matched_then_the_parser_stack_is_unwind_so_an_untried_parser_is_tried
        () {
        let root = create_complex_parser_trie();
        println!("root: {:?}", &root);
        {
            let result = root.parse("app42letree123");
            let expected = BTreeMap::from_iter(vec![("end", "123"), ("middle", "42")].into_iter());
            assert_eq!(result.unwrap().values(), &expected);
        }
    }

    #[test]
    fn test_given_parser_trie_when_the_to_be_parsed_literal_is_not_matched_then_the_parse_result_is_none
        () {
        let root = create_complex_parser_trie();
        println!("root: {:?}", &root);
        {
            let kvpairs = root.parse("lorem ipsum");
            assert_eq!(kvpairs.is_none(), true);
        }
    }

    #[test]
    fn test_given_parser_trie_when_the_to_be_parsed_literal_is_a_prefix_in_the_tree_then_the_parse_result_is_none
        () {
        let root = create_complex_parser_trie();
        println!("root: {:?}", &root);
        {
            let kvpairs = root.parse("bamb");
            assert_eq!(kvpairs.is_none(), true);
        }
    }

    #[test]
    fn test_given_empty_parser_node_when_it_is_used_for_parsing_then_it_returns_none() {
        let root = SuffixTree::new();
        println!("root: {:?}", &root);
        {
            let kvpairs = root.parse("bamb");
            assert_eq!(kvpairs.is_none(), true);
        }
        {
            let kvpairs = root.parse("");
            assert_eq!(kvpairs.is_none(), true);
        }
    }

    #[test]
    fn test_given_node_when_the_message_is_too_short_we_do_not_try_to_unwrap_a_childs_pattern() {
        let mut root = SuffixTree::new();
        let cp_1 = CompiledPatternBuilder::new()
                      .literal("app")
                      .parser(Box::new(SetParser::from_str("middle", "01234")))
                      .literal("x")
                      .parser(Box::new(SetParser::from_str("space", " ")))
                      .build();

        let mut pattern_1 = Pattern::with_random_uuid();
        pattern_1.set_pattern(cp_1);
        root.insert(pattern_1);

        let kvpairs = root.parse("app12x");
        assert_eq!(kvpairs.is_none(), true);
        let kvpairs = root.parse("app12x ");
        assert_eq!(kvpairs.is_some(), true);
    }


    #[test]
    fn test_given_patterns_when_inserted_into_the_prefix_tree_then_the_proper_tree_is_built() {
        let mut trie = SuffixTree::new();
        let cp_1 = CompiledPatternBuilder::new()
                      .literal("app")
                      .parser(Box::new(SetParser::from_str("test", "01234")))
                      .literal("le")
                      .build();
        {
            let mut pattern = Pattern::with_random_uuid();
            pattern.set_pattern(cp_1);
            trie.insert(pattern);
            println!("{:?}", &trie);
        }
        {
            let mut pattern = Pattern::with_random_uuid();
            let cp_2 = CompiledPatternBuilder::new()
                          .literal("appletree")
                          .build();
            pattern.set_pattern(cp_2);
            trie.insert(pattern);
        }
    }

    #[test]
    fn test_given_pattern_when_inserted_into_the_parser_tree_then_the_pattern_is_stored_in_the_leaf
        () {
        let mut trie = SuffixTree::new();
        let cp_1 = CompiledPatternBuilder::new()
                      .literal("app")
                      .parser(Box::new(SetParser::from_str("test", "01234")))
                      .literal("le")
                      .build();
        let mut pattern = Pattern::with_random_uuid();
        pattern.set_pattern(cp_1);

        trie.insert(pattern);
        println!("{:?}", &trie);
        let expected = BTreeMap::from_iter(vec![("test", "23")].into_iter());
        match trie.parse("app23le") {
            Some(res) => {
                println!("{:?}", res);
                assert_eq!(res.values(), &expected);
            }
            None => unreachable!(),
        }
    }

    #[test]
    fn test_given_pattern_with_two_neighbouring_parser_when_the_pattern_is_inserted_into_the_trie_then_everything_is_ok
        () {
        let mut trie = SuffixTree::new();
        let expected = BTreeMap::from_iter(vec![("test", "ccc"), ("test2", "12"), ("test3", "le")].into_iter());
        let cp_1 = CompiledPatternBuilder::new()
                      .literal("app")
                      .parser(Box::new(SetParser::from_str("test", "abcd")))
                      .parser(Box::new(IntParser::with_name("test2")))
                      .parser(Box::new(GreedyParser::with_name("test3".to_owned())))
                      .build();
        let mut pattern = Pattern::with_random_uuid();
        pattern.set_pattern(cp_1);

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
