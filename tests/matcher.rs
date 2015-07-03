extern crate actiondb;

use actiondb::{Matcher, BuildFromFileError};

#[test]
fn test_given_pattern_file_when_its_syntax_is_ok_then_matcher_can_be_built_from_it() {
    let pattern_file_path = "tests/ssh_ok.pattern";
    let matcher = Matcher::from_file(pattern_file_path);
    assert_eq!(matcher.is_ok(), true);
}

#[test]
fn test_given_pattern_file_when_its_syntax_is_not_ok_then_matcher_cannot_be_built_from_it() {
    let pattern_file_path = "tests/ssh_wrong.pattern";
    match Matcher::from_file(pattern_file_path) {
        Err(BuildFromFileError::PatternParseError(_)) => {},
        _ => unreachable!()
    }
}
