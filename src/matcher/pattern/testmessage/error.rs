use std::error;
use std::fmt;

#[derive(Debug)]
pub enum TestPairsError {
    InvalidLength{expected: usize, got: usize},
    ValueNotMatch{key: String, expected_value: String, got_value: String},
    KeyNotFound{key: String}
}

impl TestPairsError {
    pub fn invalid_length(expected: usize, got: usize) -> TestPairsError {
        TestPairsError::InvalidLength{expected: expected, got: got}
    }

    pub fn value_not_match(key: &str, expected_value: &str, got_value: &str) -> TestPairsError {
        TestPairsError::ValueNotMatch{key: key.to_string(), expected_value: expected_value.to_string(), got_value: got_value.to_string()}
    }

    pub fn key_not_found(key: &str) -> TestPairsError {
        TestPairsError::KeyNotFound{key: key.to_string()}
    }
}

impl fmt::Display for TestPairsError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            &TestPairsError::InvalidLength{expected, got} => {
                fmt.write_fmt(format_args!("The number of parsed key-value pairs does not equal to their expected number: expected={} got={}", expected, got))
            },
            &TestPairsError::ValueNotMatch{ref key, ref expected_value, ref got_value} => {
                fmt.write_fmt(format_args!("A parsed value does not equal to its expected value: key={} expected={} got={}", key, expected_value, got_value))
            },
            &TestPairsError::KeyNotFound{ref key} => {
                fmt.write_fmt(format_args!("A parsed key in not found among the expected ones: key={}", key))
            }
        }
    }
}

