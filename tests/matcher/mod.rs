extern crate actiondb;

use actiondb::Matcher;
use actiondb::matcher::matcher::builder::BuildError;

#[test]
fn test_given_pattern_file_when_its_syntax_is_ok_then_matcher_can_be_built_from_it() {
    let pattern_file_path = "tests/matcher/ssh_ok.pattern";
    let matcher = Matcher::from_file(pattern_file_path);
    assert_eq!(matcher.is_ok(), true);
}

#[test]
fn test_given_pattern_file_when_its_syntax_is_not_ok_then_matcher_cannot_be_built_from_it() {
    let pattern_file_path = "tests/matcher/ssh_wrong.pattern";
    match Matcher::from_file(pattern_file_path) {
        Err(BuildError::FromPlain(_)) => {},
        _ => unreachable!()
    }
}

#[test]
fn test_given_json_file_when_its_syntax_is_ok_then_matcher_can_be_built_from_it() {
    let pattern_file_path = "tests/matcher/ssh_ok.json";
    let matcher = Matcher::from_json_file(pattern_file_path);
    println!("{:?}", &matcher);
    matcher.ok().expect("Failed to create a Matched from a valid JSON pattern file");
}

#[test]
fn test_given_json_file_when_its_syntax_is_not_ok_then_matcher_cannot_be_built_from_it() {
    let pattern_file_path = "tests/matcher/ssh_wrong.json";
    let matcher = Matcher::from_json_file(pattern_file_path);
    matcher.err().expect("Failed to get an error when a Matcher is created from an invalid JSON file");
}

#[test]
fn test_given_non_existing_json_file_when_it_is_loaded_then_matcher_cannot_be_created_from_it() {
    let pattern_file_path = "tests/matcher/ssh_non_existing.json";
    let matcher = Matcher::from_json_file(pattern_file_path);
    matcher.err().expect("Failed to get an error when a Matcher is created from a non-existing JSON file");
}
