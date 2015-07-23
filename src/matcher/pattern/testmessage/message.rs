use std::collections::BTreeMap;
use std::borrow::Borrow;
use super::Error;

#[derive(Clone, Debug)]
pub struct TestMessage {
    message: String,
    values: BTreeMap<String, String>
}

impl TestMessage {
    pub fn new(message: String, values: BTreeMap<String, String>) -> TestMessage {
        TestMessage{
            message: message,
            values: values
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn values(&self) -> &BTreeMap<String, String> {
        &self.values
    }

    pub fn test_pairs(&self, pairs: &[(&str, &str)]) -> Result<(), Error> {
        if pairs.len() != self.values().len() {
            Err(Error::invalid_length(self.values.len(), pairs.len()))
        } else  {
            self.test_pairs_values(pairs)
        }
    }

    pub fn test_pairs_values(& self, pairs: &[(&str, &str)]) -> Result<(), Error> {
        for &(key, value) in pairs {
            let expected_value = self.values().get(key).map(|x| x.borrow());
            if let Some(exp) = expected_value {
                if exp != value {
                    return Err(Error::value_not_match(key, exp, value));
                }
            } else {
                return Err(Error::key_not_found(key));
            }
        }
        Ok(())
    }
}
