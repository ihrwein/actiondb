use std::error;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    InvalidLength{expected: usize, got: usize},
    ValueNotMatch{key: String, expected_value: String, got_value: String},
    KeyNotFound{key: String},
    TestMessageDoesntMatch,
    UnexpectedTags{expected: Option<Vec<String>>, got: Option<Vec<String>>}
}

impl Error {
    pub fn invalid_length(expected: usize, got: usize) -> Error {
        Error::InvalidLength{expected: expected, got: got}
    }

    pub fn value_not_match(key: &str, expected_value: &str, got_value: &str) -> Error {
        Error::ValueNotMatch{key: key.to_string(), expected_value: expected_value.to_string(), got_value: got_value.to_string()}
    }

    pub fn key_not_found(key: &str) -> Error {
        Error::KeyNotFound{key: key.to_string()}
    }

    pub fn test_message_does_not_match() -> Error {
        Error::TestMessageDoesntMatch
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
            &Error::InvalidLength{expected, got} => {
                fmt.write_fmt(format_args!("The number of parsed key-value pairs does not equal to their expected number: expected={} got={}", expected, got))
            },
            &Error::ValueNotMatch{ref key, ref expected_value, ref got_value} => {
                fmt.write_fmt(format_args!("A parsed value does not equal to its expected value: key={} expected={} got={}", key, expected_value, got_value))
            },
            &Error::KeyNotFound{ref key} => {
                fmt.write_fmt(format_args!("A parsed key in not found among the expected ones: key={}", key))
            }
            &Error::TestMessageDoesntMatch => {
                fmt.write_str("A test message cannot be parsed but its pattern is inserted")
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
            &Error::InvalidLength{expected: _, got: _} => {
                "The number of parsed key-value pairs does not equal to their expected number"
            },
            &Error::ValueNotMatch{key: _, expected_value: _, got_value: _} => {
                "A parsed value does not equal to its expected value"
            },
            &Error::KeyNotFound{key: _} => {
                "A parsed key in not found among the expected ones"
            },
            &Error::TestMessageDoesntMatch => {
                "A test message cannot be parsed but its pattern is inserted"
            },
            &Error::UnexpectedTags{expected: _, got: _} => {
                "Unexpected tags found either in the parse result or among the expected ones"
            }
        }
    }
}
