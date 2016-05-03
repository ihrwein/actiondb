use uuid::Uuid;

use std::error;
use std::fmt;

use super::TestMessage;

#[derive(Debug)]
pub enum Error {
    ValueNotMatch {
        pattern_uuid: String,
        key: String,
        expected_value: String,
        got_value: String,
    },
    KeyNotFound {
        pattern_uuid: String,
        key: String,
    },
    TestMessageDoesntMatch {
        pattern_uuid: String,
        message: String,
    },
    MatchedToOtherPattern {
        expected_uuid: String,
        got_uuid: String,
        message: String,
    },
    UnexpectedTags {
        pattern_uuid: String,
        expected: Option<Vec<String>>,
        got: Option<Vec<String>>,
    },
}

impl Error {
    pub fn value_not_match(pattern_uuid: &Uuid,
                           key: &str,
                           expected_value: &str,
                           got_value: &str)
                           -> Error {
        Error::ValueNotMatch {
            pattern_uuid: pattern_uuid.hyphenated().to_string(),
            key: key.to_owned(),
            expected_value: expected_value.to_owned(),
            got_value: got_value.to_owned(),
        }
    }

    pub fn key_not_found(pattern_uuid: &Uuid, key: &str) -> Error {
        Error::KeyNotFound {
            pattern_uuid: pattern_uuid.hyphenated().to_string(),
            key: key.to_owned(),
        }
    }

    pub fn test_message_does_not_match(pattern_uuid: &Uuid, test_msg: &TestMessage) -> Error {
        Error::TestMessageDoesntMatch {
            pattern_uuid: pattern_uuid.hyphenated().to_string(),
            message: test_msg.message().to_owned(),
        }
    }

    pub fn matched_to_other_pattern(expected_uuid: &Uuid,
                                    got_uuid: &Uuid,
                                    test_message: &str)
                                    -> Error {
        Error::MatchedToOtherPattern {
            expected_uuid: expected_uuid.hyphenated().to_string(),
            got_uuid: got_uuid.hyphenated().to_string(),
            message: test_message.to_owned(),
        }
    }

    pub fn unexpected_tags(pattern_uuid: &Uuid,
                           expected: Option<Vec<String>>,
                           got: Option<Vec<String>>)
                           -> Error {
        Error::UnexpectedTags {
            pattern_uuid: pattern_uuid.hyphenated().to_string(),
            expected: expected,
            got: got,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            Error::ValueNotMatch{ref pattern_uuid, ref key, ref expected_value, ref got_value} => {
                fmt.write_fmt(format_args!("A parsed value does not equal to its expected value: \
                                            uuid={} key={} expected={} got={}",
                                           pattern_uuid,
                                           key,
                                           expected_value,
                                           got_value))
            }
            Error::KeyNotFound{ref pattern_uuid, ref key} => {
                fmt.write_fmt(format_args!("A parsed key in not found among the expected ones: \
                                            uuid={} key={}",
                                           pattern_uuid,
                                           key))
            }
            Error::TestMessageDoesntMatch{ref pattern_uuid, ref message} => {
                fmt.write_fmt(format_args!("A test message did not match its pattern: uuid={} \
                                            message='{}'",
                                           pattern_uuid,
                                           message))
            }
            Error::MatchedToOtherPattern{ref expected_uuid, ref got_uuid, ref message} => {
                fmt.write_fmt(format_args!("The test message matched to an other pattern: \
                                            expected_uuid={} got_uuid={} test_message='{}'",
                                           expected_uuid,
                                           got_uuid,
                                           message))
            }
            Error::UnexpectedTags{ref pattern_uuid, ref expected, ref got} => {
                fmt.write_fmt(format_args!("Unexpected tags found either in the parse result or \
                                            among the expected ones: uuid={} expected: {:?} \
                                            got={:?}",
                                           pattern_uuid,
                                           expected,
                                           got))
            }
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::ValueNotMatch{..} => {
                "A parsed value does not equal to its expected value"
            }
            Error::KeyNotFound{..} => {
                "A parsed key in not found among the expected ones"
            }
            Error::TestMessageDoesntMatch{..} => {
                "A test message cannot be parsed but its pattern is inserted"
            }
            Error::MatchedToOtherPattern{..} => {
                "The test message matched to an other pattern"
            }
            Error::UnexpectedTags{..} => {
                "Unexpected tags found either in the parse result or among the expected ones"
            }
        }
    }
}
