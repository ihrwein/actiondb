extern crate actiondb;

use actiondb::matcher::PatternLoader;
use actiondb::matcher::trie::factory::TrieMatcherFactory;

#[test]
fn test_given_json_file_when_its_syntax_is_ok_then_matcher_can_be_built_from_it() {
    let pattern_file_path = "tests/matcher/ssh_ok.json";
    let matcher = PatternLoader::from_file::<TrieMatcherFactory>(pattern_file_path);
    println!("{:?}", &matcher);
    matcher.ok().expect("Failed to create a Matched from a valid JSON pattern file");
}

#[test]
fn test_given_json_file_when_its_syntax_is_not_ok_then_matcher_cannot_be_built_from_it() {
    let pattern_file_path = "tests/matcher/ssh_wrong.json";
    let matcher = PatternLoader::from_file::<TrieMatcherFactory>(pattern_file_path);
    matcher.err()
           .expect("Failed to get an error when a Matcher is created from an invalid JSON file");
}

#[test]
fn test_given_non_existing_json_file_when_it_is_loaded_then_matcher_cannot_be_created_from_it() {
    let pattern_file_path = "tests/matcher/ssh_non_existing.json";
    let matcher = PatternLoader::from_file::<TrieMatcherFactory>(pattern_file_path);
    matcher.err()
           .expect("Failed to get an error when a Matcher is created from a non-existing JSON \
                    file");
}

#[test]
fn test_given_json_file_when_matcher_is_created_by_factory_then_the_right_file_type_is_used_based_on_the_extension
    () {
    let pattern_file_path = "tests/matcher/ssh_ok.json";
    let matcher = PatternLoader::from_file::<TrieMatcherFactory>(pattern_file_path);
    println!("{:?}", &matcher);
    matcher.ok().expect("Failed to create a Matcher from a valid JSON pattern file");
}

#[test]
fn test_given_json_file_when_the_tests_contain_tags_but_the_pattern_does_not_have_them_then_we_fail
    () {
    let pattern_file_path = "tests/matcher/ssh_tags_are_not_there.json";
    let matcher = PatternLoader::from_file::<TrieMatcherFactory>(pattern_file_path);
    matcher.err()
           .expect("Failed to get an error when the expected number of tags doesn't match with \
                    the got one");
}

#[test]
fn test_given_json_file_when_a_pattern_contains_test_tags_then_we_only_check_the_expected_ones() {
    let pattern_file_path = "tests/matcher/ssh_only_expected_tags_are_checked.json";
    let matcher = PatternLoader::from_file::<TrieMatcherFactory>(pattern_file_path);
    matcher.err().expect("We should only check the expected tags");
}

#[test]
fn test_given_json_file_when_a_pattern_contains_test_values_then_we_only_check_the_expected_ones
                                                                                                 () {
    let pattern_file_path = "tests/matcher/ssh_only_expected_values_are_checked.json";
    let matcher = PatternLoader::from_file::<TrieMatcherFactory>(pattern_file_path);
    println!("matcher: {:?}", &matcher);
    matcher.ok().expect("We should only check the expected tags");
}

#[test]
fn test_given_json_file_when_an_expected_value_is_not_found_then_we_fail() {
    let pattern_file_path = "tests/matcher/ssh_when_an_expected_value_is_not_found_we_fail.json";
    let matcher = PatternLoader::from_file::<TrieMatcherFactory>(pattern_file_path);
    println!("matcher: {:?}", &matcher);
    matcher.err().expect("An expected value was not found but we created the Matcher object");
}

#[test]
fn test_given_json_file_when_a_pattern_contains_cr_characters_then_we_handle_it_properly() {
    let pattern_file_path = "tests/matcher/ssh_we_can_parse_multiline_messages.json";
    let matcher = PatternLoader::from_file::<TrieMatcherFactory>(pattern_file_path);
    println!("matcher: {:?}", &matcher);
    let _ = matcher.ok().expect("We should be able to create a Matcher when a pattern has a CR \
                                 characer in it");
}

#[test]
fn test_given_json_file_when_we_check_the_test_messages_then_the_resulting_pattern_should_be_the_tested_one
    () {
    let pattern_file_path = "tests/matcher/pattern_validation_should_not_be_local.json";
    let matcher = PatternLoader::from_file::<TrieMatcherFactory>(pattern_file_path);
    println!("matcher: {:?}", &matcher);
    let _ = matcher.err()
                   .expect("The UUID of the resulting pattern should be the same as the freshly \
                            inserted one");
}
