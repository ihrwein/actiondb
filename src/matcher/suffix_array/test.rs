use super::impls::SuffixTable;
use super::interface::{SuffixArray, LiteralEntry};
use parsers::IntParser;
use matcher::compiled_pattern::CompiledPatternBuilder;
use parsers::SetParser;
use matcher::pattern::Pattern;
use matcher::Matcher;

use std::iter::FromIterator;
use std::collections::BTreeMap;

fn create_populated_suffix_table() -> SuffixTable {
    let mut root = SuffixTable::new();
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
    let mut pattern_2 = Pattern::with_random_uuid();
    pattern_2.set_pattern(cp_2);
    let mut pattern_3 = Pattern::with_random_uuid();
    pattern_3.set_pattern(cp_3);
    let mut pattern4 = Pattern::with_random_uuid();
    pattern4.set_pattern(cp_4);

    root.insert(pattern_1);
    root.insert(pattern_2);
    root.insert(pattern_3);
    root.insert(pattern4);

    root
}

#[test]
fn test_given_parser_trie_when_a_parser_is_not_matched_then_the_parser_stack_is_unwind_so_an_untried_parser_is_tried() {
    let root = create_populated_suffix_table();
    println!("root: {:?}", &root);
    {
        let result = root.parse("app42letree123");
        let expected = BTreeMap::from_iter(vec![("end", "123"), ("middle", "42")].into_iter());

        assert_eq!(&expected, result.expect("Failed to get result").values());
    }
}

#[test]
fn test_given_suffix_array_when_a_parser_entry_is_inserted_it_is_only_added_if_it_is_a_new_parser() {
    let mut root = SuffixTable::new();
    let cp_1 = CompiledPatternBuilder::new()
                .parser(Box::new(IntParser::new()))
                .build();
    let cp_2 = CompiledPatternBuilder::new()
                .parser(Box::new(IntParser::new()))
                .build();

    let mut pattern_1 = Pattern::with_random_uuid();
    pattern_1.set_pattern(cp_1);
    root.insert(pattern_1);

    let mut pattern_2 = Pattern::with_random_uuid();
    pattern_2.set_pattern(cp_2);
    root.insert(pattern_2);

    assert_eq!(true, root.parse("42").is_some());
}

#[test]
fn test_given_suffix_array_when_there_is_no_match_then_the_parsing_is_unsuccessful() {
    let mut root = SuffixTable::new();
    let cp_1 = CompiledPatternBuilder::new()
                .literal("app")
                .parser(Box::new(IntParser::new()))
                .build();
    let mut pattern = Pattern::with_random_uuid();
    pattern.set_pattern(cp_1);
    root.insert(pattern);

    assert_eq!(true, root.parse("XYZ").is_none());
}

#[test]
fn test_given_suffix_array_when_the_match_is_too_short_then_we_dont_panic() {
    let mut root = SuffixTable::new();
    let cp_1 = CompiledPatternBuilder::new()
                .literal("app")
                .parser(Box::new(IntParser::new()))
                .build();
    let mut pattern = Pattern::with_random_uuid();
    pattern.set_pattern(cp_1);
    root.insert(pattern);

    assert_eq!(true, root.parse("XYZ").is_none());
}

#[test]
fn test_given_suffix_array_when_during_parsing_the_parsed_value_is_not_empty_but_we_cant_go_forward_then_the_parsing_is_unsuccessful() {
    let mut root = SuffixTable::new();
    let cp_1 = CompiledPatternBuilder::new()
                .literal("app")
                .build();
    let mut pattern = Pattern::with_random_uuid();
    pattern.set_pattern(cp_1);
    root.insert(pattern);

    assert_eq!(true, root.parse("apple").is_none());
}

#[test]
fn test_given_suffix_array_when_a_literal_entry_is_found_then_it_is_returned() {
    let mut root = SuffixTable::new();
    let cp_1 = CompiledPatternBuilder::new()
                .literal("app")
                .build();
    let mut pattern = Pattern::with_random_uuid();
    pattern.set_pattern(cp_1);
    root.insert(pattern);

    assert_eq!(true, root.parse("app").is_some());
}

#[test]
fn test_given_suffix_array_when_literals_are_inserted_then_it_can_find_the_string_with_the_longest_common_prefix() {
    let mut root = SuffixTable::new();
    let cp_1 = CompiledPatternBuilder::new()
                .literal("app")
                .build();
    let mut pattern = Pattern::with_random_uuid();
    pattern.set_pattern(cp_1);
    root.insert(pattern);

    assert_eq!("app", root.longest_common_prefix("app42").unwrap().literal());
}

#[test]
fn test_given_parser_when_it_receives_utf_8_strings_then_it_does_not_panic() {
    let pattern = "%{GREEDY}¡%{GREEDY}";
    let compiled_pattern = ::grammar::parser::pattern(pattern)
                  .expect("Failed to compile pattern when it includes UTF-8 multibyte characters");

    let mut pattern = Pattern::with_random_uuid();
    pattern.set_pattern(compiled_pattern);

    let mut root = SuffixTable::new();
    root.insert(pattern);

    assert_eq!(true, root.parse("micek ¡micek").is_some());
}
