use uuid::Uuid;

use std::error;
use std::fmt;

use super::TestMessage;

#[derive(Debug)]
pub enum Error {
    ValueNotMatch{pattern_uuid: String, key: String, expected_value: String, got_value: String},
    KeyNotFound{pattern_uuid: String, key: String},
    TestMessageDoesntMatch{message: String},
    UnexpectedTags{expected: Option<Vec<String>>, got: Option<Vec<String>>}
}

impl Error {
    pub fn value_not_match(pattern_uuid: &Uuid, key: &str, expected_value: &str, got_value: &str) -> Error {
        Error::ValueNotMatch{pattern_uuid: pattern_uuid.to_hyphenated_string(), key: key.to_string(), expected_value: expected_value.to_string(), got_value: got_value.to_string()}
    }

    pub fn key_not_found(pattern_uuid: &Uuid, key: &str) -> Error {
        Error::KeyNotFound{pattern_uuid: pattern_uuid.to_hyphenated_string(), key: key.to_string()}
    }

    pub fn test_message_does_not_match(test_msg: &TestMessage) -> Error {
        Error::TestMessageDoesntMatch{message: test_msg.message().to_owned()}
    }

    pub fn unexpected_tags(expected: Option<Vec<String>>, got: Option<Vec<String>>) -> Error {
        Error::UnexpectedTags {
            expected: expected,
            got: got
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            &Error::ValueNotMatch{ref pattern_uuid, ref key, ref expected_value, ref got_value} => {
                fmt.write_fmt(format_args!("A parsed value does not equal to its expected value: uuid={} key={} expected={} got={}", pattern_uuid, key, expected_value, got_value))
            },
            &Error::KeyNotFound{ref pattern_uuid, ref key} => {
                fmt.write_fmt(format_args!("A parsed key in not found among the expected ones: uuid={} key={}", pattern_uuid, key))
            }
            &Error::TestMessageDoesntMatch{ref message} => {
                fmt.write_fmt(format_args!("A test message cannot be parsed but its pattern is inserted: message='{}'", message))
            },
            &Error::UnexpectedTags{ref expected, ref got} => {
                fmt.write_fmt(format_args!("Unexpected tags found either in the parse result or among the expected ones: expected: {:?} got={:?}", expected, got))
            }
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match self {
            &Error::ValueNotMatch{pattern_uuid: _, key: _, expected_value: _, got_value: _} => {
                "A parsed value does not equal to its expected value"
            },
            &Error::KeyNotFound{pattern_uuid: _, key: _} => {
                "A parsed key in not found among the expected ones"
            },
            &Error::TestMessageDoesntMatch{message: _} => {
                "A test message cannot be parsed but its pattern is inserted"
            },
            &Error::UnexpectedTags{expected: _, got: _} => {
                "Unexpected tags found either in the parse result or among the expected ones"
            }
        }
    }
}
