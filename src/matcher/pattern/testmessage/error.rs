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
